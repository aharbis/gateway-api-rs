//! Schema conformance of the preview's types against the vendored upstream
//! CRDs, with the same comparison as `gateway-api/tests/schema.rs`: fields,
//! types and formats, required fields and enum values. Every difference is
//! printed, and the test fails on any that [`EXPECTED`] doesn't list.
//!
//! The comparison code below `Attributes` is copied from that file.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition, JSONSchemaProps, JSONSchemaPropsOrArray, JSONSchemaPropsOrBool,
};
use kube::{CustomResourceExt, Resource};

include!("generated/roots.rs");

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Channel {
    Standard,
    Experimental,
}

impl Channel {
    fn name(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Experimental => "experimental",
        }
    }
}

/// A known difference: the kind (`*` for any), a path suffix, the attribute,
/// and why.
struct Expected {
    kind: &'static str,
    path_suffix: &'static str,
    attribute: SchemaAttribute,
    reason: &'static str,
}

/// Differences the preview accepts.
const EXPECTED: &[Expected] = &[
    Expected {
        kind: "*",
        path_suffix: ".conditions[].status",
        attribute: SchemaAttribute::Enum,
        reason: "k8s-openapi's `Condition::status` is a plain string, as in the crate",
    },
    Expected {
        kind: "Gateway",
        path_suffix: ".addresses[]/oneOf",
        attribute: SchemaAttribute::Exists,
        reason: "validation isn't generated, as in the crate",
    },
    Expected {
        kind: "HTTPRoute",
        path_suffix: ".requestRedirect.statusCode",
        attribute: SchemaAttribute::Enum,
        reason: "an integer enum stays a plain integer, as in the crate",
    },
    Expected {
        kind: "*",
        path_suffix: ".allowedRoutes.namespaces.from",
        attribute: SchemaAttribute::Enum,
        reason: "one `FromNamespaces` enum with Go's four values; the CRD narrows `allowedRoutes` to three",
    },
    Expected {
        kind: "ReferenceGrant",
        path_suffix: "",
        attribute: SchemaAttribute::Required,
        reason: "kube's `CustomResource` derive always requires `spec`, as in the crate",
    },
];

/// The vendored upstream CRDs of `channel`, by resource.
fn upstream_crds(channel: Channel) -> BTreeMap<String, CustomResourceDefinition> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../gateway-api/tests/fixtures/crds")
        .join(channel.name());
    let mut crds = BTreeMap::new();
    for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("failed to read {dir}: {e}", dir = dir.display())) {
        let path = entry.expect("failed to read a directory entry").path();
        let yaml =
            fs::read_to_string(&path).unwrap_or_else(|e| panic!("failed to read {path}: {e}", path = path.display()));
        let crd: CustomResourceDefinition = serde_saphyr::from_str(&yaml)
            .unwrap_or_else(|e| panic!("failed to parse {path}: {e}", path = path.display()));
        crds.insert(crd.spec.names.plural.clone(), crd);
    }
    crds
}

/// One API version's CRD compared with the upstream one.
struct Comparison {
    channel: Channel,
    kind: String,
    version: String,
    diffs: Vec<Diff>,
}

fn compare<K>(channel: Channel, resource: &str, version: &str) -> Comparison
where
    K: CustomResourceExt + Resource<DynamicType = ()>,
{
    let crds = upstream_crds(channel);
    let upstream = crds
        .get(resource)
        .unwrap_or_else(|| panic!("no upstream CRD for {channel:?} {resource}"));
    Comparison {
        channel,
        kind: K::kind(&()).into_owned(),
        version: version.to_owned(),
        diffs: diff(&Crd::new(upstream, version), &Crd::new(&K::crd(), &K::version(&()))),
    }
}

