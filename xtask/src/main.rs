#![deny(unsafe_code)]
#![warn(clippy::all, clippy::pedantic, rust_2018_idioms)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

use std::{
    collections::BTreeSet,
    env,
    fmt::Write,
    fs,
    path::Path,
    process::{self, Command, Stdio},
    thread,
};

use anyhow::{Context, Result, bail};
use proc_macro2::Span;
use serde::Deserialize;

use crate::shared_types::Table;

mod consolidate;
mod derive;
mod gofacts;
mod shared_types;

// -----------------------------------------------------------------------------
// Configuration
// -----------------------------------------------------------------------------

const APIS_DIR: &str = "gateway-api/src/apis";

/// Vendored upstream CRDs, installed by the integration tests.
const CRD_FIXTURES_DIR: &str = "gateway-api/tests/fixtures/crds";

/// Integration test bodies, shared across API versions and channels.
const TEST_SUITE_DIR: &str = "gateway-api/tests/integration/suite";

/// The `apis!` macro, in the support module shared by the test crates.
const TEST_APIS_FILE: &str = "gateway-api/tests/support/apis.rs";

const CRD_BASE_URL: &str = "https://raw.githubusercontent.com/kubernetes-sigs/gateway-api";

const CRD_PATH: &str = "config/crd";

const CRD_PREFIX_STANDARD: &str = "gateway.networking.k8s.io_";
const CRD_PREFIX_EXPERIMENTAL: &str = "gateway.networking.x-k8s.io_";

const KOPIUM_ARGS: &[&str] = &[
    "--schema=derived",
    "--derive=JsonSchema",
    "--derive=Default",
    "--derive=PartialEq",
    "--docs",
    "-f",
    "-",
];

const GENERATED_HEADER: &str = "// WARNING: generated file — do not edit\n";

// -----------------------------------------------------------------------------
// API channels
// -----------------------------------------------------------------------------

struct Api {
    name: &'static str,
    crd_prefix: &'static str,
    /// API version re-exported at the resource module root.
    root_version: &'static str,
}

impl Api {
    const fn standard(name: &'static str, root_version: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_STANDARD,
            root_version,
        }
    }

    const fn experimental(name: &'static str, root_version: &'static str) -> Self {
        Self {
            name,
            crd_prefix: CRD_PREFIX_EXPERIMENTAL,
            root_version,
        }
    }
}

struct Channel {
    name: &'static str,
    apis: &'static [Api],
    enum_defaults: &'static [(&'static str, &'static str)],
    conditions: &'static [(&'static str, &'static [&'static str])],
}

const STANDARD: Channel = Channel {
    name: "standard",
    apis: &[
        Api::standard("backendtlspolicies", "v1"),
        Api::standard("gatewayclasses", "v1"),
        Api::standard("gateways", "v1"),
        Api::standard("grpcroutes", "v1"),
        Api::standard("httproutes", "v1"),
        Api::standard("listenersets", "v1"),
        Api::standard("referencegrants", "v1"),
        Api::standard("tlsroutes", "v1"),
    ],
    enum_defaults: &[
        ("BackendTlsPolicyValidationSubjectAltNamesType", "Hostname"),
        ("GrpcRouteFilterType", "RequestHeaderModifier"),
        ("HttpPathModifierType", "ReplaceFullPath"),
        ("HttpRouteFilterType", "RequestHeaderModifier"),
    ],
    conditions: &[
        ("GatewayClassConditionType", &["Accepted"]),
        (
            "GatewayClassConditionReason",
            &["Accepted", "InvalidParameters", "Pending", "Unsupported", "Waiting"],
        ),
        ("GatewayConditionType", &["Programmed", "Accepted", "Ready"]),
        (
            "GatewayConditionReason",
            &[
                "Programmed",
                "Invalid",
                "NoResources",
                "AddressNotAssigned",
                "AddressNotUsable",
                "Accepted",
                "ListenersNotValid",
                "Pending",
                "UnsupportedAddress",
                "InvalidParameters",
                "Ready",
                "ListenersNotReady",
            ],
        ),
        (
            "ListenerConditionType",
            &["Conflicted", "Accepted", "ResolvedRefs", "Programmed", "Ready"],
        ),
        (
            "ListenerConditionReason",
            &[
                "HostnameConflict",
                "ProtocolConflict",
                "NoConflicts",
                "Accepted",
                "PortUnavailable",
                "UnsupportedProtocol",
                "ResolvedRefs",
                "InvalidCertificateRef",
                "InvalidRouteKinds",
                "RefNotPermitted",
                "Programmed",
                "Invalid",
                "Pending",
                "Ready",
            ],
        ),
    ],
};

