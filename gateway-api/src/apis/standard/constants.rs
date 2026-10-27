// WARNING: generated file — do not edit

/// GatewayConditionType is a type of condition associated with a
/// Gateway. This type should be used with the GatewayStatus.Conditions
/// field.
///
/// Upstream: `GatewayConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for GatewayConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// GatewayConditionReason defines the set of reasons that explain why a
/// particular Gateway condition type has been raised.
///
/// Upstream: `GatewayConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for GatewayConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerConditionType is a type of condition associated with the
/// listener. This type should be used with the ListenerStatus.Conditions
/// field.
///
/// Upstream: `ListenerConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
    OverlappingTLSConfig,
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
}

impl std::fmt::Display for ListenerConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerConditionReason defines the set of reasons that explain
/// why a particular Listener condition type has been raised.
///
/// Upstream: `ListenerConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
    NoValidCACertificate,
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
    InvalidCACertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when one or more
    /// CACertificate References used to configure Client Certificate
    /// validation for Gateway has unknown or unsupported kind.
    InvalidCACertificateKind,
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
}

impl std::fmt::Display for ListenerConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// GatewayClassConditionType is the type for status conditions on
/// Gateway resources. This type should be used with the
/// GatewayClassStatus.Conditions field.
///
/// Upstream: `GatewayClassConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for GatewayClassConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// GatewayClassConditionReason defines the set of reasons that explain why a
/// particular GatewayClass condition type has been raised.
///
/// Upstream: `GatewayClassConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for GatewayClassConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerSetConditionType is a type of condition associated with a
/// ListenerSet. This type should be used with the ListenerSetStatus.Conditions
/// field.
///
/// Upstream: `ListenerSetConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for ListenerSetConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerSetConditionReason defines the set of reasons that explain why a
/// particular ListenerSet condition type has been raised.
///
/// Upstream: `ListenerSetConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for ListenerSetConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerEntryConditionType is a type of condition associated with the
/// listener. This type should be used with the ListenerEntryStatus.Conditions
/// field.
///
/// Upstream: `ListenerEntryConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for ListenerEntryConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// ListenerEntryConditionReason defines the set of reasons that explain
/// why a particular ListenerEntry condition type has been raised.
///
/// Upstream: `ListenerEntryConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for ListenerEntryConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// PolicyConditionType is a type of condition for a policy. This type should be
/// used with a Policy resource Status.Conditions field.
///
/// Upstream: `PolicyConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
    /// PolicyConditionAccepted indicates whether the policy has been accepted or
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
}

impl std::fmt::Display for PolicyConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// PolicyConditionReason is a reason for a policy condition.
///
/// Upstream: `PolicyConditionReason`.
#[derive(Debug, PartialEq, Eq)]
pub enum PolicyConditionReason {
    /// This reason is used with the "Accepted" condition when it is
    /// set to false because all CACertificateRefs of the
    /// BackendTLSPolicy are invalid.
    NoValidCACertificate,
    /// This reason is used with the "ResolvedRefs" condition when the condition
    /// is true.
    ResolvedRefs,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// BackendTLSPolicy's CACertificateRefs is invalid.
    /// A CACertificateRef is considered invalid when it refers to a nonexistent
    /// resource or when the data within that resource is malformed.
    InvalidCACertificateRef,
    /// This reason is used with the "ResolvedRefs" condition when one of the
    /// BackendTLSPolicy's CACertificateRefs references an unknown or unsupported
    /// Group and/or Kind.
    InvalidKind,
    /// PolicyReasonAccepted is used with the "Accepted" condition when the policy
    /// has been accepted by the targeted resource.
    Accepted,
    /// PolicyReasonConflicted is used with the "Accepted" condition when the
    /// policy has not been accepted by a targeted resource because there is
    /// another policy that targets the same resource and a merge is not possible.
    Conflicted,
    /// PolicyReasonInvalid is used with the "Accepted" condition when the policy
    /// is syntactically or semantically invalid.
    Invalid,
    /// PolicyReasonTargetNotFound is used with the "Accepted" condition when the
    /// policy is attached to an invalid target resource.
    TargetNotFound,
}

impl std::fmt::Display for PolicyConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// RouteConditionType is a type of condition for a route.
///
/// Upstream: `RouteConditionType`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for RouteConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// RouteConditionReason is a reason for a route condition.
///
/// Upstream: `RouteConditionReason`.
#[derive(Debug, PartialEq, Eq)]
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
}

impl std::fmt::Display for RouteConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