#[test]
fn matches_upstream() {
    let mut unexpected = Vec::new();
    let mut expected = 0;
    for comparison in comparisons() {
        let Comparison {
            channel,
            kind,
            version,
            diffs,
        } = &comparison;
        for diff in diffs {
            let known = match diff {
                Diff::Crd { .. } => None,
                Diff::Schema { path, attribute, .. } => EXPECTED.iter().find(|e| {
                    (e.kind == "*" || e.kind == kind) && path.ends_with(e.path_suffix) && e.attribute == *attribute
                }),
            };
            let line = format!("{channel} {kind} {version}: {diff}", channel = channel.name());
            if let Some(known) = known {
                expected += 1;
                println!("expected   {line} ({reason})", reason = known.reason);
            } else {
                println!("UNEXPECTED {line}");
                unexpected.push(line);
            }
        }
    }
    println!(
        "{expected} expected differences, {unexpected} unexpected",
        unexpected = unexpected.len()
    );
    assert!(
        unexpected.is_empty(),
        "unexpected differences:\n{unexpected}",
        unexpected = unexpected.join("\n")
    );
}

// -----------------------------------------------------------------------------
// Attributes
// -----------------------------------------------------------------------------

/// One API version of a CRD, as compared: the CRD's own attributes, and its
/// schema's nodes.
struct Crd {
    group: String,
    version: String,
    kind: String,
    plural: String,
    scope: String,
    status_subresource: bool,
    /// The schema's nodes by path, `""` for the root.
    schemas: BTreeMap<String, Node>,
}

impl Crd {
    /// `version` of `crd`.
    fn new(crd: &CustomResourceDefinition, version: &str) -> Self {
        let spec = &crd.spec;
        let kind = &spec.names.kind;
        let v = spec
            .versions
            .iter()
            .find(|v| v.name == version)
            .unwrap_or_else(|| panic!("{kind} CRD doesn't define {version}"));
        let schema = v
            .schema
            .as_ref()
            .and_then(|s| s.open_api_v3_schema.as_ref())
            .unwrap_or_else(|| panic!("{kind} {version} has no schema"));

        let mut schemas = BTreeMap::new();
        walk(schema, "", &mut schemas);
        Self {
            group: spec.group.clone(),
            version: v.name.clone(),
            kind: kind.clone(),
            plural: spec.names.plural.clone(),
            scope: spec.scope.clone(),
            status_subresource: v.subresources.as_ref().is_some_and(|s| s.status.is_some()),
            schemas,
        }
    }

    /// The CRD's own attributes, as text, without its schemas.
    fn attributes(&self) -> [(CrdAttribute, String); 6] {
        let Self {
            group,
            version,
            kind,
            plural,
            scope,
            status_subresource,
            schemas: _,
        } = self;
        [
            (CrdAttribute::Group, group.clone()),
            (CrdAttribute::Version, version.clone()),
            (CrdAttribute::Kind, kind.clone()),
            (CrdAttribute::Plural, plural.clone()),
            (CrdAttribute::Scope, scope.clone()),
            (CrdAttribute::StatusSubresource, status_subresource.to_string()),
        ]
    }
}

/// What's at one schema path.
enum Node {
    Schema(Schema),
    /// An `allOf`, `anyOf` or `oneOf`, whose branches are nodes of their own.
    /// Only whether it exists is compared, so one missing on one side is one
    /// difference, not one per branch.
    Combinator,
}

/// The characters that start a step in a schema path.
const SEPARATORS: [char; 4] = ['.', '[', '{', '/'];

