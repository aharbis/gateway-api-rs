//! What one profile generates, worked out from its IR: the Go types it
//! emits, which of them are enums, and its kinds.

use std::collections::{BTreeMap, BTreeSet};

use anyhow::{Context, Result, bail};

use crate::{
    config::{Config, Profile},
    index::{Index, key, split_key},
    ir::{Crd, CrdVersion, Field, Ir, TypeKind, TypeRef},
};

/// One profile's model.
pub struct Model<'a> {
    pub config: &'a Config,
    pub profile: &'a Profile,
    pub index: Index<'a>,
    /// The Go types emitted, by key after aliases of named types: those the
    /// kinds reach, and the library types.
    pub types: BTreeSet<String>,
    /// The emitted Go types that are enums, by key.
    pub enums: BTreeMap<String, Enum>,
    /// Each CRD version, in IR order.
    pub kinds: Vec<KindVersion<'a>>,
}

/// A Go string type emitted as an enum.
pub struct Enum {
    /// Its values, in Go const order.
    pub values: Vec<String>,
    /// Whether the CRD validates the values (from enum markers), rather than
    /// the values being an open set (from consts).
    pub validated: bool,
    /// The value every field that uses it defaults to, if they all agree.
    pub default: Option<String>,
}

/// One version of a CRD.
pub struct KindVersion<'a> {
    pub crd: &'a Crd,
    pub version: &'a CrdVersion,
    /// The Go kind's key.
    pub kind: String,
    /// The spec's and status's Go types, after aliases.
    pub spec: String,
    pub status: Option<String>,
}

impl<'a> Model<'a> {
    pub fn new(config: &'a Config, profile: &'a Profile, ir: &'a Ir) -> Result<Self> {
        let mut model = Self {
            config,
            profile,
            index: Index::new(ir),
            types: BTreeSet::new(),
            enums: BTreeMap::new(),
            kinds: Vec::new(),
        };
        for crd in &ir.crds {
            for version in &crd.versions {
                let kind = model
                    .kind_version(crd, version)
                    .with_context(|| format!("{kind} {version}", kind = crd.kind, version = version.name))?;
                model.kinds.push(kind);
            }
        }

        let mut roots: Vec<String> = model
            .kinds
            .iter()
            .flat_map(|kind| [Some(kind.spec.clone()), kind.status.clone()])
            .flatten()
            .collect();
        roots.extend(model.library_types()?);
        model.types = model.closure(roots)?;
        model.enums = model.find_enums()?;
        Ok(model)
    }

