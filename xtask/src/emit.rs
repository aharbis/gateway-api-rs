//! Emits Rust types straight from the Go facts, without kopium: a preview of
//! generating the crate from the Go source. Modules mirror upstream's Go
//! packages (`v1`, `v1beta1`, ..., `apisx::v1alpha1`), each Go type is one
//! Rust type named after it, and a Go alias is a re-export. See
//! `docs/specs/go-source-generation.md`.

use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
};

use anyhow::{Context, Result, bail};
use heck::ToSnakeCase;

use crate::{
    derive::rust_name,
    gofacts::{Facts, Field, Index, Package, TypeKind, TypeRef},
};

/// The Go files whose types upstream shares across CRDs. The preview emits
/// all of their types, for crates that define their own routes and policies,
/// even where no CRD uses one.
const SHARED_FILES: &[&str] = &["shared_types.go", "object_reference_types.go", "policy_types.go"];

/// The Kubernetes types the API packages use, and their `k8s-openapi` paths.
const K8S_TYPES: &[(&str, &str)] = &[
    (
        "k8s.io/apimachinery/pkg/apis/meta/v1.Condition",
        "k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition",
    ),
    (
        "k8s.io/apimachinery/pkg/apis/meta/v1.LabelSelector",
        "k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector",
    ),
    (
        "k8s.io/apimachinery/pkg/apis/meta/v1.Time",
        "k8s_openapi::apimachinery::pkg::apis::meta::v1::Time",
    ),
];

const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern", "false", "fn",
    "for", "gen", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self",
    "static", "struct", "super", "trait", "true", "try", "type", "unsafe", "use", "where", "while", "yield",
];

/// One API version of a resource the channel has, from its CRD.
pub struct KindVersion {
    pub resource: String,
    pub group: String,
    pub version: String,
    pub kind: String,
    pub plural: String,
    pub namespaced: bool,
}

/// The generated modules of one channel, by module path relative to the
/// channel, e.g. `v1` or `apisx/v1alpha1`.
pub struct Channel {
    pub modules: BTreeMap<String, String>,
    /// For the schema test: each API version's root type path, by resource
    /// and version.
    pub roots: Vec<(String, String, String)>,
}

/// An enum the preview generates for a field with enum values but no Go
/// type, such as `HTTPRequestRedirectFilter.Scheme`.
struct FieldEnum {
    name: String,
    values: Vec<String>,
}

/// A re-export of a Go alias: the alias's Rust name and the Go key of what
/// it stands for.
type Reexport = (String, String);

/// A kind's spec in one API version: its Go spec and status types, after
/// aliases.
struct KubeSpec<'k> {
    spec: String,
    status: Option<String>,
    kind: &'k KindVersion,
}

struct Emitter<'f> {
    index: Index<'f>,
    channel: String,
    /// The Go types emitted, after aliases.
    types: BTreeSet<String>,
    /// Enum values per Go type: the type's own, or else every value its
    /// fields allow.
    enums: BTreeMap<String, Vec<String>>,
    /// By the Go key of the struct and the field's Go name.
    field_enums: BTreeMap<(String, String), FieldEnum>,
}

/// The Go types a channel's kinds reach, after aliases.
pub fn reachable(facts: &Facts, kinds: &[KindVersion]) -> Result<BTreeSet<String>> {
    let mut emitter = Emitter::new(facts);
    emitter.reach(facts, kinds, None)?;
    Ok(emitter.types)
}

