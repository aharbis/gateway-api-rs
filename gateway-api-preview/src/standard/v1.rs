// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apis/v1`, standard channel.

#[allow(unused_imports)]
use std::collections::BTreeMap;

#[allow(unused_imports)]
use kube::CustomResource;
#[allow(unused_imports)]
use schemars::JsonSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

/// PortNumber defines a network port.
pub type PortNumber = i32;

/// BackendTLSPolicySpec defines the desired state of BackendTLSPolicy.
///
/// Support: Extended
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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

/// GatewayClassSpec reflects the configuration of a class of Gateways.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GatewayClass",
    root = "GatewayClass",
    plural = "gatewayclasses"
)]
#[kube(status = "GatewayClassStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
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

/// GatewaySpec defines the desired state of Gateway.
///
/// Not all possible combinations of options specified in the Spec are
/// valid. Some invalid configurations can be caught synchronously via CRD
/// validation, but there are many cases that will require asynchronous
/// signaling via the GatewayStatus block.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "Gateway",
    root = "Gateway",
    plural = "gateways"
)]
#[kube(namespaced)]
#[kube(status = "GatewayStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
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

/// GRPCRouteSpec defines the desired state of GRPCRoute
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GRPCRoute",
    root = "GrpcRoute",
    plural = "grpcroutes"
)]
#[kube(namespaced)]
#[kube(status = "GrpcRouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct GrpcRouteSpec {
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
    /// Hostnames defines a set of hostnames to match against the GRPC
    /// Host header to select a GRPCRoute to process the request. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label MUST appear by itself as the first label.
    ///
    /// If a hostname is specified by both the Listener and GRPCRoute, there
    /// MUST be at least one intersecting hostname for the GRPCRoute to be
    /// attached to the Listener. For example:
    ///
    /// * A Listener with `test.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames, or have specified at
    ///   least one of `test.example.com` or `*.example.com`.
    /// * A Listener with `*.example.com` as the hostname matches GRPCRoutes
    ///   that have either not specified any hostnames or have specified at least
    ///   one hostname that matches the Listener hostname. For example,
    ///   `test.example.com` and `*.example.com` would both match. On the other
    ///   hand, `example.com` and `test.example.net` would not match.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, any
    /// GRPCRoute hostnames that do not match the Listener hostname MUST be
    /// ignored. For example, if a Listener specified `*.example.com`, and the
    /// GRPCRoute specified `test.example.com` and `test.example.net`,
    /// `test.example.net` MUST NOT be considered for a match.
    ///
    /// If both the Listener and GRPCRoute have specified hostnames, and none
    /// match with the criteria above, then the GRPCRoute MUST NOT be accepted by
    /// the implementation. The implementation MUST raise an 'Accepted' Condition
    /// with a status of `False` in the corresponding RouteParentStatus.
    ///
    /// If a Route (A) of type HTTPRoute or GRPCRoute is attached to a
    /// Listener and that listener already has another Route (B) of the other
    /// type attached and the intersection of the hostnames of A and B is
    /// non-empty, then the implementation MUST accept exactly one of these two
    /// routes, determined by the following criteria, in order:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// The rejected Route MUST raise an 'Accepted' condition with a status of
    /// 'False' in the corresponding RouteParentStatus.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostnames: Option<Vec<Hostname>>,
    /// Rules are a list of GRPC matchers, filters and actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<GrpcRouteRule>>,
}

/// HTTPRouteSpec defines the desired state of HTTPRoute
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "HTTPRoute",
    root = "HttpRoute",
    plural = "httproutes"
)]
#[kube(namespaced)]
#[kube(status = "HttpRouteStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HttpRouteRule>>,
}

/// ListenerSetSpec defines the desired state of a ListenerSet.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "ListenerSet",
    root = "ListenerSet",
    plural = "listenersets"
)]
#[kube(namespaced)]
#[kube(status = "ListenerSetStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct ListenerSetSpec {
    /// ParentRef references the Gateway that the listeners are attached to.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentGatewayReference,
    /// Listeners associated with this ListenerSet. Listeners define
    /// logical endpoints that are bound on this referenced parent Gateway's addresses.
    ///
    /// Listeners in a `Gateway` and their attached `ListenerSets` are concatenated
    /// as a list when programming the underlying infrastructure. Each listener
    /// name does not need to be unique across the Gateway and ListenerSets.
    /// See ListenerEntry.Name for more details.
    ///
    /// Implementations MUST treat the parent Gateway as having the merged
    /// list of all listeners from itself and attached ListenerSets using
    /// the following precedence:
    ///
    /// 1. "parent" Gateway
    /// 2. ListenerSet ordered by creation time (oldest first)
    /// 3. ListenerSet ordered alphabetically by "{namespace}/{name}".
    ///
    /// An implementation MAY reject listeners by setting the ListenerEntryStatus
    /// `Accepted` condition to False with the Reason `TooManyListeners`
    ///
    /// If a listener has a conflict, this will be reported in the
    /// Status.ListenerEntryStatus setting the `Conflicted` condition to True.
    ///
    /// Implementations SHOULD be cautious about what information from the
    /// parent or siblings are reported to avoid accidentally leaking
    /// sensitive information that the child would not otherwise have access
    /// to. This can include contents of secrets etc.
    pub listeners: Vec<ListenerEntry>,
}

/// ReferenceGrantSpec identifies a cross namespace relationship that is trusted
/// for Gateway API.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "ReferenceGrant",
    root = "ReferenceGrant",
    plural = "referencegrants"
)]
#[kube(namespaced)]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
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

/// TLSRouteSpec defines the expected behavior of a TLSRoute.
/// A TLSRoute MUST be attached to a Listener of protocol TLS.
/// Core: The listener CAN be of type Passthrough
/// Extended: The listener CAN be of type Terminate
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parentRefs")]
    pub parent_refs: Option<Vec<ParentReference>>,
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

/// BackendTLSPolicyValidation contains backend TLS validation configuration.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendTlsPolicyValidation {
    /// CACertificateRefs contains one or more references to Kubernetes objects that
    /// contain a PEM-encoded TLS CA certificate bundle, which is used to
    /// validate a TLS handshake between the Gateway and backend Pod.
    ///
    /// If CACertificateRefs is empty or unspecified, then WellKnownCACertificates must be
    /// specified. Only one of CACertificateRefs or WellKnownCACertificates may be specified,
    /// not both. If CACertificateRefs is empty or unspecified, the configuration for
    /// WellKnownCACertificates MUST be honored instead if supported by the implementation.
    ///
    /// A CACertificateRef is invalid if:
    ///
    /// * It refers to a resource that cannot be resolved (e.g., the referenced resource
    ///   does not exist) or is misconfigured (e.g., a ConfigMap does not contain a key
    ///   named `ca.crt`). In this case, the Reason must be set to `InvalidCACertificateRef`
    ///   and the Message of the Condition must indicate which reference is invalid and why.
    ///
    /// * It refers to an unknown or unsupported kind of resource. In this case, the Reason
    ///   must be set to `InvalidKind` and the Message of the Condition must explain which
    ///   kind of resource is unknown or unsupported.
    ///
    /// * It refers to a resource in another namespace. This may change in future
    ///   spec updates.
    ///
    /// Implementations MAY choose to perform further validation of the certificate
    /// content (e.g., checking expiry or enforcing specific formats). In such cases,
    /// an implementation-specific Reason and Message must be set for the invalid reference.
    ///
    /// In all cases, the implementation MUST ensure the `ResolvedRefs` Condition on
    /// the BackendTLSPolicy is set to `status: False`, with a Reason and Message
    /// that indicate the cause of the error. Connections using an invalid
    /// CACertificateRef MUST fail, and the client MUST receive an HTTP 5xx error
    /// response. If ALL CACertificateRefs are invalid, the implementation MUST also
    /// ensure the `Accepted` Condition on the BackendTLSPolicy is set to
    /// `status: False`, with a Reason `NoValidCACertificate`.
    ///
    /// A single CACertificateRef to a Kubernetes ConfigMap kind has "Core" support.
    /// Implementations MAY choose to support attaching multiple certificates to
    /// a backend, but this behavior is implementation-specific.
    ///
    /// Support: Core - An optional single reference to a Kubernetes ConfigMap,
    /// with the CA certificate in a key named `ca.crt`.
    ///
    /// Support: Implementation-specific - More than one reference, other kinds
    /// of resources, or a single reference that includes multiple certificates.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "caCertificateRefs")]
    pub ca_certificate_refs: Option<Vec<LocalObjectReference>>,
    /// WellKnownCACertificates specifies whether a well-known set of CA certificates
    /// may be used in the TLS handshake between the gateway and backend pod.
    ///
    /// If WellKnownCACertificates is unspecified or empty (""), then CACertificateRefs
    /// must be specified with at least one entry for a valid configuration. Only one of
    /// CACertificateRefs or WellKnownCACertificates may be specified, not both.
    /// If an implementation does not support the WellKnownCACertificates field, or
    /// the supplied value is not recognized, the implementation MUST ensure the
    /// `Accepted` Condition on the BackendTLSPolicy is set to `status: False`, with
    /// a Reason `Invalid`.
    ///
    /// Valid values include:
    /// * "System" - indicates that well-known system CA certificates should be used.
    ///
    /// Implementations MAY define their own sets of CA certificates. Such definitions
    /// MUST use an implementation-specific, prefixed name, such as
    /// `mycompany.com/my-custom-ca-certificates`.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "wellKnownCACertificates")]
    pub well_known_ca_certificates: Option<WellKnownCaCertificatesType>,
    /// Hostname is used for two purposes in the connection between Gateways and
    /// backends:
    ///
    /// 1. Hostname MUST be used as the SNI to connect to the backend (RFC 6066).
    /// 2. Hostname MUST be used for authentication and MUST match the certificate
    ///    served by the matching backend, unless SubjectAltNames is specified.
    /// 3. If SubjectAltNames are specified, Hostname can be used for certificate selection
    ///    but MUST NOT be used for authentication. If you want to use the value
    ///    of the Hostname field for authentication, you MUST add it to the SubjectAltNames list.
    ///
    /// Support: Core
    pub hostname: PreciseHostname,
    /// SubjectAltNames contains one or more Subject Alternative Names.
    /// When specified the certificate served from the backend MUST
    /// have at least one Subject Alternate Name matching one of the specified SubjectAltNames.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<SubjectAltName>>,
}

/// SubjectAltName represents Subject Alternative Name.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct SubjectAltName {
    /// Type determines the format of the Subject Alternative Name. Always required.
    ///
    /// Support: Core
    pub r#type: SubjectAltNameType,
    /// Hostname contains Subject Alternative Name specified in DNS name format.
    /// Required when Type is set to Hostname, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,
    /// URI contains Subject Alternative Name specified in a full URI format.
    /// It MUST include both a scheme (e.g., "http" or "ftp") and a scheme-specific-part.
    /// Common values include SPIFFE IDs like "spiffe://mycluster.example.com/ns/myns/sa/svc1sa".
    /// Required when Type is set to URI, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<AbsoluteUri>,
}

/// WellKnownCACertificatesType is the type of CA certificate that will be used
/// when the caCertificateRefs field is unspecified.
pub type WellKnownCaCertificatesType = String;

/// SubjectAltNameType is the type of the Subject Alternative Name.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum SubjectAltNameType {
    /// HostnameSubjectAltNameType specifies hostname-based SAN.
    ///
    /// Support: Core
    Hostname,
    /// URISubjectAltNameType specifies URI-based SAN, e.g. SPIFFE id.
    ///
    /// Support: Core
    #[serde(rename = "URI")]
    Uri,
}

impl Default for SubjectAltNameType {
    fn default() -> Self {
        Self::Hostname
    }
}

/// AllowedListeners defines which ListenerSets can be attached to this Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct AllowedListeners {
    /// Namespaces defines which namespaces ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<ListenerNamespaces>,
}

/// ListenerNamespaces indicate which namespaces ListenerSets should be selected from.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerNamespaces {
    /// From indicates where ListenerSets can attach to this Gateway. Possible
    /// values are:
    ///
    /// * Same: Only ListenerSets in the same namespace may be attached to this Gateway.
    /// * Selector: ListenerSets in namespaces selected by the selector may be attached to this Gateway.
    /// * All: ListenerSets in all namespaces may be attached to this Gateway.
    /// * None: Only listeners defined in the Gateway's spec are allowed
    ///
    /// The default value None
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FromNamespaces>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only ListenerSets in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