const EXPERIMENTAL: Channel = Channel {
    name: "experimental",
    apis: &[
        Api::standard("backendtlspolicies", "v1"),
        Api::standard("gatewayclasses", "v1"),
        Api::standard("gateways", "v1"),
        Api::standard("grpcroutes", "v1"),
        Api::standard("httproutes", "v1"),
        Api::standard("listenersets", "v1"),
        Api::standard("referencegrants", "v1"),
        Api::standard("tcproutes", "v1alpha2"),
        Api::standard("tlsroutes", "v1"),
        Api::standard("udproutes", "v1alpha2"),
        Api::experimental("xbackendtrafficpolicies", "v1alpha1"),
        Api::experimental("xmeshes", "v1alpha1"),
    ],
    enum_defaults: &[
        ("BackendTlsPolicyValidationSubjectAltNamesType", "Hostname"),
        ("GrpcRouteFilterType", "RequestHeaderModifier"),
        ("HttpPathModifierType", "ReplaceFullPath"),
        ("HttpRouteExternalAuthProtocol", "Http"),
        ("HttpRouteFilterType", "RequestHeaderModifier"),
    ],
    conditions: &[
        ("GatewayClassConditionType", &["Accepted", "SupportedVersion"]),
        (
            "GatewayClassConditionReason",
            &[
                "Accepted",
                "InvalidParameters",
                "Pending",
                "Unsupported",
                "Waiting",
                "SupportedVersion",
                "UnsupportedVersion",
            ],
        ),
        ("GatewayConditionType", &["Programmed", "Accepted", "Ready"]),
        (
            "GatewayConditionReason",
            &[
                "Programmed",
                "Invalid",
                "NoResources",
                "AddressNotAssigned",
                "AddressNotUsable",
                "Accepted",
                "ListenersNotValid",
                "Pending",
                "UnsupportedAddress",
                "InvalidParameters",
                "Ready",
                "ListenersNotReady",
            ],
        ),
        (
            "ListenerConditionType",
            &["Conflicted", "Accepted", "ResolvedRefs", "Programmed", "Ready"],
        ),
        (
            "ListenerConditionReason",
            &[
                "HostnameConflict",
                "ProtocolConflict",
                "NoConflicts",
                "Accepted",
                "PortUnavailable",
                "UnsupportedProtocol",
                "ResolvedRefs",
                "InvalidCertificateRef",
                "InvalidRouteKinds",
                "RefNotPermitted",
                "Programmed",
                "Invalid",
                "Pending",
                "Ready",
            ],
        ),
    ],
};

// -----------------------------------------------------------------------------
// Entrypoint
// -----------------------------------------------------------------------------

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some("generate") = args.get(1).map(String::as_str) {
        let version = args
            .get(2)
            .context("usage: cargo xtask generate <version>\n  e.g. cargo xtask generate v1.4.1")?;
        generate(version)
    } else {
        eprintln!("usage: cargo xtask generate <version>");
        process::exit(1);
    }
}

// -----------------------------------------------------------------------------
// Generation orchestration
// -----------------------------------------------------------------------------

/// One channel's generated bindings, held in memory until every channel has
/// generated, so a failure leaves the tree untouched.
struct GeneratedChannel {
    channel: &'static Channel,
    resources: Vec<GeneratedResource>,
    /// The `common` module's items, once consolidated.
    common: Vec<syn::Item>,
}

/// One resource's generated bindings.
struct GeneratedResource {
    api: &'static Api,
    crd: FetchedCrd,
    /// The root type kopium generated, e.g. `HttpRoute`.
    root_type: String,
    /// One module per API version the CRD defines, in CRD order.
    versions: Vec<VersionModule>,
}

/// One API version's module: kopium's output, parsed.
struct VersionModule {
    api_version: String,
    /// Module docs and kopium's header comments, which `syn` doesn't keep.
    header: String,
    file: syn::File,
}