/// Records the node for `schema` at `path`, then for its properties, map
/// values, array items and combinators (`allOf`, `anyOf`, `oneOf`, `not`).
fn walk(schema: &JSONSchemaProps, path: &str, schemas: &mut BTreeMap<String, Node>) {
    schemas.insert(path.into(), Node::Schema(Schema::from(schema)));

    for (name, property) in schema.properties.iter().flatten() {
        assert!(
            !name.contains(SEPARATORS),
            "{path}.{name}: property name contains a path separator, one of {SEPARATORS:?}"
        );
        // Upstream's schemas describe the object's own `apiVersion`, `kind`
        // and `metadata`; kube's derived schemas only describe `spec` and
        // `status`. The API server handles all three itself either way.
        if path.is_empty() && matches!(name.as_str(), "apiVersion" | "kind" | "metadata") {
            continue;
        }
        walk(property, &format!("{path}.{name}"), schemas);
    }
    if let Some(JSONSchemaPropsOrBool::Schema(values)) = &schema.additional_properties {
        walk(values, &format!("{path}{{*}}"), schemas);
    }
    match &schema.items {
        Some(JSONSchemaPropsOrArray::Schema(items)) => walk(items, &format!("{path}[]"), schemas),
        Some(JSONSchemaPropsOrArray::Schemas(items)) => {
            for (i, item) in items.iter().enumerate() {
                walk(item, &format!("{path}[{i}]"), schemas);
            }
        },
        None => {},
    }
    for (name, branches) in [
        ("allOf", &schema.all_of),
        ("anyOf", &schema.any_of),
        ("oneOf", &schema.one_of),
    ] {
        if let Some(branches) = branches {
            let path = format!("{path}/{name}");
            for (i, branch) in branches.iter().enumerate() {
                walk(branch, &format!("{path}[{i}]"), schemas);
            }
            schemas.insert(path, Node::Combinator);
        }
    }
    if let Some(not) = &schema.not {
        walk(not, &format!("{path}/not"), schemas);
    }
}

/// A schema itself, without its subschemas. `nullable` isn't compared: the
/// derived schemas mark every `Option` field nullable and upstream marks none,
/// but the generated fields skip `None` when serializing, so the types never
/// send `null`.
struct Schema {
    type_: Option<String>,
    format: Option<String>,
    required: BTreeSet<String>,
    /// Enum values as JSON, so `"301"` and `301` differ.
    enum_values: BTreeSet<String>,
    int_or_string: bool,
    preserve_unknown_fields: bool,
    /// Whether other properties are allowed, if set to a bool. A schema here
    /// is walked as `{*}` instead.
    additional_properties: Option<bool>,
}

impl From<&JSONSchemaProps> for Schema {
    fn from(schema: &JSONSchemaProps) -> Self {
        // The API server reads an integer without a format as an `int64`,
        // which is also what kopium generates for one.
        let format = match (schema.type_.as_deref(), &schema.format) {
            (Some("integer"), None) => Some("int64".into()),
            (_, format) => format.clone(),
        };
        let additional_properties = match schema.additional_properties {
            Some(JSONSchemaPropsOrBool::Bool(b)) => Some(b),
            _ => None,
        };
        Self {
            type_: schema.type_.clone(),
            format,
            required: schema.required.iter().flatten().cloned().collect(),
            // schemars lists `null` for an `Option`, which `required` already
            // covers.
            enum_values: schema
                .enum_
                .iter()
                .flatten()
                .filter(|v| !v.0.is_null())
                .map(|v| v.0.to_string())
                .collect(),
            int_or_string: schema.x_kubernetes_int_or_string == Some(true),
            preserve_unknown_fields: schema.x_kubernetes_preserve_unknown_fields == Some(true),
            additional_properties,
        }
    }
}

impl Schema {
    /// The schema's attributes, as text.
    fn attributes(&self) -> [(SchemaAttribute, String); 7] {
        let Self {
            type_,
            format,
            required,
            enum_values,
            int_or_string,
            preserve_unknown_fields,
            additional_properties,
        } = self;
        let none = || "(none)".to_owned();
        [
            (SchemaAttribute::Type, type_.clone().unwrap_or_else(none)),
            (SchemaAttribute::Format, format.clone().unwrap_or_else(none)),
            (SchemaAttribute::Required, list(required)),
            (SchemaAttribute::Enum, list(enum_values)),
            (SchemaAttribute::IntOrString, int_or_string.to_string()),
            (
                SchemaAttribute::PreserveUnknownFields,
                preserve_unknown_fields.to_string(),
            ),
            (
                SchemaAttribute::AdditionalProperties,
                additional_properties.map_or_else(none, |b| b.to_string()),
            ),
        ]
    }
}