    /// A CRD version's Go kind, spec and status.
    fn kind_version(&self, crd: &'a Crd, version: &'a CrdVersion) -> Result<KindVersion<'a>> {
        let kind = key(&version.package, &crd.kind);
        let (_, ty) = self.index.get(&kind)?;
        if ty.kind != TypeKind::Struct || !ty.root {
            bail!("Go kind {kind} isn't a root struct");
        }
        let (mut spec, mut status) = (None, None);
        for field in self.flattened(&kind)? {
            let target = || {
                field
                    .ty
                    .element()
                    .map(|named| self.index.resolve(named))
                    .with_context(|| format!("{kind}.{name} isn't a named type", name = field.name))
            };
            match field.json.as_deref() {
                Some("spec") => spec = Some(target()?),
                Some("status") => status = Some(target()?),
                Some("metadata") => {},
                json => bail!("Go kind {kind} has a field kube can't generate: {json:?}"),
            }
        }
        Ok(KindVersion {
            crd,
            version,
            spec: spec.with_context(|| format!("Go kind {kind} has no spec"))?,
            status,
            kind,
        })
    }

    /// A struct's fields, with embedded structs inlined. Another module's
    /// embedded types, such as `metav1.TypeMeta`, are left out: kube
    /// generates them.
    pub fn flattened(&self, key: &str) -> Result<Vec<&'a Field>> {
        let (_, ty) = self.index.get(key)?;
        let mut fields = Vec::new();
        for field in &ty.fields {
            if !field.inline {
                fields.push(field);
                continue;
            }
            let embedded = field.ty.element().context("an embedded field names a type")?;
            let embedded = self.index.resolve(embedded);
            if self.index.find(&embedded).is_some() {
                fields.extend(self.flattened(&embedded)?);
            } else if field.json.is_some() {
                bail!("{key}.{name} inlines another module's type", name = field.name);
            }
        }
        Ok(fields)
    }

    /// The types reachable from `roots`, after aliases of named types.
    fn closure(&self, roots: Vec<String>) -> Result<BTreeSet<String>> {
        let mut types = BTreeSet::new();
        let mut queue = roots;
        while let Some(key) = queue.pop() {
            let key = self.index.resolve(&key);
            if !types.insert(key.clone()) {
                continue;
            }
            let Some((_, ty)) = self.index.find(&key) else {
                if self.config.external(&key).is_none() {
                    bail!("no Rust path for Go type {key}: add it to the config's external map");
                }
                continue;
            };
            match ty.kind {
                TypeKind::Struct => {
                    for field in self.flattened(&key)? {
                        queue.extend(field.ty.element().map(str::to_string));
                    }
                },
                TypeKind::Alias => queue.extend(ty.alias.as_ref().and_then(TypeRef::element).map(str::to_string)),
                TypeKind::Scalar => {},
                TypeKind::Other => bail!("Go type {key} is neither a struct, a scalar nor an alias"),
            }
        }
        Ok(types)
    }

    /// The groups of the profile's CRDs.
    pub fn groups(&self) -> BTreeSet<&'a str> {
        self.kinds.iter().map(|kind| kind.crd.group.as_str()).collect()
    }

    /// The types emitted even where no kind reaches them, in the profile's
    /// groups: the library files' types, except aliases of named types,
    /// which are re-exported instead; and the string types with consts, the
    /// open sets such as condition types. Types reachable only through
    /// fields the profile drops are left out.
    fn library_types(&self) -> Result<Vec<String>> {
        let ir = self.index.ir;
        let dropped_roots = ir
            .packages
            .iter()
            .flat_map(|package| &package.types)
            .flat_map(|ty| &ty.dropped)
            .filter_map(|field| field.ty.element().map(str::to_string))
            .collect();
        let dropped = self.closure(dropped_roots)?;

        let groups = self.groups();
        let mut types = Vec::new();
        for package in ir
            .packages
            .iter()
            .filter(|package| package.group.as_deref().is_some_and(|group| groups.contains(group)))
        {
            for ty in &package.types {
                let key = key(&package.path, &ty.name);
                let alias_of_named = ty.alias.as_ref().is_some_and(|alias| alias.named.is_some());
                let library = self.config.library_files.contains(&ty.file) && !alias_of_named && !ty.root;
                let open_set = ty.basic.as_deref() == Some("string") && !self.index.consts(&key).is_empty();
                if (library || open_set) && !dropped.contains(&key) {
                    types.push(key);
                }
            }
        }
        Ok(types)
    }

    /// Every field of every emitted struct, flattened, by the key of the
    /// type its element is, after aliases.
    pub fn field_uses(&self) -> Result<BTreeMap<String, Vec<&'a Field>>> {
        let mut uses: BTreeMap<String, Vec<&Field>> = BTreeMap::new();
        for key in &self.types {
            if self.index.find(key).is_none_or(|(_, ty)| ty.kind != TypeKind::Struct) {
                continue;
            }
            for field in self.flattened(key)? {
                if let Some(named) = field.ty.element() {
                    uses.entry(self.index.resolve(named)).or_default().push(field);
                }
            }
        }
        Ok(uses)
    }

    /// The emitted string types that are enums: those with enum values from
    /// their own marker, else from their fields' markers, else from their
    /// consts.
    fn find_enums(&self) -> Result<BTreeMap<String, Enum>> {
        let uses = self.field_uses()?;
        let mut enums = BTreeMap::new();
        for key in &self.types {
            let Some((_, ty)) = self.index.find(key) else {
                continue;
            };
            if ty.kind != TypeKind::Scalar || ty.basic.as_deref() != Some("string") {
                continue;
            }
            let fields = uses.get(key).map_or(&[][..], Vec::as_slice);
            let consts = self.index.consts(key);
            let (mut values, validated) = if !ty.enum_values.is_empty() {
                (ty.enum_values.clone(), true)
            } else if fields.iter().any(|field| !field.enum_values.is_empty()) {
                let mut values: Vec<String> = Vec::new();
                for value in fields.iter().flat_map(|field| &field.enum_values) {
                    if !values.contains(value) {
                        values.push(value.clone());
                    }
                }
                (values, true)
            } else if !consts.is_empty() {
                (consts.iter().map(|c| c.value.clone()).collect(), false)
            } else {
                continue;
            };
            values.sort_by_key(|value| consts.iter().position(|c| c.value == *value).unwrap_or(usize::MAX));

            let defaults: BTreeSet<Option<&str>> = fields
                .iter()
                .map(|field| field.default.as_deref().map(|value| value.trim_matches('"')))
                .collect();
            let default = match defaults.into_iter().collect::<Vec<_>>()[..] {
                [Some(value)] if values.iter().any(|v| v == value) => Some(value.to_string()),
                _ => None,
            };
            enums.insert(
                key.clone(),
                Enum {
                    values,
                    validated,
                    default,
                },
            );
        }
        Ok(enums)
    }

    /// The Rust module path of a Go package in the profile's tree, e.g.
    /// `crate::apis::standard::v1`.
    pub fn module_path(&self, package: &str) -> Result<String> {
        let tree = self.profile.out.replace('/', "::");
        let module = self.config.module(package)?;
        Ok(format!("crate::{tree}::{module}"))
    }

    /// The Rust name of a Go type.
    pub fn rust_name(key: &str) -> String {
        crate::naming::type_name(split_key(key).1)
    }
}
