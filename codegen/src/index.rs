//! Lookups over one profile's IR.

use std::collections::BTreeMap;

use anyhow::{Context, Result};

use crate::ir::{Const, Ir, Package, Type, TypeKind};

/// One profile's types by key, `<package>.<name>`, and its consts by type.
pub struct Index<'ir> {
    pub ir: &'ir Ir,
    types: BTreeMap<String, (&'ir Package, &'ir Type)>,
    consts: BTreeMap<&'ir str, Vec<&'ir Const>>,
}

impl<'ir> Index<'ir> {
    pub fn new(ir: &'ir Ir) -> Self {
        let mut types = BTreeMap::new();
        let mut consts: BTreeMap<_, Vec<_>> = BTreeMap::new();
        for package in &ir.packages {
            for ty in &package.types {
                types.insert(key(&package.path, &ty.name), (package, ty));
            }
            for c in &package.consts {
                consts.entry(c.ty.as_str()).or_default().push(c);
            }
        }
        Self { ir, types, consts }
    }

    /// The type with a key, if the IR has it. Other modules' types aren't in
    /// the IR.
    pub fn find(&self, key: &str) -> Option<(&'ir Package, &'ir Type)> {
        self.types.get(key).copied()
    }

    pub fn get(&self, key: &str) -> Result<(&'ir Package, &'ir Type)> {
        self.find(key).with_context(|| format!("no Go type {key}"))
    }

    /// The type an alias stands for, following aliases of named types. The
    /// result is a declared type, another module's type, or an alias of a
    /// type with no name, as in `type PortNumber = int32`.
    pub fn resolve(&self, key: &str) -> String {
        let mut key = key.to_string();
        while let Some((_, ty)) = self.find(&key)
            && ty.kind == TypeKind::Alias
            && let Some(target) = ty.alias.as_ref().and_then(|alias| alias.named.as_ref())
        {
            key.clone_from(target);
        }
        key
    }

    /// The consts of a named type, in source order.
    pub fn consts(&self, key: &str) -> &[&'ir Const] {
        self.consts.get(key).map_or(&[], Vec::as_slice)
    }

    pub fn package(&self, path: &str) -> Result<&'ir Package> {
        self.ir
            .packages
            .iter()
            .find(|package| package.path == path)
            .with_context(|| format!("no Go package {path}"))
    }
}

/// A type's key.
pub fn key(package: &str, name: &str) -> String {
    format!("{package}.{name}")
}

/// A key's package and name.
pub fn split_key(key: &str) -> (&str, &str) {
    key.rsplit_once('.').expect("a Go key has a package")
}