/// Renders `items` as `[a, b]`.
fn list(items: &BTreeSet<String>) -> String {
    let items: Vec<_> = items.iter().map(String::as_str).collect();
    format!("[{items}]", items = items.join(", "))
}

// -----------------------------------------------------------------------------
// Diff
// -----------------------------------------------------------------------------

/// An attribute of the CRD itself.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CrdAttribute {
    Group,
    Version,
    Kind,
    Plural,
    Scope,
    StatusSubresource,
}

/// An attribute of a schema. `Exists` is never recorded in a [`Schema`]: it's
/// the difference when a path is on one side only.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SchemaAttribute {
    Exists,
    Type,
    Format,
    Required,
    Enum,
    IntOrString,
    PreserveUnknownFields,
    AdditionalProperties,
}

enum Diff {
    /// The CRD itself differs.
    Crd {
        attribute: CrdAttribute,
        upstream: String,
        ours: String,
    },
    /// The schema differs at `path`, `""` for the root.
    Schema {
        path: String,
        attribute: SchemaAttribute,
        upstream: String,
        ours: String,
    },
}

impl std::fmt::Display for Diff {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Crd {
                attribute,
                upstream,
                ours,
            } => write!(f, "CRD {attribute:?}: upstream {upstream}, ours {ours}"),
            Self::Schema {
                path,
                attribute,
                upstream,
                ours,
            } => {
                let path = if path.is_empty() { "(root)" } else { path };
                write!(f, "{path} {attribute:?}: upstream {upstream}, ours {ours}")
            },
        }
    }
}

/// The differences between `upstream` and `ours`: first the CRDs' own
/// attributes, then each schema path.
fn diff(upstream: &Crd, ours: &Crd) -> Vec<Diff> {
    let mut diffs: Vec<_> = changed(upstream.attributes(), ours.attributes())
        .map(|(attribute, upstream, ours)| Diff::Crd {
            attribute,
            upstream,
            ours,
        })
        .collect();

    let mut missing = BTreeSet::new();
    let paths: BTreeSet<_> = upstream.schemas.keys().chain(ours.schemas.keys()).collect();
    for path in paths {
        // A path on one side only is one difference; its descendants, whose
        // ancestors end where a separator starts, aren't more.
        let mut ancestors = path.match_indices(SEPARATORS).map(|(i, _)| &path[..i]);
        if ancestors.any(|a| missing.contains(a)) {
            continue;
        }
        let (u, o) = match (upstream.schemas.get(path), ours.schemas.get(path)) {
            (Some(Node::Schema(u)), Some(Node::Schema(o))) => (u, o),
            (Some(Node::Combinator), Some(Node::Combinator)) => continue,
            // Only a combinator's path ends in `/allOf`, `/anyOf` or `/oneOf`:
            // `walk` rejects property names that would.
            (Some(_), Some(_)) => unreachable!("{path} is a schema on one side and a combinator on the other"),
            (u, o) => {
                missing.insert(path.as_str());
                diffs.push(Diff::Schema {
                    path: path.clone(),
                    attribute: SchemaAttribute::Exists,
                    upstream: u.is_some().to_string(),
                    ours: o.is_some().to_string(),
                });
                continue;
            },
        };
        diffs.extend(
            changed(u.attributes(), o.attributes()).map(|(attribute, upstream, ours)| Diff::Schema {
                path: path.clone(),
                attribute,
                upstream,
                ours,
            }),
        );
    }
    diffs
}

/// The attributes whose values differ, as `(attribute, upstream, ours)`. Both
/// sides must list the same attributes in the same order.
fn changed<A: PartialEq + std::fmt::Debug, const N: usize>(
    upstream: [(A, String); N],
    ours: [(A, String); N],
) -> impl Iterator<Item = (A, String, String)> {
    upstream.into_iter().zip(ours).filter_map(|((u_a, u), (o_a, o))| {
        assert_eq!(u_a, o_a, "attributes listed in a different order");
        (u != o).then_some((u_a, u, o))
    })
}
