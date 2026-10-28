// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apis/v1beta1` at v1.5.1.
//!
//! Its kinds, as the CRDs in `config/crd/standard` define them:
//!
//! - [`Gateway`]: `gateway.networking.k8s.io/v1beta1`
//!   - served: yes
//!   - storage: no
//!   - deprecated: no
//!
//! - [`GatewayClass`]: `gateway.networking.k8s.io/v1beta1`
//!   - served: yes
//!   - storage: no
//!   - deprecated: no
//!
//! - [`HttpRoute`]: `gateway.networking.k8s.io/v1beta1`
//!   - served: yes
//!   - storage: no
//!   - deprecated: no
//!
//! - [`ReferenceGrant`]: `gateway.networking.k8s.io/v1beta1`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
pub use crate::apis::standard::v1::{
    AddressType, AllowedRoutes, AnnotationKey, AnnotationValue, BackendObjectReference, BackendRef, CommonRouteSpec,
    Duration, FromNamespaces, GatewayClassConditionReason, GatewayClassConditionType, GatewayClassStatus,
    GatewayConditionReason, GatewayConditionType, GatewayController, GatewaySpecAddress, GatewayStatus, Group,
    HeaderMatchType, Hostname, HttpBackendRef, HttpHeader, HttpHeaderFilter, HttpHeaderMatch, HttpHeaderName,
    HttpMethod, HttpPathMatch, HttpPathModifier, HttpPathModifierType, HttpQueryParamMatch, HttpRequestMirrorFilter,
    HttpRequestRedirectFilter, HttpRouteFilter, HttpRouteFilterType, HttpRouteMatch, HttpRouteRule, HttpRouteStatus,
    HttpRouteTimeouts, HttpUrlRewriteFilter, Kind, Listener, ListenerConditionReason, ListenerConditionType,
    ListenerStatus, ListenerTlsConfig, LocalObjectReference, Namespace, ObjectName, ParametersReference,
    ParentReference, PathMatchType, PortNumber, PreciseHostname, ProtocolType, QueryParamMatchType, ReferenceGrantFrom,
    ReferenceGrantTo, RouteConditionReason, RouteConditionType, RouteGroupKind, RouteNamespaces, RouteParentStatus,
    RouteStatus, SecretObjectReference, SectionName, TlsModeType,
};
use crate::apis::standard::v1::{AllowedListeners, GatewayInfrastructure, GatewayTlsConfig};
/// GatewaySpec defines the desired state of Gateway.
///
/// Not all possible combinations of options specified in the Spec are
/// valid. Some invalid configurations can be caught synchronously via CRD
/// validation, but there are many cases that will require asynchronous
/// signaling via the GatewayStatus block.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "Gateway",
    root = "Gateway",
    plural = "gateways"
)]
#[kube(namespaced)]
#[kube(status = "GatewayStatus")]
#[kube(shortname = "gtw")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Class", type_ = "string", json_path = ".spec.gatewayClassName"))]
#[kube(printcolumn(name = "Address", type_ = "string", json_path = ".status.addresses[*].value"))]
#[kube(printcolumn(
    name = "Programmed",
    type_ = "string",
    json_path = ".status.conditions[?(@.type==\"Programmed\")].status"
))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "Gateway represents an instance of a service-traffic handling infrastructure\nby binding Listeners to a set of IP addresses."
)]
pub struct GatewaySpec {
    /// GatewayClassName used for this Gateway. This is the name of a
    /// GatewayClass resource.
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: ObjectName,
    /// Listeners associated with this Gateway. Listeners define
    /// logical endpoints that are bound on this Gateway's addresses.
    /// At least one Listener MUST be specified.
    ///
    /// ## Distinct Listeners
    ///
    /// Each Listener in a set of Listeners (for example, in a single Gateway)
    /// MUST be _distinct_, in that a traffic flow MUST be able to be assigned to
    /// exactly one listener. (This section uses "set of Listeners" rather than
    /// "Listeners in a single Gateway" because implementations MAY merge configuration
    /// from multiple Gateways onto a single data plane, and these rules _also_
    /// apply in that case).
    ///
    /// Practically, this means that each listener in a set MUST have a unique
    /// combination of Port, Protocol, and, if supported by the protocol, Hostname.
    ///
    /// Some combinations of port, protocol, and TLS settings are considered
    /// Core support and MUST be supported by implementations based on the objects
    /// they support:
    ///
    /// HTTPRoute
    ///
    /// 1. HTTPRoute, Port: 80, Protocol: HTTP
    /// 2. HTTPRoute, Port: 443, Protocol: HTTPS, TLS Mode: Terminate, TLS keypair provided
    ///
    /// TLSRoute
    ///
    /// 1. TLSRoute, Port: 443, Protocol: TLS, TLS Mode: Passthrough
    ///
    /// "Distinct" Listeners have the following property:
    ///
    /// **The implementation can match inbound requests to a single distinct
    /// Listener**.
    ///
    /// When multiple Listeners share values for fields (for
    /// example, two Listeners with the same Port value), the implementation
    /// can match requests to only one of the Listeners using other
    /// Listener fields.
    ///
    /// When multiple listeners have the same value for the Protocol field, then
    /// each of the Listeners with matching Protocol values MUST have different
    /// values for other fields.
    ///
    /// The set of fields that MUST be different for a Listener differs per protocol.
    /// The following rules define the rules for what fields MUST be considered for
    /// Listeners to be distinct with each protocol currently defined in the
    /// Gateway API spec.
    ///
    /// The set of listeners that all share a protocol value MUST have _different_
    /// values for _at least one_ of these fields to be distinct:
    ///
    /// * **HTTP, HTTPS, TLS**: Port, Hostname
    /// * **TCP, UDP**: Port
    ///
    /// One **very** important rule to call out involves what happens when an
    /// implementation:
    ///
    /// * Supports TCP protocol Listeners, as well as HTTP, HTTPS, or TLS protocol
    ///   Listeners, and
    /// * sees HTTP, HTTPS, or TLS protocols with the same `port` as one with TCP
    ///   Protocol.
    ///
    /// In this case all the Listeners that share a port with the
    /// TCP Listener are not distinct and so MUST NOT be accepted.
    ///
    /// If an implementation does not support TCP Protocol Listeners, then the
    /// previous rule does not apply, and the TCP Listeners SHOULD NOT be
    /// accepted.
    ///
    /// Note that the `tls` field is not used for determining if a listener is distinct, because
    /// Listeners that _only_ differ on TLS config will still conflict in all cases.
    ///
    /// ### Listeners that are distinct only by Hostname
    ///
    /// When the Listeners are distinct based only on Hostname, inbound request
    /// hostnames MUST match from the most specific to least specific Hostname
    /// values to choose the correct Listener and its associated set of Routes.
    ///
    /// Exact matches MUST be processed before wildcard matches, and wildcard
    /// matches MUST be processed before fallback (empty Hostname value)
    /// matches. For example, `"foo.example.com"` takes precedence over
    /// `"*.example.com"`, and `"*.example.com"` takes precedence over `""`.
    ///
    /// Additionally, if there are multiple wildcard entries, more specific
    /// wildcard entries must be processed before less specific wildcard entries.
    /// For example, `"*.foo.example.com"` takes precedence over `"*.example.com"`.
    ///
    /// The precise definition here is that the higher the number of dots in the
    /// hostname to the right of the wildcard character, the higher the precedence.
    ///
    /// The wildcard character will match any number of characters _and dots_ to
    /// the left, however, so `"*.example.com"` will match both
    /// `"foo.bar.example.com"` _and_ `"bar.example.com"`.
    ///
    /// ## Handling indistinct Listeners
    ///
    /// If a set of Listeners contains Listeners that are not distinct, then those
    /// Listeners are _Conflicted_, and the implementation MUST set the "Conflicted"
    /// condition in the Listener Status to "True".
    ///
    /// The words "indistinct" and "conflicted" are considered equivalent for the
    /// purpose of this documentation.
    ///
    /// Implementations MAY choose to accept a Gateway with some Conflicted
    /// Listeners only if they only accept the partial Listener set that contains
    /// no Conflicted Listeners.
    ///
    /// Specifically, an implementation MAY accept a partial Listener set subject to
    /// the following rules:
    ///
    /// * The implementation MUST NOT pick one conflicting Listener as the winner.
    ///   ALL indistinct Listeners must not be accepted for processing.
    /// * At least one distinct Listener MUST be present, or else the Gateway effectively
    ///   contains _no_ Listeners, and must be rejected from processing as a whole.
    ///
    /// The implementation MUST set a "ListenersNotValid" condition on the
    /// Gateway Status when the Gateway contains Conflicted Listeners whether or
    /// not they accept the Gateway. That Condition SHOULD clearly
    /// indicate in the Message which Listeners are conflicted, and which are
    /// Accepted. Additionally, the Listener status for those listeners SHOULD
    /// indicate which Listeners are conflicted and not Accepted.
    ///
    /// ## General Listener behavior
    ///
    /// Note that, for all distinct Listeners, requests SHOULD match at most one Listener.
    /// For example, if Listeners are defined for "foo.example.com" and "*.example.com", a
    /// request to "foo.example.com" SHOULD only be routed using routes attached
    /// to the "foo.example.com" Listener (and not the "*.example.com" Listener).
    ///
    /// This concept is known as "Listener Isolation", and it is an Extended feature
    /// of Gateway API. Implementations that do not support Listener Isolation MUST
    /// clearly document this, and MUST NOT claim support for the
    /// `GatewayHTTPListenerIsolation` feature.
    ///
    /// Implementations that _do_ support Listener Isolation SHOULD claim support
    /// for the Extended `GatewayHTTPListenerIsolation` feature and pass the associated
    /// conformance tests.
    ///
    /// ## Compatible Listeners
    ///
    /// A Gateway's Listeners are considered _compatible_ if:
    ///
    /// 1. They are distinct.
    /// 2. The implementation can serve them in compliance with the Addresses
    ///    requirement that all Listeners are available on all assigned
    ///    addresses.
    ///
    /// Compatible combinations in Extended support are expected to vary across
    /// implementations. A combination that is compatible for one implementation
    /// may not be compatible for another.
    ///
    /// For example, an implementation that cannot serve both TCP and UDP listeners
    /// on the same address, or cannot mix HTTPS and generic TLS listens on the same port
    /// would not consider those cases compatible, even though they are distinct.
    ///
    /// Implementations MAY merge separate Gateways onto a single set of
    /// Addresses if all Listeners across all Gateways are compatible.
    ///
    /// In a future release the MinItems=1 requirement MAY be dropped.
    ///
    /// Support: Core
    pub listeners: Vec<Listener>,
    /// Addresses requested for this Gateway. This is optional and behavior can
    /// depend on the implementation. If a value is set in the spec and the
    /// requested address is invalid or unavailable, the implementation MUST
    /// indicate this in an associated entry in GatewayStatus.Conditions.
    ///
    /// The Addresses field represents a request for the address(es) on the
    /// "outside of the Gateway", that traffic bound for this Gateway will use.
    /// This could be the IP address or hostname of an external load balancer or
    /// other networking infrastructure, or some other address that traffic will
    /// be sent to.
    ///
    /// If no Addresses are specified, the implementation MAY schedule the
    /// Gateway in an implementation-specific manner, assigning an appropriate
    /// set of Addresses.
    ///
    /// The implementation MUST bind all Listeners to every GatewayAddress that
    /// it assigns to the Gateway and add a corresponding entry in
    /// GatewayStatus.Addresses.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewaySpecAddress>>,
    /// Infrastructure defines infrastructure level attributes about this Gateway instance.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<GatewayInfrastructure>,
    /// AllowedListeners defines which ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedListeners")]
    pub allowed_listeners: Option<AllowedListeners>,
    /// TLS specifies frontend and backend tls configuration for entire gateway.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayTlsConfig>,
}
impl GatewaySpec {
    /// Creates a `GatewaySpec` from its required fields, with the optional fields `None`.
    pub fn new(gateway_class_name: impl Into<ObjectName>, listeners: impl Into<Vec<Listener>>) -> Self {
        Self {
            gateway_class_name: gateway_class_name.into(),
            listeners: listeners.into(),
            addresses: None,
            infrastructure: None,
            allowed_listeners: None,
            tls: None,
        }
    }
}
/// GatewayClassSpec reflects the configuration of a class of Gateways.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "GatewayClass",
    root = "GatewayClass",
    plural = "gatewayclasses"
)]
#[kube(status = "GatewayClassStatus")]
#[kube(shortname = "gc")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Controller", type_ = "string", json_path = ".spec.controllerName"))]
#[kube(printcolumn(
    name = "Accepted",
    type_ = "string",
    json_path = ".status.conditions[?(@.type==\"Accepted\")].status"
))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(printcolumn(
    name = "Description",
    type_ = "string",
    json_path = ".spec.description",
    priority = 1i32
))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "GatewayClass describes a class of Gateways available to the user for creating\nGateway resources.\n\nIt is recommended that this resource be used as a template for Gateways. This\nmeans that a Gateway is based on the state of the GatewayClass at the time it\nwas created and changes to the GatewayClass or associated parameters are not\npropagated down to existing Gateways. This recommendation is intended to\nlimit the blast radius of changes to GatewayClass or associated parameters.\nIf implementations choose to propagate GatewayClass changes to existing\nGateways, that MUST be clearly documented by the implementation.\n\nWhenever one or more Gateways are using a GatewayClass, implementations SHOULD\nadd the `gateway-exists-finalizer.gateway.networking.k8s.io` finalizer on the\nassociated GatewayClass. This ensures that a GatewayClass associated with a\nGateway is not deleted while in use.\n\nGatewayClass is a Cluster level resource."
)]
pub struct GatewayClassSpec {
    /// ControllerName is the name of the controller that is managing Gateways of
    /// this class. The value of this field MUST be a domain prefixed path.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// This field is not mutable and cannot be empty.
    ///
    /// Support: Core
    #[serde(rename = "controllerName")]
    pub controller_name: GatewayController,
    /// ParametersRef is a reference to a resource that contains the configuration
    /// parameters corresponding to the GatewayClass. This is optional if the
    /// controller does not require any additional configuration.
    ///
    /// ParametersRef can reference a standard Kubernetes resource, i.e. ConfigMap,
    /// or an implementation-specific custom resource. The resource can be
    /// cluster-scoped or namespace-scoped.
    ///
    /// If the referent cannot be found, refers to an unsupported kind, or when
    /// the data within that resource is malformed, the GatewayClass SHOULD be
    /// rejected with the "Accepted" status condition set to "False" and an
    /// "InvalidParameters" reason.
    ///
    /// A Gateway for this GatewayClass may provide its own `parametersRef`. When both are specified,
    /// the merging behavior is implementation specific.
    /// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parametersRef")]
    pub parameters_ref: Option<ParametersReference>,
    /// Description helps describe a GatewayClass with more details.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl GatewayClassSpec {
    /// Creates a `GatewayClassSpec` from its required fields, with the optional fields `None`.
    pub fn new(controller_name: impl Into<GatewayController>) -> Self {
        Self {
            controller_name: controller_name.into(),
            parameters_ref: None,
            description: None,
        }
    }
}
/// HTTPRouteSpec defines the desired state of HTTPRoute
#[derive(
    kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "HTTPRoute",
    root = "HttpRoute",
    plural = "httproutes"
)]
#[kube(namespaced)]
#[kube(status = "HttpRouteStatus")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Hostnames", type_ = "string", json_path = ".spec.hostnames"))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "HTTPRoute provides a way to route HTTP requests. This includes the capability\nto match requests by hostname, path, header, or query param. Filters can be\nused to specify additional processing steps. Backends specify where matching\nrequests should be routed."
)]
pub struct HttpRouteSpec {
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<ParentReference>>,
    /// Hostnames defines a set of hostnames that should match against the HTTP Host
    /// header to select a HTTPRoute used to process the request. Implementations
    /// MUST ignore any port value specified in the HTTP Host header while
    /// performing a match and (absent of any applicable header modification
    /// configuration) MUST forward this header unmodified to the backend.
    ///
    /// Valid values for Hostnames are determined by RFC 1123 definition of a
    /// hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and HTTPRoute, there
    /// must be at least one intersecting hostname for the HTTPRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches HTTPRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `*.example.com`, `test.example.com`, and `foo.test.example.com` would
    ///   all match. On the other hand, `example.com` and `test.example.net` would
    ///   not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, any
    /// HTTPRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// HTTPRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` must not be considered for a match.
    ///
    /// If both the Listener and HTTPRoute have specified hostnames, and none
    /// match with the criteria above, then the HTTPRoute is not accepted. The
    /// implementation must raise an 'Accepted' Condition with a status of
    /// `False` in the corresponding RouteParentStatus.
    ///
    /// In the event that multiple HTTPRoutes specify intersecting hostnames (e.g.
    /// overlapping wildcard matching and exact matching hostnames), precedence must
    /// be given to rules from the HTTPRoute with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    ///
    /// If ties exist across multiple Routes, the matching precedence rules for
    /// HTTPRouteMatches takes over.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<Hostname>>,
    /// Rules are a list of HTTP matchers, filters and actions.
    ///
    /// Default: `{{matches: {{path: {type: "PathPrefix", value: "/"}}}}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HttpRouteRule>>,
}
/// ReferenceGrantSpec identifies a cross namespace relationship that is trusted
/// for Gateway API.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1beta1",
    kind = "ReferenceGrant",
    root = "ReferenceGrant",
    plural = "referencegrants"
)]
#[kube(namespaced)]
#[kube(shortname = "refgrant")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "ReferenceGrant identifies kinds of resources in other namespaces that are\ntrusted to reference the specified kinds of resources in the same namespace\nas the policy.\n\nEach ReferenceGrant can be used to represent a unique trust relationship.\nAdditional Reference Grants can be used to add to the set of trusted\nsources of inbound references for the namespace they are defined within.\n\nAll cross-namespace references in Gateway API (with the exception of cross-namespace\nGateway-route attachment) require a ReferenceGrant.\n\nReferenceGrant is a form of runtime verification allowing users to assert\nwhich cross-namespace object references are permitted. Implementations that\nsupport ReferenceGrant MUST NOT permit cross-namespace references which have\nno grant, and MUST respond to the removal of a grant by revoking the access\nthat the grant allowed."
)]
pub struct ReferenceGrantSpec {
    /// From describes the trusted namespaces and kinds that can reference the
    /// resources described in "To". Each entry in this list MUST be considered
    /// to be an additional place that references can be valid from, or to put
    /// this another way, entries MUST be combined using OR.
    ///
    /// Support: Core
    pub from: Vec<ReferenceGrantFrom>,
    /// To describes the resources that may be referenced by the resources
    /// described in "From". Each entry in this list MUST be considered to be an
    /// additional place that references can be valid to, or to put this another
    /// way, entries MUST be combined using OR.
    ///
    /// Support: Core
    pub to: Vec<ReferenceGrantTo>,
}
impl ReferenceGrantSpec {
    /// Creates a `ReferenceGrantSpec` from its required fields, with the optional fields `None`.
    pub fn new(from: impl Into<Vec<ReferenceGrantFrom>>, to: impl Into<Vec<ReferenceGrantTo>>) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
        }
    }
}