/// Listener embodies the concept of a logical endpoint where a Gateway accepts
/// network connections.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct Listener {
    /// Name is the name of the Listener. This name MUST be unique within a
    /// Gateway.
    ///
    /// Support: Core
    pub name: SectionName,
    /// Hostname specifies the virtual hostname to match for protocol types that
    /// define this concept. When unspecified, all hostnames are matched. This
    /// field is ignored for protocols that don't require hostname based
    /// matching.
    ///
    /// Implementations MUST apply Hostname matching appropriately for each of
    /// the following protocols:
    ///
    /// * TLS: The Listener Hostname MUST match the SNI.
    /// * HTTP: The Listener Hostname MUST match the Host header of the request.
    /// * HTTPS: The Listener Hostname SHOULD match both the SNI and Host header.
    ///   Note that this does not require the SNI and Host header to be the same.
    ///   The semantics of this are described in more detail below.
    ///
    /// To ensure security, Section 11.1 of RFC-6066 emphasizes that server
    /// implementations that rely on SNI hostname matching MUST also verify
    /// hostnames within the application protocol.
    ///
    /// Section 9.1.2 of RFC-7540 provides a mechanism for servers to reject the
    /// reuse of a connection by responding with the HTTP 421 Misdirected Request
    /// status code. This indicates that the origin server has rejected the
    /// request because it appears to have been misdirected.
    ///
    /// To detect misdirected requests, Gateways SHOULD match the authority of
    /// the requests with all the SNI hostname(s) configured across all the
    /// Gateway Listeners on the same port and protocol:
    ///
    /// * If another Listener has an exact match or more specific wildcard entry,
    ///   the Gateway SHOULD return a 421.
    /// * If the current Listener (selected by SNI matching during ClientHello)
    ///   does not match the Host:
    ///     * If another Listener does match the Host, the Gateway SHOULD return a
    ///       421.
    ///     * If no other Listener matches the Host, the Gateway MUST return a
    ///       404.
    ///
    /// For HTTPRoute and TLSRoute resources, there is an interaction with the
    /// `spec.hostnames` array. When both listener and route specify hostnames,
    /// there MUST be an intersection between the values for a Route to be
    /// accepted. For more information, refer to the Route specific Hostnames
    /// documentation.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,
    /// Port is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    ///
    /// Support: Core
    pub port: PortNumber,
    /// Protocol specifies the network protocol this listener expects to receive.
    ///
    /// Support: Core
    pub protocol: ProtocolType,
    /// TLS is the TLS configuration for the Listener. This field is required if
    /// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
    /// if the Protocol field is "HTTP", "TCP", or "UDP".
    ///
    /// The association of SNIs to Certificate defined in ListenerTLSConfig is
    /// defined based on the Hostname field for this listener.
    ///
    /// The GatewayClass MUST use the longest matching SNI out of all
    /// available certificates for any TLS handshake.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ListenerTlsConfig>,
    /// AllowedRoutes defines the types of routes that MAY be attached to a
    /// Listener and the trusted namespaces where those Route resources MAY be
    /// present.
    ///
    /// Although a client request may match multiple route rules, only one rule
    /// may ultimately receive the request. Matching precedence MUST be
    /// determined in order of the following criteria:
    ///
    /// * The most specific match as defined by the Route type.
    /// * The oldest Route based on creation timestamp. For example, a Route with
    ///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
    ///   a Route with a creation timestamp of "2020-09-08 01:02:04".
    /// * If everything else is equivalent, the Route appearing first in
    ///   alphabetical order (namespace/name) should be given precedence. For
    ///   example, foo/bar is given precedence over foo/baz.
    ///
    /// All valid rules within a Route attached to this Listener should be
    /// implemented. Invalid Route rules can be ignored (sometimes that will mean
    /// the full Route). If a Route rule transitions from valid to invalid,
    /// support for that Route rule should be dropped to ensure consistency. For
    /// example, even if a filter specified by a Route rule is invalid, the rest
    /// of the rules within that Route should still be supported.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<AllowedRoutes>,
}

/// ProtocolType defines the application protocol accepted by a Listener.
/// Implementations are not required to accept all the defined protocols. If an
/// implementation does not support a specified protocol, it MUST set the
/// "Accepted" condition to False for the affected Listener with a reason of
/// "UnsupportedProtocol".
///
/// Core ProtocolType values are listed in the table below.
///
/// Implementations can define their own protocols if a core ProtocolType does not
/// exist. Such definitions must use prefixed name, such as
/// `mycompany.com/my-custom-protocol`. Un-prefixed names are reserved for core
/// protocols. Any protocol defined by implementations will fall under
/// Implementation-specific conformance.
///
/// Valid values include:
///
/// * "HTTP" - Core support
/// * "example.com/bar" - Implementation-specific support
///
/// Invalid values include:
///
/// * "example.com" - must include path if domain is used
/// * "foo.example.com" - must include path if domain is used
pub type ProtocolType = String;

/// GatewayBackendTLS describes backend TLS configuration for gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayBackendTls {
    /// ClientCertificateRef references an object that contains a client certificate
    /// and its associated private key. It can reference standard Kubernetes resources,
    /// i.e., Secret, or implementation-specific custom resources.
    ///
    /// A ClientCertificateRef is considered invalid if:
    ///
    /// * It refers to a resource that cannot be resolved (e.g., the referenced resource
    ///   does not exist) or is misconfigured (e.g., a Secret does not contain the keys
    ///   named `tls.crt` and `tls.key`). In this case, the `ResolvedRefs` condition
    ///   on the Gateway MUST be set to False with the Reason `InvalidClientCertificateRef`
    ///   and the Message of the Condition MUST indicate why the reference is invalid.
    ///
    /// * It refers to a resource in another namespace UNLESS there is a ReferenceGrant
    ///   in the target namespace that allows the certificate to be attached.
    ///   If a ReferenceGrant does not allow this reference, the `ResolvedRefs` condition
    ///   on the Gateway MUST be set to False with the Reason `RefNotPermitted`.
    ///
    /// Implementations MAY choose to perform further validation of the certificate
    /// content (e.g., checking expiry or enforcing specific formats). In such cases,
    /// an implementation-specific Reason and Message MUST be set.
    ///
    /// Support: Core - Reference to a Kubernetes TLS Secret (with the type `kubernetes.io/tls`).
    /// Support: Implementation-specific - Other resource kinds or Secrets with a
    /// different type (e.g., `Opaque`).
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "clientCertificateRef")]
    pub client_certificate_ref: Option<SecretObjectReference>,
}

/// ListenerTLSConfig describes a TLS configuration for a listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerTlsConfig {
    /// Mode defines the TLS behavior for the TLS session initiated by the client.
    /// There are two possible modes:
    ///
    /// - Terminate: The TLS session between the downstream client and the
    ///   Gateway is terminated at the Gateway. This mode requires certificates
    ///   to be specified in some way, such as populating the certificateRefs
    ///   field.
    /// - Passthrough: The TLS session is NOT terminated by the Gateway. This
    ///   implies that the Gateway can't decipher the TLS stream except for
    ///   the ClientHello message of the TLS protocol. The certificateRefs field
    ///   is ignored in this mode.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TlsModeType>,
    /// CertificateRefs contains a series of references to Kubernetes objects that
    /// contains TLS certificates and private keys. These certificates are used to
    /// establish a TLS handshake for requests that match the hostname of the
    /// associated listener.
    ///
    /// A single CertificateRef to a Kubernetes Secret has "Core" support.
    /// Implementations MAY choose to support attaching multiple certificates to
    /// a Listener, but this behavior is implementation-specific.
    ///
    /// References to a resource in different namespace are invalid UNLESS there
    /// is a ReferenceGrant in the target namespace that allows the certificate
    /// to be attached. If a ReferenceGrant does not allow this reference, the
    /// "ResolvedRefs" condition MUST be set to False for this listener with the
    /// "RefNotPermitted" reason.
    ///
    /// This field is required to have at least one element when the mode is set
    /// to "Terminate" (default) and is optional otherwise.
    ///
    /// CertificateRefs can reference to standard Kubernetes resources, i.e.
    /// Secret, or implementation-specific custom resources.
    ///
    /// Support: Core - A single reference to a Kubernetes Secret of type kubernetes.io/tls
    ///
    /// Support: Implementation-specific (More than one reference or other resource types)
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "certificateRefs")]
    pub certificate_refs: Option<Vec<SecretObjectReference>>,
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

/// GatewayTLSConfig specifies frontend and backend tls configuration for gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayTlsConfig {
    /// Backend describes TLS configuration for gateway when connecting
    /// to backends.
    ///
    /// Note that this contains only details for the Gateway as a TLS client,
    /// and does _not_ imply behavior about how to choose which backend should
    /// get a TLS connection. That is determined by the presence of a BackendTLSPolicy.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<GatewayBackendTls>,
    /// Frontend describes TLS config when client connects to Gateway.
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<FrontendTlsConfig>,
}

/// FrontendTLSConfig specifies frontend tls configuration for gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct FrontendTlsConfig {
    /// Default specifies the default client certificate validation configuration
    /// for all Listeners handling HTTPS traffic, unless a per-port configuration
    /// is defined.
    ///
    /// support: Core
    pub default: TlsConfig,
    /// PerPort specifies tls configuration assigned per port.
    /// Per port configuration is optional. Once set this configuration overrides
    /// the default configuration for all Listeners handling HTTPS traffic
    /// that match this port.
    /// Each override port requires a unique TLS configuration.
    ///
    /// support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perPort")]
    pub per_port: Option<Vec<TlsPortConfig>>,
}

/// TLSModeType type defines how a Gateway handles TLS sessions.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum TlsModeType {
    /// In this mode, TLS session between the downstream client
    /// and the Gateway is terminated at the Gateway.
    Terminate,
    /// In this mode, the TLS session is NOT terminated by the Gateway. This
    /// implies that the Gateway can't decipher the TLS stream except for
    /// the ClientHello message of the TLS protocol.
    ///
    /// Note that SSL passthrough is only supported by TLSRoute.
    Passthrough,
}

impl Default for TlsModeType {
    fn default() -> Self {
        Self::Terminate
    }
}

/// TLSConfig describes TLS configuration that can apply to multiple Listeners
/// within this Gateway. Currently, it stores only the client certificate validation
/// configuration, but this may be extended in the future.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TlsConfig {
    /// Validation holds configuration information for validating the frontend (client).
    /// Setting this field will result in mutual authentication when connecting to the gateway.
    /// In browsers this may result in a dialog appearing
    /// that requests a user to specify the client certificate.
    /// The maximum depth of a certificate chain accepted in verification is Implementation specific.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<FrontendTlsValidation>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct TlsPortConfig {
    /// The Port indicates the Port Number to which the TLS configuration will be
    /// applied. This configuration will be applied to all Listeners handling HTTPS
    /// traffic that match this port.
    ///
    /// Support: Core
    pub port: PortNumber,
    /// TLS store the configuration that will be applied to all Listeners handling
    /// HTTPS traffic and matching given port.
    ///
    /// Support: Core
    pub tls: TlsConfig,
}

/// FrontendTLSValidation holds configuration information that can be used to validate
/// the frontend initiating the TLS connection
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct FrontendTlsValidation {
    /// CACertificateRefs contains one or more references to Kubernetes
    /// objects that contain a PEM-encoded TLS CA certificate bundle, which
    /// is used as a trust anchor to validate the certificates presented by
    /// the client.
    ///
    /// A CACertificateRef is invalid if:
    ///
    /// * It refers to a resource that cannot be resolved (e.g., the
    ///   referenced resource does not exist) or is misconfigured (e.g., a
    ///   ConfigMap does not contain a key named `ca.crt`). In this case, the
    ///   Reason on all matching HTTPS listeners must be set to `InvalidCACertificateRef`
    ///   and the Message of the Condition must indicate which reference is invalid and why.
    ///
    /// * It refers to an unknown or unsupported kind of resource. In this
    ///   case, the Reason on all matching HTTPS listeners must be set to
    ///   `InvalidCACertificateKind` and the Message of the Condition must explain
    ///   which kind of resource is unknown or unsupported.
    ///
    /// * It refers to a resource in another namespace UNLESS there is a
    ///   ReferenceGrant in the target namespace that allows the CA
    ///   certificate to be attached. If a ReferenceGrant does not allow this
    ///   reference, the `ResolvedRefs` on all matching HTTPS listeners condition
    ///   MUST be set with the Reason `RefNotPermitted`.
    ///
    /// Implementations MAY choose to perform further validation of the
    /// certificate content (e.g., checking expiry or enforcing specific formats).
    /// In such cases, an implementation-specific Reason and Message MUST be set.
    ///
    /// In all cases, the implementation MUST ensure that the `ResolvedRefs`
    /// condition is set to `status: False` on all targeted listeners (i.e.,
    /// listeners serving HTTPS on a matching port). The condition MUST
    /// include a Reason and Message that indicate the cause of the error. If
    /// ALL CACertificateRefs are invalid, the implementation MUST also ensure
    /// the `Accepted` condition on the listener is set to `status: False`, with
    /// the Reason `NoValidCACertificate`.
    /// Implementations MAY choose to support attaching multiple CA certificates
    /// to a listener, but this behavior is implementation-specific.
    ///
    /// Support: Core - A single reference to a Kubernetes ConfigMap, with the
    /// CA certificate in a key named `ca.crt`.
    ///
    /// Support: Implementation-specific - More than one reference, other kinds
    /// of resources, or a single reference that includes multiple certificates.
    #[serde(rename = "caCertificateRefs")]
    pub ca_certificate_refs: Vec<ObjectReference>,
    /// FrontendValidationMode defines the mode for validating the client certificate.
    /// There are two possible modes:
    ///
    /// - AllowValidOnly: In this mode, the gateway will accept connections only if
    ///   the client presents a valid certificate. This certificate must successfully
    ///   pass validation against the CA certificates specified in `CACertificateRefs`.
    /// - AllowInsecureFallback: In this mode, the gateway will accept connections
    ///   even if the client certificate is not presented or fails verification.
    ///
    ///   This approach delegates client authorization to the backend and introduce
    ///   a significant security risk. It should be used in testing environments or
    ///   on a temporary basis in non-testing environments.
    ///
    /// Defaults to AllowValidOnly.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FrontendValidationModeType>,
}

/// FrontendValidationModeType type defines how a Gateway validates client certificates.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum FrontendValidationModeType {
    /// AllowValidOnly indicates that a client certificate is required
    /// during the TLS handshake and MUST pass validation.
    ///
    /// Support: Core
    AllowValidOnly,
    /// AllowInsecureFallback indicates that a client certificate may not be
    /// presented during the handshake or the validation against CA certificates may fail.
    ///
    /// Support: Extended
    AllowInsecureFallback,
}