fn generate(version: &str) -> Result<()> {
    let mut channels = Vec::new();
    for channel in [&STANDARD, &EXPERIMENTAL] {
        channels.push(generate_channel(version, channel)?);
    }

    let facts = gofacts::generate(version)?;
    let table = derive_table(&channels, &facts)?;
    let mut matched = consolidate::Matched::default();
    for channel in &mut channels {
        consolidate_channel(channel, &table, &mut matched)?;
    }

    for channel in &channels {
        write_channel(version, channel)?;
    }
    write_apis_module()?;
    for channel in &channels {
        for resource in &channel.resources {
            write_test_stub(resource.api.name)?;
        }
    }
    fs::write(TEST_APIS_FILE, gen_test_apis(&channels))?;
    rustfmt()?;
    Ok(())
}

/// Derives the shared types table from kopium's output and the Go facts.
fn derive_table(channels: &[GeneratedChannel], facts: &[gofacts::Facts]) -> Result<Table> {
    let mut inputs = Vec::new();
    for generated in channels {
        let channel_name = generated.channel.name;
        let facts = facts
            .iter()
            .find(|facts| facts.channel == channel_name)
            .with_context(|| format!("no Go facts for the {channel_name} channel"))?;
        let mut modules = Vec::new();
        for GeneratedResource { api, crd, versions, .. } in &generated.resources {
            for module in versions {
                modules.push(derive::Module {
                    resource: api.name,
                    version: &module.api_version,
                    root: module.api_version == api.root_version,
                    group: &crd.spec.group,
                    kind: &crd.spec.names.kind,
                    file: &module.file,
                });
            }
        }
        inputs.push((facts, modules));
    }
    derive::derive(&inputs)
}

fn generate_channel(version: &str, channel: &'static Channel) -> Result<GeneratedChannel> {
    let channel_name = channel.name;
    let mut resources = Vec::new();

    // Fetch CRDs and run kopium once per API version
    for api in channel.apis {
        let api_name = api.name;
        let root_version = api.root_version;
        let crd = fetch_crd(version, channel_name, api)?;

        let Some(root) = crd.spec.versions.iter().find(|v| v.name == root_version) else {
            bail!("{channel_name} {api_name}: root version {root_version} is not defined by the CRD");
        };
        if !root.served {
            bail!("{channel_name} {api_name}: root version {root_version} is not served by the CRD");
        }

        let mut root_types = BTreeSet::new();
        let mut versions = Vec::new();
        for crd_version in &crd.spec.versions {
            let module = generate_version(version, channel, api, &crd, crd_version)?;
            let api_version = &module.api_version;
            let root_type =
                root_type(&module.file).with_context(|| format!("{channel_name} {api_name} {api_version}"))?;
            root_types.insert(root_type);
            versions.push(module);
        }
        let root_type = match root_types.pop_first() {
            Some(root_type) if root_types.is_empty() => root_type,
            _ => bail!("{channel_name} {api_name}: API versions don't share one root type"),
        };

        resources.push(GeneratedResource {
            api,
            crd,
            root_type,
            versions,
        });
    }

    Ok(GeneratedChannel {
        channel,
        resources,
        common: Vec::new(),
    })
}

/// Moves a channel's shared types into its `common` module, then adds the
/// enum defaults, which are keyed by the consolidated names.
fn consolidate_channel<'t>(
    generated: &mut GeneratedChannel,
    table: &'t Table,
    matched: &mut consolidate::Matched<'t>,
) -> Result<()> {
    let channel = generated.channel;
    let channel_name = channel.name;

    let mut defaults = BTreeSet::new();
    if let Some((name, _)) = channel.enum_defaults.iter().find(|(name, _)| !defaults.insert(name)) {
        bail!("{channel_name}: enum default for {name} is listed twice");
    }

    // Root versions first, so a shared type takes its docs from one
    let mut modules = Vec::new();
    for resource in &mut generated.resources {
        let api_name = resource.api.name;
        let root_version = resource.api.root_version;
        let mut versions: Vec<_> = resource.versions.iter_mut().collect();
        versions.sort_by_key(|module| module.api_version != root_version);
        for module in versions {
            modules.push(consolidate::Module {
                resource: api_name.to_string(),
                version: module.api_version.clone(),
                root: module.api_version == root_version,
                file: &mut module.file,
            });
        }
    }

    let consolidated = consolidate::consolidate(channel_name, table, &mut modules, matched)
        .with_context(|| format!("{channel_name}: consolidating shared types"))?;
    let mut common = gen_common_prelude();
    common.extend(consolidated.common);

    let mut unused: BTreeSet<&str> = channel.enum_defaults.iter().map(|(name, _)| *name).collect();
    for module in &mut modules {
        add_enum_defaults(&mut module.file.items, channel.enum_defaults, &mut unused);
    }
    add_enum_defaults(&mut common, channel.enum_defaults, &mut unused);
    if let Some(name) = unused.first() {
        bail!("{channel_name}: enum default for {name} matches no generated enum");
    }

    generated.common = common;
    Ok(())
}

