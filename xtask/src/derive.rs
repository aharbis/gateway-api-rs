//! Derives the shared types table from the Go facts, instead of keeping it by
//! hand. Each kopium module is walked beside the Go type it was generated
//! from, field by field, which names the Go type behind every kopium type.
//! See `docs/specs/go-source-generation.md`.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail};

use crate::{
    gofacts::{Facts, Index, TypeKind},
    shared_types::{PerVersion, Placement, SharedType, Table},
};

/// The Go files whose types upstream shares across CRDs. Their types go in
/// `common` even where one resource uses them.
const SHARED_FILES: &[&str] = &["shared_types.go", "object_reference_types.go", "policy_types.go"];

/// Kubernetes' own types, which `k8s-openapi` defines.
const K8S_PREFIX: &str = "k8s.io/";

/// The Go package of the Kubernetes types, as the table names them.
const METAV1: (&str, &str) = ("k8s.io/apimachinery/pkg/apis/meta/v1", "metav1");

/// One kopium module to derive from.
pub struct Module<'a> {
    pub resource: &'a str,
    pub version: &'a str,
    pub root: bool,
    pub group: &'a str,
    pub kind: &'a str,
    pub file: &'a syn::File,
}

/// A kopium type and the Go type it was generated from.
struct Site {
    resource: String,
    version: String,
    root: bool,
    kopium: String,
    /// After aliases; `None` for a type with no Go name, such as an enum on a
    /// `*string` field.
    go: Option<String>,
    /// The enum values the field itself sets, which can narrow its Go type's.
    field_enum: Vec<String>,
}

/// One Go type's sites, merged across channels.
#[derive(Default)]
struct Uses {
    kopium: BTreeSet<String>,
    resources: BTreeSet<String>,
    /// The Go package and file that declare it.
    path: String,
    file: String,
    /// Its position in source order, which orders the table.
    position: usize,
}

/// The types each non-root API version declares itself, by resource and
/// version.
type PerVersionTypes = BTreeMap<(String, String), BTreeSet<String>>;

/// Derives the table from every channel's kopium modules and Go facts.
pub fn derive(channels: &[(&Facts, Vec<Module<'_>>)]) -> Result<Table> {
    let mut uses = BTreeMap::new();
    let mut per_version = PerVersionTypes::new();
    for (facts, modules) in channels {
        add_channel(facts, modules, &mut uses, &mut per_version)
            .with_context(|| format!("{channel}: deriving shared types", channel = facts.channel))?;
    }

    let mut shared_types = Vec::new();
    for (go, uses) in &uses {
        let placement = if uses.path.starts_with(K8S_PREFIX) {
            Placement::K8s
        } else if uses.resources.len() > 1 || SHARED_FILES.contains(&uses.file.as_str()) {
            Placement::Common
        } else if uses.kopium.len() > 1 {
            Placement::Resource(uses.resources.first().expect("a used type has a resource").clone())
        } else {
            continue;
        };
        let (_, name) = go.rsplit_once('.').expect("a Go key has a package");
        let go_name = if uses.path == METAV1.0 {
            format!("{package}.{name}", package = METAV1.1)
        } else {
            name.to_string()
        };
        shared_types.push((
            uses.position,
            SharedType {
                go: go_name,
                rust: rust_name(name),
                placement,
                kopium: uses.kopium.iter().cloned().collect(),
            },
        ));
    }
    shared_types.sort_by_key(|(position, _)| *position);

    let table = Table {
        shared_types: shared_types.into_iter().map(|(_, entry)| entry).collect(),
        per_version: per_version
            .into_iter()
            .map(|((resource, version), types)| PerVersion {
                resource,
                version,
                types: types.into_iter().collect(),
            })
            .collect(),
    };
    let mut seen = BTreeSet::new();
    if let Some(entry) = table.shared_types.iter().find(|entry| !seen.insert(&entry.go)) {
        bail!("two shared Go types are named {go}", go = entry.go);
    }
    Ok(table)
}

/// Adds one channel's kopium types to the uses of each Go type, and to the
/// types non-root versions declare themselves.
fn add_channel(
    facts: &Facts,
    modules: &[Module<'_>],
    uses: &mut BTreeMap<String, Uses>,
    per_version: &mut PerVersionTypes,
) -> Result<()> {
    let index = Index::new(facts);
    let mut sites = Vec::new();
    for module in modules {
        let (resource, version) = (module.resource, module.version);
        pair_module(&index, module, &mut sites)
            .with_context(|| format!("{resource} {version}: pairing kopium types with Go types"))?;
    }

    // A non-root version's type is its own where Go declares it so, and
    // otherwise is the root version's.
    let root: BTreeMap<(&str, &str), &Option<String>> = sites
        .iter()
        .filter(|site| site.root)
        .map(|site| ((site.resource.as_str(), site.kopium.as_str()), &site.go))
        .collect();
    let mut shared = Vec::new();
    for site in &sites {
        match root.get(&(site.resource.as_str(), site.kopium.as_str())) {
            Some(go) if !site.root && **go != site.go => {
                per_version
                    .entry((site.resource.clone(), site.version.clone()))
                    .or_default()
                    .insert(site.kopium.clone());
            },
            _ => shared.push(site),
        }
    }

    // One Go type with different field-level enums is more than one Rust
    // type: the CRD shape wins. The group with the most kopium names takes
    // the Go name; the others keep their kopium names.
    let mut by_go: BTreeMap<&str, BTreeMap<&[String], Vec<&Site>>> = BTreeMap::new();
    for site in shared {
        if let Some(go) = &site.go {
            by_go
                .entry(go)
                .or_default()
                .entry(&site.field_enum)
                .or_default()
                .push(site);
        }
    }
    for (go, groups) in by_go {
        fn names<'s>(sites: &[&'s Site]) -> BTreeSet<&'s str> {
            sites.iter().map(|site| site.kopium.as_str()).collect()
        }
        let mut groups: Vec<_> = groups.into_values().collect();
        groups.sort_by_key(|sites| std::cmp::Reverse(names(sites).len()));
        if let Some(other) = groups.get(1).filter(|sites| names(sites).len() > 1) {
            let other = names(other).into_iter().collect::<Vec<_>>().join(", ");
            bail!("{go} is narrowed by its fields' enums at more than one group of paths: {other}");
        }
        let (package, ty) = index.get(go)?;
        let entry = uses.entry(go.to_string()).or_insert_with(|| Uses {
            path: package.path.clone(),
            file: ty.file.clone(),
            position: index.position(go),
            ..Uses::default()
        });
        for site in &groups[0] {
            entry.kopium.insert(site.kopium.clone());
            entry.resources.insert(site.resource.clone());
        }
    }
    Ok(())
}