impl Default for FrontendValidationModeType {
    fn default() -> Self {
        Self::AllowValidOnly
    }
}

/// AllowedRoutes defines which Routes may be attached to this Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct AllowedRoutes {
    /// Namespaces indicates namespaces from which Routes may be attached to this
    /// Listener. This is restricted to the namespace of this Gateway by default.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<RouteNamespaces>,
    /// Kinds specifies the groups and kinds of Routes that are allowed to bind
    /// to this Gateway Listener. When unspecified or empty, the kinds of Routes
    /// selected are determined using the Listener protocol.
    ///
    /// A RouteGroupKind MUST correspond to kinds of Routes that are compatible
    /// with the application protocol specified in the Listener's Protocol field.
    /// If an implementation does not support or recognize this resource type, it
    /// MUST set the "ResolvedRefs" condition to False for this Listener with the
    /// "InvalidRouteKinds" reason.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kinds: Option<Vec<RouteGroupKind>>,
}

/// FromNamespaces specifies namespace from which Routes/ListenerSets may be attached to a
/// Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum FromNamespaces {
    /// Routes/ListenerSets in all namespaces may be attached to this Gateway.
    All,
    /// Only Routes/ListenerSets in namespaces selected by the selector may be attached to
    /// this Gateway.
    Selector,
    /// Only Routes/ListenerSets in the same namespace as the Gateway may be attached to this
    /// Gateway.
    Same,
    /// No Routes/ListenerSets may be attached to this Gateway.
    None,
}

impl Default for FromNamespaces {
    fn default() -> Self {
        Self::All
    }
}

/// RouteNamespaces indicate which namespaces Routes should be selected from.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteNamespaces {
    /// From indicates where Routes will be selected for this Gateway. Possible
    /// values are:
    ///
    /// * All: Routes in all namespaces may be used by this Gateway.
    /// * Selector: Routes in namespaces selected by the selector may be used by
    ///   this Gateway.
    /// * Same: Only Routes in the same namespace may be used by this Gateway.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FromNamespaces>,
    /// Selector must be specified when From is set to "Selector". In that case,
    /// only Routes in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector>,
}

/// RouteGroupKind indicates the group and kind of a Route resource.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteGroupKind {
    /// Group is the group of the Route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is the kind of the Route.
    pub kind: Kind,
}

/// GatewaySpecAddress describes an address that can be bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewaySpecAddress {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AddressType>,
    /// When a value is unspecified, an implementation SHOULD automatically
    /// assign an address matching the requested type if possible.
    ///
    /// If an implementation does not support an empty value, they MUST set the
    /// "Programmed" condition in status to False with a reason of "AddressNotAssigned".
    ///
    /// Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// GatewayStatusAddress describes a network address that is bound to a Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatusAddress {
    /// Type of the address.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AddressType>,
    /// Value of the address. The validity of the values will depend
    /// on the type and support by the controller.
    ///
    /// Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    pub value: String,
}

/// GatewayStatus defines the observed state of Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayStatus {
    /// Addresses lists the network addresses that have been bound to the
    /// Gateway.
    ///
    /// This list may differ from the addresses provided in the spec under some
    /// conditions:
    ///
    ///   * no addresses are specified, all addresses are dynamically assigned
    ///   * a combination of specified and dynamic addresses are assigned
    ///   * a specified address was unusable (e.g. already in use)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<GatewayStatusAddress>>,
    /// Conditions describe the current conditions of the Gateway.
    ///
    /// Implementations should prefer to express Gateway conditions
    /// using the `GatewayConditionType` and `GatewayConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe Gateway state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    /// * "Ready"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerStatus>>,
    /// AttachedListenerSets represents the total number of ListenerSets that have been
    /// successfully attached to this Gateway.
    ///
    /// A ListenerSet is successfully attached to a Gateway when all the following conditions are met:
    /// - The ListenerSet is selected by the Gateway's AllowedListeners field
    /// - The ListenerSet has a valid ParentRef selecting the Gateway
    /// - The ListenerSet's status has the condition "Accepted: true"
    ///
    /// Uses for this field include troubleshooting AttachedListenerSets attachment and
    /// measuring blast radius/impact of changes to a Gateway.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "attachedListenerSets")]
    pub attached_listener_sets: Option<i32>,
}

/// GatewayInfrastructure defines infrastructure level attributes about a Gateway instance.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayInfrastructure {
    /// Labels that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.labels` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "labels" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific labels as they see fit.
    ///
    /// If an implementation maps these labels to Pods, or any other resource that would need to be recreated when labels
    /// change, it SHOULD clearly warn about this behavior in documentation.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub labels: Option<BTreeMap<String, LabelValue>>,
    /// Annotations that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.annotations` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "annotations" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific annotations as they see fit.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<BTreeMap<String, AnnotationValue>>,
    /// ParametersRef is a reference to a resource that contains the configuration
    /// parameters corresponding to the Gateway. This is optional if the
    /// controller does not require any additional configuration.
    ///
    /// This follows the same semantics as GatewayClass's `parametersRef`, but on a per-Gateway basis
    ///
    /// The Gateway's GatewayClass may provide its own `parametersRef`. When both are specified,
    /// the merging behavior is implementation specific.
    /// It is generally recommended that GatewayClass provides defaults that can be overridden by a Gateway.
    ///
    /// If the referent cannot be found, refers to an unsupported kind, or when
    /// the data within that resource is malformed, the Gateway SHOULD be
    /// rejected with the "Accepted" status condition set to "False" and an
    /// "InvalidParameters" reason.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parametersRef")]
    pub parameters_ref: Option<LocalParametersReference>,
}

/// LocalParametersReference identifies an API object containing controller-specific
/// configuration resource within the namespace.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct LocalParametersReference {
    /// Group is the group of the referent.
    pub group: Group,
    /// Kind is kind of the referent.
    pub kind: Kind,
    /// Name is the name of the referent.
    pub name: String,
}

/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerStatus {
    /// Name is the name of the Listener that this status corresponds to.
    pub name: SectionName,
    /// SupportedKinds is the list indicating the Kinds supported by this
    /// listener. This MUST represent the kinds supported by an implementation for
    /// that Listener configuration.
    ///
    /// If kinds are specified in Spec that are not supported, they MUST NOT
    /// appear in this list and an implementation MUST set the "ResolvedRefs"
    /// condition to "False" with the "InvalidRouteKinds" reason. If both valid
    /// and invalid Route kinds are specified, the implementation MUST
    /// reference the valid Route kinds that have been specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedKinds")]
    pub supported_kinds: Option<Vec<RouteGroupKind>>,
    /// AttachedRoutes represents the total number of Routes that have been
    /// successfully attached to this Listener.
    ///
    /// Successful attachment of a Route to a Listener is based solely on the
    /// combination of the AllowedRoutes field on the corresponding Listener
    /// and the Route's ParentRefs field. A Route is successfully attached to
    /// a Listener when it is selected by the Listener's AllowedRoutes field
    /// AND the Route has a valid ParentRef selecting the whole Gateway
    /// resource or a specific Listener as a parent resource (more detail on
    /// attachment semantics can be found in the documentation on the various
    /// Route kinds ParentRefs fields). Listener or Route status does not impact
    /// successful attachment, i.e. the AttachedRoutes field count MUST be set
    /// for Listeners, even if the Accepted condition of an individual Listener is set
    /// to "False". The AttachedRoutes number represents the number of Routes with
    /// the Accepted condition set to "True" that have been attached to this Listener.
    /// Routes with any other value for the Accepted condition MUST NOT be included
    /// in this count.
    ///
    /// Uses for this field include troubleshooting Route attachment and
    /// measuring blast radius/impact of changes to a Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}

/// ParametersReference identifies an API object containing controller-specific
/// configuration resource within the cluster.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParametersReference {
    /// Group is the group of the referent.
    pub group: Group,
    /// Kind is kind of the referent.
    pub kind: Kind,
    /// Name is the name of the referent.
    pub name: String,
    /// Namespace is the namespace of the referent.
    /// This field is required when referring to a Namespace-scoped resource and
    /// MUST be unset when referring to a Cluster-scoped resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

/// GatewayClassStatus is the current status for the GatewayClass.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GatewayClassStatus {
    /// Conditions is the current status from the controller for
    /// this GatewayClass.
    ///
    /// Controllers should prefer to publish conditions using values
    /// of GatewayClassConditionType for the type of each Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    /// SupportedFeatures is the set of features the GatewayClass support.
    /// It MUST be sorted in ascending alphabetical order by the Name key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedFeatures")]
    pub supported_features: Option<Vec<SupportedFeature>>,
}

/// FeatureName is used to describe distinct features that are covered by
/// conformance tests.
pub type FeatureName = String;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct SupportedFeature {
    pub name: FeatureName,
}

/// GRPCRouteStatus defines the observed state of GRPCRoute.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteStatus {
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

/// GRPCRouteRule defines the semantics for matching a gRPC request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteRule {
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// Matches define conditions used for matching the rule against incoming
    /// gRPC requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ```text
    /// matches:
    /// - method:
    ///     service: foo.bar
    ///   headers:
    ///     values:
    ///       version: 2
    /// - method:
    ///     service: foo.bar.v2
    /// ```
    ///
    /// For a request to match against this rule, it MUST satisfy
    /// EITHER of the two conditions:
    ///
    /// - service of foo.bar AND contains the header `version: 2`
    /// - service of foo.bar.v2
    ///
    /// See the documentation for GRPCRouteMatch on how to specify multiple
    /// match conditions to be ANDed together.
    ///
    /// If no matches are specified, the implementation MUST match every gRPC request.
    ///
    /// Proxy or Load Balancer routing configuration generated from GRPCRoutes
    /// MUST prioritize rules based on the following criteria, continuing on
    /// ties. Merging MUST not be done between GRPCRoutes and HTTPRoutes.
    /// Precedence MUST be given to the rule with the largest number of:
    ///
    /// * Characters in a matching non-wildcard hostname.
    /// * Characters in a matching hostname.
    /// * Characters in a matching service.
    /// * Characters in a matching method.
    /// * Header matches.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within the Route that has been given precedence,
    /// matching precedence MUST be granted to the first matching rule meeting
    /// the above criteria.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<GrpcRouteMatch>>,
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// The effects of ordering of multiple behaviors are currently unspecified.
    /// This can change in the future based on feedback during the alpha stage.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations that support
    ///   GRPCRoute.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// If an implementation cannot support a combination of filters, it must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteFilter>>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive an `UNAVAILABLE` status.
    ///
    /// See the GRPCBackendRef definition for the rules about what makes a single
    /// GRPCBackendRef invalid.
    ///
    /// When a GRPCBackendRef is invalid, `UNAVAILABLE` statuses MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive an `UNAVAILABLE` status.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic MUST receive an `UNAVAILABLE` status.
    /// Implementations may choose how that 50 percent is determined.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<GrpcBackendRef>>,
}

/// GRPCRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a gRPC request only if its service
/// is `foo` AND it contains the `version: v1` header:
///
/// ```text
/// matches:
///   - method:
///     type: Exact
///     service: "foo"
///   - headers:
///     name: "version"
///     value "v1"
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteMatch {
    /// Method specifies a gRPC request service/method matcher. If this field is
    /// not specified, all services and methods will match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<GrpcMethodMatch>,
    /// Headers specifies gRPC request header matchers. Multiple match values are
    /// ANDed together, meaning, a request MUST match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<GrpcHeaderMatch>>,
}

/// GRPCMethodMatch describes how to select a gRPC route by matching the gRPC
/// request service and/or method.
///
/// At least one of Service and Method MUST be a non-empty string.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcMethodMatch {
    /// Type specifies how to match against the service and/or method.
    /// Support: Core (Exact with service and method specified)
    ///
    /// Support: Implementation-specific (Exact with method specified but no service specified)
    ///
    /// Support: Implementation-specific (RegularExpression)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GrpcMethodMatchType>,
    /// Value of the service to match against. If left empty or omitted, will
    /// match any service.
    ///
    /// At least one of Service and Method MUST be a non-empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// Value of the method to match against. If left empty or omitted, will
    /// match all services.
    ///
    /// At least one of Service and Method MUST be a non-empty string.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

/// MethodMatchType specifies the semantics of how gRPC methods and services are compared.
/// Valid MethodMatchType values, along with their conformance levels, are:
///
/// * "Exact" - Core
/// * "RegularExpression" - Implementation Specific
///
/// Exact methods MUST be syntactically valid:
///
/// - Must not contain `/` character
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GrpcMethodMatchType {
    /// Matches the method or service exactly and with case sensitivity.
    Exact,
    /// Matches if the method or service matches the given regular expression with
    /// case sensitivity.
    ///
    /// Since `"RegularExpression"` has implementation-specific conformance,
    /// implementations can support POSIX, PCRE, RE2 or any other regular expression
    /// dialect.
    /// Please read the implementation's documentation to determine the supported
    /// dialect.
    RegularExpression,
}

impl Default for GrpcMethodMatchType {
    fn default() -> Self {
        Self::Exact
    }
}

/// GRPCHeaderMatch describes how to select a gRPC route by matching gRPC request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcHeaderMatch {
    /// Type specifies how to match against the value of the header.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<GrpcHeaderMatchType>,
    /// Name is the name of the gRPC Header to be matched.
    ///
    /// If multiple entries specify equivalent header names, only the first
    /// entry with an equivalent name MUST be considered for a match. Subsequent
    /// entries with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    pub name: GrpcHeaderName,
    /// Value is the value of the gRPC Header to be matched.
    pub value: String,
}

