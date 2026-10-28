//! The emitter on a small IR: a kind with versions in two packages, an
//! enum, an alias of a basic type and an alias of another package's type.

use quote::ToTokens;
use syn::parse_quote;

use super::modules;
use crate::{Config, Ir, model::Model};

const CONFIG: &str = "
module: example.com/api
crate: widgets
packages:
  apis/v1: v1
  apis/v1beta1: v1beta1
crds: config/crd
out: apis
library_files: [shared_types.go]
external:
  k8s.io/apimachinery/pkg/apis/meta/v1:
    Condition: k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition
";

const IR: &str = r#"{
  "irVersion": 1,
  "module": "example.com/api",
  "version": "v1.0.0",
  "profile": "default",
  "packages": [
    {
      "path": "apis/v1",
      "name": "v1",
      "group": "example.io",
      "types": [
        {
          "name": "Widget", "file": "widget_types.go", "kind": "struct", "root": true,
          "doc": "Widget is a widget.",
          "fields": [
            {"name": "TypeMeta", "inline": true, "type": {"named": "k8s.io/apimachinery/pkg/apis/meta/v1.TypeMeta"}},
            {"name": "ObjectMeta", "json": "metadata", "type": {"named": "k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta"}, "optional": true},
            {"name": "Spec", "json": "spec", "type": {"named": "apis/v1.WidgetSpec"}},
            {"name": "Status", "json": "status", "type": {"named": "apis/v1.WidgetStatus"}, "optional": true}
          ]
        },
        {
          "name": "WidgetSpec", "file": "widget_types.go", "kind": "struct", "doc": "WidgetSpec is the spec.",
          "fields": [
            {"name": "Type", "json": "type", "type": {"named": "apis/v1.Shape"}, "doc": "Type is the shape, such as ShapeRound."},
            {"name": "Port", "json": "port", "type": {"pointer": {"named": "apis/v1.PortNumber"}}, "optional": true, "default": "80",
             "doc": "Port is a PortNumber, reported in WidgetStatus."},
            {"name": "From", "json": "from", "type": {"pointer": {"named": "apis/v1.Shape"}}, "optional": true, "enum": ["Round"]}
          ]
        },
        {
          "name": "WidgetStatus", "file": "widget_types.go", "kind": "struct",
          "doc": "WidgetStatus is the status of a `Widget`, not \"WidgetStatus\".",
          "fields": [
            {"name": "Conditions", "json": "conditions", "type": {"slice": {"named": "k8s.io/apimachinery/pkg/apis/meta/v1.Condition"}}, "optional": true}
          ]
        },
        {"name": "Shape", "file": "widget_types.go", "kind": "scalar", "basic": "string", "enum": ["Square", "Round"], "doc": "Shape is a shape."},
        {"name": "PortNumber", "file": "shared_types.go", "kind": "alias", "alias": {"basic": "int32"}},
        {"name": "Unused", "file": "widget_types.go", "kind": "scalar", "basic": "string"}
      ],
      "consts": [
        {"name": "ShapeRound", "type": "apis/v1.Shape", "value": "Round", "doc": "ShapeRound is round, unlike ShapeSquare."},
        {"name": "ShapeSquare", "type": "apis/v1.Shape", "value": "Square"}
      ]
    },
    {
      "path": "apis/v1beta1",
      "name": "v1beta1",
      "group": "example.io",
      "types": [
        {
          "name": "Widget", "file": "widget_types.go", "kind": "struct", "root": true,
          "defined": {"named": "apis/v1.Widget"},
          "fields": [
            {"name": "TypeMeta", "inline": true, "type": {"named": "k8s.io/apimachinery/pkg/apis/meta/v1.TypeMeta"}},
            {"name": "ObjectMeta", "json": "metadata", "type": {"named": "k8s.io/apimachinery/pkg/apis/meta/v1.ObjectMeta"}, "optional": true},
            {"name": "Spec", "json": "spec", "type": {"named": "apis/v1.WidgetSpec"}}
          ]
        },
        {"name": "Shape", "file": "shared_types.go", "kind": "alias", "alias": {"named": "apis/v1.Shape"}}
      ]
    }
  ],
  "crds": [
    {
      "file": "example.io_widgets.yaml", "group": "example.io", "kind": "Widget", "plural": "widgets",
      "scope": "Namespaced", "shortNames": ["wd"],
      "versions": [
        {"name": "v1", "package": "apis/v1", "served": true, "storage": true, "status": true,
         "printerColumns": [{"name": "Age", "type": "date", "jsonPath": ".metadata.creationTimestamp"}]},
        {"name": "v1beta1", "package": "apis/v1beta1", "served": true, "storage": false,
         "deprecated": true, "deprecationWarning": "Use v1 instead."}
      ]
    }
  ]
}"#;

