// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apis/v1alpha3`, experimental channel.

#[allow(unused_imports)]
use std::collections::BTreeMap;

#[allow(unused_imports)]
use kube::CustomResource;
#[allow(unused_imports)]
use schemars::JsonSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

// Go aliases of other packages' types
pub use crate::experimental::v1::CommonRouteSpec;
use crate::experimental::v1::{
    AnnotationValue, BackendTlsPolicyValidation, GatewayDefaultScope, LocalPolicyTargetReferenceWithSectionName,
    ParentReference, PolicyStatus, TlsRouteRule, TlsRouteStatus,
};
pub use crate::experimental::v1::{BackendRef, Hostname, RouteStatus, SectionName};

/// BackendTLSPolicySpec defines the desired state of BackendTLSPolicy.
///
/// Support: Extended
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha3",
    kind = "BackendTLSPolicy",
    root = "BackendTlsPolicy",
    plural = "backendtlspolicies"
)]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct BackendTlsPolicySpec {
    /// TargetRefs identifies an API object to apply the policy to.
    /// Note that this config applies to the entire referenced resource
    /// by default, but this default may change in the future to provide
    /// a more granular application of the policy.
    ///
    /// TargetRefs must be _distinct_. This means either that:
    ///
    /// * They select different targets. If this is the case, then targetRef
    ///   entries are distinct. In terms of fields, this means that the
    ///   multi-part key defined by `group`, `kind`, and `name` must
    ///   be unique across all targetRef entries in the BackendTLSPolicy.
    /// * They select different sectionNames in the same target.
    ///
    /// When more than one BackendTLSPolicy selects the same target and
    /// sectionName, implementations MUST determine precedence using the
    /// following criteria, continuing on ties:
    ///
    /// * The older policy by creation timestamp takes precedence. For
    ///   example, a policy with a creation timestamp of "2021-07-15
    ///   01:02:03" MUST be given precedence over a policy with a
    ///   creation timestamp of "2021-07-15 01:02:04".
    /// * The policy appearing first in alphabetical order by {namespace}/{name}.
    ///   For example, a policy named `foo/bar` is given precedence over a
    ///   policy named `foo/baz`.
    ///
    /// For any BackendTLSPolicy that does not take precedence, the
    /// implementation MUST ensure the `Accepted` Condition is set to
    /// `status: False`, with Reason `Conflicted`.
    ///
    /// Implementations SHOULD NOT support more than one targetRef at this
    /// time. Although the API technically allows for this, the current guidance
    /// for conflict resolution and status handling is lacking. Until that can be
    /// clarified in a future release, the safest approach is to support a single
    /// targetRef.
    ///
    /// Support Levels:
    ///
    /// * Extended: Kubernetes Service referenced by HTTPRoute backendRefs.
    ///
    /// * Implementation-Specific: Services not connected via HTTPRoute, and any
    ///   other kind of backend. Implementations MAY use BackendTLSPolicy for:
    ///   - Services not referenced by any Route (e.g., infrastructure services)
    ///   - Gateway feature backends (e.g., ExternalAuth, rate-limiting services)
    ///   - Service mesh workload-to-service communication
    ///   - Other resource types beyond Service
    ///
    /// Implementations SHOULD aim to ensure that BackendTLSPolicy behavior is consistent,
    /// even outside of the extended HTTPRoute -(backendRef) -> Service path.
    /// They SHOULD clearly document how BackendTLSPolicy is interpreted in these
    /// scenarios, including:
    ///   - Which resources beyond Service are supported
    ///   - How the policy is discovered and applied
    ///   - Any implementation-specific semantics or restrictions
    ///
    /// Note that this config applies to the entire referenced resource
    /// by default, but this default may change in the future to provide
    /// a more granular application of the policy.
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<LocalPolicyTargetReferenceWithSectionName>,
    /// Validation contains backend TLS validation configuration.
    pub validation: BackendTlsPolicyValidation,
    /// Options are a list of key/value pairs to enable extended TLS
    /// configuration for each implementation. For example, configuring the
    /// minimum TLS version or supported cipher suites.
    ///
    /// A set of common keys MAY be defined by the API in the future. To avoid
    /// any ambiguity, implementation-specific definitions MUST use
    /// domain-prefixed names, such as `example.com/my-custom-option`.
    /// Un-prefixed names are reserved for key names defined by Gateway API.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BTreeMap<String, AnnotationValue>>,
}

