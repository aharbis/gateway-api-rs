//! Rust names for Go names and values.

use anyhow::{Result, bail};

/// Initialisms that a run of capitals splits into, so `HTTPURLRewriteFilter`
/// becomes `HttpUrlRewriteFilter`, not `HttpurlRewriteFilter`.
const INITIALISMS: &[&str] = &["HTTP", "GRPC", "URL", "CORS", "TLS", "TCP", "UDP", "API"];

/// Rust keywords, which a field name takes a `_` suffix for, as in
/// k8s-openapi's `type_`.
const KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate", "do", "dyn", "else",
    "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl", "in", "let", "loop", "macro", "match", "mod",
    "move", "mut", "override", "priv", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
    "try", "type", "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// The Rust name for a Go type name: each word capitalized, with runs of
/// initialisms split, e.g. `HTTPURLRewriteFilter` is `HttpUrlRewriteFilter`.
pub fn type_name(go: &str) -> String {
    let mut out = String::new();
    for word in words(go) {
        for part in split_initialisms(&word) {
            let mut chars = part.chars();
            if let Some(first) = chars.next() {
                out.extend(first.to_uppercase());
                out.push_str(&chars.as_str().to_lowercase());
            }
        }
    }
    out
}

/// A field's name: its snake-cased JSON name, with a `_` suffix for a Rust
/// keyword.
pub fn field_name(json: &str) -> String {
    let name = words(json)
        .iter()
        .map(|word| word.to_lowercase())
        .collect::<Vec<_>>()
        .join("_");
    if KEYWORDS.contains(&name.as_str()) {
        format!("{name}_")
    } else {
        name
    }
}

/// An enum variant's name for a value, e.g. `URLRewrite` is `UrlRewrite`
/// and `*` is `Wildcard`.
pub fn variant_name(value: &str) -> Result<String> {
    if value == "*" {
        return Ok("Wildcard".to_string());
    }
    let parts: Vec<&str> = value.split(['-', '_', '.', '/']).collect();
    if parts
        .iter()
        .any(|part| part.is_empty() || !part.chars().all(|c| c.is_ascii_alphanumeric()))
        || !value.starts_with(|c: char| c.is_ascii_alphabetic())
    {
        bail!("enum value {value:?} has no variant name");
    }
    Ok(parts
        .iter()
        .map(|part| {
            let mut chars = part.chars();
            let first: String = chars.next().into_iter().flat_map(char::to_uppercase).collect();
            type_name(&format!("{first}{rest}", rest = chars.as_str()))
        })
        .collect())
}

/// Splits a camel-cased name into words. A capital starts a word after a
/// lowercase letter or digit, or before a lowercase letter after another
/// capital, so `HTTPRoute` is `HTTP`, `Route`.
fn words(name: &str) -> Vec<String> {
    let chars: Vec<char> = name.chars().collect();
    let mut words = Vec::new();
    let mut word = String::new();
    for (i, &c) in chars.iter().enumerate() {
        let prev = i.checked_sub(1).map(|i| chars[i]);
        let next = chars.get(i + 1);
        let boundary = c.is_uppercase()
            && prev.is_some_and(|prev| {
                prev.is_lowercase()
                    || prev.is_ascii_digit()
                    || (prev.is_uppercase() && next.is_some_and(|next| next.is_lowercase()))
            });
        if boundary && !word.is_empty() {
            words.push(std::mem::take(&mut word));
        }
        word.push(c);
    }
    words.push(word);
    words
}

/// Splits an all-capitals word made of initialisms, e.g. `HTTPURL`, into
/// them. Any other word is returned whole.
fn split_initialisms(word: &str) -> Vec<&str> {
    let mut parts = Vec::new();
    let mut rest = word;
    while !rest.is_empty() {
        let Some(initialism) = INITIALISMS.iter().find(|initialism| rest.starts_with(**initialism)) else {
            return vec![word];
        };
        parts.push(&rest[..initialism.len()]);
        rest = &rest[initialism.len()..];
    }
    parts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn type_names_split_initialisms() {
        assert_eq!(type_name("HTTPURLRewriteFilter"), "HttpUrlRewriteFilter");
        assert_eq!(type_name("HTTPCORSFilter"), "HttpCorsFilter");
        assert_eq!(type_name("GRPCRouteFilter"), "GrpcRouteFilter");
        assert_eq!(type_name("ListenerTLSConfig"), "ListenerTlsConfig");
        assert_eq!(type_name("XBackendTrafficPolicy"), "XBackendTrafficPolicy");
        assert_eq!(type_name("ParentReference"), "ParentReference");
        assert_eq!(type_name("IPAddress"), "IpAddress");
    }

    #[test]
    fn field_names_are_snake_case() {
        assert_eq!(field_name("backendRefs"), "backend_refs");
        assert_eq!(field_name("type"), "type_");
        assert_eq!(field_name("caCertificateRefs"), "ca_certificate_refs");
        assert_eq!(field_name("matchCIDR"), "match_cidr");
        assert_eq!(field_name("maxSize"), "max_size");
    }

    #[test]
    fn variant_names() {
        assert_eq!(variant_name("URLRewrite").unwrap(), "UrlRewrite");
        assert_eq!(variant_name("*").unwrap(), "Wildcard");
        assert_eq!(variant_name("GET").unwrap(), "Get");
        assert_eq!(variant_name("HTTPS").unwrap(), "Https");
        assert_eq!(variant_name("http").unwrap(), "Http");
        assert_eq!(variant_name("example.com/bar").unwrap(), "ExampleComBar");
        assert!(variant_name("301").is_err());
        assert!(variant_name("a b").is_err());
    }
}
