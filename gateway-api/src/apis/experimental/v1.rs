// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apis/v1` at v1.5.1.
//!
//! Kinds available in the `gateway.networking.k8s.io/v1` apiVersion:
//!
//! - [`BackendTlsPolicy`]
//!   - kind: `BackendTLSPolicy`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`GrpcRoute`]
//!   - kind: `GRPCRoute`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`Gateway`]
//!   - kind: `Gateway`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`GatewayClass`]
//!   - kind: `GatewayClass`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`HttpRoute`]
//!   - kind: `HTTPRoute`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`ListenerSet`]
//!   - kind: `ListenerSet`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`ReferenceGrant`]
//!   - kind: `ReferenceGrant`
//!   - served: yes
//!   - storage: no
//!   - deprecated: no
//!
//! - [`TlsRoute`]
//!   - kind: `TLSRoute`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{Condition, LabelSelector};
/// [`BackendTlsPolicySpec`] defines the desired state of BackendTLSPolicy.
///
/// Support: Extended
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "BackendTLSPolicy",
    root = "BackendTlsPolicy",
    plural = "backendtlspolicies"
)]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(shortname = "btlspolicy")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(doc = "BackendTLSPolicy provides a way to configure how a Gateway\nconnects to a Backend via TLS.")]
pub struct BackendTlsPolicySpec {
    /// `target_refs` identifies an API object to apply the policy to.
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
    /// `validation` contains backend TLS validation configuration.
    pub validation: BackendTlsPolicyValidation,
    /// `options` are a list of key/value pairs to enable extended TLS
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
    pub options: Option<std::collections::BTreeMap<AnnotationKey, AnnotationValue>>,
}
impl BackendTlsPolicySpec {
    /// Creates a `BackendTlsPolicySpec` from its required fields, with the optional fields `None`.
    pub fn new(
        target_refs: impl Into<Vec<LocalPolicyTargetReferenceWithSectionName>>,
        validation: impl Into<BackendTlsPolicyValidation>,
    ) -> Self {
        Self {
            target_refs: target_refs.into(),
            validation: validation.into(),
            options: None,
        }
    }
}
/// [`BackendTlsPolicyValidation`] contains backend TLS validation configuration.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct BackendTlsPolicyValidation {
    /// `ca_certificate_refs` contains one or more references to Kubernetes objects that
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
    /// `well_known_ca_certificates` specifies whether a well-known set of CA certificates
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
    /// `hostname` is used for two purposes in the connection between Gateways and
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
    /// `subject_alt_names` contains one or more Subject Alternative Names.
    /// When specified the certificate served from the backend MUST
    /// have at least one Subject Alternate Name matching one of the specified SubjectAltNames.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "subjectAltNames")]
    pub subject_alt_names: Option<Vec<SubjectAltName>>,
}
impl BackendTlsPolicyValidation {
    /// Creates a `BackendTlsPolicyValidation` from its required fields, with the optional fields `None`.
    pub fn new(hostname: impl Into<PreciseHostname>) -> Self {
        Self {
            ca_certificate_refs: None,
            well_known_ca_certificates: None,
            hostname: hostname.into(),
            subject_alt_names: None,
        }
    }
}
/// [`SubjectAltName`] represents Subject Alternative Name.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct SubjectAltName {
    /// `type_` determines the format of the Subject Alternative Name. Always required.
    ///
    /// Support: Core
    #[serde(rename = "type")]
    pub type_: SubjectAltNameType,
    /// `hostname` contains Subject Alternative Name specified in DNS name format.
    /// Required when Type is set to Hostname, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<Hostname>,
    /// `uri` contains Subject Alternative Name specified in a full URI format.
    /// It MUST include both a scheme (e.g., "http" or "ftp") and a scheme-specific-part.
    /// Common values include SPIFFE IDs like "spiffe://mycluster.example.com/ns/myns/sa/svc1sa".
    /// Required when Type is set to URI, ignored otherwise.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uri: Option<AbsoluteUri>,
}
impl SubjectAltName {
    /// Creates a `SubjectAltName` from its required fields, with the optional fields `None`.
    pub fn new(type_: impl Into<SubjectAltNameType>) -> Self {
        Self {
            type_: type_.into(),
            hostname: None,
            uri: None,
        }
    }
}
/// [`WellKnownCaCertificatesType`] is the type of CA certificate that will be used
/// when the caCertificateRefs field is unspecified.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum WellKnownCaCertificatesType {
    /// [`WellKnownCaCertificatesType::System`] indicates that well known system CA certificates should be used.
    System,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl WellKnownCaCertificatesType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::System => "System",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for WellKnownCaCertificatesType {
    fn from(value: &str) -> Self {
        match value {
            "System" => Self::System,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for WellKnownCaCertificatesType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "System" => Self::System,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for WellKnownCaCertificatesType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for WellKnownCaCertificatesType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for WellKnownCaCertificatesType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for WellKnownCaCertificatesType {}
impl std::hash::Hash for WellKnownCaCertificatesType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for WellKnownCaCertificatesType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for WellKnownCaCertificatesType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for WellKnownCaCertificatesType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for WellKnownCaCertificatesType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for WellKnownCaCertificatesType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "WellKnownCaCertificatesType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "WellKnownCaCertificatesType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`SubjectAltNameType`] is the type of the Subject Alternative Name.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SubjectAltNameType {
    /// [`SubjectAltNameType::Hostname`] specifies hostname-based SAN.
    ///
    /// Support: Core
    Hostname,
    /// [`SubjectAltNameType::Uri`] specifies URI-based SAN, e.g. SPIFFE id.
    ///
    /// Support: Core
    Uri,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl SubjectAltNameType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Hostname => "Hostname",
            Self::Uri => "URI",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for SubjectAltNameType {
    fn from(value: &str) -> Self {
        match value {
            "Hostname" => Self::Hostname,
            "URI" => Self::Uri,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for SubjectAltNameType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Hostname" => Self::Hostname,
            "URI" => Self::Uri,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for SubjectAltNameType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for SubjectAltNameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for SubjectAltNameType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for SubjectAltNameType {}
impl std::hash::Hash for SubjectAltNameType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for SubjectAltNameType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for SubjectAltNameType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for SubjectAltNameType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SubjectAltNameType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for SubjectAltNameType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SubjectAltNameType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "SubjectAltNameType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string", "enum" : ["Hostname", "URI"] })
    }
}
/// [`GatewaySpec`] defines the desired state of Gateway.
///
/// Not all possible combinations of options specified in the Spec are
/// valid. Some invalid configurations can be caught synchronously via CRD
/// validation, but there are many cases that will require asynchronous
/// signaling via the GatewayStatus block.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    /// `gateway_class_name` used for this Gateway. This is the name of a
    /// GatewayClass resource.
    #[serde(rename = "gatewayClassName")]
    pub gateway_class_name: ObjectName,
    /// `listeners` associated with this Gateway. Listeners define
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
    /// `addresses` requested for this Gateway. This is optional and behavior can
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
    /// `infrastructure` defines infrastructure level attributes about this Gateway instance.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub infrastructure: Option<GatewayInfrastructure>,
    /// `allowed_listeners` defines which ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedListeners")]
    pub allowed_listeners: Option<AllowedListeners>,
    /// `tls` specifies frontend and backend tls configuration for entire gateway.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tls: Option<GatewayTlsConfig>,
    /// `default_scope`, when set, configures the Gateway as a default Gateway,
    /// meaning it will dynamically and implicitly have Routes (e.g. HTTPRoute)
    /// attached to it, according to the scope configured here.
    ///
    /// If unset (the default) or set to None, the Gateway will not act as a
    /// default Gateway; if set, the Gateway will claim any Route with a
    /// matching scope set in its UseDefaultGateway field, subject to the usual
    /// rules about which routes the Gateway can attach to.
    ///
    /// Think carefully before using this functionality! While the normal rules
    /// about which Route can apply are still enforced, it is simply easier for
    /// the wrong Route to be accidentally attached to this Gateway in this
    /// configuration. If the Gateway operator is not also the operator in
    /// control of the scope (e.g. namespace) with tight controls and checks on
    /// what kind of workloads and Routes get added in that scope, we strongly
    /// recommend not using this just because it seems convenient, and instead
    /// stick to direct Route attachment.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "defaultScope")]
    pub default_scope: Option<GatewayDefaultScope>,
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
            default_scope: None,
        }
    }
}
/// [`AllowedListeners`] defines which ListenerSets can be attached to this Gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct AllowedListeners {
    /// `namespaces` defines which namespaces ListenerSets can be attached to this Gateway.
    /// The default value is to allow no ListenerSets.
    ///
    /// Default: `{from: None}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<ListenerNamespaces>,
}
/// [`ListenerNamespaces`] indicate which namespaces ListenerSets should be selected from.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct ListenerNamespaces {
    /// `from` indicates where ListenerSets can attach to this Gateway. Possible
    /// values are:
    ///
    /// * Same: Only ListenerSets in the same namespace may be attached to this Gateway.
    /// * Selector: ListenerSets in namespaces selected by the selector may be attached to this Gateway.
    /// * All: ListenerSets in all namespaces may be attached to this Gateway.
    /// * None: Only listeners defined in the Gateway's spec are allowed
    ///
    /// The default value None
    ///
    /// Default: `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<FromNamespaces>,
    /// `selector` must be specified when From is set to "Selector". In that case,
    /// only ListenerSets in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}