/// TLSRouteSpec defines the expected behavior of a TLSRoute.
/// A TLSRoute MUST be attached to a Listener of protocol TLS.
/// Core: The listener CAN be of type Passthrough
/// Extended: The listener CAN be of type Terminate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha3",
    kind = "TLSRoute",
    root = "TlsRoute",
    plural = "tlsroutes"
)]
#[kube(namespaced)]
#[kube(status = "TlsRouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct TlsRouteSpec {
    /// ParentRefs references the resources (usually Gateways) that a Route wants
    /// to be attached to. Note that the referenced parent resource needs to
    /// allow this for the attachment to be complete. For Gateways, that means
    /// the Gateway needs to allow attachment from Routes of this kind and
    /// namespace. For Services, that means the Service must either be in the same
    /// namespace for a "producer" route, or the mesh implementation must support
    /// and allow "consumer" routes for the referenced Service. ReferenceGrant is
    /// not applicable for governing ParentRefs to Services - it is not possible to
    /// create a "producer" route for a Service in a different namespace from the
    /// Route.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, ClusterIP Services only)
    ///
    /// This API may be extended in the future to support additional kinds of parent
    /// resources.
    ///
    /// ParentRefs must be _distinct_. This means either that:
    ///
    /// * They select different objects.  If this is the case, then parentRef
    ///   entries are distinct. In terms of fields, this means that the
    ///   multi-part key defined by `group`, `kind`, `namespace`, and `name` must
    ///   be unique across all parentRef entries in the Route.
    /// * They do not select different objects, but for each optional field used,
    ///   each ParentRef that selects the same object must set the same set of
    ///   optional fields to different values. If one ParentRef sets a
    ///   combination of optional fields, all must set the same combination.
    ///
    /// Some examples:
    ///
    /// * If one ParentRef sets `sectionName`, all ParentRefs referencing the
    ///   same object must also set `sectionName`.
    /// * If one ParentRef sets `port`, all ParentRefs referencing the same
    ///   object must also set `port`.
    /// * If one ParentRef sets `sectionName` and `port`, all ParentRefs
    ///   referencing the same object must also set `sectionName` and `port`.
    ///
    /// It is possible to separately reference multiple distinct objects that may
    /// be collapsed by an implementation. For example, some implementations may
    /// choose to merge compatible Gateway Listeners together. If that is the
    /// case, the list of routes attached to those resources should also be
    /// merged.
    ///
    /// Note that for ParentRefs that cross namespace boundaries, there are specific
    /// rules. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example,
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable other kinds of cross-namespace reference.
    ///
    /// ParentRefs from a Route to a Service in the same namespace are "producer"
    /// routes, which apply default routing rules to inbound connections from
    /// any namespace to the Service.
    ///
    /// ParentRefs from a Route to a Service in a different namespace are
    /// "consumer" routes, and these routing rules are only applied to outbound
    /// connections originating from the same namespace as the Route, for which
    /// the intended destination of the connections are a Service targeted as a
    /// ParentRef of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<ParentReference>>,
    /// UseDefaultGateways indicates the default Gateway scope to use for this
    /// Route. If unset (the default) or set to None, the Route will not be
    /// attached to any default Gateway; if set, it will be attached to any
    /// default Gateway supporting the named scope, subject to the usual rules
    /// about which Routes a Gateway is allowed to claim.
    ///
    /// Think carefully before using this functionality! The set of default
    /// Gateways supporting the requested scope can change over time without
    /// any notice to the Route author, and in many situations it will not be
    /// appropriate to request a default Gateway for a given Route -- for
    /// example, a Route with specific security requirements should almost
    /// certainly not use a default Gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "useDefaultGateways")]
    pub use_default_gateways: Option<GatewayDefaultScope>,
    /// Hostnames defines a set of SNI hostnames that should match against the
    /// SNI attribute of TLS ClientHello message in TLS handshake. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI hostnames per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    pub hostnames: Vec<Hostname>,
    /// Rules are a list of actions.
    pub rules: Vec<TlsRouteRule>,
}