/// Walks a kopium module from its spec and status, pairing each kopium type
/// with the Go type of the field it was generated for.
fn pair_module(index: &Index<'_>, module: &Module<'_>, sites: &mut Vec<Site>) -> Result<()> {
    let items: BTreeMap<String, &syn::Item> = module
        .file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Struct(s) => Some((s.ident.to_string(), item)),
            syn::Item::Enum(e) => Some((e.ident.to_string(), item)),
            _ => None,
        })
        .collect();
    let package = index.package(module.group, module.version)?;
    let kind = package
        .kinds
        .iter()
        .find(|kind| kind.name == module.kind)
        .with_context(|| format!("no Go kind {kind} in {path}", kind = module.kind, path = package.path))?;
    let spec = module
        .file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Struct(s) if s.attrs.iter().any(|attr| attr.path().is_ident("kube")) => {
                Some(s.ident.to_string())
            },
            _ => None,
        })
        .context("no kopium spec struct")?;
    let root = spec
        .strip_suffix("Spec")
        .context("kopium's spec struct isn't named <root>Spec")?;

    let mut queue = Vec::new();
    for (kopium, go) in [(spec.clone(), &kind.spec), (format!("{root}Status"), &kind.status)] {
        let Some(go) = go.as_ref().and_then(|go| go.element()) else {
            continue;
        };
        if items.contains_key(&kopium) {
            queue.push((kopium, index.resolve(go)?, Vec::new()));
        }
    }

    let mut paired: BTreeMap<String, Option<String>> = BTreeMap::new();
    while let Some((kopium, go, field_enum)) = queue.pop() {
        if let Some(seen) = paired.get(&kopium) {
            if *seen != go {
                bail!("{kopium} is generated from both {seen:?} and {go:?}");
            }
            continue;
        }
        paired.insert(kopium.clone(), go.clone());
        sites.push(Site {
            resource: module.resource.to_string(),
            version: module.version.to_string(),
            root: module.root,
            kopium: kopium.clone(),
            go: go.clone(),
            field_enum,
        });

        let (Some(go), syn::Item::Struct(item)) = (&go, items[&kopium]) else {
            continue;
        };
        let go_fields = flattened_fields(index, go)?;
        let mut kopium_fields = BTreeSet::new();
        for field in &item.fields {
            let json = json_name(field)?;
            let go_field = go_fields
                .get(json.as_str())
                .with_context(|| format!("{kopium}.{json} has no field in Go {go}"))?;
            kopium_fields.insert(json);
            let Some(local) = local_type(&field.ty, &items) else {
                continue;
            };
            let target = match go_field.ty.element() {
                Some(named) => index.resolve(named)?,
                None => None,
            };
            queue.push((local, target, go_field.enum_values.clone()));
        }
        if let Some(missing) = go_fields.keys().find(|json| !kopium_fields.contains(**json)) {
            bail!("Go {go}.{missing} has no field in {kopium}");
        }
    }
    Ok(())
}

