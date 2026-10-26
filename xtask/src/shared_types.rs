//! Which kopium types are one upstream Go type, and which types that share a
//! shape are distinct upstream. The CRDs don't name their types, so these are
//! kept by hand from the Go source in `xtask/shared-types.yaml`. See
//! `docs/specs/shared-types.md`.

use std::collections::BTreeSet;

use anyhow::{Context, Result, bail};
use heck::ToUpperCamelCase;
use serde::Deserialize;

const TABLE_FILE: &str = "xtask/shared-types.yaml";

/// Embedded, so the xtask rebuilds when the table changes.
const TABLE_YAML: &str = include_str!("../shared-types.yaml");

/// The hand-kept table.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Table {
    /// In table order, which is the order types are generated in.
    pub shared_types: Vec<SharedType>,
    /// Groups of types that share a shape but are distinct upstream.
    pub distinct: Vec<Vec<String>>,
    /// The types non-root API versions declare themselves.
    pub per_version: Vec<PerVersion>,
}

impl Table {
    pub fn load() -> Result<Self> {
        Self::parse(TABLE_YAML).with_context(|| format!("invalid {TABLE_FILE}"))
    }

    /// Parses a table, failing on a Go name listed twice.
    fn parse(yaml: &str) -> Result<Self> {
        let table: Self = serde_saphyr::from_str(yaml)?;
        let mut seen = BTreeSet::new();
        if let Some(entry) = table.shared_types.iter().find(|entry| !seen.insert(&entry.go)) {
            bail!("shared type {go} is listed twice", go = entry.go);
        }
        Ok(table)
    }
}

/// The types one non-root API version of a resource declares itself in Go,
/// rather than as aliases of the root version's: a defined type
/// (`type X v1.X`) or a struct of its own. Named by their kopium names.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PerVersion {
    pub resource: String,
    pub version: String,
    pub types: Vec<String>,
}

/// Where a shared type's one definition is: `common`, `k8s`, or a resource's
/// name.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(from = "String")]
pub enum Placement {
    /// The channel's `common` module.
    Common,
    /// `k8s-openapi`, which defines the Kubernetes types kopium doesn't
    /// recognise. Nothing is generated.
    K8s,
    /// The resource's root version module, for a type only it uses.
    Resource(String),
}

impl From<String> for Placement {
    fn from(name: String) -> Self {
        match name.as_str() {
            "common" => Self::Common,
            "k8s" => Self::K8s,
            _ => Self::Resource(name),
        }
    }
}

/// An upstream Go type that kopium generates at more than one path.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SharedType {
    /// The Go type's name.
    pub go: String,
    /// The Rust name, where kopium's casing of the Go name reads badly.
    #[serde(default)]
    pub rust: Option<String>,
    pub placement: Placement,
    /// Every kopium name the type replaces, in any channel or API version.
    pub kopium: Vec<String>,
}

impl SharedType {
    /// The Rust name: the Go name with kopium's casing, e.g.
    /// `HTTPHeaderFilter` becomes `HttpHeaderFilter`, unless the table gives
    /// one.
    pub fn rust_name(&self) -> String {
        self.rust.clone().unwrap_or_else(|| self.go.to_upper_camel_case())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn table_loads() {
        let table = Table::load().unwrap();
        assert_eq!(table.shared_types[0].go, "ParentReference");
    }

    #[test]
    fn repeated_go_names_fail() {
        let yaml = "shared_types:\n  \
                      - {go: Fraction, placement: common, kopium: [A]}\n  \
                      - {go: Fraction, placement: common, kopium: [B]}\n\
                    distinct: []\n\
                    per_version: []\n";
        let error = Table::parse(yaml).err().expect("parsing should fail");
        assert_eq!(error.to_string(), "shared type Fraction is listed twice");
    }

    #[test]
    fn rust_names_follow_kopium_casing() {
        let name = |go: &str| {
            SharedType {
                go: go.to_string(),
                rust: None,
                placement: Placement::Common,
                kopium: Vec::new(),
            }
            .rust_name()
        };
        assert_eq!(name("HTTPHeaderFilter"), "HttpHeaderFilter");
        assert_eq!(name("ListenerTLSConfig"), "ListenerTlsConfig");
        assert_eq!(name("TLSModeType"), "TlsModeType");
        assert_eq!(name("ParentReference"), "ParentReference");
    }
}
