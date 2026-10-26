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

use crate::shared_types::{Placement, SharedType};

/// One generated module that takes part in consolidation.
pub struct Module<'a> {
    /// Where the module is, for errors, e.g. `httproutes v1`.
    pub name: String,
    pub file: &'a mut syn::File,
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
    /// The table entry, or `None` for a nested type that moved with its parent.
    entry: Option<&'t SharedType>,
}

/// Consolidates `modules`, the modules of one channel, by `table`. Ordered
/// root versions first, they decide which instance documents a shared type.
/// Returns the `common` module's types, and adds every kopium name `table`
/// matched to `matched`.
pub fn consolidate<'t>(
    channel: &str,
    table: &'t [SharedType],
    modules: &mut [Module<'_>],
    matched: &mut BTreeSet<&'t str>,
) -> Result<Vec<syn::Item>> {
    let items: Vec<_> = modules.iter().map(|module| item_indices(&module.file.items)).collect();
    let renames = plan_renames(table, modules, &items, matched)?;

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
    let common = emit_common(table, modules, &items, &renames, &groups);
    replace_sites(channel, modules, &renames)?;
    Ok(common)
}

/// Maps the name of each struct and enum in `items` to its index.
fn item_indices(items: &[syn::Item]) -> BTreeMap<String, usize> {
    items
        .iter()
        .enumerate()
        .filter_map(|(index, item)| Some((item_ident(item)?.to_string(), index)))
        .collect()
}

/// The names of the structs and enums in `items`.
pub fn type_names(items: &[syn::Item]) -> BTreeSet<String> {
    item_indices(items).into_keys().collect()
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
/// one. A nested type without its own table entry swaps its parent's kopium
/// name, which prefixes it, for the parent's new name.
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
                entry: Some(entry),
            };
            if let Some(other) = renames.insert(kopium.clone(), rename) {
                let (go, other_go) = (&entry.go, other.entry.map_or("", |entry| &entry.go));
                bail!("{kopium} is listed by both {other_go} and {go}");
            }
        }
    }

    let mut parents: Vec<String> = renames.keys().cloned().collect();
    while let Some(parent) = parents.pop() {
        let new_parent = renames[&parent].name.clone();
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
                        renames.insert(child.clone(), Rename { name, entry: None });
                        parents.push(child);
                    },
                }
            }
        }
    }
    Ok(renames)
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
                let describe = |site: &Site| format!("{} ({})", site.kopium, modules[site.module].name);
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

/// Builds the `common` module's types: each table entry placed there, in
/// table order, then the types nested under it.
fn emit_common(
    table: &[SharedType],
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
            let renames = self.renames;
            rewrite_refs(&mut item, &mut |old| Some(renamed_path(renames.get(old)?)));
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
    for entry in table.iter().filter(|entry| entry.placement == Placement::Common) {
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

/// The path a reference to a renamed type takes in the module it moved to.
fn renamed_path(rename: &Rename<'_>) -> syn::Path {
    let name = ident(&rename.name);
    syn::parse_quote!(#name)
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

/// Replaces each renamed type with a deprecated alias to its new name, points
/// the rest at the new names, and imports the ones they use.
fn replace_sites(channel: &str, modules: &mut [Module<'_>], renames: &BTreeMap<String, Rename<'_>>) -> Result<()> {
    let channel_ident = ident(channel);
    let public = crate::public_path(channel);

    for module in modules {
        let file = &mut *module.file;
        let local: BTreeSet<String> = item_indices(&file.items).into_keys().collect();
        let mut used = BTreeSet::new();
        let mut qualified = |rename: &Rename<'_>| -> syn::Path {
            let name = ident(&rename.name);
            // A kopium name can equal its new name, and its alias takes that
            // name in the module.
            if local.contains(&rename.name) {
                syn::parse_quote!(crate::apis::#channel_ident::common::#name)
            } else {
                used.insert(rename.name.clone());
                syn::parse_quote!(#name)
            }
        };

        for item in &mut file.items {
            let Some(old) = item_ident(item).map(ToString::to_string) else {
                continue;
            };
            if let Some(rename) = renames.get(&old) {
                let (old, name) = (ident(&old), ident(&rename.name));
                let note = format!("use `{public}::common::{name}`");
                *item = syn::parse_quote! {
                    #[doc(hidden)]
                    #[deprecated(note = #note)]
                    pub type #old = crate::apis::#channel_ident::common::#name;
                };
                continue;
            }
            rewrite_refs(item, &mut |old| Some(qualified(renames.get(old)?)));
            rewrite_status(item, &mut qualified, renames)?;
        }

        if !used.is_empty() {
            let names = used.iter().map(|name| ident(name));
            let import: syn::Item = syn::parse_quote!(use crate::apis::#channel_ident::common::{#(#names),*};);
            let at = file
                .items
                .iter()
                .position(|item| matches!(item, syn::Item::Use(_)))
                .map_or(0, |prelude| prelude + 1);
            file.items.insert(at, import);
        }
    }
    Ok(())
}

/// Points `#[kube(status = "...")]` at a renamed status type.
fn rewrite_status(
    item: &mut syn::Item,
    qualified: &mut dyn FnMut(&Rename<'_>) -> syn::Path,
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
            let path = qualified(rename).to_token_stream().to_string().replace(' ', "");
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shared_types::Placement;

    fn table() -> Vec<SharedType> {
        vec![SharedType {
            go: "ParentRef".into(),
            placement: Placement::Common,
            kopium: ["AParent", "BParents", "CParent"].map(String::from).to_vec(),
        }]
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

    fn run(files: &mut [syn::File]) -> Result<(Vec<syn::Item>, BTreeSet<String>)> {
        let mut modules: Vec<_> = files
            .iter_mut()
            .enumerate()
            .map(|(index, file)| Module {
                name: format!("module {index}"),
                file,
            })
            .collect();
        let table = table();
        let mut matched = BTreeSet::new();
        let common = consolidate("standard", &table, &mut modules, &mut matched)?;
        Ok((common, matched.into_iter().map(String::from).collect()))
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
            type_names(&common),
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
            error.starts_with("ParentRef: AParent (module 0) and BParents (module 1) differ"),
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
            error.starts_with("ParentRefKind: AParentKind (module 0) and BParentsKind (module 1) differ"),
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
}
