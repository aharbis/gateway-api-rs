#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

use std::{env, fmt::Write, fs, path::Path, process};

use anyhow::Result;
use gateway_api_codegen::{Codegen, Generated, Resource};

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

/// The crate's codegen directory: its config, Go module pin and IR.
const CODEGEN_DIR: &str = "gateway-api/codegen";

/// Integration test bodies, shared across API versions and channels.
const TEST_SUITE_DIR: &str = "gateway-api/tests/integration/suite";

/// The `apis!` macro, in the support module shared by the test crates.
const TEST_APIS_FILE: &str = "gateway-api/tests/support/apis.rs";

const GENERATED_HEADER: &str = "// WARNING: generated file — do not edit\n";

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some("generate") = args.get(1).map(String::as_str) {
        generate()
    } else {
        eprintln!("usage: cargo xtask generate");
        process::exit(1);
    }
}

/// Reads the Go source into the IR with gofacts, generates the crate from
/// it, then writes the test support that lists what was generated.
fn generate() -> Result<()> {
    let root = env::current_dir()?;
    let codegen = Codegen::generate_ir(&root, Path::new(CODEGEN_DIR))?;
    let generated = codegen.emit()?;
    for profile in &generated.profiles {
        for resource in &profile.resources {
            write_test_stub(&resource.plural)?;
        }
    }
    fs::write(TEST_APIS_FILE, gen_test_apis(&generated))?;
    Ok(())
}

// -----------------------------------------------------------------------------
// `apis!` and stub generation
// -----------------------------------------------------------------------------

/// Generates the `apis!` macro, which calls a callback macro once per channel
/// with each resource's root type, every API version its CRD defines and the
/// module the version's root type is in.
fn gen_test_apis(generated: &Generated) -> String {
    let mut out = format!(
        "{GENERATED_HEADER}\
         \n\
         /// Calls `$callback!` once per channel, with each resource's root type,\n\
         /// every API version its CRD defines and the module of the version's\n\
         /// root type in the channel, e.g.\n\
         /// `$callback!(standard {{ httproutes: HttpRoute [v1 => v1, v1beta1 => v1beta1] ... }})`.\n\
         /// Every channel is passed in every build: one behind a feature comes\n\
         /// with its `#[cfg(feature = ...)]`, for the callback to put on\n\
         /// whatever uses the channel's types.\n\
         #[macro_export]\n\
         macro_rules! apis {{\n    \
             ($callback:ident) => {{\n"
    );
    for profile in &generated.profiles {
        let name = &profile.name;
        let cfg = match &profile.feature {
            Some(feature) => format!("#[cfg(feature = \"{feature}\")] "),
            None => String::new(),
        };
        writeln!(out, "        $callback!({cfg}{name} {{").unwrap();
        for Resource { plural, root, versions } in &profile.resources {
            let versions: Vec<String> = versions
                .iter()
                .map(|version| format!("{name} => {module}", name = version.name, module = version.module))
                .collect();
            let versions = versions.join(", ");
            writeln!(out, "            {plural}: {root} [{versions}]").unwrap();
        }
        out.push_str("        });\n");
    }
    out.push_str("    };\n}\n");
    out
}

/// Writes a `<resource>.rs` integration test body whose test fails with
/// `todo!` if there's none, as for a newly added CRD. The integration tests
/// use it for every API version of the resource unless their `body!` routes
/// a version elsewhere.
fn write_test_stub(resource: &str) -> Result<()> {
    let path = Path::new(TEST_SUITE_DIR).join(format!("{resource}.rs"));
    if !path.exists() {
        eprintln!("writing integration test stub {path}", path = path.display());
        fs::write(&path, gen_test_stub(resource))?;
    }
    Ok(())
}

fn gen_test_stub(resource: &str) -> String {
    format!(
        "// TODO: written by `cargo xtask generate` because `{resource}` had no test\n\
         // body. Build a valid object from `self::api` and check it with\n\
         // `common::crud`, like the other bodies in this directory.\n\
         \n\
         #[allow(unused_imports)] // until the test uses `api`\n\
         use self::api as _;\n\
         \n\
         #[ignore]\n\
         #[tokio::test]\n\
         async fn crud() {{\n    \
             todo!(\"integration test for {resource}\")\n\
         }}\n"
    )
}
