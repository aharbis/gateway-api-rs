//! Structs, and the kube attributes of a kind's spec.

use anyhow::{Context, Result, bail};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::{
    ModuleEmitter,
    docs::{doc_attrs, replace_lead},
};
use crate::{
    ir::{Field, PrinterColumn},
    model::KindVersion,
    naming::{field_name, type_name},
};

/// One struct field, ready to emit.
struct StructField {
    ident: syn::Ident,
    /// The field's type, without `Option`.
    ty: TokenStream,
    optional: bool,
    attrs: TokenStream,
}

impl ModuleEmitter<'_, '_> {
    /// A struct for the Go struct `key`, with its embedded structs flattened,
    /// and the kube attributes of `kind` if it's the kind's spec.
    ///
    /// It derives `Default` only if it has no required fields. If it has
    /// some, it gets a `new()` that takes them.
    pub(super) fn struct_item(
        &mut self,
        name: &str,
        key: &str,
        doc: &str,
        kind: Option<&KindVersion<'_>>,
    ) -> Result<TokenStream> {
        let mut fields = Vec::new();
        for field in self.model.flattened(key)? {
            fields.push(
                self.struct_field(field)
                    .with_context(|| format!("{key}.{name}", name = field.name))?,
            );
        }
        let default = fields.iter().all(|field| field.optional);
        let ident = format_ident!("{name}");

        let mut derives = vec![
            quote! { serde::Serialize },
            quote! { serde::Deserialize },
            quote! { Clone },
            quote! { Debug },
            quote! { schemars::JsonSchema },
            quote! { PartialEq },
        ];
        if default {
            derives.push(quote! { Default });
        }
        let kube = match kind {
            Some(kind) => {
                derives.insert(0, quote! { kube::CustomResource });
                self.kube_attrs(kind, default)?
            },
            None => TokenStream::new(),
        };

        let doc = doc_attrs(doc, false);
        let field_items = fields.iter().map(|field| {
            let StructField {
                ident,
                ty,
                optional,
                attrs,
                ..
            } = field;
            let ty = if *optional {
                quote! { Option<#ty> }
            } else {
                ty.clone()
            };
            quote! {
                #attrs
                pub #ident: #ty,
            }
        });
        let constructor = (!default).then(|| constructor(name, &fields));
        Ok(quote! {
            #doc
            #[derive(#(#derives),*)]
            #kube
            pub struct #ident {
                #(#field_items)*
            }
            #constructor
        })
    }

