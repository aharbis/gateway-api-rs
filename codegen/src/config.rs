//! A crate's codegen config, `<crate>/codegen/config.yaml`. gofacts reads the
//! same file for the keys that concern the Go side.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// The name of the one profile a config without `profiles` has.
pub const DEFAULT_PROFILE: &str = "default";

/// A crate's codegen config. Unknown keys are errors.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// The Go module the types come from, pinned by the `go.mod` next to the
    /// config.
    pub module: String,
    /// The crate's directory, relative to the repository root.
    #[serde(rename = "crate")]
    pub crate_dir: PathBuf,
    /// The Go packages to read, relative to the module, and the Rust module
    /// path each becomes, e.g. `apisx/v1alpha1: apisx::v1alpha1`.
    pub packages: BTreeMap<String, String>,
    /// One generated module tree each. Without them, `crds` and `out` make
    /// one profile named [`DEFAULT_PROFILE`].
    #[serde(default)]
    pub profiles: Vec<Profile>,
    #[serde(default)]
    crds: Option<String>,
    #[serde(default)]
    out: Option<String>,
    /// gofacts applies them; they're only checked here.
    #[serde(default)]
    pub tags: Option<Tags>,
    /// Files whose types are generated even where no CRD uses them.
    #[serde(default)]
    pub library_files: Vec<String>,
    /// Rust paths for Go types from other modules, by package and type
    /// name.
    #[serde(default)]
    pub external: BTreeMap<String, BTreeMap<String, String>>,
    /// Where gofacts copies the CRDs, relative to the crate, one directory
    /// per profile.
    #[serde(default)]
    pub fixtures: Option<PathBuf>,
}

/// One generated module tree, from one CRD directory.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    /// The CRD directory, relative to the module.
    pub crds: String,
    /// The module tree, relative to the crate's `src/`, e.g.
    /// `apis/experimental` for `crate::apis::experimental`.
    pub out: String,
    /// The Cargo feature that gates the tree.
    #[serde(default)]
    pub feature: Option<String>,
}

/// The tag rules, which gofacts applies. See `codegen/README.md`.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Tags {
    pub prefix: String,
    pub rules: Vec<TagRule>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TagRule {
    pub tag: String,
    #[serde(default)]
    pub drop: Option<Drop>,
    #[serde(default, rename = "in")]
    pub in_: Option<In>,
    #[serde(default)]
    pub r#override: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Drop {
    Item,
    Block,
}

/// A list of profiles, or all of them.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum In {
    All(AllProfiles),
    Profiles(Vec<String>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AllProfiles {
    All,
}

impl Config {
    /// Reads and checks a config file.
    pub fn load(path: &Path) -> Result<Self> {
        let path_display = path.display();
        let yaml = fs::read_to_string(path).with_context(|| format!("failed to read {path_display}"))?;
        Self::from_yaml(&yaml).with_context(|| format!("{path_display}"))
    }

    /// Parses and checks a config.
    pub fn from_yaml(yaml: &str) -> Result<Self> {
        let mut config: Self = serde_saphyr::from_str(yaml).context("failed to parse the config")?;
        config.check()?;
        Ok(config)
    }

    fn check(&mut self) -> Result<()> {
        match (self.profiles.is_empty(), self.crds.take(), self.out.take()) {
            (true, Some(crds), Some(out)) => self.profiles.push(Profile {
                name: DEFAULT_PROFILE.to_string(),
                crds,
                out,
                feature: None,
            }),
            (true, ..) => bail!("without profiles, crds and out are required"),
            (false, None, None) => {},
            (false, ..) => bail!("crds and out go in each profile when there are profiles"),
        }
        let mut names = BTreeSet::new();
        for profile in &self.profiles {
            let name = &profile.name;
            if !names.insert(name.as_str()) {
                bail!("profile {name} is defined twice");
            }
            if profile.out.is_empty() || profile.out.split('/').any(|part| part.is_empty() || part == "..") {
                bail!(
                    "profile {name}: out {out:?} isn't a relative module path",
                    out = profile.out
                );
            }
        }
        Ok(())
    }

    /// The Rust module path for a Go package, relative to a profile's tree.
    pub fn module(&self, package: &str) -> Result<&str> {
        self.packages
            .get(package)
            .map(String::as_str)
            .with_context(|| format!("package {package} isn't in the config"))
    }

    /// The Rust path for a Go type from another module, by its IR key.
    pub fn external(&self, key: &str) -> Option<&str> {
        let (package, name) = key.rsplit_once('.')?;
        self.external.get(package)?.get(name).map(String::as_str)
    }
}