/// GRPCHeaderMatchType specifies the semantics of how GRPC header values should
/// be compared. Valid GRPCHeaderMatchType values, along with their conformance
/// levels, are:
///
/// * "Exact" - Core
/// * "RegularExpression" - Implementation Specific
///
/// Note that new values may be added to this enum in future releases of the API,
/// implementations MUST ensure that unknown values will not cause a crash.
///
/// Unknown values here MUST result in the implementation setting the Accepted
/// Condition for the Route to `status: False`, with a Reason of
/// `UnsupportedValue`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GrpcHeaderMatchType {
    Exact,
    RegularExpression,
}

impl Default for GrpcHeaderMatchType {
    fn default() -> Self {
        Self::Exact
    }
}

pub type GrpcHeaderName = String;

/// GRPCRouteFilterType identifies a type of GRPCRoute filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum GrpcRouteFilterType {
    /// GRPCRouteFilterRequestHeaderModifier can be used to add or remove a gRPC
    /// header from a gRPC request before it is sent to the upstream target.
    ///
    /// Support in GRPCRouteRule: Core
    ///
    /// Support in GRPCBackendRef: Extended
    RequestHeaderModifier,
    /// GRPCRouteFilterRequestHeaderModifier can be used to add or remove a gRPC
    /// header from a gRPC response before it is sent to the client.
    ///
    /// Support in GRPCRouteRule: Core
    ///
    /// Support in GRPCBackendRef: Extended
    ResponseHeaderModifier,
    /// GRPCRouteFilterRequestMirror can be used to mirror gRPC requests to a
    /// different backend. The responses from this backend MUST be ignored by
    /// the Gateway.
    ///
    /// Support in GRPCRouteRule: Extended
    ///
    /// Support in GRPCBackendRef: Extended
    RequestMirror,
    /// GRPCRouteFilterExtensionRef should be used for configuring custom
    /// gRPC filters.
    ///
    /// Support in GRPCRouteRule: Implementation-specific
    ///
    /// Support in GRPCBackendRef: Implementation-specific
    ExtensionRef,
}

impl Default for GrpcRouteFilterType {
    fn default() -> Self {
        Self::RequestHeaderModifier
    }
}

/// GRPCRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcRouteFilter {
    /// Type identifies the type of filter to apply. As with other API fields,
    /// types are classified into three conformance levels:
    ///
    /// - Core: Filter types and their corresponding configuration defined by
    ///   "Support: Core" in this package, e.g. "RequestHeaderModifier". All
    ///   implementations supporting GRPCRoute MUST support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` MUST be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    pub r#type: GrpcRouteFilterType,
    /// RequestHeaderModifier defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaderModifier")]
    pub request_header_modifier: Option<HttpHeaderFilter>,
    /// ResponseHeaderModifier defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderModifier")]
    pub response_header_modifier: Option<HttpHeaderFilter>,
    /// RequestMirror defines a schema for a filter that mirrors requests.
    /// Requests are sent to the specified destination, but responses from
    /// that destination are ignored.
    ///
    /// This filter can be used multiple times within the same rule. Note that
    /// not all implementations will be able to support mirroring to multiple
    /// backends.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<HttpRequestMirrorFilter>,
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// Support: Implementation-specific
    ///
    /// This filter can be used multiple times within the same rule.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionRef")]
    pub extension_ref: Option<LocalObjectReference>,
}

/// GRPCBackendRef defines how a GRPCRoute forwards a gRPC request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct GrpcBackendRef {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is the Kubernetes resource kind of the referent. For example
    /// "Service".
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations SHOULD NOT
    /// support ExternalName Services.
    ///
    /// Support: Core (Services with a type other than ExternalName)
    ///
    /// Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Filters defined at this level MUST be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in GRPCRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteFilter>>,
}

/// HTTPRouteRule defines semantics for matching an HTTP request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteRule {
    /// Name is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// Matches define conditions used for matching the rule against incoming
    /// HTTP requests. Each match is independent, i.e. this rule will be matched
    /// if **any** one of the matches is satisfied.
    ///
    /// For example, take the following matches configuration:
    ///
    /// ```text
    /// matches:
    /// - path:
    ///     value: "/foo"
    ///   headers:
    ///   - name: "version"
    ///     value: "v2"
    /// - path:
    ///     value: "/v2/foo"
    /// ```
    ///
    /// For a request to match against this rule, a request must satisfy
    /// EITHER of the two conditions:
    ///
    /// - path prefixed with `/foo` AND contains the header `version: v2`
    /// - path prefix of `/v2/foo`
    ///
    /// See the documentation for HTTPRouteMatch on how to specify multiple
    /// match conditions that should be ANDed together.
    ///
    /// If no matches are specified, the default is a prefix
    /// path match on "/", which has the effect of matching every
    /// HTTP request.
    ///
    /// Proxy or Load Balancer routing configuration generated from HTTPRoutes
    /// MUST prioritize matches based on the following criteria, continuing on
    /// ties. Across all rules specified on applicable Routes, precedence must be
    /// given to the match having:
    ///
    /// * "Exact" path match.
    /// * "Prefix" path match with largest number of characters.
    /// * Method match.
    /// * Largest number of header matches.
    /// * Largest number of query param matches.
    ///
    /// Note: The precedence of RegularExpression path matches are implementation-specific.
    ///
    /// If ties still exist across multiple Routes, matching precedence MUST be
    /// determined in order of the following criteria, continuing on ties:
    ///
    /// * The oldest Route based on creation timestamp.
    /// * The Route appearing first in alphabetical order by
    ///   "{namespace}/{name}".
    ///
    /// If ties still exist within an HTTPRoute, matching precedence MUST be granted
    /// to the FIRST matching rule (in list order) with a match meeting the above
    /// criteria.
    ///
    /// When no rules matching a request have been successfully attached to the
    /// parent a request is coming from, a HTTP 404 status code MUST be returned.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<HttpRouteMatch>>,
    /// Filters define the filters that are applied to requests that match
    /// this rule.
    ///
    /// Wherever possible, implementations SHOULD implement filters in the order
    /// they are specified.
    ///
    /// Implementations MAY choose to implement this ordering strictly, rejecting
    /// any combination or order of filters that cannot be supported. If implementations
    /// choose a strict interpretation of filter ordering, they MUST clearly document
    /// that behavior.
    ///
    /// To reject an invalid combination or order of filters, implementations SHOULD
    /// consider the Route Rules with this configuration invalid. If all Route Rules
    /// in a Route are invalid, the entire Route would be considered invalid. If only
    /// a portion of Route Rules are invalid, implementations MUST set the
    /// "PartiallyInvalid" condition for the Route.
    ///
    /// Conformance-levels at this level are defined based on the type of filter:
    ///
    /// - ALL core filters MUST be supported by all implementations.
    /// - Implementers are encouraged to support extended filters.
    /// - Implementation-specific custom filters have no API guarantees across
    ///   implementations.
    ///
    /// Specifying the same filter multiple times is not supported unless explicitly
    /// indicated in the filter.
    ///
    /// All filters are expected to be compatible with each other except for the
    /// URLRewrite and RequestRedirect filters, which may not be combined. If an
    /// implementation cannot support other combinations of filters, they must clearly
    /// document that limitation. In cases where incompatible or unsupported
    /// filters are specified and cause the `Accepted` condition to be set to status
    /// `False`, implementations may use the `IncompatibleFilters` reason to specify
    /// this configuration error.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HttpRouteFilter>>,
    /// BackendRefs defines the backend(s) where matching requests should be
    /// sent.
    ///
    /// Failure behavior here depends on how many BackendRefs are specified and
    /// how many are invalid.
    ///
    /// If *all* entries in BackendRefs are invalid, and there are also no filters
    /// specified in this route rule, *all* traffic which matches this rule MUST
    /// receive a 500 status code.
    ///
    /// See the HTTPBackendRef definition for the rules about what makes a single
    /// HTTPBackendRef invalid.
    ///
    /// When a HTTPBackendRef is invalid, 500 status codes MUST be returned for
    /// requests that would have otherwise been routed to an invalid backend. If
    /// multiple backends are specified, and some are invalid, the proportion of
    /// requests that would otherwise have been routed to an invalid backend
    /// MUST receive a 500 status code.
    ///
    /// For example, if two backends are specified with equal weights, and one is
    /// invalid, 50 percent of traffic must receive a 500. Implementations may
    /// choose how that 50 percent is determined.
    ///
    /// When a HTTPBackendRef refers to a Service that has no ready endpoints,
    /// implementations SHOULD return a 503 for requests to that backend instead.
    /// If an implementation chooses to do this, all of the above rules for 500 responses
    /// MUST also apply for responses that return a 503.
    ///
    /// Support: Core for Kubernetes Service
    ///
    /// Support: Extended for Kubernetes ServiceImport
    ///
    /// Support: Implementation-specific for any other resource
    ///
    /// Support for weight: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRefs")]
    pub backend_refs: Option<Vec<HttpBackendRef>>,
    /// Timeouts defines the timeouts that can be configured for an HTTP request.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<HttpRouteTimeouts>,
}

/// HTTPRouteTimeouts defines timeouts that can be configured for an HTTPRoute.
/// Timeout values are represented with Gateway API Duration formatting.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteTimeouts {
    /// Request specifies the maximum duration for a gateway to respond to an HTTP request.
    /// If the gateway has not been able to respond before this deadline is met, the gateway
    /// MUST return a timeout error.
    ///
    /// For example, setting the `rules.timeouts.request` field to the value `10s` in an
    /// `HTTPRoute` will cause a timeout if a client request is taking longer than 10 seconds
    /// to complete.
    ///
    /// Setting a timeout to the zero duration (e.g. "0s") SHOULD disable the timeout
    /// completely. Implementations that cannot completely disable the timeout MUST
    /// instead interpret the zero duration as the longest possible value to which
    /// the timeout can be set.
    ///
    /// This timeout is intended to cover as close to the whole request-response transaction
    /// as possible although an implementation MAY choose to start the timeout after the entire
    /// request stream has been received instead of immediately after the transaction is
    /// initiated by the client.
    ///
    /// The value of Request is a Gateway API Duration string as defined by GEP-2257. When this
    /// field is unspecified, request timeout behavior is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request: Option<Duration>,
    /// BackendRequest specifies a timeout for an individual request from the gateway
    /// to a backend. This covers the time from when the request first starts being
    /// sent from the gateway to when the full response has been received from the backend.
    ///
    /// Setting a timeout to the zero duration (e.g. "0s") SHOULD disable the timeout
    /// completely. Implementations that cannot completely disable the timeout MUST
    /// instead interpret the zero duration as the longest possible value to which
    /// the timeout can be set.
    ///
    /// An entire client HTTP transaction with a gateway, covered by the Request timeout,
    /// may result in more than one call from the gateway to the destination backend,
    /// for example, if automatic retries are supported.
    ///
    /// The value of BackendRequest must be a Gateway API Duration string as defined by
    /// GEP-2257.  When this field is unspecified, its behavior is implementation-specific;
    /// when specified, the value of BackendRequest must be no more than the value of the
    /// Request timeout (since the Request timeout encompasses the BackendRequest timeout).
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "backendRequest")]
    pub backend_request: Option<Duration>,
}

/// PathMatchType specifies the semantics of how HTTP paths should be compared.
/// Valid PathMatchType values, along with their support levels, are:
///
/// * "Exact" - Core
/// * "PathPrefix" - Core
/// * "RegularExpression" - Implementation Specific
///
/// PathPrefix and Exact paths must be syntactically valid:
///
/// - Must begin with the `/` character
/// - Must not contain consecutive `/` characters (e.g. `/foo///`, `//`).
///
/// Note that values may be added to this enum, implementations
/// must ensure that unknown values will not cause a crash.
///
/// Unknown values here must result in the implementation setting the
/// Accepted Condition for the Route to `status: False`, with a
/// Reason of `UnsupportedValue`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum PathMatchType {
    /// Matches the URL path exactly and with case sensitivity. This means that
    /// an exact path match on `/abc` will only match requests to `/abc`, NOT
    /// `/abc/`, `/Abc`, or `/abcd`.
    Exact,
    /// Matches based on a URL path prefix split by `/`. Matching is
    /// case-sensitive and done on a path element by element basis. A
    /// path element refers to the list of labels in the path split by
    /// the `/` separator. When specified, a trailing `/` is ignored.
    ///
    /// For example, the paths `/abc`, `/abc/`, and `/abc/def` would all match
    /// the prefix `/abc`, but the path `/abcd` would not.
    ///
    /// "PathPrefix" is semantically equivalent to the "Prefix" path type in the
    /// Kubernetes Ingress API.
    PathPrefix,
    /// Matches if the URL path matches the given regular expression with
    /// case sensitivity.
    ///
    /// Since `"RegularExpression"` has implementation-specific conformance,
    /// implementations can support POSIX, PCRE, RE2 or any other regular expression
    /// dialect.
    /// Please read the implementation's documentation to determine the supported
    /// dialect.
    RegularExpression,
}

impl Default for PathMatchType {
    fn default() -> Self {
        Self::Exact
    }
}

/// HTTPPathMatch describes how to select a HTTP route by matching the HTTP request path.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpPathMatch {
    /// Type specifies how to match against the path Value.
    ///
    /// Support: Core (Exact, PathPrefix)
    ///
    /// Support: Implementation-specific (RegularExpression)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PathMatchType>,
    /// Value of the HTTP path to match against.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// HeaderMatchType specifies the semantics of how HTTP header values should be