/// [`Listener`] embodies the concept of a logical endpoint where a Gateway accepts
/// network connections.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct Listener {
    /// `name` is the name of the Listener. This name MUST be unique within a
    /// Gateway.
    ///
    /// Support: Core
    pub name: SectionName,
    /// `hostname` specifies the virtual hostname to match for protocol types that
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
    /// `port` is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    ///
    /// Support: Core
    pub port: PortNumber,
    /// `protocol` specifies the network protocol this listener expects to receive.
    ///
    /// Support: Core
    pub protocol: ProtocolType,
    /// `tls` is the TLS configuration for the Listener. This field is required if
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
    /// `allowed_routes` defines the types of routes that MAY be attached to a
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
    ///
    /// Default: `{namespaces:{from: Same}}`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<AllowedRoutes>,
}
impl Listener {
    /// Creates a `Listener` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<SectionName>, port: impl Into<PortNumber>, protocol: impl Into<ProtocolType>) -> Self {
        Self {
            name: name.into(),
            hostname: None,
            port: port.into(),
            protocol: protocol.into(),
            tls: None,
            allowed_routes: None,
        }
    }
}
/// [`ProtocolType`] defines the application protocol accepted by a Listener.
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ProtocolType {
    /// Accepts cleartext HTTP/1.1 sessions over TCP. Implementations MAY also
    /// support HTTP/2 over cleartext. If implementations support HTTP/2 over
    /// cleartext on "HTTP" listeners, that MUST be clearly documented by the
    /// implementation.
    Http,
    /// Accepts HTTP/1.1 or HTTP/2 sessions over TLS.
    Https,
    /// Accepts TLS sessions over TCP.
    Tls,
    /// Accepts TCP sessions.
    Tcp,
    /// Accepts UDP packets.
    Udp,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ProtocolType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Http => "HTTP",
            Self::Https => "HTTPS",
            Self::Tls => "TLS",
            Self::Tcp => "TCP",
            Self::Udp => "UDP",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ProtocolType {
    fn from(value: &str) -> Self {
        match value {
            "HTTP" => Self::Http,
            "HTTPS" => Self::Https,
            "TLS" => Self::Tls,
            "TCP" => Self::Tcp,
            "UDP" => Self::Udp,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ProtocolType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "HTTP" => Self::Http,
            "HTTPS" => Self::Https,
            "TLS" => Self::Tls,
            "TCP" => Self::Tcp,
            "UDP" => Self::Udp,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ProtocolType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ProtocolType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ProtocolType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ProtocolType {}
impl std::hash::Hash for ProtocolType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ProtocolType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ProtocolType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ProtocolType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ProtocolType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ProtocolType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ProtocolType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ProtocolType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`GatewayBackendTls`] describes backend TLS configuration for gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewayBackendTls {
    /// `client_certificate_ref` references an object that contains a client certificate
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
/// [`ListenerTlsConfig`] describes a TLS configuration for a listener.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct ListenerTlsConfig {
    /// `mode` defines the TLS behavior for the TLS session initiated by the client.
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
    ///
    /// Default: `Terminate`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<TlsModeType>,
    /// `certificate_refs` contains a series of references to Kubernetes objects that
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
    /// `options` are a list of key/value pairs to enable extended TLS
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
    pub options: Option<std::collections::BTreeMap<AnnotationKey, AnnotationValue>>,
}
/// [`GatewayTlsConfig`] specifies frontend and backend tls configuration for gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewayTlsConfig {
    /// `backend` describes TLS configuration for gateway when connecting
    /// to backends.
    ///
    /// Note that this contains only details for the Gateway as a TLS client,
    /// and does _not_ imply behavior about how to choose which backend should
    /// get a TLS connection. That is determined by the presence of a BackendTLSPolicy.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backend: Option<GatewayBackendTls>,
    /// `frontend` describes TLS config when client connects to Gateway.
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub frontend: Option<FrontendTlsConfig>,
}
/// [`FrontendTlsConfig`] specifies frontend tls configuration for gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct FrontendTlsConfig {
    /// `default` specifies the default client certificate validation configuration
    /// for all Listeners handling HTTPS traffic, unless a per-port configuration
    /// is defined.
    ///
    /// support: Core
    pub default: TlsConfig,
    /// `per_port` specifies tls configuration assigned per port.
    /// Per port configuration is optional. Once set this configuration overrides
    /// the default configuration for all Listeners handling HTTPS traffic
    /// that match this port.
    /// Each override port requires a unique TLS configuration.
    ///
    /// support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "perPort")]
    pub per_port: Option<Vec<TlsPortConfig>>,
}
impl FrontendTlsConfig {
    /// Creates a `FrontendTlsConfig` from its required fields, with the optional fields `None`.
    pub fn new(default: impl Into<TlsConfig>) -> Self {
        Self {
            default: default.into(),
            per_port: None,
        }
    }
}
/// [`TlsModeType`] type defines how a Gateway handles TLS sessions.
#[derive(Clone, Debug)]
#[non_exhaustive]
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
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl TlsModeType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Terminate => "Terminate",
            Self::Passthrough => "Passthrough",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for TlsModeType {
    fn from(value: &str) -> Self {
        match value {
            "Terminate" => Self::Terminate,
            "Passthrough" => Self::Passthrough,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for TlsModeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Terminate" => Self::Terminate,
            "Passthrough" => Self::Passthrough,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for TlsModeType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for TlsModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for TlsModeType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for TlsModeType {}
impl std::hash::Hash for TlsModeType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for TlsModeType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for TlsModeType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for TlsModeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for TlsModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for TlsModeType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "TlsModeType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "TlsModeType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Terminate", "Passthrough"] }
        )
    }
}
impl Default for TlsModeType {
    fn default() -> Self {
        Self::Terminate
    }
}
/// [`TlsConfig`] describes TLS configuration that can apply to multiple Listeners
/// within this Gateway. Currently, it stores only the client certificate validation
/// configuration, but this may be extended in the future.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct TlsConfig {
    /// `validation` holds configuration information for validating the frontend (client).
    /// Setting this field will result in mutual authentication when connecting to the gateway.
    /// In browsers this may result in a dialog appearing
    /// that requests a user to specify the client certificate.
    /// The maximum depth of a certificate chain accepted in verification is Implementation specific.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validation: Option<FrontendTlsValidation>,
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TlsPortConfig {
    /// The Port indicates the Port Number to which the TLS configuration will be
    /// applied. This configuration will be applied to all Listeners handling HTTPS
    /// traffic that match this port.
    ///
    /// Support: Core
    pub port: PortNumber,
    /// `tls` store the configuration that will be applied to all Listeners handling
    /// HTTPS traffic and matching given port.
    ///
    /// Support: Core
    pub tls: TlsConfig,
}
impl TlsPortConfig {
    /// Creates a `TlsPortConfig` from its required fields, with the optional fields `None`.
    pub fn new(port: impl Into<PortNumber>, tls: impl Into<TlsConfig>) -> Self {
        Self {
            port: port.into(),
            tls: tls.into(),
        }
    }
}
/// [`FrontendTlsValidation`] holds configuration information that can be used to validate
/// the frontend initiating the TLS connection
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct FrontendTlsValidation {
    /// `ca_certificate_refs` contains one or more references to Kubernetes
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
    ///
    /// Default: `AllowValidOnly`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mode: Option<FrontendValidationModeType>,
}
impl FrontendTlsValidation {
    /// Creates a `FrontendTlsValidation` from its required fields, with the optional fields `None`.
    pub fn new(ca_certificate_refs: impl Into<Vec<ObjectReference>>) -> Self {
        Self {
            ca_certificate_refs: ca_certificate_refs.into(),
            mode: None,
        }
    }
}
/// [`FrontendValidationModeType`] type defines how a Gateway validates client certificates.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum FrontendValidationModeType {
    /// [`FrontendValidationModeType::AllowValidOnly`] indicates that a client certificate is required
    /// during the TLS handshake and MUST pass validation.
    ///
    /// Support: Core
    AllowValidOnly,
    /// [`FrontendValidationModeType::AllowInsecureFallback`] indicates that a client certificate may not be
    /// presented during the handshake or the validation against CA certificates may fail.
    ///
    /// Support: Extended
    AllowInsecureFallback,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl FrontendValidationModeType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AllowValidOnly => "AllowValidOnly",
            Self::AllowInsecureFallback => "AllowInsecureFallback",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for FrontendValidationModeType {
    fn from(value: &str) -> Self {
        match value {
            "AllowValidOnly" => Self::AllowValidOnly,
            "AllowInsecureFallback" => Self::AllowInsecureFallback,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for FrontendValidationModeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "AllowValidOnly" => Self::AllowValidOnly,
            "AllowInsecureFallback" => Self::AllowInsecureFallback,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for FrontendValidationModeType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for FrontendValidationModeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for FrontendValidationModeType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FrontendValidationModeType {}
impl std::hash::Hash for FrontendValidationModeType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for FrontendValidationModeType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for FrontendValidationModeType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for FrontendValidationModeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FrontendValidationModeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for FrontendValidationModeType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FrontendValidationModeType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "FrontendValidationModeType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["AllowValidOnly", "AllowInsecureFallback"] }
        )
    }
}
impl Default for FrontendValidationModeType {
    fn default() -> Self {
        Self::AllowValidOnly
    }
}
/// [`AllowedRoutes`] defines which Routes may be attached to this Listener.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct AllowedRoutes {
    /// `namespaces` indicates namespaces from which Routes may be attached to this
    /// Listener. This is restricted to the namespace of this Gateway by default.
    ///
    /// Support: Core
    ///
    /// Default: `{from: Same}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespaces: Option<RouteNamespaces>,
    /// `kinds` specifies the groups and kinds of Routes that are allowed to bind
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
/// [`FromNamespaces`] specifies namespace from which Routes/ListenerSets may be attached to a
/// Gateway.
#[derive(Clone, Debug)]
#[non_exhaustive]
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
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl FromNamespaces {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "All",
            Self::Selector => "Selector",
            Self::Same => "Same",
            Self::None => "None",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for FromNamespaces {
    fn from(value: &str) -> Self {
        match value {
            "All" => Self::All,
            "Selector" => Self::Selector,
            "Same" => Self::Same,
            "None" => Self::None,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for FromNamespaces {
    fn from(value: String) -> Self {
        match value.as_str() {
            "All" => Self::All,
            "Selector" => Self::Selector,
            "Same" => Self::Same,
            "None" => Self::None,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for FromNamespaces {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for FromNamespaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for FromNamespaces {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for FromNamespaces {}
impl std::hash::Hash for FromNamespaces {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for FromNamespaces {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for FromNamespaces {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for FromNamespaces {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for FromNamespaces {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for FromNamespaces {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "FromNamespaces".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "FromNamespaces").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["All", "Selector", "Same", "None"] }
        )
    }
}
/// [`RouteNamespaces`] indicate which namespaces Routes should be selected from.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct RouteNamespaces {
    /// `from` indicates where Routes will be selected for this Gateway. Possible
    /// values are:
    ///
    /// * All: Routes in all namespaces may be used by this Gateway.
    /// * Selector: Routes in namespaces selected by the selector may be used by
    ///   this Gateway.
    /// * Same: Only Routes in the same namespace may be used by this Gateway.
    ///
    /// Support: Core
    ///
    /// Default: `Same`.
    ///
    /// Allowed values here: `All`, `Selector`, `Same`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("enum" = ["All", "Selector", "Same"]))]
    pub from: Option<FromNamespaces>,
    /// `selector` must be specified when From is set to "Selector". In that case,
    /// only Routes in Namespaces matching this Selector will be selected by this
    /// Gateway. This field is ignored for other values of "From".
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selector: Option<LabelSelector>,
}
/// [`RouteGroupKind`] indicates the group and kind of a Route resource.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct RouteGroupKind {
    /// `group` is the group of the Route.
    ///
    /// Default: `gateway.networking.k8s.io`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is the kind of the Route.
    pub kind: Kind,
}
impl RouteGroupKind {
    /// Creates a `RouteGroupKind` from its required fields, with the optional fields `None`.
    pub fn new(kind: impl Into<Kind>) -> Self {
        Self {
            group: None,
            kind: kind.into(),
        }
    }
}
/// [`GatewaySpecAddress`] describes an address that can be bound to a Gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewaySpecAddress {
    /// `type_` of the address.
    ///
    /// Default: `IPAddress`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<AddressType>,
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
/// [`GatewayStatusAddress`] describes a network address that is bound to a Gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct GatewayStatusAddress {
    /// `type_` of the address.
    ///
    /// Default: `IPAddress`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<AddressType>,
    /// `value` of the address. The validity of the values will depend
    /// on the type and support by the controller.
    ///
    /// Examples: `1.2.3.4`, `128::1`, `my-ip-address`.
    pub value: String,
}
impl GatewayStatusAddress {
    /// Creates a `GatewayStatusAddress` from its required fields, with the optional fields `None`.
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            type_: None,
            value: value.into(),
        }
    }
}
/// [`GatewayStatus`] defines the observed state of Gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewayStatus {
    /// `addresses` lists the network addresses that have been bound to the
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
    /// `conditions` describe the current conditions of the Gateway.
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
    ///
    /// Default: `{{type: "Accepted", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"},{type: "Programmed", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// `listeners` provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerStatus>>,
    /// `attached_listener_sets` represents the total number of ListenerSets that have been
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
/// [`GatewayInfrastructure`] defines infrastructure level attributes about a Gateway instance.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewayInfrastructure {
    /// `labels` that SHOULD be applied to any resources created in response to this Gateway.
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
    pub labels: Option<std::collections::BTreeMap<LabelKey, LabelValue>>,
    /// `annotations` that SHOULD be applied to any resources created in response to this Gateway.
    ///
    /// For implementations creating other Kubernetes objects, this should be the `metadata.annotations` field on resources.
    /// For other implementations, this refers to any relevant (implementation specific) "annotations" concepts.
    ///
    /// An implementation may chose to add additional implementation-specific annotations as they see fit.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub annotations: Option<std::collections::BTreeMap<AnnotationKey, AnnotationValue>>,
    /// `parameters_ref` is a reference to a resource that contains the configuration
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
/// [`LocalParametersReference`] identifies an API object containing controller-specific
/// configuration resource within the namespace.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalParametersReference {
    /// `group` is the group of the referent.
    pub group: Group,
    /// `kind` is kind of the referent.
    pub kind: Kind,
    /// `name` is the name of the referent.
    pub name: String,
}
impl LocalParametersReference {
    /// Creates a `LocalParametersReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
        }
    }
}
/// [`GatewayConditionType`] is a type of condition associated with a
/// Gateway. This type should be used with the GatewayStatus.Conditions
/// field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GatewayConditionType {
    /// This condition indicates whether a Gateway has generated some
    /// configuration that is assumed to be ready soon in the underlying data
    /// plane.
    ///
    /// It is a positive-polarity summary condition, and so should always be
    /// present on the resource with ObservedGeneration set.
    ///
    /// It should be set to Unknown if the controller performs updates to the
    /// status before it has all the information it needs to be able to determine
    /// if the condition is true.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Programmed"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "Pending"
    /// * "NoResources"
    /// * "AddressNotAssigned"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Programmed,
    /// This condition is true when the Gateway FrontendValidationModeType is
    /// configured to allow insecure fallback behavior.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "ConfigurationChanged"
    ///
    /// This condition is removed as soon as FrontendValidationModeType is changed back to `AllowValidOnly`.
    InsecureFrontendValidationMode,
    /// This condition is true when the controller managing the Gateway is
    /// syntactically and semantically valid enough to produce some configuration
    /// in the underlying data plane. This does not indicate whether or not the
    /// configuration has been propagated to the data plane.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    /// * "ListenersNotValid"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "InvalidParameters"
    /// * "NotReconciled"
    /// * "UnsupportedAddress"
    /// * "ListenersNotValid"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Accepted,
    /// Deprecated: use "Accepted" instead.
    Scheduled,
    /// This condition indicates whether the controller was able to resolve all
    /// the object references for the Gateway that are not part of a specific
    /// Listener configuration, and also provides a positive-polarity summary of
    /// Listener's "ResolvedRefs" condition. This condition does not directly
    /// impact the Gateway's Accepted or Programmed conditions.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "ResolvedRefs"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "RefNotPermitted"
    /// * "InvalidClientCertificateRef"
    /// * "ListenersNotResolved"
    ///
    /// Controllers may raise this condition with other reasons, but should
    /// prefer to use the reasons listed above to improve interoperability.
    ///
    /// Note: This condition is considered Experimental and may change in future
    /// releases of the API.
    ResolvedRefs,
    /// "Ready" is a condition type reserved for future use. It should not be used by implementations.
    ///
    /// If used in the future, "Ready" will represent the final state where all configuration is confirmed good
    /// _and has completely propagated to the data plane_. That is, it is a _guarantee_ that, as soon as something
    /// sees the Condition as `true`, then connections will be correctly routed _immediately_.
    ///
    /// This is a very strong guarantee, and to date no implementation has satisfied it enough to implement it.
    /// This reservation can be discussed in the future if necessary.
    ///
    /// Note: This condition is not really "deprecated", but rather "reserved"; however, deprecated triggers Go linters
    /// to alert about usage.
    ///
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GatewayConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Programmed => "Programmed",
            Self::InsecureFrontendValidationMode => "InsecureFrontendValidationMode",
            Self::Accepted => "Accepted",
            Self::Scheduled => "Scheduled",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::Ready => "Ready",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GatewayConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Programmed" => Self::Programmed,
            "InsecureFrontendValidationMode" => Self::InsecureFrontendValidationMode,
            "Accepted" => Self::Accepted,
            "Scheduled" => Self::Scheduled,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Ready" => Self::Ready,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GatewayConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Programmed" => Self::Programmed,
            "InsecureFrontendValidationMode" => Self::InsecureFrontendValidationMode,
            "Accepted" => Self::Accepted,
            "Scheduled" => Self::Scheduled,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Ready" => Self::Ready,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GatewayConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GatewayConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GatewayConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GatewayConditionType {}
impl std::hash::Hash for GatewayConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GatewayConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GatewayConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GatewayConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatewayConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GatewayConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GatewayConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GatewayConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`GatewayConditionReason`] defines the set of reasons that explain why a
/// particular Gateway condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GatewayConditionReason {
    /// This reason is used with the "Programmed" condition when the condition is
    /// true.
    Programmed,
    /// This reason is used with the "Programmed" and "Accepted" conditions when
    /// the Gateway is syntactically or semantically invalid. For example, this
    /// could include unspecified TLS configuration, or some unrecognized or
    /// invalid values in the TLS configuration.
    Invalid,
    /// This reason is used with the "Programmed" condition when the
    /// Gateway is not scheduled because insufficient infrastructure
    /// resources are available.
    NoResources,
    /// This reason is used with the "Programmed" condition when the underlying
    /// implementation and network have yet to dynamically assign addresses for a
    /// Gateway.
    ///
    /// Some example situations where this reason can be used:
    ///
    ///   * IPAM address exhaustion
    ///   * Address not yet allocated
    ///
    /// When this reason is used the implementation SHOULD provide a clear
    /// message explaining the underlying problem, ideally with some hints as to
    /// what actions can be taken that might resolve the problem.
    AddressNotAssigned,
    /// This reason is used with the "Programmed" condition when the underlying
    /// implementation (and possibly, network) are unable to use an address that
    /// was provided in the Gateway specification.
    ///
    /// Some example situations where this reason can be used:
    ///
    ///   * a named address not being found
    ///   * a provided static address can't be used
    ///   * the address is already in use
    ///
    /// When this reason is used the implementation SHOULD provide prescriptive
    /// information on which address is causing the problem and how to resolve it
    /// in the condition message.
    AddressNotUsable,
    /// This reason is used with the "InsecureFrontendValidationMode" condition when
    /// the FrontendValidationModeType has been changed from `AllowValidOnly` to
    /// `AllowInsecureFallback`, either at the Gateway level or via a per-port override.
    ConfigurationChanged,
    /// This reason is used with the "Accepted" condition when the condition is
    /// True.
    Accepted,
    /// This reason is used with the "Accepted" condition when one or
    /// more Listeners have an invalid or unsupported configuration
    /// and cannot be configured on the Gateway.
    /// This can be the reason when "Accepted" is "True" or "False", depending on whether
    /// the listener being invalid causes the entire Gateway to not be accepted.
    ListenersNotValid,
    /// This reason is used with the "Accepted" and "Programmed"
    /// conditions when the status is "Unknown" and no controller has reconciled
    /// the Gateway.
    Pending,
    /// This reason is used with the "Accepted" condition to indicate that the
    /// Gateway could not be accepted because an address that was provided is a
    /// type which is not supported by the implementation.
    UnsupportedAddress,
    /// This reason is used with the "Accepted" condition when the
    /// Gateway was not accepted because the parametersRef field
    /// was invalid, with more detail in the message.
    InvalidParameters,
    /// This reason is used with the "Scheduled" condition when the condition is
    /// True.
    ///
    /// Deprecated: use the "Accepted" condition with reason "Accepted" instead.
    Scheduled,
    /// Deprecated: Use "Pending" instead.
    NotReconciled,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when the Gateway
    /// has an invalid ClientCertificateRef in its backend TLS configuration.
    /// A ClientCertificateRef is considered invalid when it refers to a
    /// nonexistent or unsupported resource or kind, or when the data within
    /// that resource is malformed.
    /// This reason must be used only when the reference is allowed, either by
    /// referencing an object in the same namespace as the Gateway, or when
    /// a cross-namespace reference has been explicitly allowed by a ReferenceGrant.
    /// If the reference is not allowed, the reason RefNotPermitted must be used
    /// instead.
    InvalidClientCertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when the Gateway
    /// has a top-level backend TLS configuration that references an object in
    /// another namespace, where the object in the other namespace does not have
    /// a ReferenceGrant explicitly allowing the reference.
    RefNotPermitted,
    /// This reason is used with the "ResolvedRefs" condition when one or more
    /// Listeners have their "ResolvedRefs" condition set to false in their status.
    ListenersNotResolved,
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// Deprecated: Ready is reserved for future use
    ListenersNotReady,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GatewayConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Programmed => "Programmed",
            Self::Invalid => "Invalid",
            Self::NoResources => "NoResources",
            Self::AddressNotAssigned => "AddressNotAssigned",
            Self::AddressNotUsable => "AddressNotUsable",
            Self::ConfigurationChanged => "ConfigurationChanged",
            Self::Accepted => "Accepted",
            Self::ListenersNotValid => "ListenersNotValid",
            Self::Pending => "Pending",
            Self::UnsupportedAddress => "UnsupportedAddress",
            Self::InvalidParameters => "InvalidParameters",
            Self::Scheduled => "Scheduled",
            Self::NotReconciled => "NotReconciled",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::InvalidClientCertificateRef => "InvalidClientCertificateRef",
            Self::RefNotPermitted => "RefNotPermitted",
            Self::ListenersNotResolved => "ListenersNotResolved",
            Self::Ready => "Ready",
            Self::ListenersNotReady => "ListenersNotReady",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GatewayConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "Programmed" => Self::Programmed,
            "Invalid" => Self::Invalid,
            "NoResources" => Self::NoResources,
            "AddressNotAssigned" => Self::AddressNotAssigned,
            "AddressNotUsable" => Self::AddressNotUsable,
            "ConfigurationChanged" => Self::ConfigurationChanged,
            "Accepted" => Self::Accepted,
            "ListenersNotValid" => Self::ListenersNotValid,
            "Pending" => Self::Pending,
            "UnsupportedAddress" => Self::UnsupportedAddress,
            "InvalidParameters" => Self::InvalidParameters,
            "Scheduled" => Self::Scheduled,
            "NotReconciled" => Self::NotReconciled,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidClientCertificateRef" => Self::InvalidClientCertificateRef,
            "RefNotPermitted" => Self::RefNotPermitted,
            "ListenersNotResolved" => Self::ListenersNotResolved,
            "Ready" => Self::Ready,
            "ListenersNotReady" => Self::ListenersNotReady,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GatewayConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Programmed" => Self::Programmed,
            "Invalid" => Self::Invalid,
            "NoResources" => Self::NoResources,
            "AddressNotAssigned" => Self::AddressNotAssigned,
            "AddressNotUsable" => Self::AddressNotUsable,
            "ConfigurationChanged" => Self::ConfigurationChanged,
            "Accepted" => Self::Accepted,
            "ListenersNotValid" => Self::ListenersNotValid,
            "Pending" => Self::Pending,
            "UnsupportedAddress" => Self::UnsupportedAddress,
            "InvalidParameters" => Self::InvalidParameters,
            "Scheduled" => Self::Scheduled,
            "NotReconciled" => Self::NotReconciled,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidClientCertificateRef" => Self::InvalidClientCertificateRef,
            "RefNotPermitted" => Self::RefNotPermitted,
            "ListenersNotResolved" => Self::ListenersNotResolved,
            "Ready" => Self::Ready,
            "ListenersNotReady" => Self::ListenersNotReady,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GatewayConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GatewayConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GatewayConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GatewayConditionReason {}
impl std::hash::Hash for GatewayConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GatewayConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GatewayConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GatewayConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatewayConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GatewayConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GatewayConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GatewayConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`ListenerStatus`] is the status associated with a Listener.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ListenerStatus {
    /// `name` is the name of the Listener that this status corresponds to.
    pub name: SectionName,
    /// `supported_kinds` is the list indicating the Kinds supported by this
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
    /// `attached_routes` represents the total number of Routes that have been
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
    /// `conditions` describe the current condition of this listener.
    pub conditions: Vec<Condition>,
}
impl ListenerStatus {
    /// Creates a `ListenerStatus` from its required fields, with the optional fields `None`.
    pub fn new(
        name: impl Into<SectionName>,
        attached_routes: impl Into<i32>,
        conditions: impl Into<Vec<Condition>>,
    ) -> Self {
        Self {
            name: name.into(),
            supported_kinds: None,
            attached_routes: attached_routes.into(),
            conditions: conditions.into(),
        }
    }
}
/// [`ListenerConditionType`] is a type of condition associated with the
/// listener. This type should be used with the ListenerStatus.Conditions
/// field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerConditionType {
    /// This condition indicates that the controller was unable to resolve
    /// conflicting specification requirements for this Listener. If a
    /// Listener is conflicted, its network port should not be configured
    /// on any network elements.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "HostnameConflict"
    /// * "ProtocolConflict"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "NoConflicts"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    /// If this condition is not set, it is assumed that there are no conflicts.
    Conflicted,
    /// This condition indicates that the listener is syntactically and
    /// semantically valid, and that all features used in the listener's spec are
    /// supported.
    ///
    /// In general, a Listener will be marked as Accepted when the supplied
    /// configuration will generate at least some data plane configuration.
    ///
    /// For example, a Listener with an unsupported protocol will never generate
    /// any data plane config, and so will have Accepted set to `false.`
    /// Conversely, a Listener that does not have any Routes will be able to
    /// generate data plane config, and so will have Accepted set to `true`.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "PortUnavailable"
    /// * "UnsupportedProtocol"
    /// * "NoValidCACertificate"
    /// * "UnsupportedValue"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Accepted,
    /// Deprecated: use "Accepted" instead.
    Detached,
    /// This condition indicates whether the controller was able to
    /// resolve all the object references for the Listener.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "ResolvedRefs"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "InvalidCertificateRef"
    /// * "InvalidRouteKinds"
    /// * "RefNotPermitted"
    /// * "InvalidCACertificateRef"
    /// * "InvalidCACertificateKind"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    ResolvedRefs,
    /// This condition indicates whether a Listener has generated some
    /// configuration that will soon be ready in the underlying data plane.
    ///
    /// It is a positive-polarity summary condition, and so should always be
    /// present on the resource with ObservedGeneration set.
    ///
    /// It should be set to Unknown if the controller performs updates to the
    /// status before it has all the information it needs to be able to determine
    /// if the condition is true.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Programmed"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "Pending"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Programmed,
    /// This condition indicates that TLS configuration within this Listener
    /// conflicts with TLS configuration in another Listener on the same port.
    /// This could happen for two reasons:
    ///
    /// 1) Overlapping Hostnames: Listener A matches *.example.com while Listener
    ///    B matches foo.example.com.
    /// B) Overlapping Certificates: Listener A contains a certificate with a
    ///    SAN for *.example.com, while Listener B contains a certificate with a
    ///    SAN for foo.example.com.
    ///
    /// This overlapping TLS configuration can be particularly problematic when
    /// combined with HTTP connection coalescing. When clients reuse connections
    /// using this technique, it can have confusing interactions with Gateway
    /// API, such as TLS configuration for one Listener getting used for a
    /// request reusing an existing connection that would not be used if the same
    /// request was initiating a new connection.
    ///
    /// Controllers MUST detect the presence of overlapping hostnames and MAY
    /// detect the presence of overlapping certificates.
    ///
    /// This condition MUST be set on all Listeners with overlapping TLS config.
    /// For example, consider the following listener - hostname mapping:
    ///
    /// A: foo.example.com
    /// B: foo.example.org
    /// C: *.example.com
    ///
    /// In the above example, Listeners A and C would have overlapping hostnames
    /// and therefore this condition should be set for Listeners A and C, but not
    /// B.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "OverlappingHostnames"
    /// * "OverlappingCertificates"
    ///
    /// If a controller supports checking for both possible reasons and finds
    /// that both are true, it SHOULD set the "OverlappingCertificates" Reason.
    ///
    /// This is a negative polarity condition and MUST NOT be set when it is
    /// False.
    ///
    /// Controllers may raise this condition with other reasons, but should
    /// prefer to use the reasons listed above to improve interoperability.
    OverlappingTlsConfig,
    /// "Ready" is a condition type reserved for future use. It should not be used by implementations.
    /// Note: This condition is not really "deprecated", but rather "reserved"; however, deprecated triggers Go linters
    /// to alert about usage.
    ///
    /// If used in the future, "Ready" will represent the final state where all configuration is confirmed good
    /// _and has completely propagated to the data plane_. That is, it is a _guarantee_ that, as soon as something
    /// sees the Condition as `true`, then connections will be correctly routed _immediately_.
    ///
    /// This is a very strong guarantee, and to date no implementation has satisfied it enough to implement it.
    /// This reservation can be discussed in the future if necessary.
    ///
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Conflicted => "Conflicted",
            Self::Accepted => "Accepted",
            Self::Detached => "Detached",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::Programmed => "Programmed",
            Self::OverlappingTlsConfig => "OverlappingTLSConfig",
            Self::Ready => "Ready",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Conflicted" => Self::Conflicted,
            "Accepted" => Self::Accepted,
            "Detached" => Self::Detached,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Programmed" => Self::Programmed,
            "OverlappingTLSConfig" => Self::OverlappingTlsConfig,
            "Ready" => Self::Ready,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Conflicted" => Self::Conflicted,
            "Accepted" => Self::Accepted,
            "Detached" => Self::Detached,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Programmed" => Self::Programmed,
            "OverlappingTLSConfig" => Self::OverlappingTlsConfig,
            "Ready" => Self::Ready,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerConditionType {}
impl std::hash::Hash for ListenerConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`ListenerConditionReason`] defines the set of reasons that explain
/// why a particular Listener condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerConditionReason {
    /// This reason is used with the "Conflicted" condition when
    /// the Listener conflicts with hostnames in other Listeners. For
    /// example, this reason would be used when multiple Listeners on
    /// the same port use `example.com` in the hostname field.
    HostnameConflict,
    /// This reason is used with the "Conflicted" condition when
    /// multiple Listeners are specified with the same Listener port
    /// number, but have conflicting protocol specifications.
    ProtocolConflict,
    /// This reason is used with the "Conflicted" condition when the condition
    /// is False.
    NoConflicts,
    /// This reason is used with the "Accepted" condition when the condition is
    /// True.
    Accepted,
    /// This reason is used with the "Detached" condition when the condition is
    /// False.
    ///
    /// Deprecated: use the "Accepted" condition with reason "Accepted" instead.
    Attached,
    /// This reason is used with the "Accepted" condition when the Listener
    /// requests a port that cannot be used on the Gateway. This reason could be
    /// used in a number of instances, including:
    ///
    /// * The port is already in use.
    /// * The port is not supported by the implementation.
    PortUnavailable,
    /// This reason is used with the "Accepted" condition when the
    /// Listener could not be attached to be Gateway because its
    /// protocol type is not supported.
    UnsupportedProtocol,
    /// This reason is used with the "Accepted" condition when the
    /// Listener could not resolve the references to any CACertificate used
    /// to configure Gateway's Client Certificate Validation
    NoValidCaCertificate,
    /// This reason is used with the "Accepted" condition when the
    /// Listener uses a value for a field that is not supported by the
    /// implementation
    UnsupportedValue,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when the
    /// Listener has a TLS configuration with at least one TLS CertificateRef
    /// that is invalid or does not exist.
    /// A CertificateRef is considered invalid when it refers to a nonexistent
    /// or unsupported resource or kind, or when the data within that resource
    /// is malformed.
    /// This reason must be used only when the reference is allowed, either by
    /// referencing an object in the same namespace as the Gateway, or when
    /// a cross-namespace reference has been explicitly allowed by a ReferenceGrant.
    /// If the reference is not allowed, the reason RefNotPermitted must be used
    /// instead.
    InvalidCertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when an invalid or
    /// unsupported Route kind is specified by the Listener.
    InvalidRouteKinds,
    /// This reason is used with the "ResolvedRefs" condition when the
    /// Listener has a TLS configuration that references an object in another
    /// namespace, where the object in the other namespace does not have a
    /// ReferenceGrant explicitly allowing the reference.
    RefNotPermitted,
    /// This reason is used with the "ResolvedRefs" condition when one or more
    /// CACertificate References used to configure Client Certificate
    /// validation for Gateway are invalid.
    InvalidCaCertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when one or more
    /// CACertificate References used to configure Client Certificate
    /// validation for Gateway has unknown or unsupported kind.
    InvalidCaCertificateKind,
    /// This reason is used with the "Programmed" condition when the condition is
    /// true.
    Programmed,
    /// This reason is used with the "Ready" and "Programmed" conditions when the
    /// Listener is syntactically or semantically invalid.
    Invalid,
    /// This reason is used with the "Accepted", "Ready" and "Programmed"
    /// conditions when the Listener is either not yet reconciled or not yet not
    /// online and ready to accept client traffic.
    Pending,
    /// This reason is used with the "OverlappingTLSConfig" condition when the
    /// condition is true.
    OverlappingHostnames,
    /// This reason is used with the "OverlappingTLSConfig" condition when the
    /// condition is true.
    OverlappingCertificates,
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::HostnameConflict => "HostnameConflict",
            Self::ProtocolConflict => "ProtocolConflict",
            Self::NoConflicts => "NoConflicts",
            Self::Accepted => "Accepted",
            Self::Attached => "Attached",
            Self::PortUnavailable => "PortUnavailable",
            Self::UnsupportedProtocol => "UnsupportedProtocol",
            Self::NoValidCaCertificate => "NoValidCACertificate",
            Self::UnsupportedValue => "UnsupportedValue",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::InvalidCertificateRef => "InvalidCertificateRef",
            Self::InvalidRouteKinds => "InvalidRouteKinds",
            Self::RefNotPermitted => "RefNotPermitted",
            Self::InvalidCaCertificateRef => "InvalidCACertificateRef",
            Self::InvalidCaCertificateKind => "InvalidCACertificateKind",
            Self::Programmed => "Programmed",
            Self::Invalid => "Invalid",
            Self::Pending => "Pending",
            Self::OverlappingHostnames => "OverlappingHostnames",
            Self::OverlappingCertificates => "OverlappingCertificates",
            Self::Ready => "Ready",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "HostnameConflict" => Self::HostnameConflict,
            "ProtocolConflict" => Self::ProtocolConflict,
            "NoConflicts" => Self::NoConflicts,
            "Accepted" => Self::Accepted,
            "Attached" => Self::Attached,
            "PortUnavailable" => Self::PortUnavailable,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            "NoValidCACertificate" => Self::NoValidCaCertificate,
            "UnsupportedValue" => Self::UnsupportedValue,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCertificateRef" => Self::InvalidCertificateRef,
            "InvalidRouteKinds" => Self::InvalidRouteKinds,
            "RefNotPermitted" => Self::RefNotPermitted,
            "InvalidCACertificateRef" => Self::InvalidCaCertificateRef,
            "InvalidCACertificateKind" => Self::InvalidCaCertificateKind,
            "Programmed" => Self::Programmed,
            "Invalid" => Self::Invalid,
            "Pending" => Self::Pending,
            "OverlappingHostnames" => Self::OverlappingHostnames,
            "OverlappingCertificates" => Self::OverlappingCertificates,
            "Ready" => Self::Ready,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "HostnameConflict" => Self::HostnameConflict,
            "ProtocolConflict" => Self::ProtocolConflict,
            "NoConflicts" => Self::NoConflicts,
            "Accepted" => Self::Accepted,
            "Attached" => Self::Attached,
            "PortUnavailable" => Self::PortUnavailable,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            "NoValidCACertificate" => Self::NoValidCaCertificate,
            "UnsupportedValue" => Self::UnsupportedValue,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCertificateRef" => Self::InvalidCertificateRef,
            "InvalidRouteKinds" => Self::InvalidRouteKinds,
            "RefNotPermitted" => Self::RefNotPermitted,
            "InvalidCACertificateRef" => Self::InvalidCaCertificateRef,
            "InvalidCACertificateKind" => Self::InvalidCaCertificateKind,
            "Programmed" => Self::Programmed,
            "Invalid" => Self::Invalid,
            "Pending" => Self::Pending,
            "OverlappingHostnames" => Self::OverlappingHostnames,
            "OverlappingCertificates" => Self::OverlappingCertificates,
            "Ready" => Self::Ready,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerConditionReason {}
