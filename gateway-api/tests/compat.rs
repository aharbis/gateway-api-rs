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