/// The emitted modules, parsed, by module path.
fn emit() -> Vec<(String, syn::File)> {
    let config = Config::from_yaml(CONFIG).unwrap();
    let ir: Ir = serde_json::from_str(IR).unwrap();
    let model = Model::new(&config, &config.profiles[0], &ir).unwrap();
    modules(&model)
        .unwrap()
        .into_iter()
        .map(|module| (module.path, syn::parse_file(&module.source).unwrap()))
        .collect()
}

fn module(name: &str) -> syn::File {
    emit()
        .into_iter()
        .find(|(path, _)| path == name)
        .unwrap_or_else(|| panic!("no module {name}"))
        .1
}

/// The items with an identifier, as tokens to compare.
fn item(file: &syn::File, name: &str) -> String {
    let found: Vec<_> = file
        .items
        .iter()
        .filter(|item| match item {
            syn::Item::Struct(s) => s.ident == name,
            syn::Item::Enum(e) => e.ident == name,
            syn::Item::Type(t) => t.ident == name,
            _ => false,
        })
        .collect();
    assert_eq!(found.len(), 1, "{name}");
    tokens(found[0])
}

/// An item as prettyplease prints it, so token spellings such as raw doc
/// strings and trailing commas don't matter.
fn tokens(item: &syn::Item) -> String {
    prettyplease::unparse(&syn::parse_file(&item.to_token_stream().to_string()).unwrap())
}

/// The module's inner docs, one string per line.
fn module_doc(name: &str) -> Vec<String> {
    module(name)
        .attrs
        .into_iter()
        .filter(|attr| matches!(attr.style, syn::AttrStyle::Inner(_)))
        .filter_map(|attr| match attr.meta {
            syn::Meta::NameValue(nv) => match nv.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(s), ..
                }) => Some(s.value()),
                _ => None,
            },
            _ => None,
        })
        .collect()
}

/// One bullet per kind, under the module's one apiVersion.
#[test]
fn module_doc_lists_each_kind_under_its_api_version() {
    let expected = [
        " Generated from Go package `example.com/api/apis/v1` at v1.0.0.",
        "",
        " Kinds available in the `example.io/v1` apiVersion:",
        "",
        " - [`Widget`]",
        "   - kind: `Widget`",
        "   - served: yes",
        "   - storage: yes",
        "   - deprecated: no",
    ];
    assert_eq!(module_doc("v1"), expected);

    let expected = [
        " Generated from Go package `example.com/api/apis/v1beta1` at v1.0.0.",
        "",
        " Kinds available in the `example.io/v1beta1` apiVersion:",
        "",
        " - [`Widget`]",
        "   - kind: `Widget`",
        "   - served: yes",
        "   - storage: no",
        "   - deprecated: yes (Use v1 instead.)",
    ];
    assert_eq!(module_doc("v1beta1"), expected);
}

#[test]
fn spec_struct_has_kube_attributes() {
    let expected: syn::Item = parse_quote! {
        /// [`WidgetSpec`] is the spec.
        #[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
        #[kube(group = "example.io", version = "v1", kind = "Widget", root = "Widget", plural = "widgets")]
        #[kube(namespaced)]
        #[kube(status = "WidgetStatus")]
        #[kube(shortname = "wd")]
        #[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
        #[kube(derive = "PartialEq")]
        #[kube(doc = "Widget is a widget.")]
        pub struct WidgetSpec {
            /// `type_` is the shape, such as ShapeRound.
            #[serde(rename = "type")]
            pub type_: Shape,
            /// `port` is a PortNumber, reported in WidgetStatus.
            ///
            /// Default: `80`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub port: Option<PortNumber>,
            /// Allowed values here: `Round`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[schemars(extend("enum" = ["Round"]))]
            pub from: Option<Shape>,
        }
    };
    assert_eq!(item(&module("v1"), "WidgetSpec"), tokens(&expected));
}