impl std::hash::Hash for ListenerConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`GatewayClassSpec`] reflects the configuration of a class of Gateways.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    /// `controller_name` is the name of the controller that is managing Gateways of
    /// this class. The value of this field MUST be a domain prefixed path.
    ///
    /// Example: "example.net/gateway-controller".
    ///
    /// This field is not mutable and cannot be empty.
    ///
    /// Support: Core
    #[serde(rename = "controllerName")]
    pub controller_name: GatewayController,
    /// `parameters_ref` is a reference to a resource that contains the configuration
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
    /// `description` helps describe a GatewayClass with more details.
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
/// [`ParametersReference`] identifies an API object containing controller-specific
/// configuration resource within the cluster.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ParametersReference {
    /// `group` is the group of the referent.
    pub group: Group,
    /// `kind` is kind of the referent.
    pub kind: Kind,
    /// `name` is the name of the referent.
    pub name: String,
    /// `namespace` is the namespace of the referent.
    /// This field is required when referring to a Namespace-scoped resource and
    /// MUST be unset when referring to a Cluster-scoped resource.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}
impl ParametersReference {
    /// Creates a `ParametersReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<String>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
            namespace: None,
        }
    }
}
/// [`GatewayClassConditionType`] is the type for status conditions on
/// Gateway resources. This type should be used with the
/// GatewayClassStatus.Conditions field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GatewayClassConditionType {
    /// This condition indicates whether the GatewayClass has been accepted by
    /// the controller requested in the `spec.controller` field.
    ///
    /// This condition defaults to Unknown, and MUST be set by a controller when
    /// it sees a GatewayClass using its controller string. The status of this
    /// condition MUST be set to True if the controller will support provisioning
    /// Gateways using this class. Otherwise, this status MUST be set to False.
    /// If the status is set to False, the controller SHOULD set a Message and
    /// Reason as an explanation.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "InvalidParameters"
    /// * "Unsupported"
    /// * "UnsupportedVersion"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers should prefer to use the values of GatewayClassConditionReason
    /// for the corresponding Reason, where appropriate.
    Accepted,
    /// This condition indicates whether the GatewayClass supports the version(s)
    /// of Gateway API CRDs present in the cluster. This condition MUST be set by
    /// a controller when it marks a GatewayClass "Accepted".
    ///
    /// The version of a Gateway API CRD is defined by the
    /// gateway.networking.k8s.io/bundle-version annotation on the CRD. If
    /// implementations detect any Gateway API CRDs that either do not have this
    /// annotation set, or have it set to a version that is not recognized or
    /// supported by the implementation, this condition MUST be set to false.
    ///
    /// Implementations MAY choose to either provide "best effort" support when
    /// an unrecognized CRD version is present. This would be communicated by
    /// setting the "Accepted" condition to true and the "SupportedVersion"
    /// condition to false.
    ///
    /// Alternatively, implementations MAY choose not to support CRDs with
    /// unrecognized versions. This would be communicated by setting the
    /// "Accepted" condition to false with the reason "UnsupportedVersions".
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "SupportedVersion"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "UnsupportedVersion"
    ///
    /// Controllers should prefer to use the values of GatewayClassConditionReason
    /// for the corresponding Reason, where appropriate.
    SupportedVersion,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GatewayClassConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::SupportedVersion => "SupportedVersion",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GatewayClassConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            "SupportedVersion" => Self::SupportedVersion,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GatewayClassConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            "SupportedVersion" => Self::SupportedVersion,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GatewayClassConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GatewayClassConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GatewayClassConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GatewayClassConditionType {}
impl std::hash::Hash for GatewayClassConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GatewayClassConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GatewayClassConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GatewayClassConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatewayClassConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GatewayClassConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GatewayClassConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GatewayClassConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`GatewayClassConditionReason`] defines the set of reasons that explain why a
/// particular GatewayClass condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GatewayClassConditionReason {
    /// This reason is used with the "Accepted" condition when the condition is
    /// true.
    Accepted,
    /// This reason is used with the "Accepted" condition when the GatewayClass
    /// was not accepted because the parametersRef field refers to
    /// * a namespaced resource but the Namespace field is not set, or
    /// * a cluster-scoped resource but the Namespace field is set, or
    /// * a nonexistent object, or
    /// * an unsupported resource or kind, or
    /// * an existing resource but the data within that resource is malformed.
    InvalidParameters,
    /// This reason is used with the "Accepted" condition when the
    /// requested controller has not yet made a decision about whether
    /// to admit the GatewayClass. It is the default Reason on a new
    /// GatewayClass.
    Pending,
    /// This reason is used with the "Accepted" condition when the GatewayClass
    /// was not accepted because the implementation does not support a
    /// user-defined GatewayClass.
    Unsupported,
    /// Deprecated: Use "Pending" instead.
    Waiting,
    /// This reason is used with the "SupportedVersion" condition when the
    /// condition is true.
    SupportedVersion,
    /// This reason is used with the "SupportedVersion" or "Accepted" condition
    /// when the condition is false. A message SHOULD be included in this
    /// condition that includes the detected CRD version(s) present in the
    /// cluster and the CRD version(s) that are supported by the GatewayClass.
    UnsupportedVersion,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GatewayClassConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::InvalidParameters => "InvalidParameters",
            Self::Pending => "Pending",
            Self::Unsupported => "Unsupported",
            Self::Waiting => "Waiting",
            Self::SupportedVersion => "SupportedVersion",
            Self::UnsupportedVersion => "UnsupportedVersion",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GatewayClassConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            "InvalidParameters" => Self::InvalidParameters,
            "Pending" => Self::Pending,
            "Unsupported" => Self::Unsupported,
            "Waiting" => Self::Waiting,
            "SupportedVersion" => Self::SupportedVersion,
            "UnsupportedVersion" => Self::UnsupportedVersion,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GatewayClassConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            "InvalidParameters" => Self::InvalidParameters,
            "Pending" => Self::Pending,
            "Unsupported" => Self::Unsupported,
            "Waiting" => Self::Waiting,
            "SupportedVersion" => Self::SupportedVersion,
            "UnsupportedVersion" => Self::UnsupportedVersion,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GatewayClassConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GatewayClassConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GatewayClassConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GatewayClassConditionReason {}
