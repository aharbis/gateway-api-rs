//! Consolidates the types kopium generates once per path into the shared
//! types the table names, and guards against duplicates nobody classified.

use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    fmt::Write,
};

use anyhow::{Result, bail};
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    visit::{self, Visit},
    visit_mut::{self, VisitMut},
};

use crate::shared_types::{PerVersion, Placement, SharedType, Table};

/// Where `k8s-openapi` defines the types placed at `k8s`.
const K8S_MODULE: &str = "k8s_openapi::apimachinery::pkg::apis::meta::v1";

/// One generated module that takes part in consolidation.
pub struct Module<'a> {
    pub resource: String,
    pub version: String,
    /// Whether this is the resource's root version, which defines the shared
    /// types placed in the resource.
    pub root: bool,
    pub file: &'a mut syn::File,
}

impl Module<'_> {
    /// Where the module is, for errors, e.g. `httproutes v1`.
    fn name(&self) -> String {
        format!("{} {}", self.resource, self.version)
    }

    /// Where the types defined in this module are placed, if any are.
    fn placement(&self) -> Option<Placement> {
        self.root.then(|| Placement::Resource(self.resource.clone()))
    }
}

/// The table entries consolidation matched, across channels, for failing on
/// stale ones.
#[derive(Default)]
pub struct Matched<'t> {
    /// Kopium names in `shared_types`.
    pub kopium: BTreeSet<&'t str>,
    /// Types in `per_version`, by entry index.
    pub per_version: BTreeSet<(usize, &'t str)>,
}

/// What consolidating a channel produced, besides the changes to its modules.
pub struct Consolidated {
    /// The `common` module's imports and types.
    pub common: Vec<syn::Item>,
    /// The new name of every type consolidation replaced.
    pub names: BTreeSet<String>,
}

/// Where kopium generated one instance of a type.
#[derive(Clone)]
struct Site {
    module: usize,
    item: usize,
    kopium: String,
}

/// The new name of a kopium type that consolidation replaces.
struct Rename<'t> {
    name: String,
    /// Where the new type is defined.
    placement: Placement,
    /// The table entry, or `None` for a nested type that moved with its parent.
    entry: Option<&'t SharedType>,
    /// For a type no field uses, the new name of the type it moved with.
    orphan_of: Option<String>,
}

/// Paths to new types, within one channel.
struct Paths<'a> {
    channel: &'a str,
    /// Each resource's root version.
    roots: BTreeMap<String, String>,
}

impl Paths<'_> {
    /// The path of the module that defines types placed at `placement`, e.g.
    /// `crate::apis::standard::httproutes::v1`.
    fn crate_module(&self, placement: &Placement) -> String {
        let channel = self.channel;
        match placement {
            Placement::Common => format!("crate::apis::{channel}::common"),
            Placement::K8s => K8S_MODULE.to_string(),
            Placement::Resource(resource) => {
                format!(
                    "crate::apis::{channel}::{resource}::{root}",
                    root = self.roots[resource]
                )
            },
        }
    }

    /// The public path a type placed at `placement` is documented at. A
    /// resource re-exports its root version.
    fn public(&self, placement: &Placement, name: &str) -> String {
        let public = crate::public_path(self.channel);
        match placement {
            Placement::Common => format!("{public}::common::{name}"),
            Placement::K8s => format!("{K8S_MODULE}::{name}"),
            Placement::Resource(resource) => format!("{public}::{resource}::{name}"),
        }
    }
}

/// Consolidates `modules`, the modules of one channel, by `table`. Ordered
/// root versions first, they decide which instance documents a shared type.
/// Adds the entries `table` matched to `matched`.
pub fn consolidate<'t>(
    channel: &str,
    table: &'t Table,
    modules: &mut [Module<'_>],
    matched: &mut Matched<'t>,
) -> Result<Consolidated> {
    let per_version = &table.per_version;
    let table = &table.shared_types;
    let items: Vec<_> = modules.iter().map(|module| item_indices(&module.file.items)).collect();
    let renames = plan_renames(table, modules, &items, &mut matched.kopium)?;
    let paths = Paths {
        channel,
        roots: modules
            .iter()
            .filter(|module| module.root)
            .map(|module| (module.resource.clone(), module.version.clone()))
            .collect(),
    };
    check_placements(modules, &items, &renames, &paths)?;

    // Every instance of a new name, in module order
    let mut groups: BTreeMap<&str, Vec<Site>> = BTreeMap::new();
    for (module, names) in items.iter().enumerate() {
        for (kopium, &item) in names {
            if let Some(rename) = renames.get(kopium) {
                let site = Site {
                    module,
                    item,
                    kopium: kopium.clone(),
                };
                groups.entry(&rename.name).or_default().push(site);
            }
        }
    }
    for sites in groups.values_mut() {
        sites.sort_by_key(|site| (site.module, site.item));
    }

    check_shapes(modules, &items, &renames, &groups)?;

    let mut placements: Vec<&Placement> = Vec::new();
    for entry in table {
        if !placements.contains(&&entry.placement) {
            placements.push(&entry.placement);
        }
    }
    let mut common = Vec::new();
    let mut defs = BTreeMap::new();
    for placement in placements {
        match placement {
            Placement::Common => common = emit_types(table, placement, modules, &items, &renames, &groups),
            Placement::K8s => {},
            Placement::Resource(resource) => {
                let types = emit_types(table, placement, modules, &items, &renames, &groups);
                if !types.is_empty() {
                    defs.insert(resource.clone(), types);
                }
            },
        }
    }

    reexport_root_types(modules, &items, &renames, per_version, &mut matched.per_version)?;
    replace_sites(modules, &renames, defs, &paths)?;
    rewrite_module(&mut common, Some(&Placement::Common), &renames, &paths)?;
    Ok(Consolidated {
        common,
        names: renames.values().map(|rename| rename.name.clone()).collect(),
    })
}

/// Maps the name of each struct and enum in `items` to its index.
fn item_indices(items: &[syn::Item]) -> BTreeMap<String, usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| Some((item_ident(item)?.to_string(), index)))
        .collect()
}

