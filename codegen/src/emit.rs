//! Emits one profile's Rust modules from its [`Model`]: one module per Go
//! package, with one Rust type per Go type, in Go source order.

mod docs;
mod enums;
mod structs;
#[cfg(test)]
mod tests;

use std::{collections::BTreeMap, fmt::Write as _};

use anyhow::{Context, Result, bail};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use self::docs::{doc_attrs, replace_lead};
use crate::{
    index::{key, split_key},
    ir::{Package, TypeKind, TypeRef},
    model::{KindVersion, Model},
    naming::type_name,
};

/// The comment that heads every generated file.
pub const GENERATED_HEADER: &str = "// WARNING: generated file — do not edit\n";

/// A generated module: its path in the profile's tree, e.g. `apisx::v1alpha1`,
/// and its source.
pub struct Module {
    pub path: String,
    pub source: String,
}

/// Emits the modules of every package in the profile that has anything to
/// emit, in package order.
pub fn modules(model: &Model<'_>) -> Result<Vec<Module>> {
    let mut modules = Vec::new();
    for package in &model.index.ir.packages {
        let path = model.config.module(&package.path)?;
        let emitter = ModuleEmitter::new(model, package)
            .and_then(ModuleEmitter::emit)
            .with_context(|| format!("package {path}", path = package.path))?;
        if let Some(source) = emitter {
            modules.push(Module {
                path: path.to_string(),
                source,
            });
        }
    }
    Ok(modules)
}

/// A kind's spec struct in a package that doesn't declare the spec's Go type,
/// as `v1beta1` doesn't declare `HTTPRouteSpec`: kube needs one per version.
struct SpecCopy<'m, 'a> {
    kind: &'m KindVersion<'a>,
    name: String,
}

/// Emits one package's module.
struct ModuleEmitter<'m, 'a> {
    model: &'m Model<'a>,
    package: &'a Package,
    /// The kinds the package declares, by their Go key.
    kinds: BTreeMap<String, Vec<&'m KindVersion<'a>>>,
    /// Spec structs the package doesn't declare, by the Go key of their kind.
    copies: BTreeMap<String, SpecCopy<'m, 'a>>,
    /// The names the module defines or re-exports, and the Go key each
    /// stands for.
    locals: BTreeMap<String, String>,
    /// Re-exports of Go aliases: the alias's name and the Go key it stands
    /// for.
    reexports: Vec<(String, String)>,
    /// Imports, by name: the Go key and the Rust path.
    imports: BTreeMap<String, (String, String)>,
}

impl<'m, 'a> ModuleEmitter<'m, 'a> {
    fn new(model: &'m Model<'a>, package: &'a Package) -> Result<Self> {
        let mut emitter = Self {
            model,
            package,
            kinds: BTreeMap::new(),
            copies: BTreeMap::new(),
            locals: BTreeMap::new(),
            reexports: Vec::new(),
            imports: BTreeMap::new(),
        };
        for kind in model.kinds.iter().filter(|kind| kind.version.package == package.path) {
            emitter.kinds.entry(kind.kind.clone()).or_default().push(kind);
            let (spec_package, spec_name) = split_key(&kind.spec);
            if spec_package != package.path {
                let name = type_name(spec_name);
                // A type of the package's own with the name is fine if
                // nothing uses it, as with v1alpha3's TLSRouteSpec, which
                // v1alpha3.TLSRoute (`type TLSRoute v1.TLSRoute`) doesn't.
                if model.types.contains(&key(&package.path, spec_name)) {
                    bail!(
                        "{kind} needs a spec struct {name}, but the package declares its own",
                        kind = kind.kind
                    );
                }
                emitter.copies.insert(kind.kind.clone(), SpecCopy { kind, name });
            }
        }
        emitter.find_locals()?;
        Ok(emitter)
    }

    /// Records the names the module defines and re-exports.
    fn find_locals(&mut self) -> Result<()> {
        let model = self.model;
        // Only the profile's groups re-export, so a package of another group
        // has a module only if a kind uses its types.
        let in_groups = self
            .package
            .group
            .as_deref()
            .is_some_and(|group| model.groups().contains(group));
        let mut locals: Vec<(String, String)> = Vec::new();
        for ty in &self.package.types {
            let key = key(&self.package.path, &ty.name);
            let name = type_name(&ty.name);
            let alias_of_named = ty.alias.as_ref().and_then(|alias| alias.named.as_ref());
            if alias_of_named.is_some() {
                let target = model.index.resolve(&key);
                if in_groups && model.types.contains(&target) && !self.copies.values().any(|copy| copy.name == name) {
                    self.reexports.push((name.clone(), target.clone()));
                    locals.push((name, target));
                }
            } else if model.types.contains(&key) {
                locals.push((name, key));
            }
        }
        for (kind, copy) in &self.copies {
            locals.push((copy.name.clone(), format!("{kind}#spec")));
        }
        for kinds in self.kinds.values() {
            let kind = kinds[0];
            locals.push((type_name(&kind.crd.kind), kind.kind.clone()));
        }
        for (name, key) in locals {
            if let Some(other) = self.locals.insert(name.clone(), key.clone())
                && other != key
            {
                bail!("{name} would be both {other} and {key}");
            }
        }
        Ok(())
    }