/// Emits one channel's modules. The shared files' types that no kind of the
/// channel reaches are emitted too, except those in `elsewhere`: the types
/// another channel's kinds reach, such as standard's view of experimental-only
/// `SessionPersistence`.
pub fn emit(facts: &Facts, kinds: &[KindVersion], elsewhere: &BTreeSet<String>) -> Result<Channel> {
    let groups: BTreeSet<&str> = kinds.iter().map(|kind| kind.group.as_str()).collect();
    let mut emitter = Emitter::new(facts);
    emitter.reach(facts, kinds, Some(elsewhere))?;

    let mut modules = BTreeMap::new();
    let mut roots = Vec::new();
    for package in facts
        .packages
        .iter()
        .filter(|package| package.group.as_deref().is_some_and(|group| groups.contains(group)))
    {
        let kinds: Vec<&KindVersion> = kinds
            .iter()
            .filter(|kv| Some(kv.group.as_str()) == package.group.as_deref() && kv.version == package.name)
            .collect();
        let module = emitter
            .module(package, &kinds)
            .with_context(|| format!("{channel} {path}", channel = facts.channel, path = package.path))?;
        if let Some(module) = module {
            modules.insert(package.path.trim_start_matches("apis/").to_string(), module);
        }
        for kv in kinds {
            let path = format!(
                "{module}::{root}",
                module = emitter.module_path(&package.path),
                root = rust_name(&kv.kind)
            );
            roots.push((kv.resource.clone(), kv.version.clone(), path));
        }
    }
    Ok(Channel { modules, roots })
}

impl<'f> Emitter<'f> {
    fn new(facts: &'f Facts) -> Self {
        Self {
            index: Index::new(facts),
            channel: facts.channel.clone(),
            types: BTreeSet::new(),
            enums: BTreeMap::new(),
            field_enums: BTreeMap::new(),
        }
    }

    /// Finds every type a kind reaches, plus with `extras` the shared files'
    /// types that aren't in it, and the enum values of each.
    fn reach(&mut self, facts: &'f Facts, kinds: &[KindVersion], extras: Option<&BTreeSet<String>>) -> Result<()> {
        let groups: BTreeSet<&str> = kinds.iter().map(|kind| kind.group.as_str()).collect();
        let mut queue = Vec::new();
        for kv in kinds {
            let package = self.index.package(&kv.group, &kv.version)?;
            let (spec, status) = self.kind_types(package, &kv.kind)?;
            queue.push(spec);
            queue.extend(status);
        }
        let packages = facts
            .packages
            .iter()
            .filter(|package| package.group.as_deref().is_some_and(|group| groups.contains(group)));
        for package in packages.filter(|_| extras.is_some()) {
            for ty in package
                .types
                .iter()
                .filter(|ty| SHARED_FILES.contains(&ty.file.as_str()))
            {
                let key = format!("{path}.{name}", path = package.path, name = ty.name);
                if extras.is_some_and(|elsewhere| elsewhere.contains(&key)) {
                    continue;
                }
                match ty.alias.as_ref() {
                    Some(alias) if alias.named.is_none() => {
                        self.types.insert(key);
                    },
                    Some(_) => {},
                    None => queue.push(key),
                }
            }
        }
        while let Some(key) = queue.pop() {
            if !self.types.insert(key.clone()) || K8S_TYPES.iter().any(|(go, _)| *go == key) {
                continue;
            }
            let (_, ty) = self.index.get(&key)?;
            if !ty.enum_values.is_empty() {
                self.enums.insert(key.clone(), ty.enum_values.clone());
            }
            if ty.kind != TypeKind::Struct {
                continue;
            }
            for field in self.flattened(&key)? {
                let Some(named) = field.ty.element() else {
                    if !field.enum_values.is_empty() && field.ty.is_string() {
                        self.add_field_enum(&key, field);
                    }
                    continue;
                };
                match self.index.resolve(named)? {
                    Some(target) => {
                        if !field.enum_values.is_empty() {
                            let values = self.enums.entry(target.clone()).or_default();
                            for value in &field.enum_values {
                                if !values.contains(value) {
                                    values.push(value.clone());
                                }
                            }
                        }
                        queue.push(target);
                    },
                    // An alias of a basic type, such as PortNumber, keeps its name
                    None => self.types.extend(self.alias_chain(named)?),
                }
            }
        }
        // Enum values in Go const order, where the type has consts
        for (key, values) in &mut self.enums {
            let order: Vec<String> = self.index.consts(key).iter().map(|c| c.value.clone()).collect();
            values.sort_by_key(|value| order.iter().position(|o| o == value).unwrap_or(usize::MAX));
        }
        Ok(())
    }