/// compared. Valid HeaderMatchType values, along with their conformance levels, are:
///
/// * "Exact" - Core
/// * "RegularExpression" - Implementation Specific
///
/// Note that values may be added to this enum, implementations
/// must ensure that unknown values will not cause a crash.
///
/// Unknown values here must result in the implementation setting the
/// Accepted Condition for the Route to `status: False`, with a
/// Reason of `UnsupportedValue`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HeaderMatchType {
    Exact,
    RegularExpression,
}

impl Default for HeaderMatchType {
    fn default() -> Self {
        Self::Exact
    }
}

/// HTTPHeaderName is the name of an HTTP header.
///
/// Valid values include:
///
/// * "Authorization"
/// * "Set-Cookie"
///
/// Invalid values include:
///
///   - ":method" - ":" is an invalid character. This means that HTTP/2 pseudo
///     headers are not currently supported by this type.
///   - "/invalid" - "/ " is an invalid character
pub type HttpHeaderName = String;

/// HTTPHeaderMatch describes how to select a HTTP route by matching HTTP request
/// headers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpHeaderMatch {
    /// Type specifies how to match against the value of the header.
    ///
    /// Support: Core (Exact)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Since RegularExpression HeaderMatchType has implementation-specific
    /// conformance, implementations can support POSIX, PCRE or any other dialects
    /// of regular expressions. Please read the implementation's documentation to
    /// determine the supported dialect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<HeaderMatchType>,
    /// Name is the name of the HTTP Header to be matched. Name matching MUST be
    /// case-insensitive. (See https://tools.ietf.org/html/rfc7230#section-3.2).
    ///
    /// If multiple entries specify equivalent header names, only the first
    /// entry with an equivalent name MUST be considered for a match. Subsequent
    /// entries with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    ///
    /// When a header is repeated in an HTTP request, it is
    /// implementation-specific behavior as to how this is represented.
    /// Generally, proxies should follow the guidance from the RFC:
    /// https://www.rfc-editor.org/rfc/rfc7230.html#section-3.2.2 regarding
    /// processing a repeated header, with special handling for "Set-Cookie".
    pub name: HttpHeaderName,
    /// Value is the value of HTTP Header to be matched.
    pub value: String,
}

/// QueryParamMatchType specifies the semantics of how HTTP query parameter
/// values should be compared. Valid QueryParamMatchType values, along with their
/// conformance levels, are:
///
/// * "Exact" - Core
/// * "RegularExpression" - Implementation Specific
///
/// Note that values may be added to this enum, implementations
/// must ensure that unknown values will not cause a crash.
///
/// Unknown values here must result in the implementation setting the
/// Accepted Condition for the Route to `status: False`, with a
/// Reason of `UnsupportedValue`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum QueryParamMatchType {
    Exact,
    RegularExpression,
}

impl Default for QueryParamMatchType {
    fn default() -> Self {
        Self::Exact
    }
}

/// HTTPQueryParamMatch describes how to select a HTTP route by matching HTTP
/// query parameters.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpQueryParamMatch {
    /// Type specifies how to match against the value of the query parameter.
    ///
    /// Support: Extended (Exact)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Since RegularExpression QueryParamMatchType has Implementation-specific
    /// conformance, implementations can support POSIX, PCRE or any other
    /// dialects of regular expressions. Please read the implementation's
    /// documentation to determine the supported dialect.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#type: Option<QueryParamMatchType>,
    /// Name is the name of the HTTP query param to be matched. This must be an
    /// exact string match. (See
    /// https://tools.ietf.org/html/rfc7230#section-2.7.3).
    ///
    /// If multiple entries specify equivalent query param names, only the first
    /// entry with an equivalent name MUST be considered for a match. Subsequent
    /// entries with an equivalent query param name MUST be ignored.
    ///
    /// If a query param is repeated in an HTTP request, the behavior is
    /// purposely left undefined, since different data planes have different
    /// capabilities. However, it is *recommended* that implementations should
    /// match against the first value of the param if the data plane supports it,
    /// as this behavior is expected in other load balancing contexts outside of
    /// the Gateway API.
    ///
    /// Users SHOULD NOT route traffic based on repeated query params to guard
    /// themselves against potential differences in the implementations.
    pub name: HttpHeaderName,
    /// Value is the value of HTTP query param to be matched.
    pub value: String,
}

/// HTTPMethod describes how to select a HTTP route by matching the HTTP
/// method as defined by
/// [RFC 7231](https://datatracker.ietf.org/doc/html/rfc7231#section-4) and
/// [RFC 5789](https://datatracker.ietf.org/doc/html/rfc5789#section-2).
/// The value is expected in upper case.
///
/// Note that values may be added to this enum, implementations
/// must ensure that unknown values will not cause a crash.
///
/// Unknown values here must result in the implementation setting the
/// Accepted Condition for the Route to `status: False`, with a
/// Reason of `UnsupportedValue`.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpMethod {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "PATCH")]
    Patch,
}

impl Default for HttpMethod {
    fn default() -> Self {
        Self::Get
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpMethodWithWildcard {
    #[serde(rename = "GET")]
    Get,
    #[serde(rename = "HEAD")]
    Head,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "PUT")]
    Put,
    #[serde(rename = "DELETE")]
    Delete,
    #[serde(rename = "CONNECT")]
    Connect,
    #[serde(rename = "OPTIONS")]
    Options,
    #[serde(rename = "TRACE")]
    Trace,
    #[serde(rename = "PATCH")]
    Patch,
    #[serde(rename = "*")]
    Wildcard,
}

impl Default for HttpMethodWithWildcard {
    fn default() -> Self {
        Self::Get
    }
}

/// HTTPRouteMatch defines the predicate used to match requests to a given
/// action. Multiple match types are ANDed together, i.e. the match will
/// evaluate to true only if all conditions are satisfied.
///
/// For example, the match below will match a HTTP request only if its path
/// starts with `/foo` AND it contains the `version: v1` header:
///
/// ```text
/// match:
///
/// 	path:
/// 	  value: "/foo"
/// 	headers:
/// 	- name: "version"
/// 	  value "v1"
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteMatch {
    /// Path specifies a HTTP request path matcher. If this field is not
    /// specified, a default prefix match on the "/" path is provided.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathMatch>,
    /// Headers specifies HTTP request header matchers. Multiple match values are
    /// ANDed together, meaning, a request must match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpHeaderMatch>>,
    /// QueryParams specifies HTTP query parameter matchers. Multiple match
    /// values are ANDed together, meaning, a request must match all the
    /// specified query parameters to select the route.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<Vec<HttpQueryParamMatch>>,
    /// Method specifies HTTP method matcher.
    /// When specified, this route will be matched only if the request has the
    /// specified method.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
}

/// HTTPRouteFilter defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteFilter {
    /// Type identifies the type of filter to apply. As with other API fields,
    /// types are classified into three conformance levels:
    ///
    /// - Core: Filter types and their corresponding configuration defined by
    ///   "Support: Core" in this package, e.g. "RequestHeaderModifier". All
    ///   implementations must support core filters.
    ///
    /// - Extended: Filter types and their corresponding configuration defined by
    ///   "Support: Extended" in this package, e.g. "RequestMirror". Implementers
    ///   are encouraged to support extended filters.
    ///
    /// - Implementation-specific: Filters that are defined and supported by
    ///   specific vendors.
    ///   In the future, filters showing convergence in behavior across multiple
    ///   implementations will be considered for inclusion in extended or core
    ///   conformance levels. Filter-specific configuration for such filters
    ///   is specified using the ExtensionRef field. `Type` should be set to
    ///   "ExtensionRef" for custom filters.
    ///
    /// Implementers are encouraged to define custom implementation types to
    /// extend the core API with implementation-specific behavior.
    ///
    /// If a reference to a custom filter type cannot be resolved, the filter
    /// MUST NOT be skipped. Instead, requests that would have been processed by
    /// that filter MUST receive a HTTP error response.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    pub r#type: HttpRouteFilterType,
    /// RequestHeaderModifier defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaderModifier")]
    pub request_header_modifier: Option<HttpHeaderFilter>,
    /// ResponseHeaderModifier defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderModifier")]
    pub response_header_modifier: Option<HttpHeaderFilter>,
    /// RequestMirror defines a schema for a filter that mirrors requests.
    /// Requests are sent to the specified destination, but responses from
    /// that destination are ignored.
    ///
    /// This filter can be used multiple times within the same rule. Note that
    /// not all implementations will be able to support mirroring to multiple
    /// backends.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestMirror")]
    pub request_mirror: Option<HttpRequestMirrorFilter>,
    /// RequestRedirect defines a schema for a filter that responds to the
    /// request with an HTTP redirection.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRedirect")]
    pub request_redirect: Option<HttpRequestRedirectFilter>,
    /// URLRewrite defines a schema for a filter that modifies a request during forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "urlRewrite")]
    pub url_rewrite: Option<HttpUrlRewriteFilter>,
    /// CORS defines a schema for a filter that responds to the
    /// cross-origin request based on HTTP response header.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<HttpCorsFilter>,
    /// ExtensionRef is an optional, implementation-specific extension to the
    /// "filter" behavior.  For example, resource "myroutefilter" in group
    /// "networking.example.net"). ExtensionRef MUST NOT be used for core and
    /// extended filters.
    ///
    /// This filter can be used multiple times within the same rule.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "extensionRef")]
    pub extension_ref: Option<LocalObjectReference>,
}

/// HTTPRouteFilterType identifies a type of HTTPRoute filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRouteFilterType {
    /// HTTPRouteFilterRequestHeaderModifier can be used to add or remove an HTTP
    /// header from an HTTP request before it is sent to the upstream target.
    ///
    /// Support in HTTPRouteRule: Core
    ///
    /// Support in HTTPBackendRef: Extended
    RequestHeaderModifier,
    /// HTTPRouteFilterResponseHeaderModifier can be used to add or remove an HTTP
    /// header from an HTTP response before it is sent to the client.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    ResponseHeaderModifier,
    /// HTTPRouteFilterRequestRedirect can be used to redirect a request to
    /// another location. This filter can also be used for HTTP to HTTPS
    /// redirects. This may not be used on the same Route rule or BackendRef as a
    /// URLRewrite filter.
    ///
    /// Support in HTTPRouteRule: Core
    ///
    /// Support in HTTPBackendRef: Extended
    RequestRedirect,
    /// HTTPRouteFilterURLRewrite can be used to modify a request during
    /// forwarding. At most one of these filters may be used on a Route rule.
    /// This may not be used on the same Route rule or BackendRef as a
    /// RequestRedirect filter.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    #[serde(rename = "URLRewrite")]
    UrlRewrite,
    /// HTTPRouteFilterRequestMirror can be used to mirror HTTP requests to a
    /// different backend. The responses from this backend MUST be ignored by
    /// the Gateway.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    RequestMirror,
    /// HTTPRouteFilterCORS can be used to add CORS headers to an
    /// HTTP response before it is sent to the client.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    #[serde(rename = "CORS")]
    Cors,
    /// HTTPRouteFilterExtensionRef should be used for configuring custom
    /// HTTP filters.
    ///
    /// Support in HTTPRouteRule: Implementation-specific
    ///
    /// Support in HTTPBackendRef: Implementation-specific
    ExtensionRef,
}

impl Default for HttpRouteFilterType {
    fn default() -> Self {
        Self::RequestHeaderModifier
    }
}

/// HTTPHeader represents an HTTP Header name and value as defined by RFC 7230.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpHeader {
    /// Name is the name of the HTTP Header to be matched. Name matching MUST be
    /// case-insensitive. (See https://tools.ietf.org/html/rfc7230#section-3.2).
    ///
    /// If multiple entries specify equivalent header names, the first entry with
    /// an equivalent name MUST be considered for a match. Subsequent entries
    /// with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    pub name: HttpHeaderName,
    /// Value is the value of HTTP Header to be matched.
    pub value: String,
}

/// HTTPHeaderFilter defines a filter that modifies the headers of an HTTP
/// request or response. Only one action for a given header name is
/// permitted. Filters specifying multiple actions of the same or different
/// type for any one header name are invalid. Configuration to set or add
/// multiple values for a header must use RFC 7230 header value formatting,
/// separating each value with a comma.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpHeaderFilter {
    /// Set overwrites the request with the given header (name, value)
    /// before the action.
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo
    ///
    /// Config:
    ///   set:
    ///   - name: "my-header"
    ///     value: "bar"
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header: bar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub set: Option<Vec<HttpHeader>>,
    /// Add adds the given header(s) (name, value) to the request
    /// before the action. It appends to any existing values associated
    /// with the header name.
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo
    ///
    /// Config:
    ///   add:
    ///   - name: "my-header"
    ///     value: "bar,baz"
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header: foo,bar,baz
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub add: Option<Vec<HttpHeader>>,
    /// Remove the given header(s) from the HTTP request before the action. The
    /// value of Remove is a list of HTTP header names. Note that the header
    /// names are case-insensitive (see
    /// https://datatracker.ietf.org/doc/html/rfc2616#section-4.2).
    ///
    /// Input:
    ///   GET /foo HTTP/1.1
    ///   my-header1: foo
    ///   my-header2: bar
    ///   my-header3: baz
    ///
    /// Config:
    ///   remove: ["my-header1", "my-header3"]
    ///
    /// Output:
    ///   GET /foo HTTP/1.1
    ///   my-header2: bar
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remove: Option<Vec<String>>,
}

