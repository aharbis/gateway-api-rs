//! Facts from the upstream Go source, which the CRDs don't carry: type names,
//! aliases, defined types and where each type is declared. `xtask/gofacts`, a
//! Go program, reads them with `go/types` and writes one JSON file per channel
//! to `xtask/gofacts/ir/`. See `docs/specs/go-source-generation.md`.

use std::{collections::BTreeMap, fs, process::Command};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

const GOFACTS_DIR: &str = "xtask/gofacts";

/// Relative to `GOFACTS_DIR`.
const IR_DIR: &str = "ir";

/// One channel's facts.
#[derive(Deserialize)]
pub struct Facts {
    pub version: String,
    pub channel: String,
    pub packages: Vec<Package>,
}

#[derive(Deserialize)]
pub struct Package {
    /// Relative to upstream's module for its packages, else the import path.
    pub path: String,
    /// The package name, which is the API version for upstream's packages.
    pub name: String,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub kinds: Vec<Kind>,
    pub types: Vec<Type>,
    #[serde(default)]
    pub consts: Vec<Const>,
}

/// A root type, such as `HTTPRoute`.
#[derive(Deserialize)]
pub struct Kind {
    pub name: String,
    pub spec: Option<TypeRef>,
    pub status: Option<TypeRef>,
}

#[derive(Deserialize)]
pub struct Type {
    pub name: String,
    pub file: String,
    #[serde(default)]
    pub doc: String,
    pub kind: TypeKind,
    #[serde(default)]
    pub alias: Option<TypeRef>,
    #[serde(default)]
    pub fields: Vec<Field>,
    /// For a scalar, its basic type, e.g. `string`.
    #[serde(default)]
    pub basic: Option<String>,
    /// From the type's enum marker.
    #[serde(default, rename = "enum")]
    pub enum_values: Vec<String>,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
    Alias,
    Struct,
    Scalar,
    Other,
}

#[derive(Deserialize)]
pub struct Field {
    pub name: String,
    #[serde(default)]
    pub json: Option<String>,
    #[serde(default)]
    pub inline: bool,
    #[serde(rename = "type")]
    pub ty: TypeRef,
    /// From the field's enum marker, after the channel's override.
    #[serde(default, rename = "enum")]
    pub enum_values: Vec<String>,
    /// The raw value of the field's `+kubebuilder:default` marker.
    #[serde(default)]
    pub default: Option<String>,
    /// By controller-gen's rule: `+optional`, or `omitempty` without
    /// `+required`.
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub doc: String,
}

/// A typed const, such as `GatewayConditionProgrammed`.
#[derive(Deserialize)]
pub struct Const {
    /// Its named type's key.
    #[serde(rename = "type")]
    pub ty: String,
    pub value: String,
    #[serde(default)]
    pub doc: String,
}

/// A type expression. Exactly one field is set.
#[derive(Deserialize)]
pub struct TypeRef {
    /// A named type or alias, as `<package path>.<name>`.
    #[serde(default)]
    pub named: Option<String>,
    #[serde(default)]
    basic: Option<String>,
    #[serde(default)]
    pub pointer: Option<Box<TypeRef>>,
    #[serde(default)]
    pub slice: Option<Box<TypeRef>>,
    #[serde(default)]
    pub map: Option<Box<MapRef>>,
}

#[derive(Deserialize)]
pub struct MapRef {
    pub value: TypeRef,
}

impl TypeRef {
    /// The basic type this is, e.g. `string`.
    pub fn basic(&self) -> Option<&str> {
        self.basic.as_deref()
    }

    /// Whether this is a string, through pointers and slices.
    pub fn is_string(&self) -> bool {
        match (&self.pointer, &self.slice) {
            (Some(inner), _) | (_, Some(inner)) => inner.is_string(),
            _ => self.basic() == Some("string"),
        }
    }

    /// The named type this refers to through pointers, slices and map values.
    pub fn element(&self) -> Option<&str> {
        if let Some(named) = &self.named {
            return Some(named);
        }
        if let Some(inner) = self.pointer.as_ref().or(self.slice.as_ref()) {
            return inner.element();
        }
        self.map.as_ref().and_then(|map| map.value.element())
    }
}

/// Runs `xtask/gofacts` and reads what it wrote, failing unless it read the
/// upstream version being generated.
pub fn generate(version: &str) -> Result<Vec<Facts>> {
    eprintln!("reading the upstream Go source with {GOFACTS_DIR}");
    let status = Command::new("go")
        .args(["run", ".", "-out", IR_DIR])
        .current_dir(GOFACTS_DIR)
        .status()
        .context("failed to run go — is it installed?")?;
    if !status.success() {
        bail!("{GOFACTS_DIR} failed");
    }

    let mut channels = Vec::new();
    for channel in ["standard", "experimental"] {
        let path = format!("{GOFACTS_DIR}/{IR_DIR}/{channel}.json");
        let json = fs::read_to_string(&path).with_context(|| format!("failed to read {path}"))?;
        let facts: Facts = serde_json::from_str(&json).with_context(|| format!("failed to parse {path}"))?;
        if facts.version != version {
            let read = &facts.version;
            bail!("{GOFACTS_DIR} read upstream {read}, not {version}: update its go.mod");
        }
        channels.push(facts);
    }
    Ok(channels)
}

/// A channel's types by key, `<package path>.<name>`.
pub struct Index<'f> {
    pub facts: &'f Facts,
    /// With each type's position in source order.
    types: BTreeMap<String, (&'f Package, &'f Type, usize)>,
}

impl<'f> Index<'f> {
    pub fn new(facts: &'f Facts) -> Self {
        let mut types = BTreeMap::new();
        for package in &facts.packages {
            for ty in &package.types {
                let position = types.len();
                types.insert(
                    format!("{path}.{name}", path = package.path, name = ty.name),
                    (package, ty, position),
                );
            }
        }
        Self { facts, types }
    }

    pub fn get(&self, key: &str) -> Result<(&'f Package, &'f Type)> {
        let (package, ty, _) = self.types.get(key).with_context(|| format!("no Go type {key}"))?;
        Ok((package, ty))
    }

    /// A type's position in source order: by package, file and declaration.
    pub fn position(&self, key: &str) -> usize {
        self.types.get(key).map_or(usize::MAX, |(_, _, position)| *position)
    }

    /// The type an alias stands for, following aliases until a declared
    /// type, or `None` for an alias of a type with no name, as in
    /// `type PortNumber = int32`.
    pub fn resolve(&self, key: &str) -> Result<Option<String>> {
        let mut key = key.to_string();
        loop {
            let (_, ty) = self.get(&key)?;
            if ty.kind != TypeKind::Alias {
                return Ok(Some(key));
            }
            match ty.alias.as_ref().and_then(|alias| alias.named.as_ref()) {
                Some(target) => key.clone_from(target),
                None => return Ok(None),
            }
        }
    }

    /// The consts of a named type, in source order.
    pub fn consts(&self, key: &str) -> Vec<&'f Const> {
        self.facts
            .packages
            .iter()
            .flat_map(|package| &package.consts)
            .filter(|c| c.ty == key)
            .collect()
    }

    /// The package for a CRD's group and API version.
    pub fn package(&self, group: &str, version: &str) -> Result<&'f Package> {
        self.facts
            .packages
            .iter()
            .find(|package| package.group.as_deref() == Some(group) && package.name == version)
            .with_context(|| format!("no Go package for {group}/{version}"))
    }
}