    /// A kind's spec and status types, after aliases.
    fn kind_types(&self, package: &Package, kind: &str) -> Result<(String, Option<String>)> {
        let kind = package
            .kinds
            .iter()
            .find(|k| k.name == kind)
            .with_context(|| format!("no Go kind {kind} in {path}", path = package.path))?;
        let resolve = |ty: &Option<TypeRef>| -> Result<Option<String>> {
            match ty.as_ref().and_then(TypeRef::element) {
                Some(named) => self.index.resolve(named),
                None => Ok(None),
            }
        };
        let spec = resolve(&kind.spec)?.context("a kind's spec is a named type")?;
        Ok((spec, resolve(&kind.status)?))
    }

    /// A struct's fields, with embedded structs inlined.
    fn flattened(&self, key: &str) -> Result<Vec<&'f Field>> {
        let (_, ty) = self.index.get(key)?;
        let mut fields = Vec::new();
        for field in &ty.fields {
            if field.inline {
                let embedded = field.ty.element().context("an embedded field names a type")?;
                let embedded = self.index.resolve(embedded)?.context("an embedded type has a name")?;
                fields.extend(self.flattened(&embedded)?);
            } else if field.json.is_some() {
                fields.push(field);
            }
        }
        Ok(fields)
    }

    /// An alias and the aliases it stands for, ending at one of a basic type.
    fn alias_chain(&self, key: &str) -> Result<Vec<String>> {
        let mut chain = vec![key.to_string()];
        let mut key = key.to_string();
        while let (_, ty) = self.index.get(&key)?
            && let Some(next) = ty.alias.as_ref().and_then(|alias| alias.named.clone())
        {
            chain.push(next.clone());
            key = next;
        }
        Ok(chain)
    }

    fn add_field_enum(&mut self, key: &str, field: &Field) {
        let (_, parent) = key.rsplit_once('.').expect("a Go key has a package");
        self.field_enums.insert(
            (key.to_string(), field.name.clone()),
            FieldEnum {
                name: format!(
                    "{parent}{field}",
                    parent = rust_name(parent),
                    field = rust_name(&field.name)
                ),
                values: field.enum_values.clone(),
            },
        );
    }

    /// The Rust path of a module, e.g. `crate::standard::v1`.
    fn module_path(&self, package: &str) -> String {
        let module = package.trim_start_matches("apis/").replace('/', "::");
        format!("crate::{channel}::{module}", channel = self.channel)
    }

    /// The names a module defines or re-exports, by Go key after aliases, and
    /// its re-exports of Go aliases. Writes the aliases of basic types, such
    /// as `PortNumber`, to `body`.
    fn local_names(
        &self,
        package: &Package,
        kube_specs: &BTreeMap<String, KubeSpec<'_>>,
        body: &mut String,
    ) -> Result<(BTreeMap<String, String>, Vec<Reexport>)> {
        let path = &package.path;
        let mut reexports = Vec::new();
        let mut local: BTreeMap<String, String> = BTreeMap::new();
        for ty in &package.types {
            let key = format!("{path}.{name}", name = ty.name);
            let name = rust_name(&ty.name);
            if kube_specs.contains_key(&name) {
                continue;
            }
            if ty.kind == TypeKind::Alias {
                let alias = ty.alias.as_ref().context("an alias has a target")?;
                match alias.element() {
                    Some(target) => {
                        // An alias of an alias of a basic type re-exports
                        // the one it names
                        let target = match self.index.resolve(target)? {
                            Some(target) => target,
                            None if self.types.contains(&key) => target.to_string(),
                            None => continue,
                        };
                        if self.types.contains(&target) && !K8S_TYPES.iter().any(|(go, _)| *go == target) {
                            reexports.push((name.clone(), target.clone()));
                            local.insert(name, target);
                        }
                    },
                    None => {
                        if self.types.contains(&key) {
                            writeln!(body).unwrap();
                            write_doc(body, "", &ty.doc);
                            let basic = alias.basic().context("an alias of an unnamed type is basic")?;
                            writeln!(body, "pub type {name} = {rust};", rust = basic_type(basic)?).unwrap();
                            local.insert(name, key);
                        }
                    },
                }
                continue;
            }
            if self.types.contains(&key) {
                local.insert(name, key);
            }
        }
        for (name, spec) in kube_specs {
            local.insert(name.clone(), spec.spec.clone());
        }
        for field_enum in self
            .field_enums
            .iter()
            .filter(|((owner, _), _)| owner.starts_with(&format!("{path}.")))
            .map(|(_, field_enum)| field_enum)
        {
            local.insert(
                field_enum.name.clone(),
                format!("{path}.{name}", name = field_enum.name),
            );
        }

        Ok((local, reexports))
    }

    /// One package's module, or `None` if it has nothing to emit.
    fn module(&self, package: &'f Package, kinds: &[&KindVersion]) -> Result<Option<String>> {
        let path = &package.path;
        let mut body = String::new();

        // Each kind's spec is its own struct in every version, as kube needs
        let mut kube_specs = BTreeMap::new();
        for kv in kinds {
            let (spec, status) = self.kind_types(package, &kv.kind)?;
            let name = format!("{kind}Spec", kind = rust_name(&kv.kind));
            kube_specs.insert(name, KubeSpec { spec, status, kind: kv });
        }

        let (local, reexports) = self.local_names(package, &kube_specs, &mut body)?;
        let mut imports = BTreeSet::new();
        let mut paths = Paths {
            emitter: self,
            local: &local,
            imports: &mut imports,
        };

        for (name, spec) in &kube_specs {
            self.write_kube_spec(&mut body, &mut paths, name, spec)?;
        }

        for ty in &package.types {
            let key = format!("{path}.{name}", name = ty.name);
            let name = rust_name(&ty.name);
            if ty.kind == TypeKind::Alias || kube_specs.contains_key(&name) || !self.types.contains(&key) {
                continue;
            }
            writeln!(body).unwrap();
            write_doc(&mut body, "", &ty.doc);
            match ty.kind {
                TypeKind::Struct => {
                    writeln!(
                        body,
                        "#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]"
                    )
                    .unwrap();
                    self.write_struct_body(&mut body, &mut paths, &name, &key)?;
                },
                TypeKind::Scalar => {
                    if let Some(values) = self.enums.get(&key) {
                        self.write_enum(&mut body, &name, &key, values)?;
                    } else {
                        let basic = ty.basic.as_deref().context("a scalar has a basic type")?;
                        writeln!(body, "pub type {name} = {rust};", rust = basic_type(basic)?).unwrap();
                    }
                },
                TypeKind::Alias | TypeKind::Other => bail!("can't emit Go {key}"),
            }
        }
        for ((owner, _), field_enum) in &self.field_enums {
            if owner.starts_with(&format!("{path}.")) {
                writeln!(body).unwrap();
                self.write_enum(&mut body, &field_enum.name, "", &field_enum.values)?;
            }
        }

        if body.is_empty() && reexports.is_empty() {
            return Ok(None);
        }
        let mut out = self.module_header(path, &imports, &reexports);
        out.push_str(&body);
        Ok(Some(out))
    }

    /// A kind's spec struct, which kube's `CustomResource` derive makes the
    /// root type from.
    fn write_kube_spec(
        &self,
        body: &mut String,
        paths: &mut Paths<'_, '_>,
        name: &str,
        spec: &KubeSpec<'_>,
    ) -> Result<()> {
        let KubeSpec { spec, status, kind: kv } = spec;
        let (_, ty) = self.index.get(spec)?;
        writeln!(body).unwrap();
        write_doc(body, "", &ty.doc);
        writeln!(
            body,
            "#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]"
        )
        .unwrap();
        writeln!(
            body,
            "#[kube(group = \"{group}\", version = \"{version}\", kind = \"{kind}\", root = \"{root}\", plural = \
             \"{plural}\")]",
            group = kv.group,
            version = kv.version,
            kind = kv.kind,
            root = rust_name(&kv.kind),
            plural = kv.plural
        )
        .unwrap();
        if kv.namespaced {
            writeln!(body, "#[kube(namespaced)]").unwrap();
        }
        if let Some(status) = status {
            let status = paths.path(status);
            writeln!(body, "#[kube(status = \"{status}\")]").unwrap();
        }
        writeln!(body, "#[kube(derive = \"Default\")]\n#[kube(derive = \"PartialEq\")]").unwrap();
        self.write_struct_body(body, paths, name, spec)
    }

    /// A module's docs, imports and re-exports.
    fn module_header(&self, path: &str, imports: &BTreeSet<String>, reexports: &[Reexport]) -> String {
        let mut out = String::from("// WARNING: generated file — do not edit\n");
        writeln!(
            out,
            "//! Go package `sigs.k8s.io/gateway-api/{path}`, {channel} channel.\n",
            channel = self.channel
        )
        .unwrap();
        out.push_str(
            "#[allow(unused_imports)]\nuse kube::CustomResource;\n#[allow(unused_imports)]\nuse schemars::JsonSchema;\n\
             #[allow(unused_imports)]\nuse serde::{Deserialize, Serialize};\n#[allow(unused_imports)]\nuse \
             std::collections::BTreeMap;\n",
        );
        for import in imports {
            writeln!(out, "use {import};").unwrap();
        }
        if !reexports.is_empty() {
            out.push('\n');
            out.push_str("// Go aliases of other packages' types\n");
            for (name, target) in reexports {
                let module = self.module_path(target.rsplit_once('.').expect("a Go key has a package").0);
                let (_, go) = target.rsplit_once('.').expect("a Go key has a package");
                let target_name = rust_name(go);
                if target_name == *name {
                    writeln!(out, "pub use {module}::{name};").unwrap();
                } else {
                    writeln!(out, "pub use {module}::{target_name} as {name};").unwrap();
                }
            }
        }
        out
    }

    fn write_struct_body(&self, out: &mut String, paths: &mut Paths<'_, '_>, name: &str, key: &str) -> Result<()> {
        writeln!(out, "pub struct {name} {{").unwrap();
        for field in self.flattened(key)? {
            let json = field.json.as_deref().expect("flattened fields have JSON names");
            let mut ident = json.to_snake_case();
            if KEYWORDS.contains(&ident.as_str()) {
                ident = format!("r#{ident}");
            }
            let field_enum = self.field_enums.get(&(key.to_string(), field.name.clone()));
            let ty = match field_enum {
                Some(field_enum) => wrap(&field.ty, &field_enum.name),
                None => paths.type_ref(&field.ty)?,
            };
            write_doc(out, "    ", &field.doc);
            let mut serde = Vec::new();
            if field.optional {
                serde.push("default".to_string());
                serde.push("skip_serializing_if = \"Option::is_none\"".to_string());
            }
            if ident.trim_start_matches("r#") != json {
                serde.push(format!("rename = \"{json}\""));
            }
            if !serde.is_empty() {
                writeln!(out, "    #[serde({serde})]", serde = serde.join(", ")).unwrap();
            }
            if field.optional {
                writeln!(out, "    pub {ident}: Option<{ty}>,").unwrap();
            } else {
                writeln!(out, "    pub {ident}: {ty},").unwrap();
            }
        }
        writeln!(out, "}}").unwrap();
        Ok(())
    }

    /// An enum, with variant docs from its Go consts and a `Default` from the
    /// first value, so structs holding it can derive theirs.
    fn write_enum(&self, out: &mut String, name: &str, key: &str, values: &[String]) -> Result<()> {
        let consts = self.index.consts(key);
        writeln!(
            out,
            "#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]"
        )
        .unwrap();
        writeln!(out, "pub enum {name} {{").unwrap();
        let mut variants = Vec::new();
        for value in values {
            let variant = variant_name(value)?;
            if let Some(c) = consts.iter().find(|c| c.value == *value) {
                write_doc(out, "    ", &c.doc);
            }
            if variant != *value {
                writeln!(out, "    #[serde(rename = \"{value}\")]").unwrap();
            }
            writeln!(out, "    {variant},").unwrap();
            variants.push(variant);
        }
        writeln!(out, "}}").unwrap();
        let first = variants.first().context("an enum has values")?;
        writeln!(
            out,
            "\nimpl Default for {name} {{\n    fn default() -> Self {{\n        Self::{first}\n    }}\n}}"
        )
        .unwrap();
        Ok(())
    }
}