/// HTTPPathModifierType defines the type of path redirect or rewrite.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpPathModifierType {
    /// This type of modifier indicates that the full path will be replaced
    /// by the specified value.
    ReplaceFullPath,
    /// This type of modifier indicates that any prefix path matches will be
    /// replaced by the substitution value. For example, a path with a prefix
    /// match of "/foo" and a ReplacePrefixMatch substitution of "/bar" will have
    /// the "/foo" prefix replaced with "/bar" in matching requests.
    ///
    /// Note that this matches the behavior of the PathPrefix match type. This
    /// matches full path elements. A path element refers to the list of labels
    /// in the path split by the `/` separator. When specified, a trailing `/` is
    /// ignored. For example, the paths `/abc`, `/abc/`, and `/abc/def` would all
    /// match the prefix `/abc`, but the path `/abcd` would not.
    ReplacePrefixMatch,
}

impl Default for HttpPathModifierType {
    fn default() -> Self {
        Self::ReplaceFullPath
    }
}

/// HTTPPathModifier defines configuration for path modifiers.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpPathModifier {
    /// Type defines the type of path modifier. Additional types may be
    /// added in a future release of the API.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    pub r#type: HttpPathModifierType,
    /// ReplaceFullPath specifies the value with which to replace the full path
    /// of a request during a rewrite or redirect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replaceFullPath")]
    pub replace_full_path: Option<String>,
    /// ReplacePrefixMatch specifies the value with which to replace the prefix
    /// match of a request during a rewrite or redirect. For example, a request
    /// to "/foo/bar" with a prefix match of "/foo" and a ReplacePrefixMatch
    /// of "/xyz" would be modified to "/xyz/bar".
    ///
    /// Note that this matches the behavior of the PathPrefix match type. This
    /// matches full path elements. A path element refers to the list of labels
    /// in the path split by the `/` separator. When specified, a trailing `/` is
    /// ignored. For example, the paths `/abc`, `/abc/`, and `/abc/def` would all
    /// match the prefix `/abc`, but the path `/abcd` would not.
    ///
    /// ReplacePrefixMatch is only compatible with a `PathPrefix` HTTPRouteMatch.
    /// Using any other HTTPRouteMatch type on the same HTTPRouteRule will result in
    /// the implementation setting the Accepted Condition for the Route to `status: False`.
    ///
    /// Request Path | Prefix Match | Replace Prefix | Modified Path
    /// -------------|--------------|----------------|----------
    /// /foo/bar     | /foo         | /xyz           | /xyz/bar
    /// /foo/bar     | /foo         | /xyz/          | /xyz/bar
    /// /foo/bar     | /foo/        | /xyz           | /xyz/bar
    /// /foo/bar     | /foo/        | /xyz/          | /xyz/bar
    /// /foo         | /foo         | /xyz           | /xyz
    /// /foo/        | /foo         | /xyz           | /xyz/
    /// /foo/bar     | /foo         | <empty string> | /bar
    /// /foo/        | /foo         | <empty string> | /
    /// /foo         | /foo         | <empty string> | /
    /// /foo/        | /foo         | /              | /
    /// /foo         | /foo         | /              | /
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replacePrefixMatch")]
    pub replace_prefix_match: Option<String>,
}

/// HTTPRequestRedirect defines a filter that redirects a request. This filter
/// MUST NOT be used on the same Route rule as a HTTPURLRewrite filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRequestRedirectFilter {
    /// Scheme is the scheme to be used in the value of the `Location` header in
    /// the response. When empty, the scheme of the request is used.
    ///
    /// Scheme redirects can affect the port of the redirect, for more information,
    /// refer to the documentation for the port field of this filter.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scheme: Option<HttpRequestRedirectFilterScheme>,
    /// Hostname is the hostname to be used in the value of the `Location`
    /// header in the response.
    /// When empty, the hostname in the `Host` header of the request is used.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<PreciseHostname>,
    /// Path defines parameters used to modify the path of the incoming request.
    /// The modified path is then used to construct the `Location` header. When
    /// empty, the request path is used as-is.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathModifier>,
    /// Port is the port to be used in the value of the `Location`
    /// header in the response.
    ///
    /// If no port is specified, the redirect port MUST be derived using the
    /// following rules:
    ///
    /// * If redirect scheme is not-empty, the redirect port MUST be the well-known
    ///   port associated with the redirect scheme. Specifically "http" to port 80
    ///   and "https" to port 443. If the redirect scheme does not have a
    ///   well-known port, the listener port of the Gateway SHOULD be used.
    /// * If redirect scheme is empty, the redirect port MUST be the Gateway
    ///   Listener port.
    ///
    /// Implementations SHOULD NOT add the port number in the 'Location'
    /// header in the following cases:
    ///
    /// * A Location header that will use HTTP (whether that is determined via
    ///   the Listener protocol or the Scheme field) _and_ use port 80.
    /// * A Location header that will use HTTPS (whether that is determined via
    ///   the Listener protocol or the Scheme field) _and_ use port 443.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// StatusCode is the HTTP status code to be used in response.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    pub status_code: Option<i64>,
}

/// HTTPURLRewriteFilter defines a filter that modifies a request during
/// forwarding. At most one of these filters may be used on a Route rule. This
/// MUST NOT be used on the same Route rule as a HTTPRequestRedirect filter.
///
/// Support: Extended
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpUrlRewriteFilter {
    /// Hostname is the value to be used to replace the Host header value during
    /// forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<PreciseHostname>,
    /// Path defines a path rewrite.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathModifier>,
}

/// HTTPRequestMirrorFilter defines configuration for the RequestMirror filter.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRequestMirrorFilter {
    /// BackendRef references a resource where mirrored requests are sent.
    ///
    /// Mirrored requests must be sent only to a single destination endpoint
    /// within this BackendRef, irrespective of how many endpoints are present
    /// within this BackendRef.
    ///
    /// If the referent cannot be found, this BackendRef is invalid and must be
    /// dropped from the Gateway. The controller must ensure the "ResolvedRefs"
    /// condition on the Route status is set to `status: False` and not configure
    /// this backend in the underlying implementation.
    ///
    /// If there is a cross-namespace reference to an *existing* object
    /// that is not allowed by a ReferenceGrant, the controller must ensure the
    /// "ResolvedRefs"  condition on the Route is set to `status: False`,
    /// with the "RefNotPermitted" reason and not configure this backend in the
    /// underlying implementation.
    ///
    /// In either error case, the Message of the `ResolvedRefs` Condition
    /// should be used to provide more detail about the problem.
    ///
    /// Support: Extended for Kubernetes Service
    ///
    /// Support: Implementation-specific for any other resource
    #[serde(rename = "backendRef")]
    pub backend_ref: BackendObjectReference,
    /// Percent represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    /// Fraction represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<Fraction>,
}

/// HTTPCORSFilter defines a filter that that configures Cross-Origin Request
/// Sharing (CORS).
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpCorsFilter {
    /// AllowOrigins indicates whether the response can be shared with requested
    /// resource from the given `Origin`.
    ///
    /// The `Origin` consists of a scheme and a host, with an optional port, and
    /// takes the form `<scheme>://<host>(:<port>)`.
    ///
    /// Valid values for scheme are: `http` and `https`.
    ///
    /// Valid values for port are any integer between 1 and 65535 (the list of
    /// available TCP/UDP ports). Note that, if not included, port `80` is
    /// assumed for `http` scheme origins, and port `443` is assumed for `https`
    /// origins. This may affect origin matching.
    ///
    /// The host part of the origin may contain the wildcard character `*`. These
    /// wildcard characters behave as follows:
    ///
    /// * `*` is a greedy match to the _left_, including any number of
    ///   DNS labels to the left of its position. This also means that
    ///   `*` will include any number of period `.` characters to the
    ///   left of its position.
    /// * A wildcard by itself matches all hosts.
    ///
    /// An origin value that includes _only_ the `*` character indicates requests
    /// from all `Origin`s are allowed.
    ///
    /// When the `AllowOrigins` field is configured with multiple origins, it
    /// means the server supports clients from multiple origins. If the request
    /// `Origin` matches the configured allowed origins, the gateway must return
    /// the given `Origin` and sets value of the header
    /// `Access-Control-Allow-Origin` same as the `Origin` header provided by the
    /// client.
    ///
    /// The status code of a successful response to a "preflight" request is
    /// always an OK status (i.e., 204 or 200).
    ///
    /// If the request `Origin` does not match the configured allowed origins,
    /// the gateway returns 204/200 response but doesn't set the relevant
    /// cross-origin response headers. Alternatively, the gateway responds with
    /// 403 status to the "preflight" request is denied, coupled with omitting
    /// the CORS headers. The cross-origin request fails on the client side.
    /// Therefore, the client doesn't attempt the actual cross-origin request.
    ///
    /// Conversely, if the request `Origin` matches one of the configured
    /// allowed origins, the gateway sets the response header
    /// `Access-Control-Allow-Origin` to the same value as the `Origin`
    /// header provided by the client.
    ///
    /// When config has the wildcard ("*") in allowOrigins, and the request
    /// is not credentialed (e.g., it is a preflight request), the
    /// `Access-Control-Allow-Origin` response header either contains the
    /// wildcard as well or the Origin from the request.
    ///
    /// When the request is credentialed, the gateway must not specify the `*`
    /// wildcard in the `Access-Control-Allow-Origin` response header. When
    /// also the `AllowCredentials` field is true and `AllowOrigins` field
    /// specified with the `*` wildcard, the gateway must return a single origin
    /// in the value of the `Access-Control-Allow-Origin` response header,
    /// instead of specifying the `*` wildcard. The value of the header
    /// `Access-Control-Allow-Origin` is same as the `Origin` header provided by
    /// the client.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowOrigins")]
    pub allow_origins: Option<Vec<CorsOrigin>>,
    /// AllowCredentials indicates whether the actual cross-origin request allows
    /// to include credentials.
    ///
    /// When set to true, the gateway will include the `Access-Control-Allow-Credentials`
    /// response header with value true (case-sensitive).
    ///
    /// When set to false or omitted the gateway will omit the header
    /// `Access-Control-Allow-Credentials` entirely (this is the standard CORS
    /// behavior).
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowCredentials")]
    pub allow_credentials: Option<bool>,
    /// AllowMethods indicates which HTTP methods are supported for accessing the
    /// requested resource.
    ///
    /// Valid values are any method defined by RFC9110, along with the special
    /// value `*`, which represents all HTTP methods are allowed.
    ///
    /// Method names are case-sensitive, so these values are also case-sensitive.
    /// (See https://www.rfc-editor.org/rfc/rfc2616#section-5.1.1)
    ///
    /// Multiple method names in the value of the `Access-Control-Allow-Methods`
    /// response header are separated by a comma (",").
    ///
    /// A CORS-safelisted method is a method that is `GET`, `HEAD`, or `POST`.
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-method) The
    /// CORS-safelisted methods are always allowed, regardless of whether they
    /// are specified in the `AllowMethods` field.
    ///
    /// When the `AllowMethods` field is configured with one or more methods, the
    /// gateway must return the `Access-Control-Allow-Methods` response header
    /// which value is present in the `AllowMethods` field.
    ///
    /// If the HTTP method of the `Access-Control-Request-Method` request header
    /// is not included in the list of methods specified by the response header
    /// `Access-Control-Allow-Methods`, it will present an error on the client
    /// side.
    ///
    /// If config contains the wildcard "*" in allowMethods and the request is
    /// not credentialed, the `Access-Control-Allow-Methods` response header
    /// can either use the `*` wildcard or the value of
    /// Access-Control-Request-Method from the request.
    ///
    /// When the request is credentialed, the gateway must not specify the `*`
    /// wildcard in the `Access-Control-Allow-Methods` response header. When
    /// also the `AllowCredentials` field is true and `AllowMethods` field
    /// specified with the `*` wildcard, the gateway must specify one HTTP method
    /// in the value of the Access-Control-Allow-Methods response header. The
    /// value of the header `Access-Control-Allow-Methods` is same as the
    /// `Access-Control-Request-Method` header provided by the client. If the
    /// header `Access-Control-Request-Method` is not included in the request,
    /// the gateway will omit the `Access-Control-Allow-Methods` response header,
    /// instead of specifying the `*` wildcard.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowMethods")]
    pub allow_methods: Option<Vec<HttpMethodWithWildcard>>,
    /// AllowHeaders indicates which HTTP request headers are supported for
    /// accessing the requested resource.
    ///
    /// Header names are not case-sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Allow-Headers`
    /// response header are separated by a comma (",").
    ///
    /// When the `AllowHeaders` field is configured with one or more headers, the
    /// gateway must return the `Access-Control-Allow-Headers` response header
    /// which value is present in the `AllowHeaders` field.
    ///
    /// If any header name in the `Access-Control-Request-Headers` request header
    /// is not included in the list of header names specified by the response
    /// header `Access-Control-Allow-Headers`, it will present an error on the
    /// client side.
    ///
    /// If any header name in the `Access-Control-Allow-Headers` response header
    /// does not recognize by the client, it will also occur an error on the
    /// client side.
    ///
    /// A wildcard indicates that the requests with all HTTP headers are allowed.
    /// If config contains the wildcard "*" in allowHeaders and the request is
    /// not credentialed, the `Access-Control-Allow-Headers` response header
    /// can either use the `*` wildcard or the value of
    /// Access-Control-Request-Headers from the request.
    ///
    /// When the request is credentialed, the gateway must not specify the `*`
    /// wildcard in the `Access-Control-Allow-Headers` response header. When
    /// also the `AllowCredentials` field is true and `AllowHeaders` field
    /// is specified with the `*` wildcard, the gateway must specify one or more
    /// HTTP headers in the value of the `Access-Control-Allow-Headers` response
    /// header. The value of the header `Access-Control-Allow-Headers` is same as
    /// the `Access-Control-Request-Headers` header provided by the client. If
    /// the header `Access-Control-Request-Headers` is not included in the
    /// request, the gateway will omit the `Access-Control-Allow-Headers`
    /// response header, instead of specifying the `*` wildcard.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowHeaders")]
    pub allow_headers: Option<Vec<HttpHeaderName>>,
    /// ExposeHeaders indicates which HTTP response headers can be exposed
    /// to client-side scripts in response to a cross-origin request.
    ///
    /// A CORS-safelisted response header is an HTTP header in a CORS response
    /// that it is considered safe to expose to the client scripts.
    /// The CORS-safelisted response headers include the following headers:
    /// `Cache-Control`
    /// `Content-Language`
    /// `Content-Length`
    /// `Content-Type`
    /// `Expires`
    /// `Last-Modified`
    /// `Pragma`
    /// (See https://fetch.spec.whatwg.org/#cors-safelisted-response-header-name)
    /// The CORS-safelisted response headers are exposed to client by default.
    ///
    /// When an HTTP header name is specified using the `ExposeHeaders` field,
    /// this additional header will be exposed as part of the response to the
    /// client.
    ///
    /// Header names are not case-sensitive.
    ///
    /// Multiple header names in the value of the `Access-Control-Expose-Headers`
    /// response header are separated by a comma (",").
    ///
    /// A wildcard indicates that the responses with all HTTP headers are exposed
    /// to clients. The `Access-Control-Expose-Headers` response header can only
    /// use `*` wildcard as value when the request is not credentialed.
    ///
    /// When the `exposeHeaders` config field contains the "*" wildcard and
    /// the request is credentialed, the gateway cannot use the `*` wildcard in
    /// the `Access-Control-Expose-Headers` response header.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "exposeHeaders")]
    pub expose_headers: Option<Vec<HttpHeaderName>>,
    /// MaxAge indicates the duration (in seconds) for the client to cache the
    /// results of a "preflight" request.
    ///
    /// The information provided by the `Access-Control-Allow-Methods` and
    /// `Access-Control-Allow-Headers` response headers can be cached by the
    /// client until the time specified by `Access-Control-Max-Age` elapses.
    ///
    /// The default value of `Access-Control-Max-Age` response header is 5
    /// (seconds).
    ///
    /// When the `MaxAge` field is unspecified, the gateway sets the response
    /// header "Access-Control-Max-Age: 5" by default.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i32>,
}