impl std::hash::Hash for GatewayClassConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GatewayClassConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GatewayClassConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GatewayClassConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatewayClassConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GatewayClassConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GatewayClassConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GatewayClassConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`GatewayClassStatus`] is the current status for the GatewayClass.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GatewayClassStatus {
    /// `conditions` is the current status from the controller for
    /// this GatewayClass.
    ///
    /// Controllers should prefer to publish conditions using values
    /// of GatewayClassConditionType for the type of each Condition.
    ///
    /// Default: `{{type: "Accepted", status: "Unknown", message: "Waiting for controller", reason: "Pending", lastTransitionTime: "1970-01-01T00:00:00Z"}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// `supported_features` is the set of features the GatewayClass support.
    /// It MUST be sorted in ascending alphabetical order by the Name key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedFeatures")]
    pub supported_features: Option<Vec<SupportedFeature>>,
}
/// [`FeatureName`] is used to describe distinct features that are covered by
/// conformance tests.
pub type FeatureName = String;
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct SupportedFeature {
    pub name: FeatureName,
}
impl SupportedFeature {
    /// Creates a `SupportedFeature` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<FeatureName>) -> Self {
        Self { name: name.into() }
    }
}
/// [`GrpcRouteStatus`] defines the observed state of GRPCRoute.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct GrpcRouteStatus {
    /// `parents` is a list of parent resources (usually Gateways) that are
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
impl GrpcRouteStatus {
    /// Creates a `GrpcRouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// [`GrpcRouteSpec`] defines the desired state of GRPCRoute
#[derive(
    kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "GRPCRoute",
    root = "GrpcRoute",
    plural = "grpcroutes"
)]
#[kube(namespaced)]
#[kube(status = "GrpcRouteStatus")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Hostnames", type_ = "string", json_path = ".spec.hostnames"))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "GRPCRoute provides a way to route gRPC requests. This includes the capability\nto match requests by hostname, gRPC service, gRPC method, or HTTP/2 header.\nFilters can be used to specify additional processing steps. Backends specify\nwhere matching requests will be routed.\n\nGRPCRoute falls under extended support within the Gateway API. Within the\nfollowing specification, the word \"MUST\" indicates that an implementation\nsupporting GRPCRoute must conform to the indicated requirement, but an\nimplementation not supporting this route type need not follow the requirement\nunless explicitly indicated.\n\nImplementations supporting `GRPCRoute` with the `HTTPS` `ProtocolType` MUST\naccept HTTP/2 connections without an initial upgrade from HTTP/1.1, i.e. via\nALPN. If the implementation does not support this, then it MUST set the\n\"Accepted\" condition to \"False\" for the affected listener with a reason of\n\"UnsupportedProtocol\".  Implementations MAY also accept HTTP/2 connections\nwith an upgrade from HTTP/1.\n\nImplementations supporting `GRPCRoute` with the `HTTP` `ProtocolType` MUST\nsupport HTTP/2 over cleartext TCP (h2c,\nhttps://www.rfc-editor.org/rfc/rfc7540#section-3.1) without an initial\nupgrade from HTTP/1.1, i.e. with prior knowledge\n(https://www.rfc-editor.org/rfc/rfc7540#section-3.4). If the implementation\ndoes not support this, then it MUST set the \"Accepted\" condition to \"False\"\nfor the affected listener with a reason of \"UnsupportedProtocol\".\nImplementations MAY also accept HTTP/2 connections with an upgrade from\nHTTP/1, i.e. without prior knowledge."
)]
pub struct GrpcRouteSpec {
    /// `parent_refs` references the resources (usually Gateways) that a Route wants
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
    /// `use_default_gateways` indicates the default Gateway scope to use for this
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
    /// `hostnames` defines a set of hostnames to match against the GRPC
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
    /// `rules` are a list of GRPC matchers, filters and actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<GrpcRouteRule>>,
}
/// [`GrpcRouteRule`] defines the semantics for matching a gRPC request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GrpcRouteRule {
    /// `name` is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// `matches` define conditions used for matching the rule against incoming
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
    /// `filters` define the filters that are applied to requests that match
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
    /// `backend_refs` defines the backend(s) where matching requests should be
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
    /// `session_persistence` defines and configures session persistence
    /// for the route rule.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionPersistence")]
    pub session_persistence: Option<SessionPersistence>,
}
/// [`GrpcRouteMatch`] defines the predicate used to match requests to a given
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GrpcRouteMatch {
    /// `method` specifies a gRPC request service/method matcher. If this field is
    /// not specified, all services and methods will match.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<GrpcMethodMatch>,
    /// `headers` specifies gRPC request header matchers. Multiple match values are
    /// ANDed together, meaning, a request MUST match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<GrpcHeaderMatch>>,
}
/// [`GrpcMethodMatch`] describes how to select a gRPC route by matching the gRPC
/// request service and/or method.
///
/// At least one of Service and Method MUST be a non-empty string.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GrpcMethodMatch {
    /// `type_` specifies how to match against the service and/or method.
    /// Support: Core (Exact with service and method specified)
    ///
    /// Support: Implementation-specific (Exact with method specified but no service specified)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Default: `Exact`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GrpcMethodMatchType>,
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
#[derive(Clone, Debug)]
#[non_exhaustive]
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
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GrpcMethodMatchType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "Exact",
            Self::RegularExpression => "RegularExpression",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GrpcMethodMatchType {
    fn from(value: &str) -> Self {
        match value {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GrpcMethodMatchType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GrpcMethodMatchType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GrpcMethodMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GrpcMethodMatchType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GrpcMethodMatchType {}
impl std::hash::Hash for GrpcMethodMatchType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GrpcMethodMatchType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GrpcMethodMatchType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GrpcMethodMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GrpcMethodMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GrpcMethodMatchType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GrpcMethodMatchType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GrpcMethodMatchType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Exact", "RegularExpression"] }
        )
    }
}
impl Default for GrpcMethodMatchType {
    fn default() -> Self {
        Self::Exact
    }
}
/// [`GrpcHeaderMatch`] describes how to select a gRPC route by matching gRPC request
/// headers.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct GrpcHeaderMatch {
    /// `type_` specifies how to match against the value of the header.
    ///
    /// Default: `Exact`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<GrpcHeaderMatchType>,
    /// `name` is the name of the gRPC Header to be matched.
    ///
    /// If multiple entries specify equivalent header names, only the first
    /// entry with an equivalent name MUST be considered for a match. Subsequent
    /// entries with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    pub name: GrpcHeaderName,
    /// `value` is the value of the gRPC Header to be matched.
    pub value: String,
}
impl GrpcHeaderMatch {
    /// Creates a `GrpcHeaderMatch` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<GrpcHeaderName>, value: impl Into<String>) -> Self {
        Self {
            type_: None,
            name: name.into(),
            value: value.into(),
        }
    }
}
/// [`GrpcHeaderMatchType`] specifies the semantics of how GRPC header values should
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GrpcHeaderMatchType {
    Exact,
    RegularExpression,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GrpcHeaderMatchType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "Exact",
            Self::RegularExpression => "RegularExpression",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GrpcHeaderMatchType {
    fn from(value: &str) -> Self {
        match value {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GrpcHeaderMatchType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GrpcHeaderMatchType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GrpcHeaderMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GrpcHeaderMatchType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GrpcHeaderMatchType {}
impl std::hash::Hash for GrpcHeaderMatchType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GrpcHeaderMatchType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GrpcHeaderMatchType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GrpcHeaderMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GrpcHeaderMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GrpcHeaderMatchType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GrpcHeaderMatchType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GrpcHeaderMatchType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Exact", "RegularExpression"] }
        )
    }
}
impl Default for GrpcHeaderMatchType {
    fn default() -> Self {
        Self::Exact
    }
}
pub type GrpcHeaderName = String;
/// [`GrpcRouteFilterType`] identifies a type of GRPCRoute filter.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GrpcRouteFilterType {
    /// [`GrpcRouteFilterType::RequestHeaderModifier`] can be used to add or remove a gRPC
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
    /// [`GrpcRouteFilterType::RequestMirror`] can be used to mirror gRPC requests to a
    /// different backend. The responses from this backend MUST be ignored by
    /// the Gateway.
    ///
    /// Support in GRPCRouteRule: Extended
    ///
    /// Support in GRPCBackendRef: Extended
    RequestMirror,
    /// [`GrpcRouteFilterType::ExtensionRef`] should be used for configuring custom
    /// gRPC filters.
    ///
    /// Support in GRPCRouteRule: Implementation-specific
    ///
    /// Support in GRPCBackendRef: Implementation-specific
    ExtensionRef,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GrpcRouteFilterType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::RequestHeaderModifier => "RequestHeaderModifier",
            Self::ResponseHeaderModifier => "ResponseHeaderModifier",
            Self::RequestMirror => "RequestMirror",
            Self::ExtensionRef => "ExtensionRef",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GrpcRouteFilterType {
    fn from(value: &str) -> Self {
        match value {
            "RequestHeaderModifier" => Self::RequestHeaderModifier,
            "ResponseHeaderModifier" => Self::ResponseHeaderModifier,
            "RequestMirror" => Self::RequestMirror,
            "ExtensionRef" => Self::ExtensionRef,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GrpcRouteFilterType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "RequestHeaderModifier" => Self::RequestHeaderModifier,
            "ResponseHeaderModifier" => Self::ResponseHeaderModifier,
            "RequestMirror" => Self::RequestMirror,
            "ExtensionRef" => Self::ExtensionRef,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GrpcRouteFilterType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GrpcRouteFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GrpcRouteFilterType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GrpcRouteFilterType {}
impl std::hash::Hash for GrpcRouteFilterType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GrpcRouteFilterType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GrpcRouteFilterType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GrpcRouteFilterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GrpcRouteFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GrpcRouteFilterType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GrpcRouteFilterType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GrpcRouteFilterType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["RequestHeaderModifier",
            "ResponseHeaderModifier", "RequestMirror", "ExtensionRef"] }
        )
    }
}
/// [`GrpcRouteFilter`] defines processing steps that must be completed during the
/// request or response lifecycle. GRPCRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct GrpcRouteFilter {
    /// `type_` identifies the type of filter to apply. As with other API fields,
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
    #[serde(rename = "type")]
    pub type_: GrpcRouteFilterType,
    /// `request_header_modifier` defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaderModifier")]
    pub request_header_modifier: Option<HttpHeaderFilter>,
    /// `response_header_modifier` defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderModifier")]
    pub response_header_modifier: Option<HttpHeaderFilter>,
    /// `request_mirror` defines a schema for a filter that mirrors requests.
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
    /// `extension_ref` is an optional, implementation-specific extension to the
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
impl GrpcRouteFilter {
    /// Creates a `GrpcRouteFilter` from its required fields, with the optional fields `None`.
    pub fn new(type_: impl Into<GrpcRouteFilterType>) -> Self {
        Self {
            type_: type_.into(),
            request_header_modifier: None,
            response_header_modifier: None,
            request_mirror: None,
            extension_ref: None,
        }
    }
}
/// [`GrpcBackendRef`] defines how a GRPCRoute forwards a gRPC request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// When the BackendRef points to a Kubernetes Service, implementations SHOULD
/// honor the appProtocol field if it is set for the target Service Port.
///
/// Implementations supporting appProtocol SHOULD recognize the Kubernetes
/// Standard Application Protocols defined in KEP-3726.
///
/// If a Service appProtocol isn't specified, an implementation MAY infer the
/// backend protocol through its own means. Implementations MAY infer the
/// protocol from the Route type referring to the backend Service.
///
/// If a Route is not able to send traffic to the backend using the specified
/// protocol then the backend is considered invalid. Implementations MUST set the
/// "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct GrpcBackendRef {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    ///
    /// Default: `""`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is the Kubernetes resource kind of the referent. For example
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
    ///
    /// Default: `Service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the backend. When unspecified, the local
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
    /// `port` specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// `weight` specifies the proportion of requests forwarded to the referenced
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
    ///
    /// Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// `filters` defined at this level MUST be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in GRPCRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<GrpcRouteFilter>>,
}
impl GrpcBackendRef {
    /// Creates a `GrpcBackendRef` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
            port: None,
            weight: None,
            filters: None,
        }
    }
}
/// [`HttpRouteSpec`] defines the desired state of HTTPRoute
#[derive(
    kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default,
)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    /// `parent_refs` references the resources (usually Gateways) that a Route wants
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
    /// `use_default_gateways` indicates the default Gateway scope to use for this
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
    /// `hostnames` defines a set of hostnames that should match against the HTTP Host
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
    /// `rules` are a list of HTTP matchers, filters and actions.
    ///
    /// Default: `{{matches: {{path: {type: "PathPrefix", value: "/"}}}}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rules: Option<Vec<HttpRouteRule>>,
}
/// [`HttpRouteRule`] defines semantics for matching an HTTP request based on
/// conditions (matches), processing it (filters), and forwarding the request to
/// an API object (backendRefs).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpRouteRule {
    /// `name` is the name of the route rule. This name MUST be unique within a Route if it is set.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// `matches` define conditions used for matching the rule against incoming
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
    ///
    /// Default: `{{path:{ type: "PathPrefix", value: "/"}}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub matches: Option<Vec<HttpRouteMatch>>,
    /// `filters` define the filters that are applied to requests that match
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
    /// `backend_refs` defines the backend(s) where matching requests should be
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
    /// `timeouts` defines the timeouts that can be configured for an HTTP request.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeouts: Option<HttpRouteTimeouts>,
    /// `retry` defines the configuration for when to retry an HTTP request.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub retry: Option<HttpRouteRetry>,
    /// `session_persistence` defines and configures session persistence
    /// for the route rule.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionPersistence")]
    pub session_persistence: Option<SessionPersistence>,
}
/// [`HttpRouteTimeouts`] defines timeouts that can be configured for an HTTPRoute.
/// Timeout values are represented with Gateway API Duration formatting.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpRouteTimeouts {
    /// `request` specifies the maximum duration for a gateway to respond to an HTTP request.
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
    /// `backend_request` specifies a timeout for an individual request from the gateway
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
/// [`HttpRouteRetry`] defines retry configuration for an HTTPRoute.
///
/// Implementations SHOULD retry on connection errors (disconnect, reset, timeout,
/// TCP failure) if a retry stanza is configured.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpRouteRetry {
    /// `codes` defines the HTTP response status codes for which a backend request
    /// should be retried.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub codes: Option<Vec<HttpRouteRetryStatusCode>>,
    /// `attempts` specifies the maximum number of times an individual request
    /// from the gateway to a backend should be retried.
    ///
    /// If the maximum number of retries has been attempted without a successful
    /// response from the backend, the Gateway MUST return an error.
    ///
    /// When this field is unspecified, the number of times to attempt to retry
    /// a backend request is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// `backoff` specifies the minimum duration a Gateway should wait between
    /// retry attempts and is represented in Gateway API Duration formatting.
    ///
    /// For example, setting the `rules[].retry.backoff` field to the value
    /// `100ms` will cause a backend request to first be retried approximately
    /// 100 milliseconds after timing out or receiving a response code configured
    /// to be retriable.
    ///
    /// An implementation MAY use an exponential or alternative backoff strategy
    /// for subsequent retry attempts, MAY cap the maximum backoff duration to
    /// some amount greater than the specified minimum, and MAY add arbitrary
    /// jitter to stagger requests, as long as unsuccessful backend requests are
    /// not retried before the configured minimum duration.
    ///
    /// If a Request timeout (`rules[].timeouts.request`) is configured on the
    /// route, the entire duration of the initial request and any retry attempts
    /// MUST not exceed the Request timeout duration. If any retry attempts are
    /// still in progress when the Request timeout duration has been reached,
    /// these SHOULD be canceled if possible and the Gateway MUST immediately
    /// return a timeout error.
    ///
    /// If a BackendRequest timeout (`rules[].timeouts.backendRequest`) is
    /// configured on the route, any retry attempts which reach the configured
    /// BackendRequest timeout duration without a response SHOULD be canceled if
    /// possible and the Gateway should wait for at least the specified backoff
    /// duration before attempting to retry the backend request again.
    ///
    /// If a BackendRequest timeout is _not_ configured on the route, retry
    /// attempts MAY time out after an implementation default duration, or MAY
    /// remain pending until a configured Request timeout or implementation
    /// default duration for total request time is reached.
    ///
    /// When this field is unspecified, the time to wait between retry attempts
    /// is implementation-specific.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Duration>,
}
/// [`HttpRouteRetryStatusCode`] defines an HTTP response status code for
/// which a backend request should be retried.
///
/// Implementations MUST support the following status codes as retriable:
///
/// * 500
/// * 502
/// * 503
/// * 504
///
/// Implementations MAY support specifying additional discrete values in the
/// 500-599 range.
///
/// Implementations MAY support specifying discrete values in the 400-499 range,
/// which are often inadvisable to retry.
pub type HttpRouteRetryStatusCode = i64;
/// [`PathMatchType`] specifies the semantics of how HTTP paths should be compared.
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
#[derive(Clone, Debug)]
#[non_exhaustive]
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
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl PathMatchType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "Exact",
            Self::PathPrefix => "PathPrefix",
            Self::RegularExpression => "RegularExpression",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for PathMatchType {
    fn from(value: &str) -> Self {
        match value {
            "Exact" => Self::Exact,
            "PathPrefix" => Self::PathPrefix,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for PathMatchType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exact" => Self::Exact,
            "PathPrefix" => Self::PathPrefix,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for PathMatchType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for PathMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for PathMatchType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PathMatchType {}
impl std::hash::Hash for PathMatchType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for PathMatchType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for PathMatchType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for PathMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PathMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for PathMatchType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PathMatchType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "PathMatchType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Exact", "PathPrefix", "RegularExpression"] }
        )
    }
}
impl Default for PathMatchType {
    fn default() -> Self {
        Self::PathPrefix
    }
}
/// [`HttpPathMatch`] describes how to select a HTTP route by matching the HTTP request path.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpPathMatch {
    /// `type_` specifies how to match against the path Value.
    ///
    /// Support: Core (Exact, PathPrefix)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Default: `PathPrefix`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<PathMatchType>,
    /// `value` of the HTTP path to match against.
    ///
    /// Default: `"/"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