fn item_ident(item: &syn::Item) -> Option<&syn::Ident> {
    match item {
        syn::Item::Struct(item) => Some(&item.ident),
        syn::Item::Enum(item) => Some(&item.ident),
        _ => None,
    }
}

fn item_ident_mut(item: &mut syn::Item) -> Option<&mut syn::Ident> {
    match item {
        syn::Item::Struct(item) => Some(&mut item.ident),
        syn::Item::Enum(item) => Some(&mut item.ident),
        _ => None,
    }
}

/// Names every kopium type the table replaces, and every type nested under
/// one, which is placed with its parent.
fn plan_renames<'t>(
    table: &'t [SharedType],
    modules: &[Module<'_>],
    items: &[BTreeMap<String, usize>],
    matched: &mut BTreeSet<&'t str>,
) -> Result<BTreeMap<String, Rename<'t>>> {
    let mut renames = BTreeMap::new();
    for entry in table {
        for kopium in &entry.kopium {
            if !items.iter().any(|names| names.contains_key(kopium)) {
                continue;
            }
            matched.insert(kopium);
            let rename = Rename {
                name: entry.rust_name(),
                placement: entry.placement.clone(),
                entry: Some(entry),
                orphan_of: None,
            };
            if let Some(other) = renames.insert(kopium.clone(), rename) {
                let (go, other_go) = (&entry.go, other.entry.map_or("", |entry| &entry.go));
                bail!("{kopium} is listed by both {other_go} and {go}");
            }
        }
    }

    let parents = renames.keys().cloned().collect();
    move_nested(&mut renames, parents, modules, items)?;

    // kopium also generates types no field uses, such as an enum of an integer
    // field's values. Each moves with the type whose kopium name is the
    // longest that prefixes its own.
    let mut orphans = Vec::new();
    for (module, names) in modules.iter().zip(items) {
        let used: BTreeSet<String> = module
            .file
            .items
            .iter()
            .flat_map(|item| local_refs(item, names))
            .collect();
        for name in names
            .keys()
            .filter(|name| !used.contains(*name) && !renames.contains_key(*name))
        {
            let parent = renames
                .keys()
                .filter(|parent| name.len() > parent.len() && name.starts_with(parent.as_str()))
                .max_by_key(|parent| parent.len());
            if let Some(parent) = parent {
                orphans.push((name.clone(), parent.clone()));
            }
        }
    }
    let mut parents = Vec::new();
    for (orphan, parent) in orphans {
        let Rename { name, placement, .. } = &renames[&parent];
        let rename = Rename {
            name: format!("{name}{suffix}", suffix = &orphan[parent.len()..]),
            placement: placement.clone(),
            entry: None,
            orphan_of: Some(name.clone()),
        };
        if renames.insert(orphan.clone(), rename).is_none() {
            parents.push(orphan);
        }
    }
    move_nested(&mut renames, parents, modules, items)?;
    Ok(renames)
}

/// Renames the types nested under each of `parents`, and under those, by
/// swapping the parent's kopium name, which prefixes theirs, for its new name.
/// A nested type with its own table entry keeps that.
fn move_nested(
    renames: &mut BTreeMap<String, Rename<'_>>,
    mut parents: Vec<String>,
    modules: &[Module<'_>],
    items: &[BTreeMap<String, usize>],
) -> Result<()> {
    while let Some(parent) = parents.pop() {
        let new_parent = renames[&parent].name.clone();
        let placement = renames[&parent].placement.clone();
        for (module, names) in items.iter().enumerate() {
            let Some(&item) = names.get(&parent) else {
                continue;
            };
            for child in local_refs(&modules[module].file.items[item], names) {
                if renames.get(&child).is_some_and(|rename| rename.entry.is_some()) {
                    continue;
                }
                let Some(suffix) = child.strip_prefix(parent.as_str()) else {
                    bail!("{child}, nested under {parent}, doesn't start with its name");
                };
                let name = format!("{new_parent}{suffix}");
                match renames.get(&child) {
                    Some(rename) if rename.name != name => {
                        bail!("{child} is nested under types named both {} and {name}", rename.name);
                    },
                    Some(_) => {},
                    None => {
                        let placement = placement.clone();
                        renames.insert(
                            child.clone(),
                            Rename {
                                name,
                                placement,
                                entry: None,
                                orphan_of: None,
                            },
                        );
                        parents.push(child);
                    },
                }
            }
        }
    }
    Ok(())
}

/// Fails if a type placed in a resource is generated by another, or by none
/// with a root version, or if a type replaced by `k8s-openapi`'s has a nested
/// type of its own, which would have nowhere to go.
fn check_placements(
    modules: &[Module<'_>],
    items: &[BTreeMap<String, usize>],
    renames: &BTreeMap<String, Rename<'_>>,
    paths: &Paths<'_>,
) -> Result<()> {
    for (module, names) in modules.iter().zip(items) {
        for kopium in names.keys() {
            let Some(rename) = renames.get(kopium) else {
                continue;
            };
            let resource = match &rename.placement {
                Placement::Resource(resource) => resource,
                Placement::K8s if rename.entry.is_none() => {
                    bail!("{kopium} is nested under a type k8s-openapi replaces, but has no table entry");
                },
                Placement::Common | Placement::K8s => continue,
            };
            if *resource != module.resource {
                bail!(
                    "{kopium} is placed in {resource}, but {module} generates it",
                    module = module.name()
                );
            }
            if !paths.roots.contains_key(resource) {
                bail!("{kopium} is placed in {resource}, which has no root version");
            }
        }
    }
    Ok(())
}

/// The local types `item` refers to, in order, without repeats.
fn local_refs(item: &syn::Item, locals: &BTreeMap<String, usize>) -> Vec<String> {
    struct Refs<'a> {
        locals: &'a BTreeMap<String, usize>,
        found: Vec<String>,
    }
    impl<'ast> Visit<'ast> for Refs<'_> {
        fn visit_type_path(&mut self, node: &'ast syn::TypePath) {
            if let Some(name) = local_name(node, self.locals)
                && !self.found.contains(&name)
            {
                self.found.push(name);
            }
            visit::visit_type_path(self, node);
        }
    }
    let mut refs = Refs {
        locals,
        found: Vec::new(),
    };
    refs.visit_item(item);
    refs.found
}