/// HTTPBackendRef defines how a HTTPRoute forwards a HTTP request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpBackendRef {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is the Kubernetes resource kind of the referent. For example
    /// "Service".
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations SHOULD NOT
    /// support ExternalName Services.
    ///
    /// Support: Core (Services with a type other than ExternalName)
    ///
    /// Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// Filters defined at this level should be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in HTTPRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HttpRouteFilter>>,
}

/// HTTPRouteStatus defines the observed state of HTTPRoute.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct HttpRouteStatus {
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

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerEntry {
    /// Name is the name of the Listener. This name MUST be unique within a
    /// ListenerSet.
    ///
    /// Name is not required to be unique across a Gateway and ListenerSets.
    /// Routes can attach to a Listener by having a ListenerSet as a parentRef
    /// and setting the SectionName
    pub name: SectionName,
    /// Hostname specifies the virtual hostname to match for protocol types that
    /// define this concept. When unspecified, all hostnames are matched. This
    /// field is ignored for protocols that don't require hostname based
    /// matching.
    ///
    /// Implementations MUST apply Hostname matching appropriately for each of
    /// the following protocols:
    ///
    /// * TLS: The Listener Hostname MUST match the SNI.
    /// * HTTP: The Listener Hostname MUST match the Host header of the request.
    /// * HTTPS: The Listener Hostname SHOULD match at both the TLS and HTTP
    ///   protocol layers as described above. If an implementation does not
    ///   ensure that both the SNI and Host header match the Listener hostname,
    ///   it MUST clearly document that.
    ///
    /// For HTTPRoute and TLSRoute resources, there is an interaction with the
    /// `spec.hostnames` array. When both listener and route specify hostnames,
    /// there MUST be an intersection between the values for a Route to be
    /// accepted. For more information, refer to the Route specific Hostnames
    /// documentation.
    ///
    /// Hostnames that are prefixed with a wildcard label (`*.`) are interpreted
    /// as a suffix match. That means that a match for `*.example.com` would match
    /// both `test.example.com`, and `foo.test.example.com`, but not `example.com`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,
    /// Port is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    pub port: PortNumber,
    /// Protocol specifies the network protocol this listener expects to receive.
    pub protocol: ProtocolType,
    /// TLS is the TLS configuration for the Listener. This field is required if
    /// the Protocol field is "HTTPS" or "TLS". It is invalid to set this field
    /// if the Protocol field is "HTTP", "TCP", or "UDP".
    ///
    /// The association of SNIs to Certificate defined in ListenerTLSConfig is
    /// defined based on the Hostname field for this listener.
    ///
    /// The GatewayClass MUST use the longest matching SNI out of all
    /// available certificates for any TLS handshake.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<ListenerTlsConfig>,
    /// AllowedRoutes defines the types of routes that MAY be attached to a
    /// Listener and the trusted namespaces where those Route resources MAY be
    /// present.
    ///
    /// Although a client request may match multiple route rules, only one rule
    /// may ultimately receive the request. Matching precedence MUST be
    /// determined in order of the following criteria:
    ///
    /// * The most specific match as defined by the Route type.
    /// * The oldest Route based on creation timestamp. For example, a Route with
    ///   a creation timestamp of "2020-09-08 01:02:03" is given precedence over
    ///   a Route with a creation timestamp of "2020-09-08 01:02:04".
    /// * If everything else is equivalent, the Route appearing first in
    ///   alphabetical order (namespace/name) should be given precedence. For
    ///   example, foo/bar is given precedence over foo/baz.
    ///
    /// All valid rules within a Route attached to this Listener should be
    /// implemented. Invalid Route rules can be ignored (sometimes that will mean
    /// the full Route). If a Route rule transitions from valid to invalid,
    /// support for that Route rule should be dropped to ensure consistency. For
    /// example, even if a filter specified by a Route rule is invalid, the rest
    /// of the rules within that Route should still be supported.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<AllowedRoutes>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerSetStatus {
    /// Conditions describe the current conditions of the ListenerSet.
    ///
    /// Implementations MUST express ListenerSet conditions using the
    /// `ListenerSetConditionType` and `ListenerSetConditionReason`
    /// constants so that operators and tools can converge on a common
    /// vocabulary to describe ListenerSet state.
    ///
    /// Known condition types are:
    ///
    /// * "Accepted"
    /// * "Programmed"
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    /// Listeners provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerEntryStatus>>,
}

/// ListenerStatus is the status associated with a Listener.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ListenerEntryStatus {
    /// Name is the name of the Listener that this status corresponds to.
    pub name: SectionName,
    /// SupportedKinds is the list indicating the Kinds supported by this
    /// listener. This MUST represent the kinds supported by an implementation for
    /// that Listener configuration.
    ///
    /// If kinds are specified in Spec that are not supported, they MUST NOT
    /// appear in this list and an implementation MUST set the "ResolvedRefs"
    /// condition to "False" with the "InvalidRouteKinds" reason. If both valid
    /// and invalid Route kinds are specified, the implementation MUST
    /// reference the valid Route kinds that have been specified.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedKinds")]
    pub supported_kinds: Option<Vec<RouteGroupKind>>,
    /// AttachedRoutes represents the total number of Routes that have been
    /// successfully attached to this Listener.
    ///
    /// Successful attachment of a Route to a Listener is based solely on the
    /// combination of the AllowedRoutes field on the corresponding Listener
    /// and the Route's ParentRefs field. A Route is successfully attached to
    /// a Listener when it is selected by the Listener's AllowedRoutes field
    /// AND the Route has a valid ParentRef selecting the whole Gateway
    /// resource or a specific Listener as a parent resource (more detail on
    /// attachment semantics can be found in the documentation on the various
    /// Route kinds ParentRefs fields). Listener status does not impact
    /// successful attachment, i.e. the AttachedRoutes field count MUST be set
    /// for Listeners, even if the Accepted condition of an individual Listener is set
    /// to "False". The AttachedRoutes number represents the number of Routes with
    /// the Accepted condition set to "True" that have been attached to this Listener.
    /// Routes with any other value for the Accepted condition MUST NOT be included
    /// in this count.
    ///
    /// Uses for this field include troubleshooting Route attachment and
    /// measuring blast radius/impact of changes to a Listener.
    #[serde(rename = "attachedRoutes")]
    pub attached_routes: i32,
    /// Conditions describe the current condition of this listener.
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}

/// LocalObjectReference identifies an API object within the namespace of the
/// referrer.
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct LocalObjectReference {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    pub group: Group,
    /// Kind is kind of the referent. For example "HTTPRoute" or "Service".
    pub kind: Kind,
    /// Name is the name of the referent.
    pub name: ObjectName,
}

/// SecretObjectReference identifies an API object including its namespace,
/// defaulting to Secret.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct SecretObjectReference {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is kind of the referent. For example "Secret".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

/// BackendObjectReference defines how an ObjectReference that is
/// specific to BackendRef. It includes a few additional fields and features
/// than a regular ObjectReference.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendObjectReference {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is the Kubernetes resource kind of the referent. For example
    /// "Service".
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations SHOULD NOT
    /// support ExternalName Services.
    ///
    /// Support: Core (Services with a type other than ExternalName)
    ///
    /// Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
}

/// ObjectReference identifies an API object including its namespace.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ObjectReference {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When set to the empty string, core API group is inferred.
    pub group: Group,
    /// Kind is kind of the referent. For example "ConfigMap" or "Service".
    pub kind: Kind,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the referenced object. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

/// NamespacedPolicyTargetReference identifies an API object to apply a direct or
/// inherited policy to, potentially in a different namespace. This should only
/// be used as part of Policy resources that need to be able to target resources
/// in different namespaces. For more information on how this policy attachment
/// model works, and a sample Policy resource, refer to the policy attachment
/// documentation for Gateway API.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

/// LocalPolicyTargetReferenceWithSectionName identifies an API object to apply a
/// direct policy to. This should be used as part of Policy resources that can
/// target single resources. For more information on how this policy attachment
/// mode works, and a sample Policy resource, refer to the policy attachment
/// documentation for Gateway API.
///
/// Note: This should only be used for direct policy attachment when references
/// to SectionName are actually needed. In all other cases,
/// LocalPolicyTargetReference should be used.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

/// PolicyConditionType is a type of condition for a policy. This type should be
/// used with a Policy resource Status.Conditions field.
pub type PolicyConditionType = String;

/// PolicyConditionReason is a reason for a policy condition.
pub type PolicyConditionReason = String;

/// PolicyAncestorStatus describes the status of a route with respect to an
/// associated Ancestor.
///
/// Ancestors refer to objects that are either the Target of a policy or above it
/// in terms of object hierarchy. For example, if a policy targets a Service, the
/// Policy's Ancestors are, in order, the Service, the HTTPRoute, the Gateway, and
/// the GatewayClass. Almost always, in this hierarchy, the Gateway will be the most
/// useful object to place Policy status on, so we recommend that implementations
/// SHOULD use Gateway as the PolicyAncestorStatus object unless the designers
/// have a _very_ good reason otherwise.
///
/// In the context of policy attachment, the Ancestor is used to distinguish which
/// resource results in a distinct application of this policy. For example, if a policy
/// targets a Service, it may have a distinct result per attached Gateway.
///
/// Policies targeting the same resource may have different effects depending on the
/// ancestors of those resources. For example, different Gateways targeting the same
/// Service may have different capabilities, especially if they have different underlying
/// implementations.
///
/// For example, in BackendTLSPolicy, the Policy attaches to a Service that is
/// used as a backend in a HTTPRoute that is itself attached to a Gateway.
/// In this case, the relevant object for status is the Gateway, and that is the
/// ancestor object referred to in this status.
///
/// Note that a parent is also an ancestor, so for objects where the parent is the
/// relevant object for status, this struct SHOULD still be used.
///
/// This struct is intended to be used in a slice that's effectively a map,
/// with a composite key made up of the AncestorRef and the ControllerName.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}

/// PolicyStatus defines the common attributes that all Policies should include within
/// their status.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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
    pub ancestors: Vec<PolicyAncestorStatus>,
}

/// ReferenceGrantFrom describes trusted namespaces and kinds.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ReferenceGrantFrom {
    /// Group is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,
    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core"
    /// support level for this field.
    ///
    /// When used to permit a SecretObjectReference:
    ///
    /// * Gateway
    ///
    /// When used to permit a BackendObjectReference:
    ///
    /// * GRPCRoute
    /// * HTTPRoute
    /// * TCPRoute
    /// * TLSRoute
    /// * UDPRoute
    pub kind: Kind,
    /// Namespace is the namespace of the referent.
    ///
    /// Support: Core
    pub namespace: Namespace,
}

/// ReferenceGrantTo describes what Kinds are allowed as targets of the
/// references.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ReferenceGrantTo {
    /// Group is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,
    /// Kind is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core"
    /// support level for this field:
    ///
    /// * Secret when used to permit a SecretObjectReference
    /// * Service when used to permit a BackendObjectReference
    pub kind: Kind,
    /// Name is the name of the referent. When unspecified, this policy
    /// refers to all resources of the specified Group and Kind in the local
    /// namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ObjectName>,
}