/// [`HeaderMatchType`] specifies the semantics of how HTTP header values should be
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HeaderMatchType {
    Exact,
    RegularExpression,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HeaderMatchType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "Exact",
            Self::RegularExpression => "RegularExpression",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HeaderMatchType {
    fn from(value: &str) -> Self {
        match value {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HeaderMatchType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HeaderMatchType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HeaderMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HeaderMatchType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HeaderMatchType {}
impl std::hash::Hash for HeaderMatchType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HeaderMatchType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HeaderMatchType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HeaderMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HeaderMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HeaderMatchType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HeaderMatchType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HeaderMatchType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Exact", "RegularExpression"] }
        )
    }
}
impl Default for HeaderMatchType {
    fn default() -> Self {
        Self::Exact
    }
}
/// [`HttpHeaderName`] is the name of an HTTP header.
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
/// [`HttpHeaderMatch`] describes how to select a HTTP route by matching HTTP request
/// headers.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpHeaderMatch {
    /// `type_` specifies how to match against the value of the header.
    ///
    /// Support: Core (Exact)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Since RegularExpression HeaderMatchType has implementation-specific
    /// conformance, implementations can support POSIX, PCRE or any other dialects
    /// of regular expressions. Please read the implementation's documentation to
    /// determine the supported dialect.
    ///
    /// Default: `Exact`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<HeaderMatchType>,
    /// `name` is the name of the HTTP Header to be matched. Name matching MUST be
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
    /// `value` is the value of HTTP Header to be matched.
    ///
    /// Must consist of printable US-ASCII characters, optionally separated
    /// by single tabs or spaces. See: https://tools.ietf.org/html/rfc7230#section-3.2
    pub value: String,
}
impl HttpHeaderMatch {
    /// Creates a `HttpHeaderMatch` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<HttpHeaderName>, value: impl Into<String>) -> Self {
        Self {
            type_: None,
            name: name.into(),
            value: value.into(),
        }
    }
}
/// [`QueryParamMatchType`] specifies the semantics of how HTTP query parameter
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum QueryParamMatchType {
    Exact,
    RegularExpression,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl QueryParamMatchType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Exact => "Exact",
            Self::RegularExpression => "RegularExpression",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for QueryParamMatchType {
    fn from(value: &str) -> Self {
        match value {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for QueryParamMatchType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Exact" => Self::Exact,
            "RegularExpression" => Self::RegularExpression,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for QueryParamMatchType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for QueryParamMatchType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for QueryParamMatchType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for QueryParamMatchType {}
impl std::hash::Hash for QueryParamMatchType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for QueryParamMatchType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for QueryParamMatchType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for QueryParamMatchType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for QueryParamMatchType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for QueryParamMatchType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "QueryParamMatchType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "QueryParamMatchType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["Exact", "RegularExpression"] }
        )
    }
}
impl Default for QueryParamMatchType {
    fn default() -> Self {
        Self::Exact
    }
}
/// [`HttpQueryParamMatch`] describes how to select a HTTP route by matching HTTP
/// query parameters.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpQueryParamMatch {
    /// `type_` specifies how to match against the value of the query parameter.
    ///
    /// Support: Extended (Exact)
    ///
    /// Support: Implementation-specific (RegularExpression)
    ///
    /// Since RegularExpression QueryParamMatchType has Implementation-specific
    /// conformance, implementations can support POSIX, PCRE or any other
    /// dialects of regular expressions. Please read the implementation's
    /// documentation to determine the supported dialect.
    ///
    /// Default: `Exact`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<QueryParamMatchType>,
    /// `name` is the name of the HTTP query param to be matched. This must be an
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
    /// `value` is the value of HTTP query param to be matched.
    pub value: String,
}
impl HttpQueryParamMatch {
    /// Creates a `HttpQueryParamMatch` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<HttpHeaderName>, value: impl Into<String>) -> Self {
        Self {
            type_: None,
            name: name.into(),
            value: value.into(),
        }
    }
}
/// [`HttpMethod`] describes how to select a HTTP route by matching the HTTP
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HttpMethod {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HttpMethod {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HttpMethod {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HttpMethod {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HttpMethod {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HttpMethod {}
impl std::hash::Hash for HttpMethod {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HttpMethod {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HttpMethod {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HttpMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HttpMethod {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HttpMethod".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HttpMethod").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["GET", "HEAD", "POST", "PUT", "DELETE",
            "CONNECT", "OPTIONS", "TRACE", "PATCH"] }
        )
    }
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HttpMethodWithWildcard {
    Get,
    Head,
    Post,
    Put,
    Delete,
    Connect,
    Options,
    Trace,
    Patch,
    Wildcard,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HttpMethodWithWildcard {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Get => "GET",
            Self::Head => "HEAD",
            Self::Post => "POST",
            Self::Put => "PUT",
            Self::Delete => "DELETE",
            Self::Connect => "CONNECT",
            Self::Options => "OPTIONS",
            Self::Trace => "TRACE",
            Self::Patch => "PATCH",
            Self::Wildcard => "*",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HttpMethodWithWildcard {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            "*" => Self::Wildcard,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HttpMethodWithWildcard {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GET" => Self::Get,
            "HEAD" => Self::Head,
            "POST" => Self::Post,
            "PUT" => Self::Put,
            "DELETE" => Self::Delete,
            "CONNECT" => Self::Connect,
            "OPTIONS" => Self::Options,
            "TRACE" => Self::Trace,
            "PATCH" => Self::Patch,
            "*" => Self::Wildcard,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HttpMethodWithWildcard {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HttpMethodWithWildcard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HttpMethodWithWildcard {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HttpMethodWithWildcard {}
impl std::hash::Hash for HttpMethodWithWildcard {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HttpMethodWithWildcard {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HttpMethodWithWildcard {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HttpMethodWithWildcard {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpMethodWithWildcard {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HttpMethodWithWildcard {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HttpMethodWithWildcard".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HttpMethodWithWildcard").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["GET", "HEAD", "POST", "PUT", "DELETE",
            "CONNECT", "OPTIONS", "TRACE", "PATCH", "*"] }
        )
    }
}
/// [`HttpRouteMatch`] defines the predicate used to match requests to a given
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpRouteMatch {
    /// `path` specifies a HTTP request path matcher. If this field is not
    /// specified, a default prefix match on the "/" path is provided.
    ///
    /// Default: `{type: "PathPrefix", value: "/"}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathMatch>,
    /// `headers` specifies HTTP request header matchers. Multiple match values are
    /// ANDed together, meaning, a request must match all the specified headers
    /// to select the route.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub headers: Option<Vec<HttpHeaderMatch>>,
    /// `query_params` specifies HTTP query parameter matchers. Multiple match
    /// values are ANDed together, meaning, a request must match all the
    /// specified query parameters to select the route.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "queryParams")]
    pub query_params: Option<Vec<HttpQueryParamMatch>>,
    /// `method` specifies HTTP method matcher.
    /// When specified, this route will be matched only if the request has the
    /// specified method.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<HttpMethod>,
}
/// [`HttpRouteFilter`] defines processing steps that must be completed during the
/// request or response lifecycle. HTTPRouteFilters are meant as an extension
/// point to express processing that may be done in Gateway implementations. Some
/// examples include request or response modification, implementing
/// authentication strategies, rate-limiting, and traffic shaping. API
/// guarantee/conformance is defined based on the type of the filter.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpRouteFilter {
    /// `type_` identifies the type of filter to apply. As with other API fields,
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
    #[serde(rename = "type")]
    pub type_: HttpRouteFilterType,
    /// `request_header_modifier` defines a schema for a filter that modifies request
    /// headers.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestHeaderModifier")]
    pub request_header_modifier: Option<HttpHeaderFilter>,
    /// `response_header_modifier` defines a schema for a filter that modifies response
    /// headers.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "responseHeaderModifier")]
    pub response_header_modifier: Option<HttpHeaderFilter>,
    /// `request_mirror` defines a schema for a filter that mirrors requests.
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
    /// `request_redirect` defines a schema for a filter that responds to the
    /// request with an HTTP redirection.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "requestRedirect")]
    pub request_redirect: Option<HttpRequestRedirectFilter>,
    /// `url_rewrite` defines a schema for a filter that modifies a request during forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "urlRewrite")]
    pub url_rewrite: Option<HttpUrlRewriteFilter>,
    /// `cors` defines a schema for a filter that responds to the
    /// cross-origin request based on HTTP response header.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cors: Option<HttpCorsFilter>,
    /// `external_auth` configures settings related to sending request details
    /// to an external auth service. The external service MUST authenticate
    /// the request, and MAY authorize the request as well.
    ///
    /// If there is any problem communicating with the external service,
    /// this filter MUST fail closed.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "externalAuth")]
    pub external_auth: Option<HttpExternalAuthFilter>,
    /// `extension_ref` is an optional, implementation-specific extension to the
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
impl HttpRouteFilter {
    /// Creates a `HttpRouteFilter` from its required fields, with the optional fields `None`.
    pub fn new(type_: impl Into<HttpRouteFilterType>) -> Self {
        Self {
            type_: type_.into(),
            request_header_modifier: None,
            response_header_modifier: None,
            request_mirror: None,
            request_redirect: None,
            url_rewrite: None,
            cors: None,
            external_auth: None,
            extension_ref: None,
        }
    }
}
/// [`HttpRouteFilterType`] identifies a type of HTTPRoute filter.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HttpRouteFilterType {
    /// [`HttpRouteFilterType::RequestHeaderModifier`] can be used to add or remove an HTTP
    /// header from an HTTP request before it is sent to the upstream target.
    ///
    /// Support in HTTPRouteRule: Core
    ///
    /// Support in HTTPBackendRef: Extended
    RequestHeaderModifier,
    /// [`HttpRouteFilterType::ResponseHeaderModifier`] can be used to add or remove an HTTP
    /// header from an HTTP response before it is sent to the client.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    ResponseHeaderModifier,
    /// [`HttpRouteFilterType::RequestRedirect`] can be used to redirect a request to
    /// another location. This filter can also be used for HTTP to HTTPS
    /// redirects. This may not be used on the same Route rule or BackendRef as a
    /// URLRewrite filter.
    ///
    /// Support in HTTPRouteRule: Core
    ///
    /// Support in HTTPBackendRef: Extended
    RequestRedirect,
    /// [`HttpRouteFilterType::UrlRewrite`] can be used to modify a request during
    /// forwarding. At most one of these filters may be used on a Route rule.
    /// This may not be used on the same Route rule or BackendRef as a
    /// RequestRedirect filter.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    UrlRewrite,
    /// [`HttpRouteFilterType::RequestMirror`] can be used to mirror HTTP requests to a
    /// different backend. The responses from this backend MUST be ignored by
    /// the Gateway.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    RequestMirror,
    /// [`HttpRouteFilterType::Cors`] can be used to add CORS headers to an
    /// HTTP response before it is sent to the client.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Support in HTTPBackendRef: Extended
    Cors,
    /// [`HttpRouteFilterType::ExternalAuth`] can be used to configure a Gateway implementation
    /// to call out to an external Auth server, which MUST perform Authentication
    /// and MAY perform Authorization on the matched request before the request
    /// is forwarded to the backend.
    ///
    /// Support in HTTPRouteRule: Extended
    ///
    /// Feature Name: HTTPRouteExternalAuth
    ExternalAuth,
    /// [`HttpRouteFilterType::ExtensionRef`] should be used for configuring custom
    /// HTTP filters.
    ///
    /// Support in HTTPRouteRule: Implementation-specific
    ///
    /// Support in HTTPBackendRef: Implementation-specific
    ExtensionRef,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HttpRouteFilterType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::RequestHeaderModifier => "RequestHeaderModifier",
            Self::ResponseHeaderModifier => "ResponseHeaderModifier",
            Self::RequestRedirect => "RequestRedirect",
            Self::UrlRewrite => "URLRewrite",
            Self::RequestMirror => "RequestMirror",
            Self::Cors => "CORS",
            Self::ExternalAuth => "ExternalAuth",
            Self::ExtensionRef => "ExtensionRef",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HttpRouteFilterType {
    fn from(value: &str) -> Self {
        match value {
            "RequestHeaderModifier" => Self::RequestHeaderModifier,
            "ResponseHeaderModifier" => Self::ResponseHeaderModifier,
            "RequestRedirect" => Self::RequestRedirect,
            "URLRewrite" => Self::UrlRewrite,
            "RequestMirror" => Self::RequestMirror,
            "CORS" => Self::Cors,
            "ExternalAuth" => Self::ExternalAuth,
            "ExtensionRef" => Self::ExtensionRef,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HttpRouteFilterType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "RequestHeaderModifier" => Self::RequestHeaderModifier,
            "ResponseHeaderModifier" => Self::ResponseHeaderModifier,
            "RequestRedirect" => Self::RequestRedirect,
            "URLRewrite" => Self::UrlRewrite,
            "RequestMirror" => Self::RequestMirror,
            "CORS" => Self::Cors,
            "ExternalAuth" => Self::ExternalAuth,
            "ExtensionRef" => Self::ExtensionRef,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HttpRouteFilterType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HttpRouteFilterType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HttpRouteFilterType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HttpRouteFilterType {}
impl std::hash::Hash for HttpRouteFilterType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HttpRouteFilterType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HttpRouteFilterType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HttpRouteFilterType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpRouteFilterType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HttpRouteFilterType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HttpRouteFilterType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HttpRouteFilterType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["RequestHeaderModifier",
            "ResponseHeaderModifier", "RequestRedirect", "URLRewrite", "RequestMirror",
            "CORS", "ExternalAuth", "ExtensionRef"] }
        )
    }
}
/// [`HttpHeader`] represents an HTTP Header name and value as defined by RFC 7230.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpHeader {
    /// `name` is the name of the HTTP Header to be matched. Name matching MUST be
    /// case-insensitive. (See https://tools.ietf.org/html/rfc7230#section-3.2).
    ///
    /// If multiple entries specify equivalent header names, the first entry with
    /// an equivalent name MUST be considered for a match. Subsequent entries
    /// with an equivalent header name MUST be ignored. Due to the
    /// case-insensitivity of header names, "foo" and "Foo" are considered
    /// equivalent.
    pub name: HttpHeaderName,
    /// `value` is the value of HTTP Header to be matched.
    ///
    /// Must consist of printable US-ASCII characters, optionally separated
    /// by single tabs or spaces. See: https://tools.ietf.org/html/rfc7230#section-3.2
    pub value: String,
}
impl HttpHeader {
    /// Creates a `HttpHeader` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<HttpHeaderName>, value: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}
/// [`HttpHeaderFilter`] defines a filter that modifies the headers of an HTTP
/// request or response. Only one action for a given header name is
/// permitted. Filters specifying multiple actions of the same or different
/// type for any one header name are invalid. Configuration to set or add
/// multiple values for a header must use RFC 7230 header value formatting,
/// separating each value with a comma.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpHeaderFilter {
    /// `set` overwrites the request with the given header (name, value)
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
    /// `add` adds the given header(s) (name, value) to the request
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
    /// `remove` the given header(s) from the HTTP request before the action. The
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
/// [`HttpPathModifierType`] defines the type of path redirect or rewrite.
#[derive(Clone, Debug)]
#[non_exhaustive]
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
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HttpPathModifierType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ReplaceFullPath => "ReplaceFullPath",
            Self::ReplacePrefixMatch => "ReplacePrefixMatch",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HttpPathModifierType {
    fn from(value: &str) -> Self {
        match value {
            "ReplaceFullPath" => Self::ReplaceFullPath,
            "ReplacePrefixMatch" => Self::ReplacePrefixMatch,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HttpPathModifierType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ReplaceFullPath" => Self::ReplaceFullPath,
            "ReplacePrefixMatch" => Self::ReplacePrefixMatch,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HttpPathModifierType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HttpPathModifierType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HttpPathModifierType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HttpPathModifierType {}
impl std::hash::Hash for HttpPathModifierType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HttpPathModifierType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HttpPathModifierType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HttpPathModifierType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpPathModifierType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HttpPathModifierType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HttpPathModifierType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HttpPathModifierType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!(
            { "type" : "string", "enum" : ["ReplaceFullPath", "ReplacePrefixMatch"] }
        )
    }
}
/// [`HttpPathModifier`] defines configuration for path modifiers.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpPathModifier {
    /// `type_` defines the type of path modifier. Additional types may be
    /// added in a future release of the API.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    #[serde(rename = "type")]
    pub type_: HttpPathModifierType,
    /// `replace_full_path` specifies the value with which to replace the full path
    /// of a request during a rewrite or redirect.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "replaceFullPath")]
    pub replace_full_path: Option<String>,
    /// `replace_prefix_match` specifies the value with which to replace the prefix
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
impl HttpPathModifier {
    /// Creates a `HttpPathModifier` from its required fields, with the optional fields `None`.
    pub fn new(type_: impl Into<HttpPathModifierType>) -> Self {
        Self {
            type_: type_.into(),
            replace_full_path: None,
            replace_prefix_match: None,
        }
    }
}
/// HTTPRequestRedirect defines a filter that redirects a request. This filter
/// MUST NOT be used on the same Route rule as a HTTPURLRewrite filter.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpRequestRedirectFilter {
    /// `scheme` is the scheme to be used in the value of the `Location` header in
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
    ///
    /// Allowed values here: `http`, `https`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[schemars(extend("enum" = ["http", "https"]))]
    pub scheme: Option<String>,
    /// `hostname` is the hostname to be used in the value of the `Location`
    /// header in the response.
    /// When empty, the hostname in the `Host` header of the request is used.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<PreciseHostname>,
    /// `path` defines parameters used to modify the path of the incoming request.
    /// The modified path is then used to construct the `Location` header. When
    /// empty, the request path is used as-is.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathModifier>,
    /// `port` is the port to be used in the value of the `Location`
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
    /// `status_code` is the HTTP status code to be used in response.
    ///
    /// Note that values may be added to this enum, implementations
    /// must ensure that unknown values will not cause a crash.
    ///
    /// Unknown values here must result in the implementation setting the
    /// Accepted Condition for the Route to `status: False`, with a
    /// Reason of `UnsupportedValue`.
    ///
    /// Support: Core
    ///
    /// Default: `302`.
    ///
    /// Allowed values here: `301`, `302`, `303`, `307`, `308`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "statusCode")]
    #[schemars(extend("enum" = [301, 302, 303, 307, 308]))]
    pub status_code: Option<i64>,
}
/// [`HttpUrlRewriteFilter`] defines a filter that modifies a request during
/// forwarding. At most one of these filters may be used on a Route rule. This
/// MUST NOT be used on the same Route rule as a HTTPRequestRedirect filter.
///
/// Support: Extended
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpUrlRewriteFilter {
    /// `hostname` is the value to be used to replace the Host header value during
    /// forwarding.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hostname: Option<PreciseHostname>,
    /// `path` defines a path rewrite.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<HttpPathModifier>,
}
/// [`HttpRequestMirrorFilter`] defines configuration for the RequestMirror filter.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpRequestMirrorFilter {
    /// `backend_ref` references a resource where mirrored requests are sent.
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
    /// `percent` represents the percentage of requests that should be
    /// mirrored to BackendRef. Its minimum value is 0 (indicating 0% of
    /// requests) and its maximum value is 100 (indicating 100% of requests).
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i32>,
    /// `fraction` represents the fraction of requests that should be
    /// mirrored to BackendRef.
    ///
    /// Only one of Fraction or Percent may be specified. If neither field
    /// is specified, 100% of requests will be mirrored.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fraction: Option<Fraction>,
}
impl HttpRequestMirrorFilter {
    /// Creates a `HttpRequestMirrorFilter` from its required fields, with the optional fields `None`.
    pub fn new(backend_ref: impl Into<BackendObjectReference>) -> Self {
        Self {
            backend_ref: backend_ref.into(),
            percent: None,
            fraction: None,
        }
    }
}
/// [`HttpCorsFilter`] defines a filter that that configures Cross-Origin Request
/// Sharing (CORS).
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpCorsFilter {
    /// `allow_origins` indicates whether the response can be shared with requested
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
    /// `allow_credentials` indicates whether the actual cross-origin request allows
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
    /// `allow_methods` indicates which HTTP methods are supported for accessing the
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
    /// `allow_headers` indicates which HTTP request headers are supported for
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
    /// `expose_headers` indicates which HTTP response headers can be exposed
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
    /// `max_age` indicates the duration (in seconds) for the client to cache the
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
    ///
    /// Default: `5`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxAge")]
    pub max_age: Option<i32>,
}
/// [`HttpRouteExternalAuthProtocol`] specifies what protocol should be used
/// for communicating with an external authorization server.
///
/// Valid values are supplied as constants below.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HttpRouteExternalAuthProtocol {
    Grpc,
    Http,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl HttpRouteExternalAuthProtocol {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Grpc => "GRPC",
            Self::Http => "HTTP",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for HttpRouteExternalAuthProtocol {
    fn from(value: &str) -> Self {
        match value {
            "GRPC" => Self::Grpc,
            "HTTP" => Self::Http,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for HttpRouteExternalAuthProtocol {
    fn from(value: String) -> Self {
        match value.as_str() {
            "GRPC" => Self::Grpc,
            "HTTP" => Self::Http,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for HttpRouteExternalAuthProtocol {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for HttpRouteExternalAuthProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for HttpRouteExternalAuthProtocol {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for HttpRouteExternalAuthProtocol {}
impl std::hash::Hash for HttpRouteExternalAuthProtocol {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for HttpRouteExternalAuthProtocol {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for HttpRouteExternalAuthProtocol {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for HttpRouteExternalAuthProtocol {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for HttpRouteExternalAuthProtocol {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for HttpRouteExternalAuthProtocol {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "HttpRouteExternalAuthProtocol".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "HttpRouteExternalAuthProtocol").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string", "enum" : ["GRPC", "HTTP"] })
    }
}
/// [`HttpExternalAuthFilter`] defines a filter that modifies requests by sending
/// request details to an external authorization server.
///
/// Support: Extended
/// Feature Name: HTTPRouteExternalAuth
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpExternalAuthFilter {
    /// `protocol` describes which protocol to use when communicating with an
    /// ext_authz authorization server.
    ///
    /// When this is set to GRPC, each backend must use the Envoy ext_authz protocol
    /// on the port specified in `backendRefs`. Requests and responses are defined
    /// in the protobufs explained at:
    /// https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/auth/v3/external_auth.proto
    ///
    /// When this is set to HTTP, each backend must respond with a `200` status
    /// code in on a successful authorization. Any other code is considered
    /// an authorization failure.
    ///
    /// Feature Names:
    /// GRPC Support - HTTPRouteExternalAuthGRPC
    /// HTTP Support - HTTPRouteExternalAuthHTTP
    pub protocol: HttpRouteExternalAuthProtocol,
    /// `backend_ref` is a reference to a backend to send authorization
    /// requests to.
    ///
    /// The backend must speak the selected protocol (GRPC or HTTP) on the
    /// referenced port.
    ///
    /// If the backend service requires TLS, use BackendTLSPolicy to tell the
    /// implementation to supply the TLS details to be used to connect to that
    /// backend.
    #[serde(rename = "backendRef")]
    pub backend_ref: BackendObjectReference,
    /// `grpc` contains configuration for communication with ext_authz
    /// protocol-speaking backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub grpc: Option<GrpcAuthConfig>,
    /// `http` contains configuration for communication with HTTP-speaking
    /// backends.
    ///
    /// If unset, implementations must assume the default behavior for each
    /// included field is intended.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub http: Option<HttpAuthConfig>,
    /// `forward_body` controls if requests to the authorization server should include
    /// the body of the client request; and if so, how big that body is allowed
    /// to be.
    ///
    /// It is expected that implementations will buffer the request body up to
    /// `forwardBody.maxSize` bytes. Bodies over that size must be rejected with a
    /// 4xx series error (413 or 403 are common examples), and fail processing
    /// of the filter.
    ///
    /// If unset, or `forwardBody.maxSize` is set to `0`, then the body will not
    /// be forwarded.
    ///
    /// Feature Name: HTTPRouteExternalAuthForwardBody
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "forwardBody")]
    pub forward_body: Option<ForwardBodyConfig>,
}
impl HttpExternalAuthFilter {
    /// Creates a `HttpExternalAuthFilter` from its required fields, with the optional fields `None`.
    pub fn new(
        protocol: impl Into<HttpRouteExternalAuthProtocol>,
        backend_ref: impl Into<BackendObjectReference>,
    ) -> Self {
        Self {
            protocol: protocol.into(),
            backend_ref: backend_ref.into(),
            grpc: None,
            http: None,
            forward_body: None,
        }
    }
}
/// [`GrpcAuthConfig`] contains configuration for communication with Auth server
/// backends that speak Envoy's ext_authz gRPC protocol.
///
/// Requests and responses are defined in the protobufs explained at:
/// https://www.envoyproxy.io/docs/envoy/latest/api-v3/service/auth/v3/external_auth.proto
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct GrpcAuthConfig {
    /// `allowed_headers` specifies what headers from the client request
    /// will be sent to the authorization server.
    ///
    /// If this list is empty, then all headers must be sent.
    ///
    /// If the list has entries, only those entries must be sent.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedHeaders")]
    pub allowed_headers: Option<Vec<String>>,
}
/// [`HttpAuthConfig`] contains configuration for communication with HTTP-speaking
/// backends.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct HttpAuthConfig {
    /// `path` sets the prefix that paths from the client request will have added
    /// when forwarded to the authorization server.
    ///
    /// When empty or unspecified, no prefix is added.
    ///
    /// Valid values are the same as the "value" regex for path values in the `match`
    /// stanza, and the validation regex will screen out invalid paths in the same way.
    /// Even with the validation, implementations MUST sanitize this input before using it
    /// directly.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// `allowed_headers` specifies what additional headers from the client request
    /// will be sent to the authorization server.
    ///
    /// The following headers must always be sent to the authorization server,
    /// regardless of this setting:
    ///
    /// * `Host`
    /// * `Method`
    /// * `Path`
    /// * `Content-Length`
    /// * `Authorization`
    ///
    /// If this list is empty, then only those headers must be sent.
    ///
    /// Note that `Content-Length` has a special behavior, in that the length
    /// sent must be correct for the actual request to the external authorization
    /// server - that is, it must reflect the actual number of bytes sent in the
    /// body of the request to the authorization server.
    ///
    /// So if the `forwardBody` stanza is unset, or `forwardBody.maxSize` is set
    /// to `0`, then `Content-Length` must be `0`. If `forwardBody.maxSize` is set
    /// to anything other than `0`, then the `Content-Length` of the authorization
    /// request must be set to the actual number of bytes forwarded.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedHeaders")]
    pub allowed_headers: Option<Vec<String>>,
    /// `allowed_response_headers` specifies what headers from the authorization response
    /// will be copied into the request to the backend.
    ///
    /// If this list is empty, then all headers from the authorization server
    /// except Authority or Host must be copied.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedResponseHeaders")]
    pub allowed_response_headers: Option<Vec<String>>,
}
/// ForwardBody configures if requests to the authorization server should include
/// the body of the client request; and if so, how big that body is allowed
/// to be.
///
/// If empty or unset, do not forward the body.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct ForwardBodyConfig {
    /// `max_size` specifies how large in bytes the largest body that will be buffered
    /// and sent to the authorization server. If the body size is larger than
    /// `maxSize`, then the body sent to the authorization server must be
    /// truncated to `maxSize` bytes.
    ///
    /// Experimental note: This behavior needs to be checked against
    /// various dataplanes; it may need to be changed.
    /// See https://github.com/kubernetes-sigs/gateway-api/pull/4001#discussion_r2291405746
    /// for more.
    ///
    /// If 0, the body will not be sent to the authorization server.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "maxSize")]
    pub max_size: Option<u16>,
}
/// [`HttpBackendRef`] defines how a HTTPRoute forwards a HTTP request.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// When the BackendRef points to a Kubernetes Service, implementations SHOULD
/// honor the appProtocol field if it is set for the target Service Port.
///
/// Implementations supporting appProtocol SHOULD recognize the Kubernetes
/// Standard Application Protocols defined in KEP-3726.
///
/// If a Service appProtocol isn't specified, an implementation MAY infer the
/// backend protocol through its own means. Implementations MAY infer the
/// protocol from the Route type referring to the backend Service.
///
/// If a Route is not able to send traffic to the backend using the specified
/// protocol then the backend is considered invalid. Implementations MUST set the
/// "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpBackendRef {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    ///
    /// Default: `""`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is the Kubernetes resource kind of the referent. For example
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
    ///
    /// Default: `Service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the backend. When unspecified, the local
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
    /// `port` specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// `weight` specifies the proportion of requests forwarded to the referenced
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
    ///
    /// Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
    /// `filters` defined at this level should be executed if and only if the
    /// request is being forwarded to the backend defined here.
    ///
    /// Support: Implementation-specific (For broader support of filters, use the
    /// Filters field in HTTPRouteRule.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub filters: Option<Vec<HttpRouteFilter>>,
}
impl HttpBackendRef {
    /// Creates a `HttpBackendRef` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
            port: None,
            weight: None,
            filters: None,
        }
    }
}
/// [`HttpRouteStatus`] defines the observed state of HTTPRoute.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct HttpRouteStatus {
    /// `parents` is a list of parent resources (usually Gateways) that are
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
impl HttpRouteStatus {
    /// Creates a `HttpRouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// [`ListenerSetSpec`] defines the desired state of a ListenerSet.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
    kind = "ListenerSet",
    root = "ListenerSet",
    plural = "listenersets"
)]
#[kube(namespaced)]
#[kube(status = "ListenerSetStatus")]
#[kube(shortname = "lset")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(
    name = "Accepted",
    type_ = "string",
    json_path = ".status.conditions[?(@.type==\"Accepted\")].status"
))]
#[kube(printcolumn(
    name = "Programmed",
    type_ = "string",
    json_path = ".status.conditions[?(@.type==\"Programmed\")].status"
))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(
    doc = "ListenerSet defines a set of additional listeners to attach to an existing Gateway.\nThis resource provides a mechanism to merge multiple listeners into a single Gateway.\n\nThe parent Gateway must explicitly allow ListenerSet attachment through its\nAllowedListeners configuration. By default, Gateways do not allow ListenerSet\nattachment.\n\nRoutes can attach to a ListenerSet by specifying it as a parentRef, and can\noptionally target specific listeners using the sectionName field.\n\nPolicy Attachment:\n- Policies that attach to a ListenerSet apply to all listeners defined in that resource\n- Policies do not impact listeners in the parent Gateway\n- Different ListenerSets attached to the same Gateway can have different policies\n- If an implementation cannot apply a policy to specific listeners, it should reject the policy\n\nReferenceGrant Semantics:\n- ReferenceGrants applied to a Gateway are not inherited by child ListenerSets\n- ReferenceGrants applied to a ListenerSet do not grant permission to the parent Gateway's listeners\n- A ListenerSet can reference secrets/backends in its own namespace without a ReferenceGrant\n\nGateway Integration:\n  - The parent Gateway's status will include \"AttachedListenerSets\"\n    which is the count of ListenerSets that have successfully attached to a Gateway\n    A ListenerSet is successfully attached to a Gateway when all the following conditions are met:\n  - The ListenerSet is selected by the Gateway's AllowedListeners field\n  - The ListenerSet has a valid ParentRef selecting the Gateway\n  - The ListenerSet's status has the condition \"Accepted: true\""
)]
pub struct ListenerSetSpec {
    /// `parent_ref` references the Gateway that the listeners are attached to.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentGatewayReference,
    /// `listeners` associated with this ListenerSet. Listeners define
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
impl ListenerSetSpec {
    /// Creates a `ListenerSetSpec` from its required fields, with the optional fields `None`.
    pub fn new(parent_ref: impl Into<ParentGatewayReference>, listeners: impl Into<Vec<ListenerEntry>>) -> Self {
        Self {
            parent_ref: parent_ref.into(),
            listeners: listeners.into(),
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ListenerEntry {
    /// `name` is the name of the Listener. This name MUST be unique within a
    /// ListenerSet.
    ///
    /// Name is not required to be unique across a Gateway and ListenerSets.
    /// Routes can attach to a Listener by having a ListenerSet as a parentRef
    /// and setting the SectionName
    pub name: SectionName,
    /// `hostname` specifies the virtual hostname to match for protocol types that
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
    /// `port` is the network port. Multiple listeners may use the
    /// same port, subject to the Listener compatibility rules.
    pub port: PortNumber,
    /// `protocol` specifies the network protocol this listener expects to receive.
    pub protocol: ProtocolType,
    /// `tls` is the TLS configuration for the Listener. This field is required if
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
    /// `allowed_routes` defines the types of routes that MAY be attached to a
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
    /// Default: `{namespaces:{from: Same}}`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "allowedRoutes")]
    pub allowed_routes: Option<AllowedRoutes>,
}
impl ListenerEntry {
    /// Creates a `ListenerEntry` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<SectionName>, port: impl Into<PortNumber>, protocol: impl Into<ProtocolType>) -> Self {
        Self {
            name: name.into(),
            hostname: None,
            port: port.into(),
            protocol: protocol.into(),
            tls: None,
            allowed_routes: None,
        }
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct ListenerSetStatus {
    /// `conditions` describe the current conditions of the ListenerSet.
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
    ///
    /// Default: `{{type: "Accepted", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"},{type: "Programmed", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// `listeners` provide status for each unique listener port defined in the Spec.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub listeners: Option<Vec<ListenerEntryStatus>>,
}
/// ListenerStatus is the status associated with a Listener.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ListenerEntryStatus {
    /// `name` is the name of the Listener that this status corresponds to.
    pub name: SectionName,
    /// `supported_kinds` is the list indicating the Kinds supported by this
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
    /// `attached_routes` represents the total number of Routes that have been
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
    /// `conditions` describe the current condition of this listener.
    pub conditions: Vec<Condition>,
}
impl ListenerEntryStatus {
    /// Creates a `ListenerEntryStatus` from its required fields, with the optional fields `None`.
    pub fn new(
        name: impl Into<SectionName>,
        attached_routes: impl Into<i32>,
        conditions: impl Into<Vec<Condition>>,
    ) -> Self {
        Self {
            name: name.into(),
            supported_kinds: None,
            attached_routes: attached_routes.into(),
            conditions: conditions.into(),
        }
    }
}
/// [`ListenerSetConditionType`] is a type of condition associated with a
/// ListenerSet. This type should be used with the ListenerSetStatus.Conditions
/// field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerSetConditionType {
    /// This condition indicates whether a ListenerSet has generated some
    /// configuration that is assumed to be ready soon in the underlying data
    /// plane.
    ///
    /// It is a positive-polarity summary condition, and so should always be
    /// present on the resource with ObservedGeneration set.
    ///
    /// It should be set to Unknown if the controller performs updates to the
    /// status before it has all the information it needs to be able to determine
    /// if the condition is true.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Programmed"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "ParentNotProgrammed"
    /// * "ListenersNotValid"
    ///
    /// Additional reasons for this condition to be False are influenced by
    /// child ListenerEntry conditions:
    ///
    /// * "PortUnavailable"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Programmed,
    /// This condition is true when the controller managing the ListenerSet is
    /// syntactically and semantically valid enough to produce some configuration
    /// in the underlying data plane. This does not indicate whether or not the
    /// configuration has been propagated to the data plane.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "NotAllowed"
    /// * "ParentNotAccepted"
    /// * "ListenersNotValid"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Accepted,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerSetConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Programmed => "Programmed",
            Self::Accepted => "Accepted",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerSetConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Programmed" => Self::Programmed,
            "Accepted" => Self::Accepted,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerSetConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Programmed" => Self::Programmed,
            "Accepted" => Self::Accepted,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerSetConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerSetConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerSetConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerSetConditionType {}
impl std::hash::Hash for ListenerSetConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerSetConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerSetConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerSetConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerSetConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerSetConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerSetConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerSetConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`ListenerSetConditionReason`] defines the set of reasons that explain why a
/// particular ListenerSet condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerSetConditionReason {
    /// This reason is used with the "Programmed" condition when the condition is
    /// true.
    Programmed,
    /// This reason is used with the "Accepted" condition when the condition is
    /// True.
    Accepted,
    /// This reason is used with the "Accepted" condition when the
    /// ListenerSet is not allowed to be attached to the Gateway.
    NotAllowed,
    /// This reason is used with the "Accepted" condition when the
    /// parent Gateway is not accepted.
    ParentNotAccepted,
    /// This reason is used with the "Accepted" condition when one or
    /// more Listeners have an invalid or unsupported configuration
    /// and cannot be configured on the ListenerSet.
    /// This can be the reason when "Accepted" is "True" or "False", depending on whether
    /// the listener being invalid causes the entire ListenerSet to not be accepted.
    ListenersNotValid,
    /// This reason is used with the "Programmed" and "Accepted" conditions when
    /// the ListenerSet is syntactically or semantically invalid. For example, this
    /// could include unspecified TLS configuration, or some unrecognized or
    /// invalid values in the TLS configuration.
    Invalid,
    /// This reason is used with the "Accepted" and "Programmed"
    /// conditions when the status is "Unknown" and no controller has reconciled
    /// the ListenerSet.
    Pending,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerSetConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Programmed => "Programmed",
            Self::Accepted => "Accepted",
            Self::NotAllowed => "NotAllowed",
            Self::ParentNotAccepted => "ParentNotAccepted",
            Self::ListenersNotValid => "ListenersNotValid",
            Self::Invalid => "Invalid",
            Self::Pending => "Pending",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerSetConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "Programmed" => Self::Programmed,
            "Accepted" => Self::Accepted,
            "NotAllowed" => Self::NotAllowed,
            "ParentNotAccepted" => Self::ParentNotAccepted,
            "ListenersNotValid" => Self::ListenersNotValid,
            "Invalid" => Self::Invalid,
            "Pending" => Self::Pending,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerSetConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Programmed" => Self::Programmed,
            "Accepted" => Self::Accepted,
            "NotAllowed" => Self::NotAllowed,
            "ParentNotAccepted" => Self::ParentNotAccepted,
            "ListenersNotValid" => Self::ListenersNotValid,
            "Invalid" => Self::Invalid,
            "Pending" => Self::Pending,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerSetConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerSetConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerSetConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerSetConditionReason {}
impl std::hash::Hash for ListenerSetConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerSetConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerSetConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerSetConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerSetConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerSetConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerSetConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerSetConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`ListenerEntryConditionType`] is a type of condition associated with the
/// listener. This type should be used with the ListenerEntryStatus.Conditions
/// field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerEntryConditionType {
    /// This condition indicates that the controller was unable to resolve
    /// conflicting specification requirements for this Listener. If a
    /// Listener is conflicted, its network port should not be configured
    /// on any network elements.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "HostnameConflict"
    /// * "ProtocolConflict"
    /// * "ListenerConflict"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "NoConflicts"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Conflicted,
    /// This condition indicates that the listener is syntactically and
    /// semantically valid, and that all features used in the listener's spec are
    /// supported.
    ///
    /// In general, a Listener will be marked as Accepted when the supplied
    /// configuration will generate at least some data plane configuration.
    ///
    /// For example, a Listener with an unsupported protocol will never generate
    /// any data plane config, and so will have Accepted set to `false.`
    /// Conversely, a Listener that does not have any Routes will be able to
    /// generate data plane config, and so will have Accepted set to `true`.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "PortUnavailable"
    /// * "UnsupportedProtocol"
    /// * "TooManyListeners"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Accepted,
    /// This condition indicates whether the controller was able to
    /// resolve all the object references for the Listener.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "ResolvedRefs"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "InvalidCertificateRef"
    /// * "InvalidRouteKinds"
    /// * "RefNotPermitted"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    ResolvedRefs,
    /// This condition indicates whether a Listener has generated some
    /// configuration that will soon be ready in the underlying data plane.
    ///
    /// It is a positive-polarity summary condition, and so should always be
    /// present on the resource with ObservedGeneration set.
    ///
    /// It should be set to Unknown if the controller performs updates to the
    /// status before it has all the information it needs to be able to determine
    /// if the condition is true.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Programmed"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Invalid"
    /// * "PortUnavailable"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Programmed,
    /// "Ready" is a condition type reserved for future use. It should not be used by implementations.
    /// Note: This condition is not really "deprecated", but rather "reserved"; however, deprecated triggers Go linters
    /// to alert about usage.
    ///
    /// If used in the future, "Ready" will represent the final state where all configuration is confirmed good
    /// _and has completely propagated to the data plane_. That is, it is a _guarantee_ that, as soon as something
    /// sees the Condition as `true`, then connections will be correctly routed _immediately_.
    ///
    /// This is a very strong guarantee, and to date no implementation has satisfied it enough to implement it.
    /// This reservation can be discussed in the future if necessary.
    ///
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerEntryConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Conflicted => "Conflicted",
            Self::Accepted => "Accepted",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::Programmed => "Programmed",
            Self::Ready => "Ready",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerEntryConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Conflicted" => Self::Conflicted,
            "Accepted" => Self::Accepted,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Programmed" => Self::Programmed,
            "Ready" => Self::Ready,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerEntryConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Conflicted" => Self::Conflicted,
            "Accepted" => Self::Accepted,
            "ResolvedRefs" => Self::ResolvedRefs,
            "Programmed" => Self::Programmed,
            "Ready" => Self::Ready,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerEntryConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerEntryConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerEntryConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerEntryConditionType {}
impl std::hash::Hash for ListenerEntryConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerEntryConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerEntryConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerEntryConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerEntryConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerEntryConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerEntryConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerEntryConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`ListenerEntryConditionReason`] defines the set of reasons that explain
/// why a particular ListenerEntry condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum ListenerEntryConditionReason {
    /// This reason is used with the "Conflicted" condition when
    /// the Listener conflicts with hostnames in other Listeners. For
    /// example, this reason would be used when multiple Listeners on
    /// the same port use `example.com` in the hostname field.
    HostnameConflict,
    /// This reason is used with the "Conflicted" condition when
    /// multiple Listeners are specified with the same Listener port
    /// number, but have conflicting protocol specifications.
    ProtocolConflict,
    /// This reason is used with the "Conflicted" condition when the condition
    /// is True.
    ///
    /// Implementations should prioritize surfacing the most specific conflict
    /// reason. For example, if a Listener is conflicted because it has the same
    /// port as another Listener, and it also has the same hostname as another
    /// Listener, the reason should be "ListenerConflict" and not
    /// "HostnameConflict" or "ProtocolConflict".
    ListenerConflict,
    /// This reason is used with the "Accepted" condition when the condition is
    /// True.
    Accepted,
    /// This reason is used with the "Accepted" condition when the
    /// Listener could not be attached to the Gateway because its
    /// protocol type is not supported.
    UnsupportedProtocol,
    /// This reason is used with the "Accepted" condition when the
    /// Listener could not be attached to the Gateway because the Gateway
    /// has too many Listeners.
    TooManyListeners,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when the
    /// Listener has a TLS configuration with at least one TLS CertificateRef
    /// that is invalid or does not exist.
    /// A CertificateRef is considered invalid when it refers to a nonexistent
    /// or unsupported resource or kind, or when the data within that resource
    /// is malformed.
    /// This reason must be used only when the reference is allowed, either by
    /// referencing an object in the same namespace as the ListenerSet, or when
    /// a cross-namespace reference has been explicitly allowed by a ReferenceGrant.
    /// If the reference is not allowed, the reason RefNotPermitted must be used
    /// instead.
    InvalidCertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when an invalid or
    /// unsupported Route kind is specified by the Listener.
    InvalidRouteKinds,
    /// This reason is used with the "ResolvedRefs" condition when the
    /// Listener has a TLS configuration that references an object in another
    /// namespace, where the object in the other namespace does not have a
    /// ReferenceGrant explicitly allowing the reference.
    RefNotPermitted,
    /// This reason is used with the "Programmed" condition when the condition is
    /// true.
    Programmed,
    /// This reason is used with the "Accepted" condition when the Listener
    /// requests a port that cannot be used on the Gateway. This reason could be
    /// used in a number of instances, including:
    ///
    /// * The port is already in use.
    /// * The port is not supported by the implementation.
    /// * The implementation is unable to assign the port to the Listener.
    PortUnavailable,
    /// This reason is used with the "Accepted" and "Programmed"
    /// conditions when the Listener is either not yet reconciled or not yet not
    /// online and ready to accept client traffic.
    Pending,
    /// This reason is used with the "Accepted" and "Programmed" conditions when the
    /// Listener is syntactically or semantically invalid.
    Invalid,
    /// Deprecated: Ready is reserved for future use
    Ready,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl ListenerEntryConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::HostnameConflict => "HostnameConflict",
            Self::ProtocolConflict => "ProtocolConflict",
            Self::ListenerConflict => "ListenerConflict",
            Self::Accepted => "Accepted",
            Self::UnsupportedProtocol => "UnsupportedProtocol",
            Self::TooManyListeners => "TooManyListeners",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::InvalidCertificateRef => "InvalidCertificateRef",
            Self::InvalidRouteKinds => "InvalidRouteKinds",
            Self::RefNotPermitted => "RefNotPermitted",
            Self::Programmed => "Programmed",
            Self::PortUnavailable => "PortUnavailable",
            Self::Pending => "Pending",
            Self::Invalid => "Invalid",
            Self::Ready => "Ready",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for ListenerEntryConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "HostnameConflict" => Self::HostnameConflict,
            "ProtocolConflict" => Self::ProtocolConflict,
            "ListenerConflict" => Self::ListenerConflict,
            "Accepted" => Self::Accepted,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            "TooManyListeners" => Self::TooManyListeners,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCertificateRef" => Self::InvalidCertificateRef,
            "InvalidRouteKinds" => Self::InvalidRouteKinds,
            "RefNotPermitted" => Self::RefNotPermitted,
            "Programmed" => Self::Programmed,
            "PortUnavailable" => Self::PortUnavailable,
            "Pending" => Self::Pending,
            "Invalid" => Self::Invalid,
            "Ready" => Self::Ready,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for ListenerEntryConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "HostnameConflict" => Self::HostnameConflict,
            "ProtocolConflict" => Self::ProtocolConflict,
            "ListenerConflict" => Self::ListenerConflict,
            "Accepted" => Self::Accepted,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            "TooManyListeners" => Self::TooManyListeners,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCertificateRef" => Self::InvalidCertificateRef,
            "InvalidRouteKinds" => Self::InvalidRouteKinds,
            "RefNotPermitted" => Self::RefNotPermitted,
            "Programmed" => Self::Programmed,
            "PortUnavailable" => Self::PortUnavailable,
            "Pending" => Self::Pending,
            "Invalid" => Self::Invalid,
            "Ready" => Self::Ready,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for ListenerEntryConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for ListenerEntryConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for ListenerEntryConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for ListenerEntryConditionReason {}
impl std::hash::Hash for ListenerEntryConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for ListenerEntryConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for ListenerEntryConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for ListenerEntryConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for ListenerEntryConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for ListenerEntryConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "ListenerEntryConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "ListenerEntryConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`LocalObjectReference`] identifies an API object within the namespace of the
/// referrer.
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalObjectReference {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    pub group: Group,
    /// `kind` is kind of the referent. For example "HTTPRoute" or "Service".
    pub kind: Kind,
    /// `name` is the name of the referent.
    pub name: ObjectName,
}
impl LocalObjectReference {
    /// Creates a `LocalObjectReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<ObjectName>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
        }
    }
}
/// [`SecretObjectReference`] identifies an API object including its namespace,
/// defaulting to Secret.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct SecretObjectReference {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    ///
    /// Default: `""`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is kind of the referent. For example "Secret".
    ///
    /// Default: `Secret`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the referenced object. When unspecified, the local
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
impl SecretObjectReference {
    /// Creates a `SecretObjectReference` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
        }
    }
}
/// [`BackendObjectReference`] defines how an ObjectReference that is
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct BackendObjectReference {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    ///
    /// Default: `""`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is the Kubernetes resource kind of the referent. For example
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
    ///
    /// Default: `Service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the backend. When unspecified, the local
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
    /// `port` specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
}
impl BackendObjectReference {
    /// Creates a `BackendObjectReference` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
            port: None,
        }
    }
}
/// [`ObjectReference`] identifies an API object including its namespace.
///
/// The API object must be valid in the cluster; the Group and Kind must
/// be registered in the cluster for this reference to be valid.
///
/// References to objects with invalid Group and Kind are not valid, and must
/// be rejected by the implementation, with appropriate Conditions set
/// on the containing object.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ObjectReference {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When set to the empty string, core API group is inferred.
    pub group: Group,
    /// `kind` is kind of the referent. For example "ConfigMap" or "Service".
    pub kind: Kind,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the referenced object. When unspecified, the local
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
impl ObjectReference {
    /// Creates a `ObjectReference` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, name: impl Into<ObjectName>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: name.into(),
            namespace: None,
        }
    }
}
/// [`LocalPolicyTargetReference`] identifies an API object to apply a direct or
/// inherited policy to. This should be used as part of Policy resources
/// that can target Gateway API resources. For more information on how this
/// policy attachment model works, and a sample Policy resource, refer to
/// the policy attachment documentation for Gateway API.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalPolicyTargetReference {
    /// `group` is the group of the target resource.
    pub group: Group,
    /// `kind` is kind of the target resource.
    pub kind: Kind,
    /// `name` is the name of the target resource.
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
/// [`NamespacedPolicyTargetReference`] identifies an API object to apply a direct or
/// inherited policy to, potentially in a different namespace. This should only
/// be used as part of Policy resources that need to be able to target resources
/// in different namespaces. For more information on how this policy attachment
/// model works, and a sample Policy resource, refer to the policy attachment
/// documentation for Gateway API.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct NamespacedPolicyTargetReference {
    /// `group` is the group of the target resource.
    pub group: Group,
    /// `kind` is kind of the target resource.
    pub kind: Kind,
    /// `name` is the name of the target resource.
    pub name: ObjectName,
    /// `namespace` is the namespace of the referent. When unspecified, the local
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
/// [`LocalPolicyTargetReferenceWithSectionName`] identifies an API object to apply a
/// direct policy to. This should be used as part of Policy resources that can
/// target single resources. For more information on how this policy attachment
/// mode works, and a sample Policy resource, refer to the policy attachment
/// documentation for Gateway API.
///
/// Note: This should only be used for direct policy attachment when references
/// to SectionName are actually needed. In all other cases,
/// LocalPolicyTargetReference should be used.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct LocalPolicyTargetReferenceWithSectionName {
    /// `group` is the group of the target resource.
    pub group: Group,
    /// `kind` is kind of the target resource.
    pub kind: Kind,
    /// `name` is the name of the target resource.
    pub name: ObjectName,
    /// `section_name` is the name of a section within the target resource. When
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
/// [`PolicyConditionType`] is a type of condition for a policy. This type should be
/// used with a Policy resource Status.Conditions field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PolicyConditionType {
    /// This condition indicates whether the controller was able to resolve all
    /// object references for the BackendTLSPolicy.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "ResolvedRefs"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "InvalidCACertificateRef"
    /// * "InvalidKind"
    ///
    /// Controllers may raise this condition with other reasons, but should
    /// prefer to use the reasons listed above to improve interoperability.
    ResolvedRefs,
    /// [`PolicyConditionType::Accepted`] indicates whether the policy has been accepted or
    /// rejected by a targeted resource, and why.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "Conflicted"
    /// * "Invalid"
    /// * "TargetNotFound"
    Accepted,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl PolicyConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::ResolvedRefs => "ResolvedRefs",
            Self::Accepted => "Accepted",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for PolicyConditionType {
    fn from(value: &str) -> Self {
        match value {
            "ResolvedRefs" => Self::ResolvedRefs,
            "Accepted" => Self::Accepted,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for PolicyConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "ResolvedRefs" => Self::ResolvedRefs,
            "Accepted" => Self::Accepted,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for PolicyConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for PolicyConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for PolicyConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PolicyConditionType {}
impl std::hash::Hash for PolicyConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for PolicyConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for PolicyConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for PolicyConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for PolicyConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PolicyConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "PolicyConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`PolicyConditionReason`] is a reason for a policy condition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum PolicyConditionReason {
    /// This reason is used with the "Accepted" condition when it is
    /// set to false because all CACertificateRefs of the
    /// BackendTLSPolicy are invalid.
    NoValidCaCertificate,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// BackendTLSPolicy's CACertificateRefs is invalid.
    /// A CACertificateRef is considered invalid when it refers to a nonexistent
    /// resource or when the data within that resource is malformed.
    InvalidCaCertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// BackendTLSPolicy's CACertificateRefs references an unknown or unsupported
    /// Group and/or Kind.
    InvalidKind,
    /// [`PolicyConditionReason::Accepted`] is used with the "Accepted" condition when the policy
    /// has been accepted by the targeted resource.
    Accepted,
    /// [`PolicyConditionReason::Conflicted`] is used with the "Accepted" condition when the
    /// policy has not been accepted by a targeted resource because there is
    /// another policy that targets the same resource and a merge is not possible.
    Conflicted,
    /// [`PolicyConditionReason::Invalid`] is used with the "Accepted" condition when the policy
    /// is syntactically or semantically invalid.
    Invalid,
    /// [`PolicyConditionReason::TargetNotFound`] is used with the "Accepted" condition when the
    /// policy is attached to an invalid target resource.
    TargetNotFound,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl PolicyConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::NoValidCaCertificate => "NoValidCACertificate",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::InvalidCaCertificateRef => "InvalidCACertificateRef",
            Self::InvalidKind => "InvalidKind",
            Self::Accepted => "Accepted",
            Self::Conflicted => "Conflicted",
            Self::Invalid => "Invalid",
            Self::TargetNotFound => "TargetNotFound",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for PolicyConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "NoValidCACertificate" => Self::NoValidCaCertificate,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCACertificateRef" => Self::InvalidCaCertificateRef,
            "InvalidKind" => Self::InvalidKind,
            "Accepted" => Self::Accepted,
            "Conflicted" => Self::Conflicted,
            "Invalid" => Self::Invalid,
            "TargetNotFound" => Self::TargetNotFound,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for PolicyConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "NoValidCACertificate" => Self::NoValidCaCertificate,
            "ResolvedRefs" => Self::ResolvedRefs,
            "InvalidCACertificateRef" => Self::InvalidCaCertificateRef,
            "InvalidKind" => Self::InvalidKind,
            "Accepted" => Self::Accepted,
            "Conflicted" => Self::Conflicted,
            "Invalid" => Self::Invalid,
            "TargetNotFound" => Self::TargetNotFound,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for PolicyConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for PolicyConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for PolicyConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for PolicyConditionReason {}
impl std::hash::Hash for PolicyConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for PolicyConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for PolicyConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for PolicyConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for PolicyConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for PolicyConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "PolicyConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "PolicyConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`PolicyAncestorStatus`] describes the status of a route with respect to an
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct PolicyAncestorStatus {
    /// `ancestor_ref` corresponds with a ParentRef in the spec that this
    /// PolicyAncestorStatus struct describes the status of.
    #[serde(rename = "ancestorRef")]
    pub ancestor_ref: ParentReference,
    /// `controller_name` is a domain/path string that indicates the name of the
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
    /// `conditions` describes the status of the Policy with respect to the given Ancestor.
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
/// [`PolicyStatus`] defines the common attributes that all Policies should include within
/// their status.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct PolicyStatus {
    /// `ancestors` is a list of ancestor resources (usually Gateways) that are
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
impl PolicyStatus {
    /// Creates a `PolicyStatus` from its required fields, with the optional fields `None`.
    pub fn new(ancestors: impl Into<Vec<PolicyAncestorStatus>>) -> Self {
        Self {
            ancestors: ancestors.into(),
        }
    }
}
/// [`ReferenceGrantSpec`] identifies a cross namespace relationship that is trusted
/// for Gateway API.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    /// `from` describes the trusted namespaces and kinds that can reference the
    /// resources described in "To". Each entry in this list MUST be considered
    /// to be an additional place that references can be valid from, or to put
    /// this another way, entries MUST be combined using OR.
    ///
    /// Support: Core
    pub from: Vec<ReferenceGrantFrom>,
    /// `to` describes the resources that may be referenced by the resources
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
/// [`ReferenceGrantFrom`] describes trusted namespaces and kinds.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ReferenceGrantFrom {
    /// `group` is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,
    /// `kind` is the kind of the referent. Although implementations may support
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
    /// `namespace` is the namespace of the referent.
    ///
    /// Support: Core
    pub namespace: Namespace,
}
impl ReferenceGrantFrom {
    /// Creates a `ReferenceGrantFrom` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>, namespace: impl Into<Namespace>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            namespace: namespace.into(),
        }
    }
}
/// [`ReferenceGrantTo`] describes what Kinds are allowed as targets of the
/// references.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ReferenceGrantTo {
    /// `group` is the group of the referent.
    /// When empty, the Kubernetes core API group is inferred.
    ///
    /// Support: Core
    pub group: Group,
    /// `kind` is the kind of the referent. Although implementations may support
    /// additional resources, the following types are part of the "Core"
    /// support level for this field:
    ///
    /// * Secret when used to permit a SecretObjectReference
    /// * Service when used to permit a BackendObjectReference
    pub kind: Kind,
    /// `name` is the name of the referent. When unspecified, this policy
    /// refers to all resources of the specified Group and Kind in the local
    /// namespace.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<ObjectName>,
}
impl ReferenceGrantTo {
    /// Creates a `ReferenceGrantTo` from its required fields, with the optional fields `None`.
    pub fn new(group: impl Into<Group>, kind: impl Into<Kind>) -> Self {
        Self {
            group: group.into(),
            kind: kind.into(),
            name: None,
        }
    }
}
/// [`ParentReference`] identifies an API object (usually a Gateway) that can be considered
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
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ParentReference {
    /// `group` is the group of the referent.
    /// When unspecified, "gateway.networking.k8s.io" is inferred.
    /// To set the core API group (such as for a "Service" kind referent),
    /// Group must be explicitly set to "" (empty string).
    ///
    /// Support: Core
    ///
    /// Default: `gateway.networking.k8s.io`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is kind of the referent.
    ///
    /// There are two kinds of parent resources with "Core" support:
    ///
    /// * Gateway (Gateway conformance profile)
    /// * Service (Mesh conformance profile, ClusterIP Services only)
    ///
    /// Support for other resources is Implementation-Specific.
    ///
    /// Default: `Gateway`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `namespace` is the namespace of the referent. When unspecified, this refers
    /// to the local namespace of the Route.
    ///
    /// Note that there are specific rules for ParentRefs which cross namespace
    /// boundaries. Cross-namespace references are only valid if they are explicitly
    /// allowed by something in the namespace they are referring to. For example:
    /// Gateway has the AllowedRoutes field, and ReferenceGrant provides a
    /// generic way to enable any other kind of cross-namespace reference.
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
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
    /// `name` is the name of the referent.
    ///
    /// Support: Core
    pub name: ObjectName,
    /// `section_name` is the name of a section within the target resource. In the
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
    /// `port` is the network port this Route targets. It can be interpreted
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
    /// When the parent resource is a Service, this targets a specific port in the
    /// Service spec. When both Port (experimental) and SectionName are specified,
    /// the name and port of the selected port must match both specified values.
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
impl ParentReference {
    /// Creates a `ParentReference` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            namespace: None,
            name: name.into(),
            section_name: None,
            port: None,
        }
    }
}
/// [`GatewayDefaultScope`] defines the set of default scopes that a Gateway
/// can claim, for use in any Route type. At present the only supported
/// scopes are "All" and "None". "None" is a special scope which
/// explicitly means that the Route MUST NOT attached to any default
/// Gateway.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum GatewayDefaultScope {
    /// [`GatewayDefaultScope::All`] indicates that a Gateway can claim absolutely
    /// any Route asking for a default Gateway.
    All,
    /// [`GatewayDefaultScope::None`] indicates that a Gateway MUST NOT claim
    /// any Route asking for a default Gateway.
    None,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl GatewayDefaultScope {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::All => "All",
            Self::None => "None",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for GatewayDefaultScope {
    fn from(value: &str) -> Self {
        match value {
            "All" => Self::All,
            "None" => Self::None,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for GatewayDefaultScope {
    fn from(value: String) -> Self {
        match value.as_str() {
            "All" => Self::All,
            "None" => Self::None,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for GatewayDefaultScope {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for GatewayDefaultScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for GatewayDefaultScope {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for GatewayDefaultScope {}
impl std::hash::Hash for GatewayDefaultScope {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for GatewayDefaultScope {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for GatewayDefaultScope {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for GatewayDefaultScope {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for GatewayDefaultScope {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for GatewayDefaultScope {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GatewayDefaultScope".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "GatewayDefaultScope").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string", "enum" : ["All", "None"] })
    }
}
/// [`CommonRouteSpec`] defines the common attributes that all Routes MUST include
/// within their spec.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct CommonRouteSpec {
    /// `parent_refs` references the resources (usually Gateways) that a Route wants
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
    /// `use_default_gateways` indicates the default Gateway scope to use for this
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
}
/// [`PortNumber`] defines a network port.
pub type PortNumber = i32;
/// [`BackendRef`] defines how a Route should forward a request to a Kubernetes
/// resource.
///
/// Note that when a namespace different than the local namespace is specified, a
/// ReferenceGrant object is required in the referent namespace to allow that
/// namespace's owner to accept the reference. See the ReferenceGrant
/// documentation for details.
///
/// When the BackendRef points to a Kubernetes Service, implementations SHOULD
/// honor the appProtocol field if it is set for the target Service Port.
///
/// Implementations supporting appProtocol SHOULD recognize the Kubernetes
/// Standard Application Protocols defined in KEP-3726.
///
/// If a Service appProtocol isn't specified, an implementation MAY infer the
/// backend protocol through its own means. Implementations MAY infer the
/// protocol from the Route type referring to the backend Service.
///
/// If a Route is not able to send traffic to the backend using the specified
/// protocol then the backend is considered invalid. Implementations MUST set the
/// "ResolvedRefs" condition to "False" with the "UnsupportedProtocol" reason.
///
/// Note that when the BackendTLSPolicy object is enabled by the implementation,
/// there are some extra rules about validity to consider here. See the fields
/// where this struct is used for more information about the exact behavior.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct BackendRef {
    /// `group` is the group of the referent. For example, "gateway.networking.k8s.io".
    /// When unspecified or empty string, core API group is inferred.
    ///
    /// Default: `""`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is the Kubernetes resource kind of the referent. For example
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
    ///
    /// Default: `Service`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the backend. When unspecified, the local
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
    /// `port` specifies the destination port number to use for this resource.
    /// Port is required when the referent is a Kubernetes Service. In this
    /// case, the port number is the service port number, not the target port.
    /// For other resources, destination port might be derived from the referent
    /// resource or this field.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub port: Option<PortNumber>,
    /// `weight` specifies the proportion of requests forwarded to the referenced
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
    ///
    /// Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub weight: Option<i32>,
}
impl BackendRef {
    /// Creates a `BackendRef` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
            port: None,
            weight: None,
        }
    }
}
/// [`RouteConditionType`] is a type of condition for a route.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RouteConditionType {
    /// This condition indicates whether the route has been accepted or rejected
    /// by a Gateway, and why.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "NotAllowedByListeners"
    /// * "NoMatchingListenerHostname"
    /// * "NoMatchingParent"
    /// * "UnsupportedValue"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    Accepted,
    /// This condition indicates whether the controller was able to resolve all
    /// the object references for the Route.
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "ResolvedRefs"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "RefNotPermitted"
    /// * "InvalidKind"
    /// * "BackendNotFound"
    /// * "UnsupportedProtocol"
    ///
    /// Controllers may raise this condition with other reasons,
    /// but should prefer to use the reasons listed above to improve
    /// interoperability.
    ResolvedRefs,
    /// This condition indicates that the Route contains a combination of both
    /// valid and invalid rules.
    ///
    /// When this happens, implementations MUST take one of the following
    /// approaches:
    ///
    /// 1) Drop Rule(s): With this approach, implementations will drop the
    ///    invalid Route Rule(s) until they are fully valid again. The message
    ///    for this condition MUST start with the prefix "Dropped Rule" and
    ///    include information about which Rules have been dropped. In this
    ///    state, the "Accepted" condition MUST be set to "True" with the latest
    ///    generation of the resource.
    /// 2) Fall Back: With this approach, implementations will fall back to the
    ///    last known good state of the entire Route. The message for this
    ///    condition MUST start with the prefix "Fall Back" and include
    ///    information about why the current Rule(s) are invalid. To represent
    ///    this, the "Accepted" condition MUST be set to "True" with the
    ///    generation of the last known good state of the resource.
    ///
    /// Reverting to the last known good state should only be done by
    /// implementations that have a means of restoring that state if/when they
    /// are restarted.
    ///
    /// This condition MUST NOT be set if a Route is fully valid, fully invalid,
    /// or not accepted. By extension, that means that this condition MUST only
    /// be set when it is "True".
    ///
    /// Possible reasons for this condition to be True are:
    ///
    /// * "UnsupportedValue"
    ///
    /// Controllers may raise this condition with other reasons, but should
    /// prefer to use the reasons listed above to improve interoperability.
    PartiallyInvalid,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl RouteConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::PartiallyInvalid => "PartiallyInvalid",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for RouteConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            "ResolvedRefs" => Self::ResolvedRefs,
            "PartiallyInvalid" => Self::PartiallyInvalid,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for RouteConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            "ResolvedRefs" => Self::ResolvedRefs,
            "PartiallyInvalid" => Self::PartiallyInvalid,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for RouteConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for RouteConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for RouteConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for RouteConditionType {}
impl std::hash::Hash for RouteConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for RouteConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for RouteConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for RouteConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RouteConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for RouteConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RouteConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "RouteConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`RouteConditionReason`] is a reason for a route condition.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum RouteConditionReason {
    /// This reason is used with the "Accepted" condition when the Route has been
    /// accepted by the Gateway.
    Accepted,
    /// This reason is used with the "Accepted" condition when the route has not
    /// been accepted by a Gateway because the Gateway has no Listener whose
    /// allowedRoutes criteria permit the route
    NotAllowedByListeners,
    /// This reason is used with the "Accepted" condition when the Gateway has no
    /// compatible Listeners whose Hostname matches the route
    NoMatchingListenerHostname,
    /// This reason is used with the "Accepted" condition when there are
    /// no matching Parents. In the case of Gateways, this can occur when
    /// a Route ParentRef specifies a Port and/or SectionName that does not
    /// match any Listeners in the Gateway.
    NoMatchingParent,
    /// This reason is used with the "Accepted" condition when a value for an Enum
    /// is not recognized.
    UnsupportedValue,
    /// This reason is used with the "Accepted" when a controller has not yet
    /// reconciled the route.
    Pending,
    /// This reason is used with the "Accepted" condition when there
    /// are incompatible filters present on a route rule (for example if
    /// the URLRewrite and RequestRedirect are both present on an HTTPRoute).
    IncompatibleFilters,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when
    /// one of the Listener's Routes has a BackendRef to an object in
    /// another namespace, where the object in the other namespace does
    /// not have a ReferenceGrant explicitly allowing the reference.
    RefNotPermitted,
    /// This reason is used with the "ResolvedRefs" condition when
    /// one of the Route's rules has a reference to an unknown or unsupported
    /// Group and/or Kind.
    InvalidKind,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// Route's rules has a reference to a resource that does not exist.
    BackendNotFound,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// Route's rules has a reference to a resource with an app protocol that
    /// is not supported by this implementation.
    UnsupportedProtocol,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl RouteConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::NotAllowedByListeners => "NotAllowedByListeners",
            Self::NoMatchingListenerHostname => "NoMatchingListenerHostname",
            Self::NoMatchingParent => "NoMatchingParent",
            Self::UnsupportedValue => "UnsupportedValue",
            Self::Pending => "Pending",
            Self::IncompatibleFilters => "IncompatibleFilters",
            Self::ResolvedRefs => "ResolvedRefs",
            Self::RefNotPermitted => "RefNotPermitted",
            Self::InvalidKind => "InvalidKind",
            Self::BackendNotFound => "BackendNotFound",
            Self::UnsupportedProtocol => "UnsupportedProtocol",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for RouteConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            "NotAllowedByListeners" => Self::NotAllowedByListeners,
            "NoMatchingListenerHostname" => Self::NoMatchingListenerHostname,
            "NoMatchingParent" => Self::NoMatchingParent,
            "UnsupportedValue" => Self::UnsupportedValue,
            "Pending" => Self::Pending,
            "IncompatibleFilters" => Self::IncompatibleFilters,
            "ResolvedRefs" => Self::ResolvedRefs,
            "RefNotPermitted" => Self::RefNotPermitted,
            "InvalidKind" => Self::InvalidKind,
            "BackendNotFound" => Self::BackendNotFound,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for RouteConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            "NotAllowedByListeners" => Self::NotAllowedByListeners,
            "NoMatchingListenerHostname" => Self::NoMatchingListenerHostname,
            "NoMatchingParent" => Self::NoMatchingParent,
            "UnsupportedValue" => Self::UnsupportedValue,
            "Pending" => Self::Pending,
            "IncompatibleFilters" => Self::IncompatibleFilters,
            "ResolvedRefs" => Self::ResolvedRefs,
            "RefNotPermitted" => Self::RefNotPermitted,
            "InvalidKind" => Self::InvalidKind,
            "BackendNotFound" => Self::BackendNotFound,
            "UnsupportedProtocol" => Self::UnsupportedProtocol,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for RouteConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for RouteConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for RouteConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for RouteConditionReason {}
impl std::hash::Hash for RouteConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for RouteConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for RouteConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for RouteConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for RouteConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for RouteConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "RouteConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "RouteConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// [`RouteParentStatus`] describes the status of a route with respect to an
/// associated Parent.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct RouteParentStatus {
    /// `parent_ref` corresponds with a ParentRef in the spec that this
    /// RouteParentStatus struct describes the status of.
    #[serde(rename = "parentRef")]
    pub parent_ref: ParentReference,
    /// `controller_name` is a domain/path string that indicates the name of the
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
    /// `conditions` describes the status of the route with respect to the Gateway.
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
    pub conditions: Vec<Condition>,
}
impl RouteParentStatus {
    /// Creates a `RouteParentStatus` from its required fields, with the optional fields `None`.
    pub fn new(
        parent_ref: impl Into<ParentReference>,
        controller_name: impl Into<GatewayController>,
        conditions: impl Into<Vec<Condition>>,
    ) -> Self {
        Self {
            parent_ref: parent_ref.into(),
            controller_name: controller_name.into(),
            conditions: conditions.into(),
        }
    }
}
/// [`RouteStatus`] defines the common attributes that all Routes MUST include within
/// their status.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct RouteStatus {
    /// `parents` is a list of parent resources (usually Gateways) that are
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
impl RouteStatus {
    /// Creates a `RouteStatus` from its required fields, with the optional fields `None`.
    pub fn new(parents: impl Into<Vec<RouteParentStatus>>) -> Self {
        Self {
            parents: parents.into(),
        }
    }
}
/// [`Hostname`] is the fully qualified domain name of a network host. This matches
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
/// [`PreciseHostname`] is the fully qualified domain name of a network host. This
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
/// [`Group`] refers to a Kubernetes Group. It must either be an empty string or a
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
/// [`Kind`] refers to a Kubernetes Kind.
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
/// [`ObjectName`] refers to the name of a Kubernetes object.
/// Object names can have a variety of forms, including RFC 1123 subdomains,
/// RFC 1123 labels, or RFC 1035 labels.
pub type ObjectName = String;
/// [`Namespace`] refers to a Kubernetes namespace. It must be a RFC 1123 label.
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
/// [`SectionName`] is the name of a section in a Kubernetes resource.
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
/// [`GatewayController`] is the name of a Gateway API controller. It must be a
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
/// [`AnnotationKey`] is the key of an annotation in Gateway API. This is used for
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
/// [`AnnotationValue`] is the value of an annotation in Gateway API. This is used
/// for validation of maps such as TLS options. This roughly matches Kubernetes
/// annotation validation, although the length validation in that case is based
/// on the entire size of the annotations struct.
pub type AnnotationValue = String;
/// [`LabelKey`] is the key of a label in the Gateway API. This is used for validation
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
/// [`LabelValue`] is the value of a label in the Gateway API. This is used for validation
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
/// [`AddressType`] defines how a network address is represented as a text string.
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
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum AddressType {
    /// A textual representation of a numeric IP address. IPv4
    /// addresses must be in dotted-decimal form. IPv6 addresses
    /// must be in a standard IPv6 text representation
    /// (see [RFC 5952](https://tools.ietf.org/html/rfc5952)).
    ///
    /// This type is intended for specific addresses. Address ranges are not
    /// supported (e.g. you cannot use a CIDR range like 127.0.0.0/24 as an
    /// IPAddress).
    ///
    /// Support: Extended
    IpAddress,
    /// A Hostname represents a DNS based ingress point. This is similar to the
    /// corresponding hostname field in Kubernetes load balancer status. For
    /// example, this concept may be used for cloud load balancers where a DNS
    /// name is used to expose a load balancer.
    ///
    /// Support: Extended
    Hostname,
    /// A NamedAddress provides a way to reference a specific IP address by name.
    /// For example, this may be a name or other unique identifier that refers
    /// to a resource on a cloud provider such as a static IP.
    ///
    /// The `NamedAddress` type has been deprecated in favor of implementation
    /// specific domain-prefixed strings.
    ///
    /// Support: Implementation-specific
    NamedAddress,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl AddressType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::IpAddress => "IPAddress",
            Self::Hostname => "Hostname",
            Self::NamedAddress => "NamedAddress",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for AddressType {
    fn from(value: &str) -> Self {
        match value {
            "IPAddress" => Self::IpAddress,
            "Hostname" => Self::Hostname,
            "NamedAddress" => Self::NamedAddress,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for AddressType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "IPAddress" => Self::IpAddress,
            "Hostname" => Self::Hostname,
            "NamedAddress" => Self::NamedAddress,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for AddressType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for AddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for AddressType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for AddressType {}
impl std::hash::Hash for AddressType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for AddressType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for AddressType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for AddressType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for AddressType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for AddressType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "AddressType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "AddressType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
impl Default for AddressType {
    fn default() -> Self {
        Self::IpAddress
    }
}
/// [`HeaderName`] is the name of a header or query parameter.
pub type HeaderName = String;
/// [`Duration`] is a string value representing a duration in time. The format is as specified
/// in GEP-2257, a strict subset of the syntax parsed by Golang time.ParseDuration.
pub type Duration = String;
/// [`SessionPersistence`] defines the desired state of SessionPersistence.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct SessionPersistence {
    /// `session_name` defines the name of the persistent session token
    /// which may be reflected in the cookie or the header. Users
    /// should avoid reusing session names to prevent unintended
    /// consequences, such as rejection or unpredictable behavior.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionName")]
    pub session_name: Option<String>,
    /// `absolute_timeout` defines the absolute timeout of the persistent
    /// session. Once the AbsoluteTimeout duration has elapsed, the
    /// session becomes invalid.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "absoluteTimeout")]
    pub absolute_timeout: Option<Duration>,
    /// `idle_timeout` defines the idle timeout of the persistent session.
    /// Once the session has been idle for more than the specified
    /// IdleTimeout duration, the session becomes invalid.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "idleTimeout")]
    pub idle_timeout: Option<Duration>,
    /// `type_` defines the type of session persistence such as through
    /// the use of a header or cookie. Defaults to cookie based session
    /// persistence.
    ///
    /// Support: Core for "Cookie" type
    ///
    /// Support: Extended for "Header" type
    ///
    /// Default: `Cookie`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "type")]
    pub type_: Option<SessionPersistenceType>,
    /// `cookie_config` provides configuration settings that are specific
    /// to cookie-based session persistence.
    ///
    /// Support: Core
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "cookieConfig")]
    pub cookie_config: Option<CookieConfig>,
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum SessionPersistenceType {
    /// [`SessionPersistenceType::Cookie`] specifies cookie-based session
    /// persistence.
    ///
    /// Support: Core
    Cookie,
    /// [`SessionPersistenceType::Header`] specifies header-based session
    /// persistence.
    ///
    /// Support: Extended
    Header,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl SessionPersistenceType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Cookie => "Cookie",
            Self::Header => "Header",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for SessionPersistenceType {
    fn from(value: &str) -> Self {
        match value {
            "Cookie" => Self::Cookie,
            "Header" => Self::Header,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for SessionPersistenceType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Cookie" => Self::Cookie,
            "Header" => Self::Header,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for SessionPersistenceType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for SessionPersistenceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for SessionPersistenceType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for SessionPersistenceType {}
impl std::hash::Hash for SessionPersistenceType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for SessionPersistenceType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for SessionPersistenceType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for SessionPersistenceType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for SessionPersistenceType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for SessionPersistenceType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "SessionPersistenceType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "SessionPersistenceType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string", "enum" : ["Cookie", "Header"] })
    }
}
impl Default for SessionPersistenceType {
    fn default() -> Self {
        Self::Cookie
    }
}
/// [`CookieConfig`] defines the configuration for cookie-based session persistence.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct CookieConfig {
    /// `lifetime_type` specifies whether the cookie has a permanent or
    /// session-based lifetime. A permanent cookie persists until its
    /// specified expiry time, defined by the Expires or Max-Age cookie
    /// attributes, while a session cookie is deleted when the current
    /// session ends.
    ///
    /// When set to "Permanent", AbsoluteTimeout indicates the
    /// cookie's lifetime via the Expires or Max-Age cookie attributes
    /// and is required.
    ///
    /// When set to "Session", AbsoluteTimeout indicates the
    /// absolute lifetime of the cookie tracked by the gateway and
    /// is optional.
    ///
    /// Defaults to "Session".
    ///
    /// Support: Core for "Session" type
    ///
    /// Support: Extended for "Permanent" type
    ///
    /// Default: `Session`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "lifetimeType")]
    pub lifetime_type: Option<CookieLifetimeType>,
}
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum CookieLifetimeType {
    /// [`CookieLifetimeType::Session`] specifies the type for a session
    /// cookie.
    ///
    /// Support: Core
    Session,
    /// [`CookieLifetimeType::Permanent`] specifies the type for a permanent
    /// cookie.
    ///
    /// Support: Extended
    Permanent,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl CookieLifetimeType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Session => "Session",
            Self::Permanent => "Permanent",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for CookieLifetimeType {
    fn from(value: &str) -> Self {
        match value {
            "Session" => Self::Session,
            "Permanent" => Self::Permanent,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for CookieLifetimeType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Session" => Self::Session,
            "Permanent" => Self::Permanent,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for CookieLifetimeType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for CookieLifetimeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for CookieLifetimeType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for CookieLifetimeType {}
impl std::hash::Hash for CookieLifetimeType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for CookieLifetimeType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for CookieLifetimeType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for CookieLifetimeType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for CookieLifetimeType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for CookieLifetimeType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "CookieLifetimeType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "CookieLifetimeType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string", "enum" : ["Session", "Permanent"] })
    }
}
impl Default for CookieLifetimeType {
    fn default() -> Self {
        Self::Session
    }
}
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct Fraction {
    pub numerator: i32,
    /// Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub denominator: Option<i32>,
}
impl Fraction {
    /// Creates a `Fraction` from its required fields, with the optional fields `None`.
    pub fn new(numerator: impl Into<i32>) -> Self {
        Self {
            numerator: numerator.into(),
            denominator: None,
        }
    }
}
/// [`ParentGatewayReference`] identifies an API object including its namespace,
/// defaulting to Gateway.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct ParentGatewayReference {
    /// `group` is the group of the referent.
    ///
    /// Default: `"gateway.networking.k8s.io"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group: Option<Group>,
    /// `kind` is kind of the referent. For example "Gateway".
    ///
    /// Default: `Gateway`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<Kind>,
    /// `name` is the name of the referent.
    pub name: ObjectName,
    /// `namespace` is the namespace of the referent.  If not present,
    /// the namespace of the referent is assumed to be the same as
    /// the namespace of the referring object.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub namespace: Option<Namespace>,
}
impl ParentGatewayReference {
    /// Creates a `ParentGatewayReference` from its required fields, with the optional fields `None`.
    pub fn new(name: impl Into<ObjectName>) -> Self {
        Self {
            group: None,
            kind: None,
            name: name.into(),
            namespace: None,
        }
    }
}
/// [`TlsRouteSpec`] defines the expected behavior of a TLSRoute.
/// A TLSRoute MUST be attached to a Listener of protocol TLS.
/// Core: The listener CAN be of type Passthrough
/// Extended: The listener CAN be of type Terminate
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.k8s.io",
    version = "v1",
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
    doc = "The TLSRoute resource is similar to TCPRoute, but can be configured\nto match against TLS-specific metadata. This allows more flexibility\nin matching streams for a given TLS listener.\n\nIf you need to forward traffic to a single target for a TLS listener, you\ncould choose to use a TCPRoute with a TLS listener."
)]
pub struct TlsRouteSpec {
    /// `parent_refs` references the resources (usually Gateways) that a Route wants
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
    /// `use_default_gateways` indicates the default Gateway scope to use for this
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
    /// `hostnames` defines a set of SNI hostnames that should match against the
    /// SNI attribute of TLS ClientHello message in TLS handshake. This matches
    /// the RFC 1123 definition of a hostname with 2 notable exceptions:
    ///
    /// 1. IPs are not allowed in SNI hostnames per RFC 6066.
    /// 2. A hostname may be prefixed with a wildcard label (`*.`). The wildcard
    ///    label must appear by itself as the first label.
    pub hostnames: Vec<Hostname>,
    /// `rules` are a list of actions.
    pub rules: Vec<TlsRouteRule>,
}
impl TlsRouteSpec {
    /// Creates a `TlsRouteSpec` from its required fields, with the optional fields `None`.
    pub fn new(hostnames: impl Into<Vec<Hostname>>, rules: impl Into<Vec<TlsRouteRule>>) -> Self {
        Self {
            parent_refs: None,
            use_default_gateways: None,
            hostnames: hostnames.into(),
            rules: rules.into(),
        }
    }
}
/// [`TlsRouteStatus`] defines the observed state of TLSRoute
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TlsRouteStatus {
    /// `parents` is a list of parent resources (usually Gateways) that are
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
/// [`TlsRouteRule`] is the configuration for a given rule.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
pub struct TlsRouteRule {
    /// `name` is the name of the route rule. This name MUST be unique within a Route if it is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<SectionName>,
    /// `backend_refs` defines the backend(s) where matching requests should be
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