/// The name of the local type a type path refers to, if it does.
fn local_name(node: &syn::TypePath, locals: &BTreeMap<String, usize>) -> Option<String> {
    let name = single_name(node)?;
    locals.contains_key(&name).then_some(name)
}

/// The name a type path is, if it's a single name.
fn single_name(node: &syn::TypePath) -> Option<String> {
    node.path
        .get_ident()
        .filter(|_| node.qself.is_none())
        .map(ToString::to_string)
}

/// Fails unless every instance of each new name has the same shape.
fn check_shapes(
    modules: &[Module<'_>],
    items: &[BTreeMap<String, usize>],
    renames: &BTreeMap<String, Rename<'_>>,
    groups: &BTreeMap<&str, Vec<Site>>,
) -> Result<()> {
    let named = |name: &str| renames.get(name).map(|rename| rename.name.clone());
    let mut shapers: Vec<_> = modules
        .iter()
        .zip(items)
        .map(|(module, items)| Shaper::new(&module.file.items, items, &named))
        .collect();

    for (name, sites) in groups {
        let [first, rest @ ..] = &sites[..] else {
            continue;
        };
        let first_shape = shapers[first.module].shape_of(&first.kopium);
        for site in rest {
            if shapers[site.module].shape_of(&site.kopium) != first_shape {
                let describe = |site: &Site| format!("{} ({})", site.kopium, modules[site.module].name());
                let item = |site: &Site| &modules[site.module].file.items[site.item];
                bail!(
                    "{name}: {first} and {other} differ:\n{diff}",
                    first = describe(first),
                    other = describe(site),
                    diff = diff_items(item(first), item(site)),
                );
            }
        }
    }
    Ok(())
}

/// Computes item shapes, for comparison. An item's shape is its tokens
/// without docs, `kube` attributes or its own name. Each local type it refers
/// to becomes the name `named` gives it, or else that type's own shape.
struct Shaper<'a> {
    file: &'a [syn::Item],
    items: &'a BTreeMap<String, usize>,
    named: &'a dyn Fn(&str) -> Option<String>,
    memo: HashMap<String, String>,
}

impl<'a> Shaper<'a> {
    fn new(
        file: &'a [syn::Item],
        items: &'a BTreeMap<String, usize>,
        named: &'a dyn Fn(&str) -> Option<String>,
    ) -> Self {
        Self {
            file,
            items,
            named,
            memo: HashMap::new(),
        }
    }

    /// The shape of the local type `name`.
    fn shape_of(&mut self, name: &str) -> String {
        if let Some(shape) = self.memo.get(name) {
            return shape.clone();
        }
        let file = self.file;
        let shape = self.shape(&file[self.items[name]]);
        self.memo.insert(name.to_string(), shape.clone());
        shape
    }

    fn shape(&mut self, item: &syn::Item) -> String {
        struct Replace<'a> {
            items: &'a BTreeMap<String, usize>,
            named: &'a dyn Fn(&str) -> Option<String>,
            found: Vec<String>,
        }
        impl VisitMut for Replace<'_> {
            fn visit_type_path_mut(&mut self, node: &mut syn::TypePath) {
                if let Some(name) = local_name(node, self.items) {
                    let replacement = (self.named)(&name).unwrap_or_else(|| {
                        self.found.push(name);
                        "__Ref".to_string()
                    });
                    node.path.segments[0].ident = ident(&replacement);
                }
                visit_mut::visit_type_path_mut(self, node);
            }
        }

        let mut item = item.clone();
        strip_attrs(&mut item);
        if let Some(name) = item_ident_mut(&mut item) {
            *name = ident("__Shape");
        }
        let mut replace = Replace {
            items: self.items,
            named: self.named,
            found: Vec::new(),
        };
        replace.visit_item_mut(&mut item);

        let mut shape = item.to_token_stream().to_string();
        for name in replace.found {
            shape = format!("{shape} [{nested}]", nested = self.shape_of(&name));
        }
        shape
    }
}

/// Removes the attributes that aren't part of a shape: docs everywhere, and
/// `kube` on the item.
fn strip_attrs(item: &mut syn::Item) {
    let keep = |attr: &syn::Attribute| !attr.path().is_ident("doc") && !attr.path().is_ident("kube");
    match item {
        syn::Item::Struct(item) => {
            item.attrs.retain(keep);
            for field in &mut item.fields {
                field.attrs.retain(keep);
            }
        },
        syn::Item::Enum(item) => {
            item.attrs.retain(keep);
            for variant in &mut item.variants {
                variant.attrs.retain(keep);
                for field in &mut variant.fields {
                    field.attrs.retain(keep);
                }
            }
        },
        _ => {},
    }
}

/// The lines of two items, without docs, that only one of them has.
fn diff_items(a: &syn::Item, b: &syn::Item) -> String {
    let lines = |item: &syn::Item| {
        let mut item = item.clone();
        strip_attrs(&mut item);
        if let Some(name) = item_ident_mut(&mut item) {
            *name = ident("_");
        }
        crate::print_items(&[item])
            .lines()
            .map(str::to_string)
            .collect::<Vec<_>>()
    };
    let (a, b) = (lines(a), lines(b));
    let only = |sign: char, these: &[String], those: &[String]| {
        these
            .iter()
            .filter(|line| !those.contains(line))
            .fold(String::new(), |mut out, line| {
                writeln!(out, "{sign} {line}").unwrap();
                out
            })
    };
    format!("{}{}", only('-', &a, &b), only('+', &b, &a))
}