    /// The module's source, or `None` if it has nothing to emit.
    fn emit(mut self) -> Result<Option<String>> {
        let items = self.items()?;
        if items.is_empty() && self.reexports.is_empty() {
            return Ok(None);
        }

        let mut uses = Vec::new();
        for (name, target) in &self.reexports {
            let path = self.full_path(target)?;
            let target_name = path.rsplit("::").next().expect("a path has a name");
            let use_path: syn::Path = syn::parse_str(&path)?;
            if target_name == name {
                uses.push(quote! { pub use #use_path; });
            } else {
                let name = format_ident!("{name}");
                uses.push(quote! { pub use #use_path as #name; });
            }
        }
        for (_, path) in self.imports.values() {
            let path: syn::Path = syn::parse_str(path)?;
            uses.push(quote! { use #path; });
        }

        let doc = doc_attrs(&self.module_doc(), true);
        let file: syn::File = syn::parse2(quote! {
            #doc
            #(#uses)*
            #(#items)*
        })?;
        Ok(Some(format!("{GENERATED_HEADER}{}", prettyplease::unparse(&file))))
    }

    /// The module's items, in Go source order.
    fn items(&mut self) -> Result<Vec<TokenStream>> {
        let mut items = Vec::new();
        for ty in &self.package.types {
            let key = key(&self.package.path, &ty.name);
            if let Some(copy) = self.copies.get(&key) {
                let kind = copy.kind;
                let (_, spec) = self.model.index.get(&kind.spec)?;
                let name = copy.name.clone();
                let doc = replace_lead(&spec.doc, &spec.name, &format!("[`{name}`]"));
                items.push(self.struct_item(&name, &kind.spec, &doc, Some(kind))?);
                continue;
            }
            if !self.model.types.contains(&key) {
                continue;
            }
            let name = type_name(&ty.name);
            let doc = replace_lead(&ty.doc, &ty.name, &format!("[`{name}`]"));
            let item = match ty.kind {
                TypeKind::Struct => {
                    let kind = self.spec_of(&key)?;
                    self.struct_item(&name, &key, &doc, kind)?
                },
                TypeKind::Scalar => {
                    if let Some(e) = self.model.enums.get(&key) {
                        self.enum_item(&name, &key, &doc, e)?
                    } else {
                        let basic = basic_type(ty.basic.as_deref().context("a scalar has a basic type")?)?;
                        type_alias(&name, &doc, &basic)
                    }
                },
                TypeKind::Alias => {
                    let alias = ty.alias.as_ref().context("an alias has a right-hand side")?;
                    let rust = self.rust_type(alias)?;
                    type_alias(&name, &doc, &rust)
                },
                TypeKind::Other => bail!("can't emit Go type {key}"),
            };
            items.push(item);
        }
        Ok(items)
    }

    /// The kind in this package whose spec is the struct `key`, if any.
    fn spec_of(&self, key: &str) -> Result<Option<&'m KindVersion<'a>>> {
        let mut kinds = self
            .kinds
            .values()
            .flatten()
            .filter(|kind| kind.spec == key && !self.copies.contains_key(&kind.kind));
        let kind = kinds.next().copied();
        if kinds.next().is_some() {
            bail!("more than one kind has the spec {key}");
        }
        Ok(kind)
    }

    /// The module's docs: the Go package, and the kinds its apiVersion
    /// makes available.
    fn module_doc(&self) -> String {
        let model = self.model;
        let ir = model.index.ir;
        let module = &ir.module;
        let path = &self.package.path;
        let version = &ir.version;
        let mut doc = format!("Generated from Go package `{module}/{path}` at {version}.\n");
        let Some(first) = self.kinds.values().flatten().next() else {
            return doc;
        };
        // A kind's version is its package's (gofacts finds a version's
        // package by its group and version name), so one apiVersion covers
        // every kind here and goes at the top, not on each item.
        let group = &first.crd.group;
        let api_version = &first.version.name;
        write!(doc, "\nKinds available in the `{group}/{api_version}` apiVersion:\n").expect("writing to a String");
        let yes_no = |b: bool| if b { "yes" } else { "no" };
        for kinds in self.kinds.values() {
            for kind in kinds {
                let v = kind.version;
                let root = type_name(&kind.crd.kind);
                let go_kind = &kind.crd.kind;
                let deprecated = match (&v.deprecation_warning, v.deprecated) {
                    (Some(warning), true) => {
                        let warning = warning.split_whitespace().collect::<Vec<_>>().join(" ");
                        format!("yes ({warning})")
                    },
                    _ => yes_no(v.deprecated).to_string(),
                };
                writeln!(
                    doc,
                    "\n- [`{root}`]\n  - kind: `{go_kind}`\n  - served: {served}\n  - storage: {storage}\n  - deprecated: \
                     {deprecated}",
                    served = yes_no(v.served),
                    storage = yes_no(v.storage),
                )
                .expect("writing to a String");
            }
        }
        doc
    }

    /// The full Rust path of a Go type.
    fn full_path(&self, key: &str) -> Result<String> {
        if let Some(path) = self.model.config.external(key) {
            return Ok(path.to_string());
        }
        let (package, _) = split_key(key);
        Ok(format!(
            "{module}::{name}",
            module = self.model.module_path(package)?,
            name = Model::rust_name(key)
        ))
    }

    /// The Rust path to use for a Go type in this module: its name where the
    /// module defines, re-exports or can import it, else its full path.
    fn path(&mut self, key: &str) -> Result<String> {
        let name = Model::rust_name(key);
        if self.model.config.external(key).is_none() && self.locals.get(&name).is_some_and(|local| local == key) {
            return Ok(name);
        }
        let full = self.full_path(key)?;
        if self.locals.contains_key(&name) {
            return Ok(full);
        }
        match self.imports.get(&name) {
            Some((imported, _)) if imported == key => Ok(name),
            Some(_) => Ok(full),
            None => {
                self.imports.insert(name.clone(), (key.to_string(), full));
                Ok(name)
            },
        }
    }

    /// The Rust type for a Go type expression.
    fn rust_type(&mut self, ty: &TypeRef) -> Result<TokenStream> {
        if let Some(named) = &ty.named {
            let key = self.model.index.resolve(named);
            let path: syn::Path = syn::parse_str(&self.path(&key)?)?;
            return Ok(quote! { #path });
        }
        if let Some(basic) = &ty.basic {
            return basic_type(basic);
        }
        if let Some(inner) = &ty.pointer {
            return self.rust_type(inner);
        }
        if let Some(inner) = &ty.slice {
            let inner = self.rust_type(inner)?;
            return Ok(quote! { Vec<#inner> });
        }
        if let Some(map) = &ty.map {
            self.check_map_key(&map.key)?;
            let key = self.rust_type(&map.key)?;
            let value = self.rust_type(&map.value)?;
            return Ok(quote! { std::collections::BTreeMap<#key, #value> });
        }
        bail!("empty Go type expression")
    }

    /// A map's key must be a string, or a named string type that isn't an
    /// enum, to be a `BTreeMap` key that serializes as a JSON object key.
    fn check_map_key(&self, key: &TypeRef) -> Result<()> {
        let model = self.model;
        let ok = match (&key.basic, &key.named) {
            (Some(basic), _) => basic == "string",
            (_, Some(named)) => {
                let resolved = model.index.resolve(named);
                !model.enums.contains_key(&resolved)
                    && model.index.find(&resolved).is_some_and(|(_, ty)| {
                        ty.basic.as_deref() == Some("string")
                            || ty.alias.as_ref().and_then(|alias| alias.basic.as_deref()) == Some("string")
                    })
            },
            _ => false,
        };
        if !ok {
            bail!("unsupported map key {key:?}");
        }
        Ok(())
    }
}

/// `pub type Name = Rust;`
fn type_alias(name: &str, doc: &str, rust: &TokenStream) -> TokenStream {
    let doc = doc_attrs(doc, false);
    let name = format_ident!("{name}");
    quote! {
        #doc
        pub type #name = #rust;
    }
}

/// The Rust type for a Go basic type. Integers map exactly.
fn basic_type(basic: &str) -> Result<TokenStream> {
    Ok(match basic {
        "string" => quote! { String },
        "bool" => quote! { bool },
        "int32" => quote! { i32 },
        "int" | "int64" => quote! { i64 },
        "uint16" => quote! { u16 },
        "uint32" => quote! { u32 },
        "uint64" => quote! { u64 },
        "float64" => quote! { f64 },
        _ => bail!("unsupported Go type {basic}"),
    })
}