/// A Go struct's fields by JSON name, with embedded structs inlined.
fn flattened_fields<'f>(index: &Index<'f>, key: &str) -> Result<BTreeMap<&'f str, &'f crate::gofacts::Field>> {
    let (_, ty) = index.get(key)?;
    if ty.kind != TypeKind::Struct {
        bail!("Go {key} isn't a struct");
    }
    let mut fields = BTreeMap::new();
    for field in &ty.fields {
        if field.inline {
            let embedded = field
                .ty
                .element()
                .with_context(|| format!("Go {key}.{name} embeds no named type", name = field.name))?;
            let embedded = index.resolve(embedded)?.context("an embedded type has a name")?;
            fields.extend(flattened_fields(index, &embedded)?);
        } else if let Some(json) = &field.json {
            fields.insert(json.as_str(), field);
        }
    }
    Ok(fields)
}

/// A kopium field's JSON name: its `serde(rename)`, or else its name.
fn json_name(field: &syn::Field) -> Result<String> {
    for attr in field.attrs.iter().filter(|attr| attr.path().is_ident("serde")) {
        let mut rename = None;
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident("rename") {
                rename = Some(meta.value()?.parse::<syn::LitStr>()?.value());
            } else if meta.input.peek(syn::Token![=]) {
                meta.value()?.parse::<syn::Expr>()?;
            }
            Ok(())
        })?;
        if let Some(rename) = rename {
            return Ok(rename);
        }
    }
    let ident = field.ident.as_ref().context("kopium generated a tuple struct")?;
    Ok(ident.to_string().trim_start_matches("r#").to_string())
}

/// The module's own type a field's type refers to, through `Option`, `Vec`
/// and map values.
fn local_type(ty: &syn::Type, items: &BTreeMap<String, &syn::Item>) -> Option<String> {
    let syn::Type::Path(path) = ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let name = segment.ident.to_string();
    if items.contains_key(&name) {
        return Some(name);
    }
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    args.args.iter().rev().find_map(|arg| match arg {
        syn::GenericArgument::Type(ty) => local_type(ty, items),
        _ => None,
    })
}

/// Go initialisms that upstream's type names run together, as in
/// `HTTPURLRewriteFilter`.
const INITIALISMS: &[&str] = &["HTTP", "GRPC", "URL", "CORS", "TLS", "TCP", "UDP", "API"];

/// The Rust name for a Go type name: each word capitalized, as kopium does,
/// with runs of initialisms split, so `HTTPURLRewriteFilter` becomes
/// `HttpUrlRewriteFilter`, not `HttpurlRewriteFilter`.
pub fn rust_name(go: &str) -> String {
    let chars: Vec<char> = go.chars().collect();
    let mut words: Vec<String> = Vec::new();
    let mut word = String::new();
    for (i, &c) in chars.iter().enumerate() {
        let prev = i.checked_sub(1).map(|i| chars[i]);
        let next = chars.get(i + 1);
        let boundary = c.is_uppercase()
            && prev.is_some_and(|prev| {
                prev.is_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_uppercase() && next.is_some_and(|next| next.is_lowercase()))
            });
        if boundary && !word.is_empty() {
            words.push(std::mem::take(&mut word));
        }
        word.push(c);
    }
    words.push(word);

    let mut out = String::new();
    for word in words {
        for part in split_initialisms(&word) {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(&chars.as_str().to_lowercase());
            }
        }
    }
    out
}

/// Splits an all-capitals word made of initialisms, e.g. `HTTPURL`, into
/// them. Any other word is returned whole.
fn split_initialisms(word: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = word;
    while !rest.is_empty() {
        let Some(initialism) = INITIALISMS.iter().find(|initialism| rest.starts_with(**initialism)) else {
            return vec![word];
        };
        parts.push(&rest[..initialism.len()]);
        rest = &rest[initialism.len()..];
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rust_names_split_initialisms() {
        assert_eq!(rust_name("HTTPURLRewriteFilter"), "HttpUrlRewriteFilter");
        assert_eq!(rust_name("HTTPCORSFilter"), "HttpCorsFilter");
        assert_eq!(rust_name("GRPCRouteFilter"), "GrpcRouteFilter");
        assert_eq!(rust_name("ListenerTLSConfig"), "ListenerTlsConfig");
        assert_eq!(rust_name("TLSModeType"), "TlsModeType");
        assert_eq!(rust_name("XMesh"), "XMesh");
        assert_eq!(rust_name("ParentReference"), "ParentReference");
        assert_eq!(rust_name("LabelSelector"), "LabelSelector");
    }
}