/// Builds the types placed at `placement`: each table entry placed there, in
/// table order, then the types nested under it. Their references still use
/// kopium names.
fn emit_types(
    table: &[SharedType],
    placement: &Placement,
    modules: &[Module<'_>],
    items: &[BTreeMap<String, usize>],
    renames: &BTreeMap<String, Rename<'_>>,
    groups: &BTreeMap<&str, Vec<Site>>,
) -> Vec<syn::Item> {
    struct Emitter<'a, 't> {
        modules: &'a [Module<'a>],
        items: &'a [BTreeMap<String, usize>],
        renames: &'a BTreeMap<String, Rename<'t>>,
        groups: &'a BTreeMap<&'a str, Vec<Site>>,
        emitted: BTreeSet<String>,
        out: Vec<syn::Item>,
    }

    impl Emitter<'_, '_> {
        fn emit(&mut self, name: &str, entry: Option<&SharedType>) {
            if !self.emitted.insert(name.to_string()) {
                return;
            }
            let sites = &self.groups[name];
            // A shared type's own doc starts with its Go name; others are a
            // field's.
            let site = entry
                .and_then(|entry| sites.iter().find(|site| self.doc_starts_with(site, &entry.go)))
                .unwrap_or(&sites[0]);
            let original = &self.modules[site.module].file.items[site.item];

            let mut item = original.clone();
            if let Some(slot) = item_ident_mut(&mut item) {
                *slot = ident(name);
            }
            if let Some(entry) = entry {
                add_upstream_doc(&mut item, &entry.go);
            }
            self.out.push(item);

            for child in local_refs(original, &self.items[site.module]) {
                if let Some(rename) = self.renames.get(&child)
                    && rename.entry.is_none()
                {
                    self.emit(&rename.name.clone(), None);
                }
            }
            let orphans: Vec<String> = self
                .renames
                .values()
                .filter(|rename| rename.orphan_of.as_deref() == Some(name))
                .map(|rename| rename.name.clone())
                .collect();
            for orphan in orphans {
                self.emit(&orphan, None);
            }
        }

        fn doc_starts_with(&self, site: &Site, go: &str) -> bool {
            let item = &self.modules[site.module].file.items[site.item];
            let attrs = match item {
                syn::Item::Struct(item) => &item.attrs,
                syn::Item::Enum(item) => &item.attrs,
                _ => return false,
            };
            attrs
                .iter()
                .find_map(doc_line)
                .is_some_and(|line| line.trim_start().starts_with(go))
        }
    }

    let mut emitter = Emitter {
        modules,
        items,
        renames,
        groups,
        emitted: BTreeSet::new(),
        out: Vec::new(),
    };
    for entry in table.iter().filter(|entry| entry.placement == *placement) {
        let name = entry.rust_name();
        if groups.contains_key(name.as_str()) {
            emitter.emit(&name, Some(entry));
        }
    }
    emitter.out
}

/// The text of a `#[doc = "..."]` attribute.
fn doc_line(attr: &syn::Attribute) -> Option<String> {
    let syn::Meta::NameValue(meta) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(doc),
        ..
    }) = &meta.value
    else {
        return None;
    };
    meta.path.is_ident("doc").then(|| doc.value())
}

