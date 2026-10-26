//! Checks that kopium names replaced by shared types still compile, through
//! their deprecated aliases: struct literals, enum variants and `match`
//! patterns. It only has to build; nothing here runs.

#![allow(deprecated, dead_code)]

/// A `common` type, through aliases in a resource module.
fn common_type() -> u8 {
    use gateway_api::{
        gateways::{GatewayListenersTls, GatewayListenersTlsMode},
        httproutes::HttpRouteParentRefs,
    };

    let _parent = HttpRouteParentRefs {
        name: "test-gateway".into(),
        ..Default::default()
    };
    let tls = GatewayListenersTls {
        mode: Some(GatewayListenersTlsMode::Terminate),
        ..Default::default()
    };
    match tls.mode.unwrap_or(GatewayListenersTlsMode::Passthrough) {
        GatewayListenersTlsMode::Terminate => 0,
        GatewayListenersTlsMode::Passthrough => 1,
    }
}

/// A per-resource type, through aliases in its root version and in another
/// version.
fn resource_type() -> u8 {
    use gateway_api::httproutes::{
        HttpRouteRulesFilters, HttpRouteRulesFiltersRequestRedirect, HttpRouteRulesFiltersType,
        v1beta1::HttpRouteRulesBackendRefsFiltersType,
    };

    let filter = HttpRouteRulesFilters {
        r#type: HttpRouteRulesFiltersType::RequestRedirect,
        request_redirect: Some(HttpRouteRulesFiltersRequestRedirect {
            hostname: Some("example.com".into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    match filter.r#type {
        HttpRouteRulesBackendRefsFiltersType::RequestRedirect => 0,
        _ => 1,
    }
}

/// `k8s-openapi`'s `LabelSelector`, through aliases in a resource module. Every
/// old field is written out: cargo-semver-checks can't see behind an alias to
/// another crate's type, so this is what checks none went missing.
fn k8s_type() {
    use std::collections::BTreeMap;

    use gateway_api::gateways::{
        GatewayListenersAllowedRoutesNamespacesSelector,
        GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions,
    };

    let _selector = GatewayListenersAllowedRoutesNamespacesSelector {
        match_expressions: Some(vec![GatewayListenersAllowedRoutesNamespacesSelectorMatchExpressions {
            key: "team".into(),
            operator: "In".into(),
            values: Some(vec!["a".into()]),
        }]),
        match_labels: Some(BTreeMap::from([("env".into(), "test".into())])),
    };
}