/// Replaces a channel's bindings and CRD fixtures with what was generated.
fn write_channel(bundle: &str, generated: &GeneratedChannel) -> Result<()> {
    let GeneratedChannel {
        channel,
        resources,
        common,
    } = generated;
    let channel_name = channel.name;
    let dir = format!("{APIS_DIR}/{channel_name}");
    let fixtures_dir = format!("{CRD_FIXTURES_DIR}/{channel_name}");

    // Clean and recreate
    for dir in [&dir, &fixtures_dir] {
        let path = Path::new(dir);
        if path.exists() {
            fs::remove_dir_all(path)?;
        }
        fs::create_dir_all(path)?;
    }

    for GeneratedResource { api, crd, versions, .. } in resources {
        let api_name = api.name;
        fs::write(
            format!(
                "{fixtures_dir}/{crd_prefix}{api_name}.yaml",
                crd_prefix = api.crd_prefix
            ),
            gen_crd_fixture(bundle, crd),
        )?;

        fs::create_dir_all(format!("{dir}/{api_name}"))?;
        for module in versions {
            let api_version = &module.api_version;
            fs::write(
                format!("{dir}/{api_name}/{api_version}.rs"),
                print_version_module(module),
            )?;
        }
        fs::write(format!("{dir}/{api_name}.rs"), gen_resource_module(api, &crd.spec))?;
    }

    fs::write(
        format!("{dir}/common.rs"),
        format!("{GENERATED_HEADER}{COMMON_DOC}{}", print_items(common)),
    )?;
    fs::write(format!("{dir}.rs"), gen_channel_module(channel))?;
    fs::write(format!("{dir}/constants.rs"), gen_constants(channel.conditions))?;
    Ok(())
}

/// Runs kopium for one API version of a CRD and parses its output.
fn generate_version(
    bundle: &str,
    channel: &Channel,
    api: &Api,
    crd: &FetchedCrd,
    crd_version: &CrdVersion,
) -> Result<VersionModule> {
    let channel_name = channel.name;
    let api_name = api.name;
    let api_version = &crd_version.name;

    eprintln!("generating {channel_name} api {api_name} {api_version}");
    let kopium_rs = kopium(&crd.yaml, api_version)
        .with_context(|| format!("kopium failed for {channel_name} {api_name} {api_version}"))?;
    let (kopium_header, file) =
        parse_kopium(&kopium_rs).with_context(|| format!("{channel_name} {api_name} {api_version}"))?;

    let mut header = gen_version_doc(bundle, channel, &crd.spec, crd_version);
    header.push_str(kopium_header);
    Ok(VersionModule {
        api_version: api_version.clone(),
        header,
        file,
    })
}

/// Splits kopium's output into its header comments and the parsed items.
/// `syn` drops ordinary comments, so the header is kept as text, and any
/// other ordinary comment is an error rather than lost.
fn parse_kopium(kopium_rs: &str) -> Result<(&str, syn::File)> {
    let is_comment = |line: &str| line.starts_with("//") && !line.starts_with("///") && !line.starts_with("//!");
    let header_len = kopium_rs
        .split_inclusive('\n')
        .take_while(|line| is_comment(line) || line.trim().is_empty())
        .map(str::len)
        .sum();
    let (header, body) = kopium_rs.split_at(header_len);
    if let Some(line) = body.lines().find(|line| is_comment(line.trim_start())) {
        bail!("kopium output has a comment outside its header, which would be lost: {line}");
    }
    let file = syn::parse_file(body).context("failed to parse kopium output")?;
    Ok((header, file))
}

