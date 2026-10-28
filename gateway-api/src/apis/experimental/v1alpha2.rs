// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apis/v1alpha2` at v1.5.1.
//!
//! Its kinds, as the CRDs in `config/crd/experimental` define them:
//!
//! - [`TcpRoute`]: `gateway.networking.k8s.io/v1alpha2`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`TlsRoute`]: `gateway.networking.k8s.io/v1alpha2`
//!   - served: yes
//!   - storage: no
//!   - deprecated: yes (The v1alpha2 version of TLSRoute has been deprecated and will be removed in a future release of the API. Please upgrade to v1.)
//!
//! - [`UdpRoute`]: `gateway.networking.k8s.io/v1alpha2`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

use crate::apis::experimental::v1::GatewayDefaultScope;
pub use crate::apis::experimental::v1::{
    AddressType, AnnotationKey, AnnotationValue, BackendObjectReference, BackendRef, CommonRouteSpec, Duration,
    GatewayController, Group, Hostname, Kind, LocalObjectReference, Namespace, ObjectName, ParentReference, PortNumber,
    PreciseHostname, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo, RouteConditionReason,
    RouteConditionType, RouteParentStatus, RouteStatus, SecretObjectReference, SectionName, SessionPersistence,
};
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalPolicyTargetReference {
    /// Group is the group of the target resource.
    pub group: Group,
    /// Kind is kind of the target resource.
    pub kind: Kind,
    /// Name is the name of the target resource.
    pub name: ObjectName,
}
impl LocalPolicyTargetReference {
    /// Creates a `LocalPolicyTargetReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<ObjectName>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct NamespacedPolicyTargetReference {
    /// Group is the group of the target resource.
    pub group: Group,
    /// Kind is kind of the target resource.
    pub kind: Kind,
    /// Name is the name of the target resource.
    pub name: ObjectName,
    /// Namespace is the namespace of the referent. When unspecified, the local
    /// namespace is inferred. Even when policy targets a resource in a different
    /// namespace, it MUST only apply to traffic originating from the same
    /// namespace as the policy.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}
impl NamespacedPolicyTargetReference {
    /// Creates a `NamespacedPolicyTargetReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<ObjectName>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
            namespace: None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalPolicyTargetReferenceWithSectionName {
    /// Group is the group of the target resource.
    pub group: Group,
    /// Kind is kind of the target resource.
    pub kind: Kind,
    /// Name is the name of the target resource.
    pub name: ObjectName,
    /// SectionName is the name of a section within the target resource. When
    /// unspecified, this targetRef targets the entire resource. In the following
    /// resources, SectionName is interpreted as the following:
    ///
    /// * Gateway: Listener name
    /// * HTTPRoute: HTTPRouteRule name
    /// * Service: Port name
    ///
    /// If a SectionName is specified, but does not exist on the targeted object,
    /// the Policy must fail to attach, and the policy implementation should record
    /// a `ResolvedRefs` or similar Condition in the Policy's status.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<SectionName>,
}
impl LocalPolicyTargetReferenceWithSectionName {
    /// Creates a `LocalPolicyTargetReferenceWithSectionName` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<ObjectName>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
            section_name: None,
        }
    }
}
pub type PolicyConditionType = String;
pub type PolicyConditionReason = String;
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct PolicyAncestorStatus {
    /// AncestorRef corresponds with a ParentRef in the spec that this
    /// PolicyAncestorStatus struct describes the status of.
    #[serde(rename = "ancestorRef")]
    pub ancestor_ref: ParentReference,
    /// ControllerName is a domain/path string that indicates the name of the
    /// controller that wrote this status. This corresponds with the
    /// controllerName field on GatewayClass.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// The format of this field is DOMAIN "/" PATH, where DOMAIN and PATH are
    /// valid Kubernetes names
    /// (https://kubernetes.io/docs/concepts/overview/working-with-objects/names/#names).
    ///
    /// Controllers MUST populate this field when writing status. Controllers should ensure that
    /// entries to status populated with their ControllerName are cleaned up when they are no
    /// longer necessary.
    #[serde(rename = "controllerName")]
    pub controller_name: GatewayController,
    /// Conditions describes the status of the Policy with respect to the given Ancestor.
    pub conditions: Vec<Condition>,
}
impl PolicyAncestorStatus {
    /// Creates a `PolicyAncestorStatus` from its required fields, with the optional fields `None`.
    pub fn new(
        ancestor_ref: impl Into<ParentReference>,
        controller_name: impl Into<GatewayController>,
        conditions: impl Into<Vec<Condition>>,
    ) -> Self {
        Self {
            ancestor_ref: ancestor_ref.into(),
            controller_name: controller_name.into(),
            conditions: conditions.into(),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct PolicyStatus {
    /// Ancestors is a list of ancestor resources (usually Gateways) that are
    /// associated with the policy, and the status of the policy with respect to
    /// each ancestor. When this policy attaches to a parent, the controller that
    /// manages the parent and the ancestors MUST add an entry to this list when
    /// the controller first sees the policy and SHOULD update the entry as
    /// appropriate when the relevant ancestor is modified.
    ///
    /// Note that choosing the relevant ancestor is left to the Policy designers;
    /// an important part of Policy design is designing the right object level at
    /// which to namespace this status.
    ///
    /// Note also that implementations MUST ONLY populate ancestor status for
    /// the Ancestor resources they are responsible for. Implementations MUST
    /// use the ControllerName field to uniquely identify the entries in this list
    /// that they are responsible for.
    ///
    /// Note that to achieve this, the list of PolicyAncestorStatus structs
    /// MUST be treated as a map with a composite key, made up of the AncestorRef
    /// and ControllerName fields combined.
    ///
    /// A maximum of 16 ancestors will be represented in this list. An empty list
    /// means the Policy is not relevant for any ancestors.
    ///
    /// If this slice is full, implementations MUST NOT add further entries.
    /// Instead they MUST consider the policy unimplementable and signal that
    /// on any related resources such as the ancestor that would be referenced
    /// here. For example, if this list was full on BackendTLSPolicy, no
    /// additional Gateways would be able to reference the Service targeted by
    /// the BackendTLSPolicy.
    pub ancestors: Vec<crate::apis::experimental::v1::PolicyAncestorStatus>,
}
impl PolicyStatus {
    /// Creates a `PolicyStatus` from its required fields, with the optional fields `None`.
    pub fn new(ancestors: impl Into<Vec<crate::apis::experimental::v1::PolicyAncestorStatus>>) -> Self {
        Self {
            ancestors: ancestors.into(),
        }
    }
}
/// TCPRouteSpec defines the desired state of TCPRoute
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TCPRoute",
    root = "TcpRoute",
    plural = "tcproutes"
)]
#[kube(namespaced)]
#[kube(status = "TcpRouteStatus")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "TCPRoute provides a way to route TCP requests. When combined with a Gateway\nlistener, it can be used to forward connections on the port specified by the\nlistener to a set of backends specified by the TCPRoute."
)]
pub struct TcpRouteSpec {
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
    /// Rules are a list of TCP matchers and actions.
    pub rules: Vec<TcpRouteRule>,
}
impl TcpRouteSpec {
    /// Creates a `TcpRouteSpec` from its required fields, with the optional fields `None`.
    pub fn new(rules: impl Into<Vec<TcpRouteRule>>) -> Self {
        Self {
            parent_refs: None,
            use_default_gateways: None,
            rules: rules.into(),
        }
    }
}
/// TCPRouteStatus defines the observed state of TCPRoute
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TcpRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the route, and the status of the route with respect to
    /// each parent. When this route attaches to a parent, the controller that
    /// manages the parent must add an entry to this list when the controller
    /// first sees the route and should update the entry as appropriate when the
    /// route or gateway is modified.
    ///
    /// Note that parent references that cannot be resolved by an implementation
    /// of this API will not be added to this list. Implementations of this API
    /// can only populate Route status for the Gateways/parent resources they are
    /// responsible for.
    ///
    /// A maximum of 32 Gateways will be represented in this list. An empty list
    /// means the route has not been attached to any Gateway.
    pub parents: Vec<RouteParentStatus>,
}
impl TcpRouteStatus {
    /// Creates a `TcpRouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// TCPRouteRule is the configuration for a given rule.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TcpRouteRule {
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a nonexistent resource or a
    /// Service with no endpoints), the underlying implementation MUST actively
    /// reject connection attempts to this backend. Connection rejections must
    /// respect weight; if an invalid backend is requested to have 80% of
    /// connections, then 80% of connections must be rejected instead.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Extended
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<BackendRef>,
}
impl TcpRouteRule {
    /// Creates a `TcpRouteRule` from its required fields, with the optional fields `None`.
    pub fn new(backend_refs: impl Into<Vec<BackendRef>>) -> Self {
        Self {
            name: None,
            backend_refs: backend_refs.into(),
        }
    }
}
/// TLSRouteSpec defines the desired state of a TLSRoute resource.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "TLSRoute",
    root = "TlsRoute",
    plural = "tlsroutes"
)]
#[kube(namespaced)]
#[kube(status = "TlsRouteStatus")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "The TLSRoute resource is similar to TCPRoute, but can be configured\nto match against TLS-specific metadata. This allows more flexibility\nin matching streams for a given TLS listener."
)]
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
    /// Hostnames defines a set of SNI names that should match against the
    /// SNI attribute of TLS ClientHello message in TLS handshake. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI names per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and TLSRoute, there
    /// must be at least one intersecting hostname for the TLSRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches TLSRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches TLSRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `test.example.com` and `*.example.com` would both match. On the other
    ///   hand, `example.com` and `test.example.net` would not match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, any
    /// TLSRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// TLSRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` must not be considered for a match.
    ///
    /// If both the Listener and TLSRoute have specified hostnames, and none
    /// match with the criteria above, then the TLSRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<Hostname>>,
    /// Rules are a list of TLS matchers and actions.
    pub rules: Vec<TlsRouteRule>,
}
impl TlsRouteSpec {
    /// Creates a `TlsRouteSpec` from its required fields, with the optional fields `None`.
    pub fn new(rules: impl Into<Vec<TlsRouteRule>>) -> Self {
        Self {
            parent_refs: None,
            use_default_gateways: None,
            hostnames: None,
            rules: rules.into(),
        }
    }
}
/// TLSRouteStatus defines the observed state of TLSRoute
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TlsRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the route, and the status of the route with respect to
    /// each parent. When this route attaches to a parent, the controller that
    /// manages the parent must add an entry to this list when the controller
    /// first sees the route and should update the entry as appropriate when the
    /// route or gateway is modified.
    ///
    /// Note that parent references that cannot be resolved by an implementation
    /// of this API will not be added to this list. Implementations of this API
    /// can only populate Route status for the Gateways/parent resources they are
    /// responsible for.
    ///
    /// A maximum of 32 Gateways will be represented in this list. An empty list
    /// means the route has not been attached to any Gateway.
    pub parents: Vec<RouteParentStatus>,
}
impl TlsRouteStatus {
    /// Creates a `TlsRouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// TLSRouteRule is the configuration for a given rule.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TlsRouteRule {
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a nonexistent resource or
    /// a Service with no endpoints), the rule performs no forwarding; if no
    /// filters are specified that would result in a response being sent, the
    /// underlying implementation must actively reject request attempts to this
    /// backend, by rejecting the connection. Request rejections must respect
    /// weight; if an invalid backend is requested to have 80% of requests, then
    /// 80% of requests must be rejected instead.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Extended
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<BackendRef>,
}
impl TlsRouteRule {
    /// Creates a `TlsRouteRule` from its required fields, with the optional fields `None`.
    pub fn new(backend_refs: impl Into<Vec<BackendRef>>) -> Self {
        Self {
            name: None,
            backend_refs: backend_refs.into(),
        }
    }
}
/// UDPRouteSpec defines the desired state of UDPRoute.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1alpha2",
    kind = "UDPRoute",
    root = "UdpRoute",
    plural = "udproutes"
)]
#[kube(namespaced)]
#[kube(status = "UdpRouteStatus")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "UDPRoute provides a way to route UDP traffic. When combined with a Gateway\nlistener, it can be used to forward traffic on the port specified by the\nlistener to a set of backends specified by the UDPRoute."
)]
pub struct UdpRouteSpec {
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
    /// Rules are a list of UDP matchers and actions.
    pub rules: Vec<UdpRouteRule>,
}
impl UdpRouteSpec {
    /// Creates a `UdpRouteSpec` from its required fields, with the optional fields `None`.
    pub fn new(rules: impl Into<Vec<UdpRouteRule>>) -> Self {
        Self {
            parent_refs: None,
            use_default_gateways: None,
            rules: rules.into(),
        }
    }
}
/// UDPRouteStatus defines the observed state of UDPRoute.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct UdpRouteStatus {
    /// Parents is a list of parent resources (usually Gateways) that are
    /// associated with the route, and the status of the route with respect to
    /// each parent. When this route attaches to a parent, the controller that
    /// manages the parent must add an entry to this list when the controller
    /// first sees the route and should update the entry as appropriate when the
    /// route or gateway is modified.
    ///
    /// Note that parent references that cannot be resolved by an implementation
    /// of this API will not be added to this list. Implementations of this API
    /// can only populate Route status for the Gateways/parent resources they are
    /// responsible for.
    ///
    /// A maximum of 32 Gateways will be represented in this list. An empty list
    /// means the route has not been attached to any Gateway.
    pub parents: Vec<RouteParentStatus>,
}
impl UdpRouteStatus {
    /// Creates a `UdpRouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// UDPRouteRule is the configuration for a given rule.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct UdpRouteRule {
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent. If unspecified or invalid (refers to a nonexistent resource or a
    /// Service with no endpoints), the underlying implementation MUST actively
    /// reject connection attempts to this backend. Packet drops must
    /// respect weight; if an invalid backend is requested to have 80% of
    /// the packets, then 80% of packets must be dropped instead.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Extended
    #[serde(rename = "backendRefs")]
    pub backend_refs: Vec<BackendRef>,
}
impl UdpRouteRule {
    /// Creates a `UdpRouteRule` from its required fields, with the optional fields `None`.
    pub fn new(backend_refs: impl Into<Vec<BackendRef>>) -> Self {
        Self {
            name: None,
            backend_refs: backend_refs.into(),
        }
    }
}