    fn struct_field(&mut self, field: &Field) -> Result<StructField> {
        let json = field
            .json
            .as_deref()
            .context("a field that isn't inlined has a JSON name")?;
        let name = field_name(json);
        let ident = format_ident!("{name}");
        let ty = self.rust_type(&field.ty)?;

        let mut doc = replace_lead(&field.doc, &field.name, &format!("`{name}`"));
        let mut add_line = |line: String| {
            if !doc.is_empty() {
                doc.push_str("\n\n");
            }
            doc.push_str(&line);
        };
        if let Some(default) = &field.default {
            add_line(format!("Default: `{default}`."));
        }
        let narrowed = self.narrowed(field)?;
        if let Some(values) = &narrowed {
            let values: Vec<String> = values.iter().map(|value| format!("`{value}`")).collect();
            add_line(format!("Allowed values here: {values}.", values = values.join(", ")));
        }
        let doc = doc_attrs(&doc, false);

        let mut serde = Vec::new();
        if field.optional {
            serde.push(quote! { default });
            serde.push(quote! { skip_serializing_if = "Option::is_none" });
        }
        if name != json {
            serde.push(quote! { rename = #json });
        }
        let serde = (!serde.is_empty()).then(|| quote! { #[serde(#(#serde),*)] });
        let schemars = match narrowed {
            Some(values) => {
                let values = Self::enum_values_json(field, &values)?;
                quote! { #[schemars(extend("enum" = [#(#values),*]))] }
            },
            None => TokenStream::new(),
        };
        Ok(StructField {
            ident,
            ty,
            optional: field.optional,
            attrs: quote! { #doc #serde #schemars },
        })
    }

    /// The values a field's own enum marker narrows its type to: for a
    /// field of a basic type, its values; for an enum, its values if they're
    /// fewer than the enum's.
    fn narrowed(&self, field: &Field) -> Result<Option<Vec<String>>> {
        if field.enum_values.is_empty() {
            return Ok(None);
        }
        if field.ty.slice.is_some() || field.ty.pointer.as_ref().is_some_and(|inner| inner.slice.is_some()) {
            bail!("an enum marker on a list field isn't supported");
        }
        let Some(named) = field.ty.element() else {
            return Ok(Some(field.enum_values.clone()));
        };
        let key = self.model.index.resolve(named);
        let Some(e) = self.model.enums.get(&key) else {
            return Ok(Some(field.enum_values.clone()));
        };
        let narrowed = e.values.iter().any(|value| !field.enum_values.contains(value));
        Ok(narrowed.then(|| field.enum_values.clone()))
    }

    /// Enum values as JSON tokens for the field's schema: numbers for an
    /// integer field, else strings.
    fn enum_values_json(field: &Field, values: &[String]) -> Result<Vec<TokenStream>> {
        let integer = field
            .ty
            .element_basic()
            .is_some_and(|basic| basic.starts_with("int") || basic.starts_with("uint"));
        values
            .iter()
            .map(|value| {
                if integer {
                    let value: i64 = value.parse().with_context(|| format!("{value} isn't an integer"))?;
                    let value = proc_macro2::Literal::i64_unsuffixed(value);
                    Ok(quote! { #value })
                } else {
                    Ok(quote! { #value })
                }
            })
            .collect()
    }

    /// The `#[kube(...)]` attributes of a kind's spec, from its CRD.
    fn kube_attrs(&mut self, kind: &KindVersion<'_>, default: bool) -> Result<TokenStream> {
        let crd = kind.crd;
        let group = &crd.group;
        let version = &kind.version.name;
        let crd_kind = &crd.kind;
        let root = type_name(&crd.kind);
        let plural = &crd.plural;
        let mut attrs = vec![quote! {
            #[kube(group = #group, version = #version, kind = #crd_kind, root = #root, plural = #plural)]
        }];
        match crd.scope.as_str() {
            "Namespaced" => attrs.push(quote! { #[kube(namespaced)] }),
            "Cluster" => {},
            scope => bail!("unknown CRD scope {scope}"),
        }
        if kind.version.status {
            let status = kind
                .status
                .as_ref()
                .context("the CRD has a status subresource, but the kind no status")?;
            let status = self.path(status)?;
            attrs.push(quote! { #[kube(status = #status)] });
        }
        for shortname in &crd.short_names {
            attrs.push(quote! { #[kube(shortname = #shortname)] });
        }
        for category in &crd.categories {
            attrs.push(quote! { #[kube(category = #category)] });
        }
        for column in &kind.version.printer_columns {
            let PrinterColumn {
                name,
                ty,
                json_path,
                description,
                format,
                priority,
            } = column;
            let mut args = vec![
                quote! { name = #name },
                quote! { type_ = #ty },
                quote! { json_path = #json_path },
            ];
            args.extend(
                description
                    .iter()
                    .map(|description| quote! { description = #description }),
            );
            args.extend(format.iter().map(|format| quote! { format = #format }));
            args.extend(priority.iter().map(|priority| quote! { priority = #priority }));
            attrs.push(quote! { #[kube(printcolumn(#(#args),*))] });
        }
        if default {
            attrs.push(quote! { #[kube(derive = "Default")] });
        }
        attrs.push(quote! { #[kube(derive = "PartialEq")] });
        let (_, kind_type) = self.model.index.get(&kind.kind)?;
        let doc = super::docs::doc_lines(&kind_type.doc)
            .iter()
            .map(|line| line.strip_prefix(' ').unwrap_or(line))
            .collect::<Vec<_>>()
            .join("\n");
        if !doc.is_empty() {
            attrs.push(quote! { #[kube(doc = #doc)] });
        }
        Ok(quote! { #(#attrs)* })
    }
}

/// `new()` for a struct with required fields: it takes them, in Go field
/// order, and sets the optional fields to `None`.
fn constructor(name: &str, fields: &[StructField]) -> TokenStream {
    let ident = format_ident!("{name}");
    let params = fields.iter().filter(|field| !field.optional).map(|field| {
        let StructField { ident, ty, .. } = field;
        quote! { #ident: impl Into<#ty> }
    });
    let inits = fields.iter().map(|field| {
        let ident = &field.ident;
        if field.optional {
            quote! { #ident: None }
        } else {
            quote! { #ident: #ident.into() }
        }
    });
    let doc = format!(" Creates a `{name}` from its required fields, with the optional fields `None`.");
    quote! {
        impl #ident {
            #[doc = #doc]
            pub fn new(#(#params),*) -> Self {
                Self {
                    #(#inits),*
                }
            }
        }
    }
}