/// Resolves Go types to Rust paths for one module, collecting the imports it
/// needs.
struct Paths<'e, 'f> {
    emitter: &'e Emitter<'f>,
    local: &'e BTreeMap<String, String>,
    imports: &'e mut BTreeSet<String>,
}

impl Paths<'_, '_> {
    fn path(&mut self, key: &str) -> String {
        if let Some((_, path)) = K8S_TYPES.iter().find(|(go, _)| *go == key) {
            return (*path).to_string();
        }
        let (package, go) = key.rsplit_once('.').expect("a Go key has a package");
        let name = rust_name(go);
        match self.local.get(&name) {
            Some(local) if local == key => name,
            Some(_) => format!("{module}::{name}", module = self.emitter.module_path(package)),
            None => {
                self.imports
                    .insert(format!("{module}::{name}", module = self.emitter.module_path(package)));
                name
            },
        }
    }

    fn type_ref(&mut self, ty: &TypeRef) -> Result<String> {
        if let Some(named) = &ty.named {
            return match self.emitter.index.resolve(named)? {
                Some(key) => Ok(self.path(&key)),
                // A Go alias of a basic type keeps its name, e.g. PortNumber
                None => Ok(self.path(named)),
            };
        }
        if let Some(basic) = ty.basic() {
            return basic_type(basic).map(str::to_string);
        }
        if let Some(inner) = &ty.pointer {
            return self.type_ref(inner);
        }
        if let Some(inner) = &ty.slice {
            return Ok(format!("Vec<{inner}>", inner = self.type_ref(inner)?));
        }
        if let Some(map) = &ty.map {
            return Ok(format!("BTreeMap<String, {value}>", value = self.type_ref(&map.value)?));
        }
        bail!("unsupported Go type")
    }
}

