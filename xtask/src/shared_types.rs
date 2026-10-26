//! Which kopium types are one upstream Go type, and which types that share a
//! shape are distinct upstream. The CRDs don't name their types, so these are
//! kept by hand from the Go source. See `docs/specs/shared-types.md`.

use heck::ToUpperCamelCase;

/// Where a shared type's one definition is generated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Placement {
    /// The channel's `common` module.
    Common,
}

/// An upstream Go type that kopium generates at more than one path.
pub struct SharedType {
    /// The Go type's name.
    pub go: &'static str,
    pub placement: Placement,
    /// Every kopium name the type replaces, in any channel or API version.
    pub kopium: &'static [&'static str],
}

impl SharedType {
    /// The Rust name: the Go name with kopium's casing, e.g.
    /// `HTTPHeaderFilter` becomes `HttpHeaderFilter`.
    pub fn rust_name(&self) -> String {
        self.go.to_upper_camel_case()
    }
}

const fn common(go: &'static str, kopium: &'static [&'static str]) -> SharedType {
    SharedType {
        go,
        placement: Placement::Common,
        kopium,
    }
}

pub const SHARED_TYPES: &[SharedType] = &[
    common(
        "ParentReference",
        &[
            "HttpRouteParentRefs",
            "GrpcRouteParentRefs",
            "TlsRouteParentRefs",
            "TcpRouteParentRefs",
            "UdpRouteParentRefs",
            "HttpRouteStatusParentsParentRef",
            "GrpcRouteStatusParentsParentRef",
            "TlsRouteStatusParentsParentRef",
            "TcpRouteStatusParentsParentRef",
            "UdpRouteStatusParentsParentRef",
            "BackendTlsPolicyStatusAncestorsAncestorRef",
            "XBackendTrafficPolicyStatusAncestorsAncestorRef",
        ],
    ),
    common(
        "GatewayDefaultScope",
        &[
            "GatewayDefaultScope",
            "HttpRouteUseDefaultGateways",
            "GrpcRouteUseDefaultGateways",
            "TlsRouteUseDefaultGateways",
            "TcpRouteUseDefaultGateways",
            "UdpRouteUseDefaultGateways",
        ],
    ),
    common(
        "BackendRef",
        &[
            "TlsRouteRulesBackendRefs",
            "TcpRouteRulesBackendRefs",
            "UdpRouteRulesBackendRefs",
        ],
    ),
    common(
        "RouteParentStatus",
        &[
            "HttpRouteStatusParents",
            "GrpcRouteStatusParents",
            "TlsRouteStatusParents",
            "TcpRouteStatusParents",
            "UdpRouteStatusParents",
        ],
    ),
    common(
        "LocalObjectReference",
        &[
            "HttpRouteRulesFiltersExtensionRef",
            "HttpRouteRulesBackendRefsFiltersExtensionRef",
            "GrpcRouteRulesFiltersExtensionRef",
            "GrpcRouteRulesBackendRefsFiltersExtensionRef",
            "BackendTlsPolicyValidationCaCertificateRefs",
        ],
    ),
    common(
        "SecretObjectReference",
        &[
            "GatewayListenersTlsCertificateRefs",
            "ListenerSetListenersTlsCertificateRefs",
            "GatewayTlsBackendClientCertificateRef",
        ],
    ),
    common(
        "BackendObjectReference",
        &[
            "HttpRouteRulesFiltersRequestMirrorBackendRef",
            "HttpRouteRulesBackendRefsFiltersRequestMirrorBackendRef",
            "GrpcRouteRulesFiltersRequestMirrorBackendRef",
            "GrpcRouteRulesBackendRefsFiltersRequestMirrorBackendRef",
            "HttpRouteRulesFiltersExternalAuthBackendRef",
            "HttpRouteRulesBackendRefsFiltersExternalAuthBackendRef",
        ],
    ),
    common(
        "ObjectReference",
        &[
            "GatewayTlsFrontendDefaultValidationCaCertificateRefs",
            "GatewayTlsFrontendPerPortTlsValidationCaCertificateRefs",
        ],
    ),
    common("ParentGatewayReference", &["ListenerSetParentRef"]),
    common("LocalPolicyTargetReference", &["XBackendTrafficPolicyTargetRefs"]),
    common(
        "LocalPolicyTargetReferenceWithSectionName",
        &["BackendTlsPolicyTargetRefs"],
    ),
    common(
        "PolicyStatus",
        &["BackendTlsPolicyStatus", "XBackendTrafficPolicyStatus"],
    ),
    common(
        "PolicyAncestorStatus",
        &[
            "BackendTlsPolicyStatusAncestors",
            "XBackendTrafficPolicyStatusAncestors",
        ],
    ),
    common(
        "SessionPersistence",
        &[
            "HttpRouteRulesSessionPersistence",
            "GrpcRouteRulesSessionPersistence",
            "XBackendTrafficPolicySessionPersistence",
        ],
    ),
    common(
        "SessionPersistenceType",
        &[
            "HttpRouteRulesSessionPersistenceType",
            "GrpcRouteRulesSessionPersistenceType",
            "XBackendTrafficPolicySessionPersistenceType",
        ],
    ),
    common(
        "CookieConfig",
        &[
            "HttpRouteRulesSessionPersistenceCookieConfig",
            "GrpcRouteRulesSessionPersistenceCookieConfig",
            "XBackendTrafficPolicySessionPersistenceCookieConfig",
        ],
    ),
    common(
        "CookieLifetimeType",
        &[
            "HttpRouteRulesSessionPersistenceCookieConfigLifetimeType",
            "GrpcRouteRulesSessionPersistenceCookieConfigLifetimeType",
            "XBackendTrafficPolicySessionPersistenceCookieConfigLifetimeType",
        ],
    ),
    common(
        "Fraction",
        &[
            "HttpRouteRulesFiltersRequestMirrorFraction",
            "HttpRouteRulesBackendRefsFiltersRequestMirrorFraction",
            "GrpcRouteRulesFiltersRequestMirrorFraction",
            "GrpcRouteRulesBackendRefsFiltersRequestMirrorFraction",
        ],
    ),
    common("RequestRate", &["XBackendTrafficPolicyRetryConstraintMinRetryRate"]),
    common(
        "HTTPHeaderFilter",
        &[
            "HttpRouteRulesFiltersRequestHeaderModifier",
            "HttpRouteRulesFiltersResponseHeaderModifier",
            "HttpRouteRulesBackendRefsFiltersRequestHeaderModifier",
            "HttpRouteRulesBackendRefsFiltersResponseHeaderModifier",
            "GrpcRouteRulesFiltersRequestHeaderModifier",
            "GrpcRouteRulesFiltersResponseHeaderModifier",
            "GrpcRouteRulesBackendRefsFiltersRequestHeaderModifier",
            "GrpcRouteRulesBackendRefsFiltersResponseHeaderModifier",
        ],
    ),
    common(
        "HTTPHeader",
        &[
            "HttpRouteRulesFiltersRequestHeaderModifierAdd",
            "HttpRouteRulesFiltersRequestHeaderModifierSet",
            "HttpRouteRulesFiltersResponseHeaderModifierAdd",
            "HttpRouteRulesFiltersResponseHeaderModifierSet",
            "HttpRouteRulesBackendRefsFiltersRequestHeaderModifierAdd",
            "HttpRouteRulesBackendRefsFiltersRequestHeaderModifierSet",
            "HttpRouteRulesBackendRefsFiltersResponseHeaderModifierAdd",
            "HttpRouteRulesBackendRefsFiltersResponseHeaderModifierSet",
            "GrpcRouteRulesFiltersRequestHeaderModifierAdd",
            "GrpcRouteRulesFiltersRequestHeaderModifierSet",
            "GrpcRouteRulesFiltersResponseHeaderModifierAdd",
            "GrpcRouteRulesFiltersResponseHeaderModifierSet",
            "GrpcRouteRulesBackendRefsFiltersRequestHeaderModifierAdd",
            "GrpcRouteRulesBackendRefsFiltersRequestHeaderModifierSet",
            "GrpcRouteRulesBackendRefsFiltersResponseHeaderModifierAdd",
            "GrpcRouteRulesBackendRefsFiltersResponseHeaderModifierSet",
        ],
    ),
    common(
        "HTTPRequestMirrorFilter",
        &[
            "HttpRouteRulesFiltersRequestMirror",
            "HttpRouteRulesBackendRefsFiltersRequestMirror",
            "GrpcRouteRulesFiltersRequestMirror",
            "GrpcRouteRulesBackendRefsFiltersRequestMirror",
        ],
    ),
    common(
        "AllowedRoutes",
        &["GatewayListenersAllowedRoutes", "ListenerSetListenersAllowedRoutes"],
    ),
    common(
        "RouteNamespaces",
        &[
            "GatewayListenersAllowedRoutesNamespaces",
            "ListenerSetListenersAllowedRoutesNamespaces",
        ],
    ),
    // Not `GatewayAllowedListenersNamespacesFrom`: that field allows `None` too,
    // and has a different default.
    common(
        "FromNamespaces",
        &[
            "GatewayListenersAllowedRoutesNamespacesFrom",
            "ListenerSetListenersAllowedRoutesNamespacesFrom",
        ],
    ),
    common(
        "RouteGroupKind",
        &[
            "GatewayListenersAllowedRoutesKinds",
            "ListenerSetListenersAllowedRoutesKinds",
            "GatewayStatusListenersSupportedKinds",
            "ListenerSetStatusListenersSupportedKinds",
        ],
    ),
    common("ListenerTLSConfig", &["GatewayListenersTls", "ListenerSetListenersTls"]),
    common(
        "TLSModeType",
        &["GatewayListenersTlsMode", "ListenerSetListenersTlsMode"],
    ),
    common(
        "ParametersReference",
        &["GatewayClassParametersRef", "XMeshParametersRef"],
    ),
    common(
        "SupportedFeature",
        &["GatewayClassStatusSupportedFeatures", "XMeshStatusSupportedFeatures"],
    ),
];

/// Groups of types that share a shape but are distinct upstream. In each
/// channel, the names of every group of shape-identical types, restricted to
/// those the channel generates, must equal one of these.
pub const DISTINCT: &[&[&str]] = &[
    // LocalObjectReference, LocalParametersReference, LocalPolicyTargetReference
    &[
        "LocalObjectReference",
        "GatewayInfrastructureParametersRef",
        "LocalPolicyTargetReference",
    ],
    &["ObjectReference", "ParametersReference"],
    &["SecretObjectReference", "ParentGatewayReference"],
    // HTTPHeaderMatch, HTTPQueryParamMatch, GRPCHeaderMatch
    &[
        "HttpRouteRulesMatchesHeaders",
        "HttpRouteRulesMatchesQueryParams",
        "GrpcRouteRulesMatchesHeaders",
    ],
    // HeaderMatchType, QueryParamMatchType, GRPCHeaderMatchType, GRPCMethodMatchType
    &[
        "HttpRouteRulesMatchesHeadersType",
        "HttpRouteRulesMatchesQueryParamsType",
        "GrpcRouteRulesMatchesHeadersType",
        "GrpcRouteRulesMatchesMethodType",
    ],
    // TCPRouteRule, TLSRouteRule, UDPRouteRule
    &["TcpRouteRules", "TlsRouteRules", "UdpRouteRules"],
    // Listener, ListenerEntry
    &["GatewayListeners", "ListenerSetListeners"],
    // ListenerStatus, ListenerEntryStatus
    &["GatewayStatusListeners", "ListenerSetStatusListeners"],
    &[
        "HttpRouteStatus",
        "GrpcRouteStatus",
        "TlsRouteStatus",
        "TcpRouteStatus",
        "UdpRouteStatus",
    ],
    &["TcpRouteSpec", "UdpRouteSpec"],
    &["GatewayClassSpec", "XMeshSpec"],
    &["GatewayClassStatus", "XMeshStatus"],
    // TODO(phase 3): removed by the per-resource Go types.
    &["HttpRouteRulesFilters", "HttpRouteRulesBackendRefsFilters"],
    &["HttpRouteRulesFiltersType", "HttpRouteRulesBackendRefsFiltersType"],
    &["HttpRouteRulesFiltersCors", "HttpRouteRulesBackendRefsFiltersCors"],
    &[
        "HttpRouteRulesFiltersRequestRedirect",
        "HttpRouteRulesBackendRefsFiltersRequestRedirect",
    ],
    &[
        "HttpRouteRulesFiltersRequestRedirectScheme",
        "HttpRouteRulesBackendRefsFiltersRequestRedirectScheme",
    ],
    &[
        "HttpRouteRulesFiltersRequestRedirectStatusCode",
        "HttpRouteRulesBackendRefsFiltersRequestRedirectStatusCode",
    ],
    &[
        "HttpRouteRulesFiltersUrlRewrite",
        "HttpRouteRulesBackendRefsFiltersUrlRewrite",
    ],
    &[
        "HttpRouteRulesFiltersRequestRedirectPath",
        "HttpRouteRulesFiltersUrlRewritePath",
        "HttpRouteRulesBackendRefsFiltersRequestRedirectPath",
        "HttpRouteRulesBackendRefsFiltersUrlRewritePath",
    ],
    &[
        "HttpRouteRulesFiltersRequestRedirectPathType",
        "HttpRouteRulesFiltersUrlRewritePathType",
        "HttpRouteRulesBackendRefsFiltersRequestRedirectPathType",
        "HttpRouteRulesBackendRefsFiltersUrlRewritePathType",
    ],
    &[
        "HttpRouteRulesFiltersExternalAuth",
        "HttpRouteRulesBackendRefsFiltersExternalAuth",
    ],
    &[
        "HttpRouteRulesFiltersExternalAuthProtocol",
        "HttpRouteRulesBackendRefsFiltersExternalAuthProtocol",
    ],
    &[
        "HttpRouteRulesFiltersExternalAuthGrpc",
        "HttpRouteRulesBackendRefsFiltersExternalAuthGrpc",
    ],
    &[
        "HttpRouteRulesFiltersExternalAuthHttp",
        "HttpRouteRulesBackendRefsFiltersExternalAuthHttp",
    ],
    &[
        "HttpRouteRulesFiltersExternalAuthForwardBody",
        "HttpRouteRulesBackendRefsFiltersExternalAuthForwardBody",
    ],
    &["GrpcRouteRulesFilters", "GrpcRouteRulesBackendRefsFilters"],
    &["GrpcRouteRulesFiltersType", "GrpcRouteRulesBackendRefsFiltersType"],
    &["GatewayTlsFrontendDefault", "GatewayTlsFrontendPerPortTls"],
    &[
        "GatewayTlsFrontendDefaultValidation",
        "GatewayTlsFrontendPerPortTlsValidation",
    ],
    &[
        "GatewayTlsFrontendDefaultValidationMode",
        "GatewayTlsFrontendPerPortTlsValidationMode",
    ],
    // TODO(phase 5): removed by `k8s-openapi`'s `LabelSelector`.
    &[
        "RouteNamespacesSelectorMatchExpressions",
        "GatewayAllowedListenersNamespacesSelectorMatchExpressions",
    ],
];