#[test]
fn struct_without_required_fields_derives_default() {
    let expected: syn::Item = parse_quote! {
        /// [`WidgetStatus`] is the status of a `Widget`, not "WidgetStatus".
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
        pub struct WidgetStatus {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub conditions: Option<Vec<Condition>>,
        }
    };
    assert_eq!(item(&module("v1"), "WidgetStatus"), tokens(&expected));
}

#[test]
fn struct_with_required_fields_has_a_constructor() {
    let file = module("v1");
    let constructor = file
        .items
        .iter()
        .find_map(|item| match item {
            syn::Item::Impl(i) if i.trait_.is_none() && i.self_ty.to_token_stream().to_string() == "WidgetSpec" => {
                Some(tokens(item))
            },
            _ => None,
        })
        .expect("WidgetSpec has an impl");
    let expected: syn::Item = parse_quote! {
        impl WidgetSpec {
            /// Creates a `WidgetSpec` from its required fields, with the optional fields `None`.
            pub fn new(type_: impl Into<Shape>) -> Self {
                Self {
                    type_: type_.into(),
                    port: None,
                    from: None
                }
            }
        }
    };
    assert_eq!(constructor, tokens(&expected));
}

#[test]
fn enum_values_are_in_const_order_with_const_docs() {
    let expected: syn::Item = parse_quote! {
        /// [`Shape`] is a shape.
        #[derive(Clone, Debug)]
        #[non_exhaustive]
        pub enum Shape {
            /// [`Shape::Round`] is round, unlike ShapeSquare.
            Round,
            Square,
            /// A value without a variant: one this version of the crate
            /// doesn't know, or one outside upstream's values.
            Other(String),
        }
    };
    assert_eq!(item(&module("v1"), "Shape"), tokens(&expected));
}

#[test]
fn alias_of_a_basic_type_is_a_type_alias() {
    let expected: syn::Item = parse_quote! {
        pub type PortNumber = i32;
    };
    assert_eq!(item(&module("v1"), "PortNumber"), tokens(&expected));
}

#[test]
fn unreachable_types_outside_library_files_are_left_out() {
    let file = module("v1");
    assert!(!tokens_of(&file).contains("Unused"));
}

fn tokens_of(file: &syn::File) -> String {
    file.to_token_stream().to_string()
}

#[test]
fn other_version_reexports_aliases_and_copies_the_spec() {
    let file = module("v1beta1");
    let uses: Vec<String> = file
        .items
        .iter()
        .filter(|item| matches!(item, syn::Item::Use(_)))
        .map(tokens)
        .collect();
    let file_v1 = module("v1");
    let import_condition: syn::Item = parse_quote! { use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition; };
    assert!(
        file_v1
            .items
            .iter()
            .any(|item| tokens(item) == tokens(&import_condition))
    );

    let reexport: syn::Item = parse_quote! { pub use crate::apis::v1::Shape; };
    let import: syn::Item = parse_quote! { use crate::apis::v1::PortNumber; };
    assert_eq!(uses, [tokens(&reexport), tokens(&import)]);

    let expected: syn::Item = parse_quote! {
        /// [`WidgetSpec`] is the spec.
        #[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
        #[kube(group = "example.io", version = "v1beta1", kind = "Widget", root = "Widget", plural = "widgets")]
        #[kube(namespaced)]
        #[kube(shortname = "wd")]
        #[kube(derive = "PartialEq")]
        pub struct WidgetSpec {
            /// `type_` is the shape, such as ShapeRound.
            #[serde(rename = "type")]
            pub type_: Shape,
            /// `port` is a PortNumber, reported in WidgetStatus.
            ///
            /// Default: `80`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub port: Option<PortNumber>,
            /// Allowed values here: `Round`.
            #[serde(default, skip_serializing_if = "Option::is_none")]
            #[schemars(extend("enum" = ["Round"]))]
            pub from: Option<Shape>,
        }
    };
    assert_eq!(item(&file, "WidgetSpec"), tokens(&expected));
}