/// ParentReference identifies an API object (usually a Gateway) that can be considered
/// a parent of this resource (usually a route). There are two kinds of parent resources
/// with "Core" support:
///
/// * Gateway (Gateway conformance profile)
/// * Service (Mesh conformance profile, ClusterIP Services only)
///
/// This API may be extended in the future to support additional kinds of parent
/// resources.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentReference {
    /// Group is the group of the referent.
    /// When unspecified, "gateway.networking.k8s.io" is inferred.
    /// To set the core API group (such as for a "Service" kind referent),
    /// Group must be explicitly set to "" (empty string).
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is kind of the referent.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, ClusterIP Services only)
    ///
    /// Support for other resources is Implementation-Specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Namespace is the namespace of the referent. When unspecified, this refers
    /// to the local namespace of the Route.
    ///
    /// Note that there are specific rules for ParentRefs which cross namespace
    /// boundaries. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example:
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable any other kind of cross-namespace reference.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// Name is the name of the referent.
    ///
    /// Support: Core
    pub name: ObjectName,
    /// SectionName is the name of a section within the target resource. In the
    /// following resources, SectionName is interpreted as the following:
    ///
    /// * Gateway: Listener name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values.
    /// * Service: Port name. When both Port (experimental) and SectionName
    /// are specified, the name and port of the selected listener must match
    /// both specified values.
    ///
    /// Implementations MAY choose to support attaching Routes to other resources.
    /// If that is the case, they MUST clearly document how SectionName is
    /// interpreted.
    ///
    /// When unspecified (empty string), this will reference the entire resource.
    /// For the purpose of status, an attachment is considered successful if at
    /// least one section in the parent resource accepts it. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment from
    /// the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route, the
    /// Route MUST be considered detached from the Gateway.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sectionName")]
    pub section_name: Option<SectionName>,
    /// Port is the network port this Route targets. It can be interpreted
    /// differently based on the type of parent resource.
    ///
    /// When the parent resource is a Gateway, this targets all listeners
    /// listening on the specified port that also support this kind of Route(and
    /// select this Route). It's not recommended to set `Port` unless the
    /// networking behaviors specified in a Route must apply to a specific port
    /// as opposed to a listener(s) whose port(s) may be changed. When both Port
    /// and SectionName are specified, the name and port of the selected listener
    /// must match both specified values.
    ///
    /// Implementations MAY choose to support other parent resources.
    /// Implementations supporting other types of parent resources MUST clearly
    /// document how/if Port is interpreted.
    ///
    /// For the purpose of status, an attachment is considered successful as
    /// long as the parent resource accepts it partially. For example, Gateway
    /// listeners can restrict which Routes can attach to them by Route kind,
    /// namespace, or hostname. If 1 of 2 Gateway listeners accept attachment
    /// from the referencing Route, the Route MUST be considered successfully
    /// attached. If no Gateway listeners accept attachment from this Route,
    /// the Route MUST be considered detached from the Gateway.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
}

/// CommonRouteSpec defines the common attributes that all Routes MUST include
/// within their spec.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct CommonRouteSpec {
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
}

/// BackendRef defines how a Route should forward a request to a Kubernetes
/// resource.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// Note that when the BackendTLSPolicy object is enabled by the implementation,
/// there are some extra rules about validity to consider here. See the fields
/// where this struct is used for more information about the exact behavior.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BackendRef {
    /// Group is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is the Kubernetes resource kind of the referent. For example
    /// "Service".
    ///
    /// Defaults to "Service" when not specified.
    ///
    /// ExternalName services can refer to CNAME DNS records that may live
    /// outside of the cluster and as such are difficult to reason about in
    /// terms of conformance. They also may not be safe to forward to (see
    /// CVE-2021-25740 for more information). Implementations SHOULD NOT
    /// support ExternalName Services.
    ///
    /// Support: Core (Services with a type other than ExternalName)
    ///
    /// Support: Implementation-specific (Services with type ExternalName)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the backend. When unspecified, the local
    /// namespace is inferred.
    ///
    /// Note that when a namespace different than the local namespace is specified,
    /// a ReferenceGrant object is required in the referent namespace to allow that
    /// namespace's owner to accept the reference. See the ReferenceGrant
    /// documentation for details.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// Port specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// Weight specifies the proportion of requests forwarded to the referenced
    /// backend. This is computed as weight/(sum of all weights in this
    /// BackendRefs list). For non-zero values, there may be some epsilon from
    /// the exact proportion defined here depending on the precision an
    /// implementation supports. Weight is not a percentage and the sum of
    /// weights does not need to equal 100.
    ///
    /// If only one backend is specified and it has a weight greater than 0, 100%
    /// of the traffic is forwarded to that backend. If weight is set to 0, no
    /// traffic should be forwarded for this entry. If unspecified, weight
    /// defaults to 1.
    ///
    /// Support for this field varies based on the context where used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}

/// RouteConditionType is a type of condition for a route.
pub type RouteConditionType = String;

/// RouteConditionReason is a reason for a route condition.
pub type RouteConditionReason = String;

/// RouteParentStatus describes the status of a route with respect to an
/// associated Parent.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteParentStatus {
    /// ParentRef corresponds with a ParentRef in the spec that this
    /// RouteParentStatus struct describes the status of.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentReference,
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
    /// Conditions describes the status of the route with respect to the Gateway.
    /// Note that the route's availability is also subject to the Gateway's own
    /// status conditions and listener status.
    ///
    /// If the Route's ParentRef specifies an existing Gateway that supports
    /// Routes of this kind AND that Gateway's controller has sufficient access,
    /// then that Gateway's controller MUST set the "Accepted" condition on the
    /// Route, to indicate whether the route has been accepted or rejected by the
    /// Gateway, and why.
    ///
    /// A Route MUST be considered "Accepted" if at least one of the Route's
    /// rules is implemented by the Gateway.
    ///
    /// There are a number of cases where the "Accepted" condition may not be set
    /// due to lack of controller visibility, that includes when:
    ///
    /// * The Route refers to a nonexistent parent.
    /// * The Route is of a type that the controller does not support.
    /// * The Route is in a namespace to which the controller does not have access.
    pub conditions: Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>,
}

/// RouteStatus defines the common attributes that all Routes MUST include within
/// their status.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RouteStatus {
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

/// Hostname is the fully qualified domain name of a network host. This matches
/// the RFC 1123 definition of a hostname with 2 notable exceptions:
///
///  1. IPs are not allowed.
///  2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
///     label must appear by itself as the first label.
///
/// Hostname can be "precise" which is a domain name without the terminating
/// dot of a network host (e.g. "foo.example.com") or "wildcard", which is a
/// domain name prefixed with a single wildcard label (e.g. `*.example.com`).
///
/// Note that as per RFC1035 and RFC1123, a *label* must consist of lower case
/// alphanumeric characters or '-', and must start and end with an alphanumeric
/// character. No other punctuation is allowed.
pub type Hostname = String;

/// PreciseHostname is the fully qualified domain name of a network host. This
/// matches the RFC 1123 definition of a hostname with 1 notable exception that
/// numeric IP addresses are not allowed.
///
/// Note that as per RFC1035 and RFC1123, a *label* must consist of lower case
/// alphanumeric characters or '-', and must start and end with an alphanumeric
/// character. No other punctuation is allowed.
pub type PreciseHostname = String;

/// The AbsoluteURI MUST NOT be a relative URI, and it MUST follow the URI syntax and
/// encoding rules specified in RFC3986.  The AbsoluteURI MUST include both a
/// scheme (e.g., "http" or "spiffe") and a scheme-specific-part.  URIs that
/// include an authority MUST include a fully qualified domain name or
/// IP address as the host.
pub type AbsoluteUri = String;

/// The CORSOrigin MUST NOT be a relative URI, and it MUST follow the URI syntax and
/// encoding rules specified in RFC3986.  The CORSOrigin MUST include both a
/// scheme ("http" or "https") and a scheme-specific-part, or it should be a single '*' character.
/// URIs that include an authority MUST include a fully qualified domain name or
/// IP address as the host.
pub type CorsOrigin = String;

/// Group refers to a Kubernetes Group. It must either be an empty string or a
/// RFC 1123 subdomain.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L208
///
/// Valid values include:
///
/// * "" - empty string implies core Kubernetes API group
/// * "gateway.networking.k8s.io"
/// * "foo.example.com"
///
/// Invalid values include:
///
/// * "example.com/bar" - "/" is an invalid character
pub type Group = String;

/// Kind refers to a Kubernetes Kind.
///
/// Valid values include:
///
/// * "Service"
/// * "HTTPRoute"
///
/// Invalid values include:
///
/// * "invalid/kind" - "/" is an invalid character
pub type Kind = String;

/// ObjectName refers to the name of a Kubernetes object.
/// Object names can have a variety of forms, including RFC 1123 subdomains,
/// RFC 1123 labels, or RFC 1035 labels.
pub type ObjectName = String;

/// Namespace refers to a Kubernetes namespace. It must be a RFC 1123 label.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L187
///
/// This is used for Namespace name validation here:
/// https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/api/validation/generic.go#L63
///
/// Valid values include:
///
/// * "example"
///
/// Invalid values include:
///
/// * "example.com" - "." is an invalid character
pub type Namespace = String;

/// SectionName is the name of a section in a Kubernetes resource.
///
/// In the following resources, SectionName is interpreted as the following:
///
/// * Gateway: Listener name
/// * HTTPRoute: HTTPRouteRule name
/// * Service: Port name
///
/// Section names can have a variety of forms, including RFC 1123 subdomains,
/// RFC 1123 labels, or RFC 1035 labels.
///
/// This validation is based off of the corresponding Kubernetes validation:
/// https://github.com/kubernetes/apimachinery/blob/02cfb53916346d085a6c6c7c66f882e3c6b0eca6/pkg/util/validation/validation.go#L208
///
/// Valid values include:
///
/// * "example"
/// * "foo-example"
/// * "example.com"
/// * "foo.example.com"
///
/// Invalid values include:
///
/// * "example.com/bar" - "/" is an invalid character
pub type SectionName = String;

/// GatewayController is the name of a Gateway API controller. It must be a
/// domain prefixed path.
///
/// Valid values include:
///
/// * "example.com/bar"
///
/// Invalid values include:
///
/// * "example.com" - must include path
/// * "foo.example.com" - must include path
pub type GatewayController = String;

/// AnnotationKey is the key of an annotation in Gateway API. This is used for
/// validation of maps such as TLS options. This matches the Kubernetes
/// "qualified name" validation that is used for annotations and other common
/// values.
///
/// Valid values include:
///
/// * example
/// * example.com
/// * example.com/path
/// * example.com/path.html
///
/// Invalid values include:
///
/// * example~ - "~" is an invalid character
/// * example.com. - cannot start or end with "."
pub type AnnotationKey = String;

/// AnnotationValue is the value of an annotation in Gateway API. This is used
/// for validation of maps such as TLS options. This roughly matches Kubernetes
/// annotation validation, although the length validation in that case is based
/// on the entire size of the annotations struct.
pub type AnnotationValue = String;

/// LabelKey is the key of a label in the Gateway API. This is used for validation
/// of maps such as Gateway infrastructure labels. This matches the Kubernetes
/// "qualified name" validation that is used for labels.
///
/// Valid values include:
///
/// * example
/// * example.com
/// * example.com/path
/// * example.com/path.html
///
/// Invalid values include:
///
/// * example~ - "~" is an invalid character
/// * example.com. - cannot start or end with "."
pub type LabelKey = String;

/// LabelValue is the value of a label in the Gateway API. This is used for validation
/// of maps such as Gateway infrastructure labels. This matches the Kubernetes
/// label validation rules:
/// * must be 63 characters or less (can be empty),
/// * unless empty, must begin and end with an alphanumeric character ([a-z0-9A-Z]),
/// * could contain dashes (-), underscores (_), dots (.), and alphanumerics between.
///
/// Valid values include:
///
/// * MyValue
/// * my.name
/// * 123-my-value
pub type LabelValue = String;

/// AddressType defines how a network address is represented as a text string.
/// This may take two possible forms:
///
/// * A predefined CamelCase string identifier (currently limited to `IPAddress` or `Hostname`)
/// * A domain-prefixed string identifier (like `acme.io/CustomAddressType`)
///
/// Values `IPAddress` and `Hostname` have Extended support.
///
/// The `NamedAddress` value has been deprecated in favor of implementation
/// specific domain-prefixed strings.
///
/// All other values, including domain-prefixed values have Implementation-specific support,
/// which are used in implementation-specific behaviors. Support for additional
/// predefined CamelCase identifiers may be added in future releases.
pub type AddressType = String;

/// HeaderName is the name of a header or query parameter.
pub type HeaderName = String;

/// Duration is a string value representing a duration in time. The format is as specified
/// in GEP-2257, a strict subset of the syntax parsed by Golang time.ParseDuration.
pub type Duration = String;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denominator: Option<i32>,
}

/// ParentGatewayReference identifies an API object including its namespace,
/// defaulting to Gateway.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct ParentGatewayReference {
    /// Group is the group of the referent.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// Kind is kind of the referent. For example "Gateway".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// Name is the name of the referent.
    pub name: ObjectName,
    /// Namespace is the namespace of the referent.  If not present,
    /// the namespace of the referent is assumed to be the same as
    /// the namespace of the referring object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}

/// TLSRouteStatus defines the observed state of TLSRoute
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

/// TLSRouteRule is the configuration for a given rule.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq)]
pub enum HttpRequestRedirectFilterScheme {
    #[serde(rename = "http")]
    Http,
    #[serde(rename = "https")]
    Https,
}

impl Default for HttpRequestRedirectFilterScheme {
    fn default() -> Self {
        Self::Http
    }
}
