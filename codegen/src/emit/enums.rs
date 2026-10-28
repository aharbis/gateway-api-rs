//! Enums for Go string types with enum values or consts.

use anyhow::{Result, bail};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use super::docs::doc_attrs;
use crate::{
    model::{Enum, Model},
    naming::variant_name,
};

/// An enum for the Go string type `key`.
///
/// It's `#[non_exhaustive]`, with an `Other(String)` variant for values
/// without one, and converts from and to strings: serde, `as_str()`,
/// `Display`, `FromStr`, `From<&str>`, `From<String>`, and `PartialEq` with
/// `str`. Equality is by value, so `Other("Exact")` equals `Exact`. A
/// CRD-validated enum's schema lists its values; an open set's is a plain
/// string.
pub fn enum_item(model: &Model<'_>, name: &str, key: &str, doc: &str, e: &Enum) -> Result<TokenStream> {
    let ident = format_ident!("{name}");
    let consts = model.index.consts(key);
    let mut variants = Vec::new();
    for value in &e.values {
        let variant = format_ident!("{}", variant_name(value)?);
        if variants.iter().any(|(other, _, _)| *other == variant) || variant == "Other" {
            bail!("{key}: two values have the variant name {variant}");
        }
        let doc = consts
            .iter()
            .find(|c| c.value == *value)
            .map_or_else(TokenStream::new, |c| doc_attrs(&c.doc, false));
        variants.push((variant, value.clone(), doc));
    }

    let doc = doc_attrs(doc, false);
    let variant_items = variants.iter().map(|(variant, _, doc)| quote! { #doc #variant, });
    let as_str = variants
        .iter()
        .map(|(variant, value, _)| quote! { Self::#variant => #value, });
    let from_str = variants
        .iter()
        .map(|(variant, value, _)| quote! { #value => Self::#variant, });
    let from_str_2 = from_str.clone();
    let schema = if e.validated {
        let values = &e.values;
        quote! { schemars::json_schema!({ "type": "string", "enum": [#(#values),*] }) }
    } else {
        quote! { schemars::json_schema!({ "type": "string" }) }
    };
    let default = e.default.as_ref().map(|value| {
        let variant = format_ident!("{}", variant_name(value).expect("default is one of the values"));
        quote! {
            impl Default for #ident {
                fn default() -> Self {
                    Self::#variant
                }
            }
        }
    });

    Ok(quote! {
        #doc
        #[derive(Clone, Debug)]
        #[non_exhaustive]
        pub enum #ident {
            #(#variant_items)*
            /// A value without a variant: one this version of the crate
            /// doesn't know, or one outside upstream's values.
            Other(String),
        }

        impl #ident {
            /// The value as a string.
            pub fn as_str(&self) -> &str {
                match self {
                    #(#as_str)*
                    Self::Other(value) => value,
                }
            }
        }

        impl From<&str> for #ident {
            fn from(value: &str) -> Self {
                match value {
                    #(#from_str)*
                    _ => Self::Other(value.to_string()),
                }
            }
        }

        impl From<String> for #ident {
            fn from(value: String) -> Self {
                match value.as_str() {
                    #(#from_str_2)*
                    _ => Self::Other(value),
                }
            }
        }

        impl std::str::FromStr for #ident {
            type Err = std::convert::Infallible;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Ok(Self::from(value))
            }
        }

        impl std::fmt::Display for #ident {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(self.as_str())
            }
        }

        impl PartialEq for #ident {
            fn eq(&self, other: &Self) -> bool {
                self.as_str() == other.as_str()
            }
        }

        impl Eq for #ident {}

        impl std::hash::Hash for #ident {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                std::hash::Hash::hash(self.as_str(), state);
            }
        }

        impl PartialEq<str> for #ident {
            fn eq(&self, other: &str) -> bool {
                self.as_str() == other
            }
        }

        impl PartialEq<&str> for #ident {
            fn eq(&self, other: &&str) -> bool {
                self.as_str() == *other
            }
        }

        impl serde::Serialize for #ident {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(self.as_str())
            }
        }

        impl<'de> serde::Deserialize<'de> for #ident {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
            }
        }

        impl schemars::JsonSchema for #ident {
            fn schema_name() -> std::borrow::Cow<'static, str> {
                #name.into()
            }

            fn schema_id() -> std::borrow::Cow<'static, str> {
                concat!(module_path!(), "::", #name).into()
            }

            fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
                #schema
            }
        }

        #default
    })
}