/// A field enum's type, wrapped as the field's Go type wraps its string.
fn wrap(ty: &TypeRef, name: &str) -> String {
    if let Some(inner) = &ty.pointer {
        return wrap(inner, name);
    }
    if let Some(inner) = &ty.slice {
        return format!("Vec<{inner}>", inner = wrap(inner, name));
    }
    name.to_string()
}

fn basic_type(basic: &str) -> Result<&'static str> {
    Ok(match basic {
        "string" => "String",
        "bool" => "bool",
        "int32" => "i32",
        // controller-gen gives unsigned ints no format, which the API server
        // reads as int64
        "int" | "int64" | "uint" | "uint16" | "uint32" => "i64",
        "float64" => "f64",
        _ => bail!("unsupported Go basic type {basic}"),
    })
}

/// A variant name for an enum value, e.g. `URLRewrite` becomes `UrlRewrite`.
fn variant_name(value: &str) -> Result<String> {
    if value == "*" {
        return Ok("Wildcard".to_string());
    }
    if !value.chars().next().is_some_and(char::is_alphabetic) || !value.chars().all(char::is_alphanumeric) {
        bail!("enum value {value} has no variant name");
    }
    let mut chars = value.chars();
    let first: String = chars.next().into_iter().flat_map(char::to_uppercase).collect();
    Ok(rust_name(&format!("{first}{rest}", rest = chars.as_str())))
}

/// Writes a Go doc comment as Rust doc lines, indented by `indent`. A code
/// block without a language is marked `text`, as kopium does, since Go docs
/// hold YAML and rustdoc would test it as Rust.
pub fn write_doc(out: &mut String, indent: &str, doc: &str) {
    let mut in_block = false;
    for line in doc.lines() {
        let mut line = line.trim_end();
        if line.trim_start().starts_with("```") {
            if !in_block && line.trim_start() == "```" {
                line = "```text";
            }
            in_block = !in_block;
        }
        if line.is_empty() {
            writeln!(out, "{indent}///").unwrap();
        } else {
            writeln!(out, "{indent}/// {line}").unwrap();
        }
    }
}
