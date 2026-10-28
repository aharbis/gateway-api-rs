//! The IR gofacts writes: one profile's view of a Go module's API packages,
//! with the config's tag rules applied, and its CRDs' envelopes. See
//! `codegen/README.md`, and `codegen/gofacts/ir.go` for the writer. Unknown
//! keys are errors, so the two sides can't drift apart silently.

use std::{fs, path::Path};

use anyhow::{Context, Result, bail};
use serde::Deserialize;

/// The IR format version this crate reads.
pub const IR_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Ir {
    pub ir_version: u32,
    pub module: String,
    /// The module version `go.mod` pins.
    pub version: String,
    pub profile: String,
    pub packages: Vec<Package>,
    pub crds: Vec<Crd>,
}

impl Ir {
    /// Reads an IR file, failing on any other format version.
    pub fn load(path: &Path) -> Result<Self> {
        let path_display = path.display();
        let json = fs::read_to_string(path).with_context(|| format!("failed to read {path_display}"))?;
        let ir: Self = serde_json::from_str(&json).with_context(|| format!("failed to parse {path_display}"))?;
        if ir.ir_version != IR_VERSION {
            let found = ir.ir_version;
            bail!("{path_display}: IR version {found}, but this generator reads {IR_VERSION}");
        }
        Ok(ir)
    }
}

/// One of the config's Go packages.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Package {
    /// Relative to the module.
    pub path: String,
    pub name: String,
    /// From the package's `+groupName` marker.
    #[serde(default)]
    pub group: Option<String>,
    pub types: Vec<Type>,
    #[serde(default)]
    pub consts: Vec<Const>,
}

/// An exported type declaration.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Type {
    pub name: String,
    /// The file it's declared in, without its directory.
    pub file: String,
    pub kind: TypeKind,
    /// For an alias, its right-hand side.
    #[serde(default)]
    pub alias: Option<TypeRef>,
    /// For a defined type declared from a named type, that type.
    #[serde(default)]
    pub defined: Option<TypeRef>,
    /// For a struct, its fields, without those the tag rules drop.
    #[serde(default)]
    pub fields: Vec<Field>,
    /// For a struct, the fields the tag rules drop.
    #[serde(default)]
    pub dropped: Vec<DroppedField>,
    /// For a scalar, its basic type, e.g. `string`.
    #[serde(default)]
    pub basic: Option<String>,
    /// For any other type, its underlying type.
    #[serde(default)]
    pub underlying: Option<TypeRef>,
    /// From the type's `+kubebuilder:validation:Enum` marker.
    #[serde(default, rename = "enum")]
    pub enum_values: Vec<String>,
    /// Whether it has `+kubebuilder:object:root`.
    #[serde(default)]
    pub root: bool,
    /// Whether it has `+kubebuilder:skipversion`.
    #[serde(default)]
    pub skip_version: bool,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub markers: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TypeKind {
    Alias,
    Struct,
    Scalar,
    Other,
}

/// A struct field, or an embedded struct.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Field {
    pub name: String,
    /// The JSON name; `None` for an inlined embedded field.
    #[serde(default)]
    pub json: Option<String>,
    #[serde(default)]
    pub inline: bool,
    #[serde(rename = "type")]
    pub ty: TypeRef,
    /// By controller-gen's rule.
    #[serde(default)]
    pub optional: bool,
    /// From the field's own `+kubebuilder:validation:Enum` marker, after
    /// overrides.
    #[serde(default, rename = "enum")]
    pub enum_values: Vec<String>,
    /// The raw value of the field's `+kubebuilder:default` marker.
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub doc: String,
    #[serde(default)]
    pub markers: Vec<String>,
}

/// A field the tag rules drop in the profile.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DroppedField {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: TypeRef,
}

/// A const whose type is a named type of one of the packages.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Const {
    pub name: String,
    /// Its type's key.
    #[serde(rename = "type")]
    pub ty: String,
    /// A string's value, or the exact value of any other kind.
    pub value: String,
    #[serde(default)]
    pub doc: String,
}

/// A type expression. Exactly one field is set.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TypeRef {
    /// A named type or alias, as `<package>.<name>`: the key of a type in
    /// the IR, or of another module's type.
    #[serde(default)]
    pub named: Option<String>,
    #[serde(default)]
    pub basic: Option<String>,
    #[serde(default)]
    pub pointer: Option<Box<TypeRef>>,
    #[serde(default)]
    pub slice: Option<Box<TypeRef>>,
    #[serde(default)]
    pub map: Option<Box<MapRef>>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MapRef {
    pub key: TypeRef,
    pub value: TypeRef,
}

impl TypeRef {
    /// The named type this refers to through pointers, slices and map
    /// values.
    pub fn element(&self) -> Option<&str> {
        if let Some(named) = &self.named {
            return Some(named);
        }
        if let Some(inner) = self.pointer.as_ref().or(self.slice.as_ref()) {
            return inner.element();
        }
        self.map.as_ref().and_then(|map| map.value.element())
    }

    /// The basic type this refers to through pointers and slices.
    pub fn element_basic(&self) -> Option<&str> {
        match (&self.pointer, &self.slice) {
            (Some(inner), _) | (_, Some(inner)) => inner.element_basic(),
            _ => self.basic.as_deref(),
        }
    }
}

/// A `CustomResourceDefinition`'s envelope.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Crd {
    /// The file it's read from, in the profile's CRD directory.
    pub file: String,
    pub group: String,
    pub kind: String,
    #[serde(default)]
    pub list_kind: Option<String>,
    pub plural: String,
    #[serde(default)]
    pub singular: Option<String>,
    pub scope: String,
    #[serde(default)]
    pub short_names: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
    pub versions: Vec<CrdVersion>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CrdVersion {
    pub name: String,
    /// The package the version's Go kind is in.
    pub package: String,
    pub served: bool,
    pub storage: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub deprecation_warning: Option<String>,
    /// Whether it has the status subresource.
    #[serde(default)]
    pub status: bool,
    #[serde(default)]
    pub printer_columns: Vec<PrinterColumn>,
}

#[derive(Debug, Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PrinterColumn {
    pub name: String,
    #[serde(rename = "type")]
    pub ty: String,
    pub json_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}