/// Adds a doc line naming the upstream Go type, so it can be searched for.
fn add_upstream_doc(item: &mut syn::Item, go: &str) {
    let attrs = match item {
        syn::Item::Struct(item) => &mut item.attrs,
        syn::Item::Enum(item) => &mut item.attrs,
        _ => return,
    };
    let at = attrs
        .iter()
        .rposition(|attr| doc_line(attr).is_some())
        .map_or(0, |last| last + 1);
    let line = format!(" Upstream: `{go}`");
    let mut doc: Vec<syn::Attribute> = vec![syn::parse_quote!(#[doc = #line])];
    if at > 0 {
        doc.insert(0, syn::parse_quote!(#[doc = ""]));
    }
    attrs.splice(at..at, doc);
}

/// Re-exports, in each non-root version, the types that are Go aliases of
/// the root version's: each type the root version also has, apart from the
/// spec, which kube needs per version, those consolidation replaces, and those
/// `per_version` lists. Fails if one's shape differs from the root version's.
fn reexport_root_types<'t>(
    modules: &mut [Module<'_>],
    items: &[BTreeMap<String, usize>],
    renames: &BTreeMap<String, Rename<'_>>,
    per_version: &'t [PerVersion],
    matched: &mut BTreeSet<(usize, &'t str)>,
) -> Result<()> {
    let named = |name: &str| renames.get(name).map(|rename| rename.name.clone());
    let roots: BTreeMap<&str, usize> = modules
        .iter()
        .enumerate()
        .filter(|(_, module)| module.root)
        .map(|(index, module)| (module.resource.as_str(), index))
        .collect();

    let mut reexports = Vec::new();
    {
        let mut shapers: Vec<_> = modules
            .iter()
            .zip(items)
            .map(|(module, items)| Shaper::new(&module.file.items, items, &named))
            .collect();
        for (index, module) in modules.iter().enumerate().filter(|(_, module)| !module.root) {
            let root = roots[module.resource.as_str()];
            let own: Vec<(usize, &str)> = per_version
                .iter()
                .enumerate()
                .filter(|(_, entry)| entry.resource == module.resource && entry.version == module.version)
                .flat_map(|(entry, types)| types.types.iter().map(move |name| (entry, name.as_str())))
                .collect();
            for (name, &item) in &items[index] {
                if let Some(&(entry, name)) = own.iter().find(|(_, own)| own == name) {
                    matched.insert((entry, name));
                    continue;
                }
                if renames.contains_key(name) || is_spec(&module.file.items[item]) || !items[root].contains_key(name) {
                    continue;
                }
                if shapers[index].shape_of(name) != shapers[root].shape_of(name) {
                    let root_module = &modules[root];
                    bail!(
                        "{module}: {name} differs from {root}'s, so it isn't a Go alias of it. If upstream declares \
                         it in {version}, add it to per_version in xtask/shared-types.yaml.\n{diff}",
                        module = module.name(),
                        root = root_module.version,
                        version = module.version,
                        diff = diff_items(&root_module.file.items[items[root][name]], &module.file.items[item]),
                    );
                }
                reexports.push((index, item, name.clone(), modules[root].version.clone()));
            }
        }
    }

    // One re-export per module, where the first of its types was
    let mut by_module: BTreeMap<usize, (String, Vec<(usize, String)>)> = BTreeMap::new();
    for (module, item, name, root) in reexports {
        by_module
            .entry(module)
            .or_insert_with(|| (root, Vec::new()))
            .1
            .push((item, name));
    }
    for (module, (root, mut types)) in by_module {
        types.sort();
        let items = &mut modules[module].file.items;
        let list = types
            .iter()
            .map(|(_, name)| name.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        let reexport = syn::parse_str(&format!("pub use super::{root}::{{{list}}};")).expect("a re-export parses");
        for &(item, _) in types.iter().skip(1).rev() {
            items.remove(item);
        }
        items[types[0].0] = reexport;
    }
    Ok(())
}

/// Whether `item` is a resource's spec, which kube derives the resource from.
fn is_spec(item: &syn::Item) -> bool {
    matches!(item, syn::Item::Struct(item) if item.attrs.iter().any(|attr| attr.path().is_ident("kube")))
}

/// Replaces each renamed type with a deprecated alias to its new name, puts
/// the types placed in each resource in its root version module, where the
/// first of them was, and points every module at the new names.
fn replace_sites(
    modules: &mut [Module<'_>],
    renames: &BTreeMap<String, Rename<'_>>,
    mut defs: BTreeMap<String, Vec<syn::Item>>,
    paths: &Paths<'_>,
) -> Result<()> {
    for module in modules {
        let placement = module.placement();
        let name = module.name();
        let items = &mut module.file.items;

        let mut first_own = None;
        for (index, item) in items.iter_mut().enumerate() {
            let Some(old) = item_ident(item).map(ToString::to_string) else {
                continue;
            };
            let Some(rename) = renames.get(&old) else {
                continue;
            };
            if placement.as_ref() == Some(&rename.placement) {
                first_own.get_or_insert(index);
            }
            let old = ident(&old);
            let note = format!("use `{}`", paths.public(&rename.placement, &rename.name));
            let target = path(&format!("{}::{}", paths.crate_module(&rename.placement), rename.name));
            *item = syn::parse_quote! {
                #[doc(hidden)]
                #[deprecated(note = #note)]
                pub type #old = #target;
            };
        }

        if let Some(Placement::Resource(resource)) = &placement
            && let Some(types) = defs.remove(resource)
        {
            let Some(at) = first_own else {
                bail!("{name} is {resource}'s root version, but generates none of the types placed in it");
            };
            items.splice(at..at, types);
        }
        rewrite_module(items, placement.as_ref(), renames, paths)?;
    }
    if let Some(resource) = defs.keys().next() {
        bail!("types are placed in {resource}, but it has no root version module");
    }
    Ok(())
}

/// Points references in `items`, a module whose own types are placed at
/// `here`, at the new names, and imports the ones defined elsewhere.
fn rewrite_module(
    items: &mut Vec<syn::Item>,
    here: Option<&Placement>,
    renames: &BTreeMap<String, Rename<'_>>,
    paths: &Paths<'_>,
) -> Result<()> {
    let local: BTreeSet<String> = items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Type(item) => Some(item.ident.to_string()),
            item => item_ident(item).map(ToString::to_string),
        })
        .collect();
    let mut imports: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut path_to = |rename: &Rename<'_>| -> syn::Path {
        let name = &rename.name;
        if here == Some(&rename.placement) {
            return path(name);
        }
        let module = paths.crate_module(&rename.placement);
        // A kopium name can equal a new name, and its alias takes that name
        // in the module.
        if local.contains(name) {
            path(&format!("{module}::{name}"))
        } else {
            imports.entry(module).or_default().insert(name.clone());
            path(name)
        }
    };

    for item in items.iter_mut() {
        if item_ident(item).is_some() {
            rewrite_refs(item, &mut |old| Some(path_to(renames.get(old)?)));
            rewrite_status(item, &mut path_to, renames)?;
        }
    }

    let at = items
        .iter()
        .position(|item| matches!(item, syn::Item::Use(_)))
        .map_or(0, |prelude| prelude + 1);
    let imports = imports.into_iter().map(|(module, names)| {
        let names = names.into_iter().collect::<Vec<_>>().join(", ");
        syn::parse_str::<syn::Item>(&format!("use {module}::{{{names}}};")).expect("an import parses")
    });
    items.splice(at..at, imports);
    Ok(())
}

/// Rewrites each single-name type path that `rewrite` gives a new path for.
fn rewrite_refs(item: &mut syn::Item, rewrite: &mut dyn FnMut(&str) -> Option<syn::Path>) {
    struct Rewrite<'a>(&'a mut dyn FnMut(&str) -> Option<syn::Path>);
    impl VisitMut for Rewrite<'_> {
        fn visit_type_path_mut(&mut self, node: &mut syn::TypePath) {
            visit_mut::visit_type_path_mut(self, node);
            if let Some(path) = single_name(node).and_then(|name| (self.0)(&name)) {
                node.path = path;
            }
        }
    }
    Rewrite(rewrite).visit_item_mut(item);
}

/// Points `#[kube(status = "...")]` at a renamed status type.
fn rewrite_status(
    item: &mut syn::Item,
    path_to: &mut dyn FnMut(&Rename<'_>) -> syn::Path,
    renames: &BTreeMap<String, Rename<'_>>,
) -> Result<()> {
    let syn::Item::Struct(item) = item else {
        return Ok(());
    };
    for attr in item.attrs.iter_mut().filter(|attr| attr.path().is_ident("kube")) {
        let mut status = None;
        attr.parse_nested_meta(|meta| {
            if meta.input.peek(syn::Token![=]) {
                let value: syn::LitStr = meta.value()?.parse()?;
                if meta.path.is_ident("status") {
                    status = Some(value.value());
                }
            }
            Ok(())
        })?;
        if let Some(rename) = status.and_then(|status| renames.get(&status)) {
            let path = path_to(rename).to_token_stream().to_string().replace(' ', "");
            *attr = syn::parse_quote!(#[kube(status = #path)]);
        }
    }
    Ok(())
}

/// Fails on any group of shape-identical types in a channel that `distinct`
/// doesn't list. Types `named` names are compared by name, not shape. Adds
/// the index of each `distinct` group that matched to `matched`.
pub fn check_duplicates(
    channel: &str,
    files: &[&[syn::Item]],
    named: &BTreeSet<String>,
    distinct: &[Vec<String>],
    matched: &mut BTreeSet<usize>,
) -> Result<()> {
    let named = |name: &str| named.contains(name).then(|| name.to_string());
    let mut groups: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for file in files {
        let items = item_indices(file);
        let mut shaper = Shaper::new(file, &items, &named);
        for name in items.keys() {
            groups.entry(shaper.shape_of(name)).or_default().insert(name.clone());
        }
    }
    let generated: BTreeSet<&str> = groups.values().flatten().map(String::as_str).collect();

    let mut unclassified = Vec::new();
    for names in groups.values().filter(|names| names.len() > 1) {
        let found = distinct.iter().position(|group| {
            let group: BTreeSet<&str> = group
                .iter()
                .map(String::as_str)
                .filter(|name| generated.contains(name))
                .collect();
            group.len() == names.len() && names.iter().all(|name| group.contains(name.as_str()))
        });
        match found {
            Some(index) => {
                matched.insert(index);
            },
            None => unclassified.push(names.iter().cloned().collect::<Vec<_>>().join(", ")),
        }
    }
    if !unclassified.is_empty() {
        let groups = unclassified.join("\n  ");
        bail!(
            "{channel}: these types share a shape, but neither shared_types nor distinct in \
             xtask/shared-types.yaml classifies them:\n  {groups}"
        );
    }
    Ok(())
}

pub fn ident(name: &str) -> syn::Ident {
    syn::Ident::new(name, Span::call_site())
}

/// Parses a path built from names the generator knows are valid.
fn path(path: &str) -> syn::Path {
    syn::parse_str(path).expect("a generated path parses")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_types::Placement;

    fn shared_type(go: &str, placement: Placement, kopium: &[&str]) -> SharedType {
        SharedType {
            go: go.into(),
            rust: None,
            placement,
            kopium: kopium.iter().map(ToString::to_string).collect(),
        }
    }

    fn table() -> Vec<SharedType> {
        vec![shared_type(
            "ParentRef",
            Placement::Common,
            &["AParent", "BParents", "CParent"],
        )]
    }

    fn module_a() -> syn::File {
        syn::parse_quote! {
            use self::prelude::*;
            pub struct ASpec {
                pub parent: Option<AParent>,
            }
            /// Field docs, which don't start with the Go name.
            pub struct AParent {
                /// Name of the parent.
                pub name: String,
                pub kind: AParentKind,
            }
            pub enum AParentKind {
                Gateway,
            }
        }
    }

    fn module_b() -> syn::File {
        syn::parse_quote! {
            use self::prelude::*;
            pub struct BSpec {
                pub parents: Vec<BParents>,
            }
            /// ParentRef identifies a parent.
            pub struct BParents {
                /// Different docs don't change the shape.
                pub name: String,
                pub kind: BParentsKind,
            }
            pub enum BParentsKind {
                Gateway,
            }
        }
    }

    /// Consolidates one module per resource: `a`, `b` and so on, each only a
    /// root version `v1`.
    fn run(files: &mut [syn::File]) -> Result<(Vec<syn::Item>, BTreeSet<String>)> {
        let resources = ["a", "b", "c"];
        let modules = files
            .iter_mut()
            .zip(resources)
            .map(|(file, resource)| module(resource, "v1", true, file))
            .collect();
        let (consolidated, matched) = run_with(&table(), modules)?;
        Ok((consolidated.common, matched))
    }

    fn module<'a>(resource: &str, version: &str, root: bool, file: &'a mut syn::File) -> Module<'a> {
        Module {
            resource: resource.into(),
            version: version.into(),
            root,
            file,
        }
    }

    fn run_with(shared_types: &[SharedType], modules: Vec<Module<'_>>) -> Result<(Consolidated, BTreeSet<String>)> {
        let table = Table {
            shared_types: shared_types.to_vec(),
            distinct: Vec::new(),
            per_version: Vec::new(),
        };
        run_table(&table, modules)
    }

    fn run_table(table: &Table, mut modules: Vec<Module<'_>>) -> Result<(Consolidated, BTreeSet<String>)> {
        let mut matched = Matched::default();
        let consolidated = consolidate("standard", table, &mut modules, &mut matched)?;
        Ok((consolidated, matched.kopium.into_iter().map(String::from).collect()))
    }

    fn item<'a>(items: &'a [syn::Item], name: &str) -> &'a syn::Item {
        items
            .iter()
            .find(|item| match item {
                syn::Item::Type(item) => item.ident == name,
                item => item_ident(item).is_some_and(|ident| ident == name),
            })
            .unwrap_or_else(|| panic!("no item {name}"))
    }

    fn tokens(item: &syn::Item) -> String {
        item.to_token_stream().to_string()
    }

    #[test]
    fn consolidates_table_entries_and_their_nested_types() {
        let mut files = [module_a(), module_b()];
        let (common, matched) = run(&mut files).unwrap();

        assert_eq!(
            item_indices(&common).into_keys().collect::<BTreeSet<_>>(),
            BTreeSet::from(["ParentRef".into(), "ParentRefKind".into()])
        );
        assert_eq!(matched, BTreeSet::from(["AParent".into(), "BParents".into()]));

        // Docs come from the instance whose doc names the Go type, plus the
        // upstream line.
        let parent = tokens(item(&common, "ParentRef"));
        assert!(parent.contains("ParentRef identifies a parent."), "{parent}");
        assert!(parent.contains("Upstream: `ParentRef`"), "{parent}");
        assert!(parent.contains("pub kind : ParentRefKind"), "{parent}");

        let [a, b] = &files;
        assert!(tokens(item(&a.items, "ASpec")).contains("Option < ParentRef >"));
        assert!(tokens(item(&b.items, "BSpec")).contains("Vec < ParentRef >"));
        let alias = tokens(item(&a.items, "AParentKind"));
        assert!(
            alias.contains("pub type AParentKind = crate :: apis :: standard :: common :: ParentRefKind"),
            "{alias}"
        );
        assert!(alias.contains("use `gateway_api::common::ParentRefKind`"), "{alias}");
        let import = tokens(&a.items[1]);
        assert_eq!(import, "use crate :: apis :: standard :: common :: { ParentRef } ;");
    }

    #[test]
    fn fails_when_instances_differ() {
        let mut b = module_b();
        b.items[2] = syn::parse_quote! {
            /// ParentRef identifies a parent.
            pub struct BParents {
                pub name: String,
                pub kind: BParentsKind,
                pub port: Option<i32>,
            }
        };
        let error = run(&mut [module_a(), b])
            .err()
            .expect("consolidation should fail")
            .to_string();
        assert!(
            error.starts_with("ParentRef: AParent (a v1) and BParents (b v1) differ"),
            "{error}"
        );
        assert!(error.contains("+     pub port: Option<i32>,"), "{error}");
    }

    #[test]
    fn fails_when_nested_instances_differ() {
        let mut b = module_b();
        b.items[3] = syn::parse_quote! {
            pub enum BParentsKind {
                Gateway,
                Service,
            }
        };
        let error = run(&mut [module_a(), b])
            .err()
            .expect("consolidation should fail")
            .to_string();
        assert!(
            error.starts_with("ParentRefKind: AParentKind (a v1) and BParentsKind (b v1) differ"),
            "{error}"
        );
    }

    #[test]
    fn fails_on_a_nested_type_its_parent_doesnt_prefix() {
        let mut a = module_a();
        a.items[2] = syn::parse_quote! {
            pub struct AParent {
                pub name: String,
                pub kind: Kind,
            }
        };
        a.items[3] = syn::parse_quote!(
            pub enum Kind {
                Gateway,
            }
        );
        let error = run(&mut [a]).err().expect("consolidation should fail").to_string();
        assert_eq!(error, "Kind, nested under AParent, doesn't start with its name");
    }

    fn check(files: &[syn::File], distinct: &[&[&str]]) -> Result<BTreeSet<usize>> {
        let files: Vec<&[syn::Item]> = files.iter().map(|file| &file.items[..]).collect();
        let distinct: Vec<Vec<String>> = distinct
            .iter()
            .map(|group| group.iter().map(ToString::to_string).collect())
            .collect();
        let mut matched = BTreeSet::new();
        check_duplicates("standard", &files, &BTreeSet::new(), &distinct, &mut matched)?;
        Ok(matched)
    }

    #[test]
    fn duplicate_guard_compares_nested_types_by_shape() {
        // The `kind` fields name different enums with one shape
        let files = [module_a(), module_b()];
        let error = check(&files, &[]).unwrap_err().to_string();
        let groups: BTreeSet<&str> = error.lines().skip(1).map(str::trim).collect();
        // `ASpec` refers to one `AParent`, `BSpec` to a list of them
        assert_eq!(
            groups,
            BTreeSet::from(["AParent, BParents", "AParentKind, BParentsKind"])
        );
    }

    #[test]
    fn duplicate_guard_accepts_distinct_groups() {
        let files = [module_a(), module_b()];
        let distinct: &[&[&str]] = &[
            &["AParent", "BParents", "CParent"],
            &["AParentKind", "BParentsKind"],
            &["Stale", "Group"],
        ];
        assert_eq!(check(&files, distinct).unwrap(), BTreeSet::from([0, 1]));
    }

    #[test]
    fn duplicate_guard_needs_the_whole_group() {
        let files = [module_a(), module_b()];
        let distinct: &[&[&str]] = &[&["AParent", "BParents", "ASpec"], &["AParentKind", "BParentsKind"]];
        let error = check(&files, distinct).unwrap_err().to_string();
        let groups: Vec<&str> = error.lines().skip(1).map(str::trim).collect();
        assert_eq!(groups, ["AParent, BParents"]);
    }

    fn module_with_filters() -> syn::File {
        syn::parse_quote! {
            use self::prelude::*;
            pub struct ARules {
                pub filters: Vec<ARulesFilters>,
                pub backend_refs: Vec<ARulesBackendRefs>,
            }
            /// Filter defines a filter.
            pub struct ARulesFilters {
                pub kind: ARulesFiltersKind,
            }
            pub enum ARulesFiltersKind {
                Header,
            }
            pub struct ARulesBackendRefs {
                pub filters: Vec<ARulesBackendRefsFilters>,
            }
            pub struct ARulesBackendRefsFilters {
                pub kind: ARulesBackendRefsFiltersKind,
            }
            pub enum ARulesBackendRefsFiltersKind {
                Header,
            }
        }
    }

    #[test]
    fn places_resource_types_in_its_root_version() {
        let table = [shared_type(
            "Filter",
            Placement::Resource("a".into()),
            &["ARulesFilters", "ARulesBackendRefsFilters"],
        )];
        let (mut v1, mut v1beta1) = (module_with_filters(), module_with_filters());
        // A type only this version has
        v1beta1.items.push(syn::parse_quote! {
            pub struct ARulesExtra {
                pub filter: ARulesFilters,
            }
        });
        let modules = vec![
            module("a", "v1", true, &mut v1),
            module("a", "v1beta1", false, &mut v1beta1),
        ];
        let (consolidated, _) = run_with(&table, modules).unwrap();
        assert!(consolidated.common.is_empty());
        assert_eq!(
            consolidated.names,
            BTreeSet::from(["Filter".into(), "FilterKind".into()])
        );

        // The root version defines the types where the first of them was,
        // and refers to them by name
        let names: Vec<String> = v1
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Type(item) => Some(format!("alias {}", item.ident)),
                item => item_ident(item).map(ToString::to_string),
            })
            .collect();
        assert_eq!(
            names,
            [
                "ARules",
                "Filter",
                "FilterKind",
                "alias ARulesFilters",
                "alias ARulesFiltersKind",
                "ARulesBackendRefs",
                "alias ARulesBackendRefsFilters",
                "alias ARulesBackendRefsFiltersKind",
            ]
        );
        assert!(tokens(item(&v1.items, "ARules")).contains("Vec < Filter >"));
        assert!(tokens(item(&v1.items, "Filter")).contains("pub kind : FilterKind"));
        assert!(!v1.items.iter().any(|item| tokens(item).starts_with("use crate")));

        // Other versions import them from the root version, and re-export
        // the types they share with it
        let alias = tokens(item(&v1beta1.items, "ARulesFilters"));
        assert!(
            alias.contains("= crate :: apis :: standard :: a :: v1 :: Filter"),
            "{alias}"
        );
        assert!(alias.contains("use `gateway_api::a::Filter`"), "{alias}");
        assert_eq!(
            tokens(&v1beta1.items[1]),
            "use crate :: apis :: standard :: a :: v1 :: { Filter } ;"
        );
        assert!(tokens(item(&v1beta1.items, "ARulesExtra")).contains("pub filter : Filter"));
        assert!(
            v1beta1
                .items
                .iter()
                .any(|item| tokens(item) == "pub use super :: v1 :: { ARules , ARulesBackendRefs } ;")
        );
    }

    /// A resource `a` with a root version `v1` and a `v1beta1` whose `ARules`
    /// has an extra field when `differ` is set.
    fn run_versions(differ: bool, per_version: &[&str]) -> Result<(syn::File, BTreeSet<(usize, String)>)> {
        let mut v1: syn::File = syn::parse_quote! {
            use self::prelude::*;
            /// Spec.
            #[kube(group = "example.com")]
            pub struct ASpec {
                pub rules: Vec<ARules>,
            }
            pub struct ARules {
                pub name: String,
            }
        };
        let mut v1beta1 = v1.clone();
        if differ {
            v1beta1.items[2] = syn::parse_quote! {
                pub struct ARules {
                    pub name: String,
                    pub port: i32,
                }
            };
        }
        let table = Table {
            shared_types: Vec::new(),
            distinct: Vec::new(),
            per_version: vec![PerVersion {
                resource: "a".into(),
                version: "v1beta1".into(),
                types: per_version.iter().map(ToString::to_string).collect(),
            }],
        };
        let mut modules = vec![
            module("a", "v1", true, &mut v1),
            module("a", "v1beta1", false, &mut v1beta1),
        ];
        let mut matched = Matched::default();
        consolidate("standard", &table, &mut modules, &mut matched)?;
        let matched = matched
            .per_version
            .into_iter()
            .map(|(entry, name)| (entry, name.to_string()))
            .collect();
        Ok((v1beta1, matched))
    }

    #[test]
    fn reexports_root_types_but_not_the_spec() {
        let (v1beta1, matched) = run_versions(false, &[]).unwrap();
        let items: Vec<String> = v1beta1.items.iter().skip(1).map(tokens).collect();
        assert!(
            items[0].ends_with("pub struct ASpec { pub rules : Vec < ARules > , }"),
            "{}",
            items[0]
        );
        assert_eq!(items[1], "pub use super :: v1 :: { ARules } ;");
        assert!(matched.is_empty());
    }

    #[test]
    fn keeps_per_version_types() {
        let (v1beta1, matched) = run_versions(true, &["ARules"]).unwrap();
        assert!(tokens(&v1beta1.items[2]).contains("pub port : i32"));
        assert_eq!(matched, BTreeSet::from([(0, "ARules".into())]));
    }

    #[test]
    fn fails_when_a_shared_type_differs_from_the_root_version() {
        let error = run_versions(true, &[])
            .err()
            .expect("consolidation should fail")
            .to_string();
        assert!(
            error.starts_with("a v1beta1: ARules differs from v1's, so it isn't a Go alias of it."),
            "{error}"
        );
        assert!(error.contains("+     pub port: i32,"), "{error}");
    }

    #[test]
    fn fails_on_a_resource_type_another_resource_generates() {
        let table = [shared_type(
            "Filter",
            Placement::Resource("b".into()),
            &["ARulesFilters", "ARulesBackendRefsFilters"],
        )];
        let mut a = module_with_filters();
        let mut b = syn::parse_quote!(
            use self::prelude::*;
        );
        let modules = vec![module("a", "v1", true, &mut a), module("b", "v1", true, &mut b)];
        let error = run_with(&table, modules).err().expect("consolidation should fail");
        assert_eq!(
            error.to_string(),
            "ARulesBackendRefsFilters is placed in b, but a v1 generates it"
        );
    }

    #[test]
    fn moves_unused_types_with_the_longest_prefixing_parent() {
        let mut a = module_a();
        // Like kopium's enum for an integer field's values
        a.items.push(syn::parse_quote! {
            pub enum AParentKindCode {
                Ok,
            }
        });
        // Prefixed by no renamed type
        a.items.push(syn::parse_quote! {
            pub enum AOther {
                Ok,
            }
        });
        let (common, _) = run(&mut [a]).unwrap();
        let names: Vec<String> = common
            .iter()
            .filter_map(|item| item_ident(item).map(ToString::to_string))
            .collect();
        assert_eq!(names, ["ParentRef", "ParentRefKind", "ParentRefKindCode"]);
    }

    #[test]
    fn replaces_k8s_types_with_k8s_openapis() {
        let k8s = |go: &str, rust: &str, kopium| SharedType {
            rust: Some(rust.into()),
            ..shared_type(go, Placement::K8s, kopium)
        };
        let table = [
            k8s("metav1.Ref", "Ref", &["AParent"]),
            k8s("metav1.Kind", "Kind", &["AParentKind"]),
        ];
        let mut a = module_a();
        let (consolidated, _) = run_with(&table, vec![module("a", "v1", true, &mut a)]).unwrap();
        assert!(consolidated.common.is_empty());

        let alias = tokens(item(&a.items, "AParent"));
        assert!(
            alias.contains("= k8s_openapi :: apimachinery :: pkg :: apis :: meta :: v1 :: Ref"),
            "{alias}"
        );
        assert!(
            alias.contains("use `k8s_openapi::apimachinery::pkg::apis::meta::v1::Ref`"),
            "{alias}"
        );
        assert_eq!(
            tokens(&a.items[1]),
            "use k8s_openapi :: apimachinery :: pkg :: apis :: meta :: v1 :: { Ref } ;"
        );
    }

    #[test]
    fn fails_on_a_type_nested_under_a_k8s_type() {
        let table = [shared_type("metav1.Ref", Placement::K8s, &["AParent"])];
        let mut a = module_a();
        let error = run_with(&table, vec![module("a", "v1", true, &mut a)])
            .err()
            .expect("consolidation should fail");
        assert_eq!(
            error.to_string(),
            "AParentKind is nested under a type k8s-openapi replaces, but has no table entry"
        );
    }
}