/// Prints a version module: its header, then its items.
fn print_version_module(module: &VersionModule) -> String {
    let VersionModule { header, file, .. } = module;
    format!("{header}{items}", items = print_items(&file.items))
}

/// Prints items one at a time, so they can be separated by blank lines, which
/// `prettyplease` doesn't add.
fn print_items(items: &[syn::Item]) -> String {
    items
        .iter()
        .map(|item| {
            prettyplease::unparse(&syn::File {
                shebang: None,
                frontmatter: None,
                attrs: Vec::new(),
                items: vec![item.clone()],
            })
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// The path a channel's modules are public at.
fn public_path(channel: &str) -> String {
    if channel == STANDARD.name {
        "gateway_api".to_string()
    } else {
        format!("gateway_api::{channel}")
    }
}

/// Reads the root type name from kopium's `#[kube(group = ...)]` attribute:
/// `root` when kopium renamed the kind (`HTTPRoute` to `HttpRoute`), else
/// `kind`.
fn root_type(file: &syn::File) -> Result<String> {
    let attrs = file.items.iter().filter_map(|item| match item {
        syn::Item::Struct(item) => Some(&item.attrs),
        _ => None,
    });
    for attr in attrs.flatten().filter(|attr| attr.path().is_ident("kube")) {
        let (mut root, mut kind, mut group) = (None, None, false);
        attr.parse_nested_meta(|meta| {
            if !meta.input.peek(syn::Token![=]) {
                return Ok(());
            }
            let value: syn::LitStr = meta.value()?.parse()?;
            if meta.path.is_ident("root") {
                root = Some(value.value());
            } else if meta.path.is_ident("kind") {
                kind = Some(value.value());
            } else if meta.path.is_ident("group") {
                group = true;
            }
            Ok(())
        })?;
        if group {
            return root.or(kind).context("no `root` or `kind` in `#[kube(group = ...)]`");
        }
    }
    bail!("no `#[kube(group = ...)]` attribute in kopium output")
}

fn write_apis_module() -> Result<()> {
    let out = format!(
        "{GENERATED_HEADER}\
         #[cfg(feature = \"experimental\")]\n\
         #[cfg_attr(docsrs, doc(cfg(feature = \"experimental\")))]\n\
         pub mod experimental;\n\
         pub mod standard;\n"
    );

    fs::write(format!("{APIS_DIR}.rs"), out)?;
    Ok(())
}

/// Formats the generated bindings: rustfmt follows the `mod` declarations from
/// their root module into every generated file.
fn rustfmt() -> Result<()> {
    let status = Command::new("rustfmt")
        .args(["+nightly", &format!("{APIS_DIR}.rs")])
        .status()?;
    if !status.success() {
        bail!("rustfmt failed");
    }
    Ok(())
}

// -----------------------------------------------------------------------------
// CRD fetching and kopium conversion
// -----------------------------------------------------------------------------

/// Fetches a URL and returns the response body as a string.
fn fetch_url(url: &str) -> Result<String> {
    ureq::get(url)
        .call()
        .with_context(|| format!("failed to fetch {url}"))?
        .into_body()
        .read_to_string()
        .with_context(|| format!("failed to read response from {url}"))
}

/// A fetched CRD: where it came from, its YAML for kopium, and the parsed
/// parts of its spec.
struct FetchedCrd {
    url: String,
    yaml: String,
    spec: CrdSpec,
}

fn fetch_crd(version: &str, channel: &str, api: &Api) -> Result<FetchedCrd> {
    let Api { name, crd_prefix, .. } = api;
    let url = format!("{CRD_BASE_URL}/{version}/{CRD_PATH}/{channel}/{crd_prefix}{name}.yaml");

    let yaml = fetch_url(&url)?;
    let crd: Crd = serde_saphyr::from_str(&yaml).with_context(|| format!("failed to parse {url}"))?;
    Ok(FetchedCrd {
        url,
        yaml,
        spec: crd.spec,
    })
}

/// Runs kopium on a CRD for a single API version.
fn kopium(crd_yaml: &str, api_version: &str) -> Result<String> {
    let mut kopium = Command::new("kopium")
        .args(KOPIUM_ARGS)
        .args(["--api-version", api_version])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("failed to spawn kopium — is it installed?")?;

    // Write CRD YAML to kopium stdin from another thread while collecting its
    // output, so an early exit is reported with kopium's stderr rather than as
    // a broken pipe. The writer drops stdin when done, closing the pipe.
    let mut stdin = kopium.stdin.take().expect("stdin was piped");
    let (written, output) = thread::scope(|s| {
        let writer = s.spawn(move || {
            use std::io::Write;
            stdin.write_all(crd_yaml.as_bytes())
        });
        let output = kopium.wait_with_output();
        (writer.join().expect("kopium stdin writer panicked"), output)
    });
    let output = output.context("kopium did not exit cleanly")?;

    if !output.status.success() {
        let status = output.status;
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("kopium exited with {status}: {stderr}");
    }
    written.context("failed to write CRD to kopium stdin")?;

    Ok(String::from_utf8(output.stdout)?)
}

// -----------------------------------------------------------------------------
// Channel module generation
// -----------------------------------------------------------------------------

fn gen_channel_module(channel: &Channel) -> String {
    let mut out = String::from(GENERATED_HEADER);
    for Api { name, .. } in channel.apis {
        writeln!(out, "pub mod {name};").unwrap();
    }
    out.push_str("pub mod common;\n");
    out.push_str("pub mod constants;\n");
    out
}

// -----------------------------------------------------------------------------
// Resource and API version module generation
// -----------------------------------------------------------------------------

/// The parts of a CRD the generator needs.
#[derive(Deserialize)]
struct Crd {
    spec: CrdSpec,
}

#[derive(Deserialize)]
struct CrdSpec {
    group: String,
    names: CrdNames,
    versions: Vec<CrdVersion>,
}

#[derive(Deserialize)]
struct CrdNames {
    kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct CrdVersion {
    name: String,
    served: bool,
    storage: bool,
    #[serde(default)]
    deprecated: bool,
    deprecation_warning: Option<String>,
}

/// Generates the resource module, which declares one submodule per API version
/// and re-exports the root version.
fn gen_resource_module(api: &Api, spec: &CrdSpec) -> String {
    let root_version = api.root_version;
    let kind = &spec.names.kind;
    let group = &spec.group;

    let mut out = format!(
        "{GENERATED_HEADER}\
         //! `{kind}` (`{group}`).\n\
         //!\n\
         //! Each API version the CRD defines is a submodule. This module re-exports [`{root_version}`].\n\
         \n"
    );
    for CrdVersion { name, .. } in &spec.versions {
        writeln!(out, "pub mod {name};").unwrap();
    }
    writeln!(out, "#[doc(inline)]\npub use {root_version}::*;").unwrap();
    out
}

/// Generates the module docs for an API version, stating its status in the CRD.
fn gen_version_doc(bundle: &str, channel: &Channel, spec: &CrdSpec, v: &CrdVersion) -> String {
    let yes_no = |b: bool| if b { "yes" } else { "no" };
    let channel_name = channel.name;
    let kind = &spec.names.kind;
    let group = &spec.group;
    let api_version = &v.name;
    let served = yes_no(v.served);
    let storage = yes_no(v.storage);
    let deprecated = match (&v.deprecation_warning, v.deprecated) {
        (Some(warning), true) => {
            let warning = warning.split_whitespace().collect::<Vec<_>>().join(" ");
            format!("yes ({warning})")
        },
        _ => yes_no(v.deprecated).to_string(),
    };

    format!(
        "//! `{kind}` API version `{group}/{api_version}`.\n\
         //!\n\
         //! In the Gateway API {bundle} {channel_name} channel CRD:\n\
         //!\n\
         //! - served: {served}\n\
         //! - storage: {storage}\n\
         //! - deprecated: {deprecated}\n\
         \n"
    )
}

/// Adds a `Default` impl after each enum in `items` that kopium cannot derive
/// one for, removing each matched entry from `unused`.
fn add_enum_defaults(
    items: &mut Vec<syn::Item>,
    defaults: &[(&'static str, &str)],
    unused: &mut BTreeSet<&'static str>,
) {
    for (name, default) in defaults {
        let Some(at) = items
            .iter()
            .position(|item| matches!(item, syn::Item::Enum(item) if item.ident == name))
        else {
            continue;
        };
        unused.remove(name);
        let name = syn::Ident::new(name, Span::call_site());
        let default = syn::Ident::new(default, Span::call_site());
        items.insert(
            at + 1,
            syn::parse_quote! {
                impl Default for #name {
                    fn default() -> Self {
                        #name::#default
                    }
                }
            },
        );
    }
}

// -----------------------------------------------------------------------------
// common.rs generation
// -----------------------------------------------------------------------------

const COMMON_DOC: &str = "//! Types that upstream shares between resources, each named after its upstream Go\n\
                          //! type.\n\
                          \n";

/// The imports the `common` module's types use, as kopium writes them.
fn gen_common_prelude() -> Vec<syn::Item> {
    vec![
        syn::parse_quote! {
            #[allow(unused_imports)]
            mod prelude {
                pub use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;
                pub use schemars::JsonSchema;
                pub use serde::{Deserialize, Serialize};
                pub use std::collections::BTreeMap;
            }
        },
        syn::parse_quote!(
            use self::prelude::*;
        ),
    ]
}

// -----------------------------------------------------------------------------
// Test fixture, `apis!` and stub generation
// -----------------------------------------------------------------------------

/// Generates a vendored CRD fixture: the upstream YAML, unchanged, under a
/// comment saying where it came from.
fn gen_crd_fixture(bundle: &str, crd: &FetchedCrd) -> String {
    let FetchedCrd { url, yaml, .. } = crd;
    format!(
        "# WARNING: generated file — do not edit\n\
         # Gateway API {bundle} upstream CRD, vendored unchanged below this header by\n\
         # `cargo xtask generate {bundle}` for the integration tests. Source:\n\
         # {url}\n\
         {yaml}"
    )
}

/// Generates the `apis!` macro, which calls a callback macro once per channel
/// with each resource's root type and every API version its CRD defines.
fn gen_test_apis(channels: &[GeneratedChannel]) -> String {
    let mut out = format!(
        "{GENERATED_HEADER}\
         \n\
         /// Calls `$callback!` once per channel, with each resource's root type\n\
         /// and every API version its CRD defines, e.g.\n\
         /// `$callback!(standard {{ httproutes: HttpRoute [v1, v1beta1] ... }})`.\n\
         /// Every channel is passed in every build: `experimental` comes with\n\
         /// `#[cfg(feature = \"experimental\")]`, for the callback to put on\n\
         /// whatever uses the channel's types.\n\
         #[macro_export]\n\
         macro_rules! apis {{\n    \
             ($callback:ident) => {{\n"
    );
    for GeneratedChannel { channel, resources, .. } in channels {
        let channel_name = channel.name;
        let cfg = if channel_name == EXPERIMENTAL.name {
            "#[cfg(feature = \"experimental\")] "
        } else {
            ""
        };
        writeln!(out, "        $callback!({cfg}{channel_name} {{").unwrap();
        for GeneratedResource {
            api,
            root_type,
            versions,
            ..
        } in resources
        {
            let name = api.name;
            let versions = versions
                .iter()
                .map(|module| module.api_version.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            writeln!(out, "            {name}: {root_type} [{versions}]").unwrap();
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
         // `harness::crud`, like the other bodies in this directory.\n\
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

// -----------------------------------------------------------------------------
// constants.rs generation
// -----------------------------------------------------------------------------

fn gen_constants(conditions: &[(&str, &[&str])]) -> String {
    let mut out = String::from(GENERATED_HEADER);

    for (name, variants) in conditions {
        // Enum definition
        write!(out, "\n#[derive(Debug, PartialEq, Eq)]\npub enum {name} {{\n").unwrap();
        for v in *variants {
            writeln!(out, "    {v},").unwrap();
        }
        out.push_str("}\n");

        // Display impl
        write!(
            out,
            "\nimpl std::fmt::Display for {name} {{\n    \
                 fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {{\n        \
                     write!(f, \"{{:?}}\", self)\n    \
                 }}\n\
             }}\n"
        )
        .unwrap();
    }

    out
}
