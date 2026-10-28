//! Writes each profile's module tree, and the module that declares the
//! trees.

use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::{Context, Result, bail};

use crate::{
    Codegen,
    emit::{self, GENERATED_HEADER},
    model::Model,
    naming::type_name,
};

/// What generation wrote, for a crate's own tooling such as test macros.
pub struct Generated {
    pub profiles: Vec<GeneratedProfile>,
}

pub struct GeneratedProfile {
    pub name: String,
    pub feature: Option<String>,
    /// The tree's module path in the crate, e.g. `apis::standard`.
    pub module: String,
    /// One per CRD, in IR order.
    pub resources: Vec<Resource>,
}

/// A CRD's root type and versions.
pub struct Resource {
    pub plural: String,
    /// The Rust name of the root type, e.g. `HttpRoute`.
    pub root: String,
    pub versions: Vec<ResourceVersion>,
}

pub struct ResourceVersion {
    pub name: String,
    /// The module the version's root type is in, relative to the tree, e.g.
    /// `apisx::v1alpha1`.
    pub module: String,
}

impl Codegen {
    /// Writes every profile's module tree to the crate's `src/`, and the
    /// parent modules that declare them, then formats them.
    pub fn emit(&self) -> Result<Generated> {
        let src = self.root.join(&self.config.crate_dir).join("src");
        let mut generated = Vec::new();
        let mut written = Vec::new();
        for (profile, ir) in self.config.profiles.iter().zip(&self.irs) {
            eprintln!("generating profile {name}", name = profile.name);
            let model = Model::new(&self.config, profile, ir).with_context(|| format!("profile {}", profile.name))?;
            let modules = emit::modules(&model).with_context(|| format!("profile {}", profile.name))?;
            written.extend(write_tree(&src, &profile.out, &self.tree_doc(&profile.crds), &modules)?);

            let mut resources = Vec::new();
            for crd in &ir.crds {
                let mut versions = Vec::new();
                for version in &crd.versions {
                    versions.push(ResourceVersion {
                        name: version.name.clone(),
                        module: self.config.module(&version.package)?.to_string(),
                    });
                }
                resources.push(Resource {
                    plural: crd.plural.clone(),
                    root: type_name(&crd.kind),
                    versions,
                });
            }
            generated.push(GeneratedProfile {
                name: profile.name.clone(),
                feature: profile.feature.clone(),
                module: profile.out.replace('/', "::"),
                resources,
            });
        }
        written.extend(self.write_parents(&src)?);
        rustfmt(&written)?;
        Ok(Generated { profiles: generated })
    }

    /// The docs of a profile's tree.
    fn tree_doc(&self, crds: &str) -> String {
        let module = &self.config.module;
        let version = self.version();
        format!(
            "//! The types of the CRDs in `{crds}` of `{module}` {version}, with one\n//! module per Go package.\n\n"
        )
    }

    /// Writes the modules that declare the profiles' trees: `apis.rs` for
    /// `apis/standard` and `apis/experimental`. A tree at the top of `src/`
    /// is declared by the crate itself.
    fn write_parents(&self, src: &Path) -> Result<Vec<PathBuf>> {
        let mut parents: BTreeMap<&str, Vec<_>> = BTreeMap::new();
        for profile in &self.config.profiles {
            match profile.out.rsplit_once('/') {
                Some((parent, _)) if parent.contains('/') => {
                    bail!(
                        "profile {name}: out may be at most two modules deep",
                        name = profile.name
                    );
                },
                Some((parent, module)) => parents.entry(parent).or_default().push((module, profile)),
                None => {},
            }
        }
        let mut written = Vec::new();
        for (parent, mut children) in parents {
            children.sort_by_key(|(module, _)| *module);
            let mut out = String::from(GENERATED_HEADER);
            for (module, profile) in children {
                if let Some(feature) = &profile.feature {
                    writeln!(
                        out,
                        "#[cfg(feature = \"{feature}\")]\n#[cfg_attr(docsrs, doc(cfg(feature = \"{feature}\")))]"
                    )?;
                }
                writeln!(out, "pub mod {module};")?;
            }
            let path = src.join(format!("{parent}.rs"));
            fs::write(&path, out).with_context(|| format!("failed to write {}", path.display()))?;
            written.push(path);
        }
        Ok(written)
    }
}

/// Writes a profile's tree: `<out>.rs` declaring its modules, and a file per
/// module under `<out>/`, after removing what was there.
fn write_tree(src: &Path, out: &str, doc: &str, modules: &[emit::Module]) -> Result<Vec<PathBuf>> {
    let dir = src.join(out);
    if dir.exists() {
        fs::remove_dir_all(&dir).with_context(|| format!("failed to remove {}", dir.display()))?;
    }

    // Each module's parents, e.g. `apisx` for `apisx::v1alpha1`, declare it.
    let mut children: BTreeMap<String, Vec<&str>> = BTreeMap::new();
    for module in modules {
        let mut parent = String::new();
        for part in module.path.split("::") {
            let siblings = children.entry(parent.clone()).or_default();
            if !siblings.contains(&part) {
                siblings.push(part);
            }
            parent = if parent.is_empty() {
                part.to_string()
            } else {
                format!("{parent}::{part}")
            };
        }
    }

    let mut written = Vec::new();
    let mut write = |path: PathBuf, source: String| -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&path, source).with_context(|| format!("failed to write {}", path.display()))?;
        written.push(path);
        Ok(())
    };
    let declarations = |module: &str| -> String {
        let mut out = String::new();
        for child in children.get(module).into_iter().flatten() {
            writeln!(out, "pub mod {child};").expect("writing to a String");
        }
        out
    };
    write(
        src.join(format!("{out}.rs")),
        format!("{GENERATED_HEADER}{doc}{decls}", decls = declarations("")),
    )?;
    for (parent, _) in children.iter().filter(|(parent, _)| !parent.is_empty()) {
        if !modules.iter().any(|module| module.path == *parent) {
            let path = dir.join(format!("{}.rs", parent.replace("::", "/")));
            write(path, format!("{GENERATED_HEADER}{decls}", decls = declarations(parent)))?;
        }
    }
    for module in modules {
        let path = dir.join(format!("{}.rs", module.path.replace("::", "/")));
        let decls = declarations(&module.path);
        write(path, format!("{source}{decls}", source = module.source))?;
    }
    Ok(written)
}

/// Formats generated files with the nightly rustfmt and the repository's
/// config.
fn rustfmt(files: &[PathBuf]) -> Result<()> {
    let status = Command::new("rustfmt")
        .args(["+nightly", "--edition", "2024"])
        .args(files)
        .status()
        .context("failed to run rustfmt")?;
    if !status.success() {
        bail!("rustfmt failed");
    }
    Ok(())
}
