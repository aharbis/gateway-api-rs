//! Rust docs from Go docs.

use proc_macro2::TokenStream;
use quote::quote;

/// Doc attributes for a Go doc, one per line, outer (`///`) or inner
/// (`//!`). A code block without a language is marked `text`: Go docs hold
/// YAML examples, which rustdoc would otherwise test as Rust.
pub fn doc_attrs(doc: &str, inner: bool) -> TokenStream {
    let lines = doc_lines(doc);
    if inner {
        quote! { #(#![doc = #lines])* }
    } else {
        quote! { #(#[doc = #lines])* }
    }
}

/// A Go doc's lines as doc attribute values: each prefixed with a space, as
/// rustdoc expects from `/// line`.
pub fn doc_lines(doc: &str) -> Vec<String> {
    let mut in_block = false;
    let mut lines = Vec::new();
    for line in doc.trim_matches('\n').lines() {
        let mut line = line.trim_end();
        if line.trim_start().starts_with("```") {
            if !in_block && line.trim_start() == "```" {
                line = "```text";
            }
            in_block = !in_block;
        }
        lines.push(if line.is_empty() {
            String::new()
        } else {
            format!(" {line}")
        });
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_blocks_are_text() {
        let doc = "Example:\n\n```\nkind: HTTPRoute\n```\n\n```yaml\na: b\n```";
        assert_eq!(
            doc_lines(doc),
            [
                " Example:",
                "",
                " ```text",
                " kind: HTTPRoute",
                " ```",
                "",
                " ```yaml",
                " a: b",
                " ```"
            ]
        );
    }
}
