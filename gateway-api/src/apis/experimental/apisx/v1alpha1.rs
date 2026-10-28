// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apisx/v1alpha1` at v1.5.1.
//!
//! Its kinds, as the CRDs in `config/crd/experimental` define them:
//!
//! - [`XBackendTrafficPolicy`]: `gateway.networking.x-k8s.io/v1alpha1`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
//!
//! - [`XMesh`]: `gateway.networking.x-k8s.io/v1alpha1`
//!   - served: yes
//!   - storage: yes
//!   - deprecated: no
use k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition;

pub use crate::apis::experimental::v1::{Duration, LocalPolicyTargetReference, PolicyStatus, SessionPersistence};
use crate::apis::experimental::v1::{GatewayController, ParametersReference, SupportedFeature};
/// RequestRate expresses a rate of requests over a given period of time.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct RequestRate {
    /// Count specifies the number of requests per time interval.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    /// Interval specifies the divisor of the rate of requests, the amount of
    /// time during which the given count of requests occur.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Duration>,
}
/// BackendTrafficPolicySpec define the desired state of BackendTrafficPolicy
/// Note: there is no Override or Default policy configuration.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.x-k8s.io",
    version = "v1alpha1",
    kind = "XBackendTrafficPolicy",
    root = "XBackendTrafficPolicy",
    plural = "xbackendtrafficpolicies"
)]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(shortname = "xbtrafficpolicy")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(doc = "XBackendTrafficPolicy defines the configuration for how traffic to a\ntarget backend should be handled.")]
pub struct BackendTrafficPolicySpec {
    /// TargetRefs identifies API object(s) to apply this policy to.
    /// Currently, Backends (A grouping of like endpoints such as Service,
    /// ServiceImport, or any implementation-specific backendRef) are the only
    /// valid API target references.
    ///
    /// Currently, a TargetRef cannot be scoped to a specific port on a
    /// Service.
    #[serde(rename = "targetRefs")]
    pub target_refs: Vec<LocalPolicyTargetReference>,
    /// RetryConstraint defines the configuration for when to allow or prevent
    /// further retries to a target backend, by dynamically calculating a 'retry
    /// budget'. This budget is calculated based on the percentage of incoming
    /// traffic composed of retries over a given time interval. Once the budget
    /// is exceeded, additional retries will be rejected.
    ///
    /// For example, if the retry budget interval is 10 seconds, there have been
    /// 1000 active requests in the past 10 seconds, and the allowed percentage
    /// of requests that can be retried is 20% (the default), then 200 of those
    /// requests may be composed of retries. Active requests will only be
    /// considered for the duration of the interval when calculating the retry
    /// budget. Retrying the same original request multiple times within the
    /// retry budget interval will lead to each retry being counted towards
    /// calculating the budget.
    ///
    /// Configuring a RetryConstraint in BackendTrafficPolicy is compatible with
    /// HTTPRoute Retry settings for each HTTPRouteRule that targets the same
    /// backend. While the HTTPRouteRule Retry stanza can specify whether a
    /// request will be retried, and the number of retry attempts each client
    /// may perform, RetryConstraint helps prevent cascading failures such as
    /// retry storms during periods of consistent failures.
    ///
    /// After the retry budget has been exceeded, additional retries to the
    /// backend MUST return a 503 response to the client.
    ///
    /// Additional configurations for defining a constraint on retries MAY be
    /// defined in the future.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "retryConstraint")]
    pub retry_constraint: Option<RetryConstraint>,
    /// SessionPersistence defines and configures session persistence
    /// for the backend.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "sessionPersistence")]
    pub session_persistence: Option<SessionPersistence>,
}
impl BackendTrafficPolicySpec {
    /// Creates a `BackendTrafficPolicySpec` from its required fields, with the optional fields `None`.
    pub fn new(target_refs: impl Into<Vec<LocalPolicyTargetReference>>) -> Self {
        Self {
            target_refs: target_refs.into(),
            retry_constraint: None,
            session_persistence: None,
        }
    }
}
/// RetryConstraint defines the configuration for when to retry a request.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct RetryConstraint {
    /// Budget holds the details of the retry budget configuration.
    ///
    /// Default: `{percent: 20, interval: "10s"}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub budget: Option<BudgetDetails>,
    /// MinRetryRate defines the minimum rate of retries that will be allowable
    /// over a specified duration of time.
    ///
    /// The effective overall minimum rate of retries targeting the backend
    /// service may be much higher, as there can be any number of clients which
    /// are applying this setting locally.
    ///
    /// This ensures that requests can still be retried during periods of low
    /// traffic, where the budget for retries may be calculated as a very low
    /// value.
    ///
    /// Support: Extended
    ///
    /// Default: `{count: 10, interval: "1s"}`.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minRetryRate")]
    pub min_retry_rate: Option<RequestRate>,
}
/// BudgetDetails specifies the details of the budget configuration, like
/// the percentage of requests in the budget, and the interval between
/// checks.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct BudgetDetails {
    /// Percent defines the maximum percentage of active requests that may
    /// be made up of retries.
    ///
    /// Support: Extended
    ///
    /// Default: `20`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
    /// Interval defines the duration in which requests will be considered
    /// for calculating the budget for retries.
    ///
    /// Support: Extended
    ///
    /// Default: `"10s"`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Duration>,
}
/// MeshSpec defines the desired state of an XMesh.
#[derive(kube::CustomResource, serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq)]
#[kube(
    group = "gateway.networking.x-k8s.io",
    version = "v1alpha1",
    kind = "XMesh",
    root = "XMesh",
    plural = "xmeshes"
)]
#[kube(status = "MeshStatus")]
#[kube(shortname = "mesh")]
#[kube(category = "gateway-api")]
#[kube(printcolumn(
    name = "Accepted",
    type_ = "string",
    json_path = ".status.conditions[?(@.type==\"Accepted\")].status"
))]
#[kube(printcolumn(name = "Age", type_ = "date", json_path = ".metadata.creationTimestamp"))]
#[kube(derive = "PartialEq")]
#[kube(doc = "XMesh defines mesh-wide characteristics of a GAMMA-compliant service mesh.")]
pub struct MeshSpec {
    /// ControllerName is the name of a controller that is managing Gateway API
    /// resources for mesh traffic management. The value of this field MUST be a
    /// domain prefixed path.
    ///
    /// Example: "example.com/awesome-mesh".
    ///
    /// This field is not mutable and cannot be empty.
    ///
    /// Support: Core
    #[serde(rename = "controllerName")]
    pub controller_name: GatewayController,
    /// ParametersRef is an optional reference to a resource that contains
    /// implementation-specific configuration for this Mesh. If no
    /// implementation-specific parameters are needed, this field MUST be
    /// omitted.
    ///
    /// ParametersRef can reference a standard Kubernetes resource, i.e.
    /// ConfigMap, or an implementation-specific custom resource. The resource
    /// can be cluster-scoped or namespace-scoped.
    ///
    /// If the referent cannot be found, refers to an unsupported kind, or when
    /// the data within that resource is malformed, the Mesh MUST be rejected
    /// with the "Accepted" status condition set to "False" and an
    /// "InvalidParameters" reason.
    ///
    /// Support: Implementation-specific
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "parametersRef")]
    pub parameters_ref: Option<ParametersReference>,
    /// Description optionally provides a human-readable description of a Mesh.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}
impl MeshSpec {
    /// Creates a `MeshSpec` from its required fields, with the optional fields `None`.
    pub fn new(controller_name: impl Into<GatewayController>) -> Self {
        Self {
            controller_name: controller_name.into(),
            parameters_ref: None,
            description: None,
        }
    }
}
/// MeshConditionType is the type for status conditions on Mesh resources.
/// This type should be used with the MeshStatus.Conditions field.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MeshConditionType {
    /// The "Accepted" condition indicates whether the Mesh has been accepted
    /// by the controller requested in the `spec.controllerName` field.
    ///
    /// This condition defaults to Unknown, and MUST be set by a controller
    /// when it sees a Mesh using its controller string. The status of this
    /// condition MUST be set to True if the controller will accept the Mesh
    /// resource. Otherwise, this status MUST be set to False. If the status is
    /// set to False, the controller MUST set a Message and Reason as an
    /// explanation.
    ///
    /// Possible reasons for this condition to be true are:
    ///
    /// * "Accepted"
    ///
    /// Possible reasons for this condition to be False are:
    ///
    /// * "InvalidParameters"
    ///
    /// Possible reasons for this condition to be Unknown are:
    ///
    /// * "Pending"
    ///
    /// Controllers should prefer to use the values of MeshConditionReason for
    /// the corresponding Reason, where appropriate.
    Accepted,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl MeshConditionType {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for MeshConditionType {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for MeshConditionType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for MeshConditionType {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for MeshConditionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for MeshConditionType {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for MeshConditionType {}
impl std::hash::Hash for MeshConditionType {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for MeshConditionType {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for MeshConditionType {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for MeshConditionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MeshConditionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for MeshConditionType {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "MeshConditionType".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "MeshConditionType").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// MeshConditionReason defines the set of reasons that explain why a
/// particular Mesh condition type has been raised.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum MeshConditionReason {
    /// This reason is used with the "Accepted" condition when the condition is
    /// true.
    Accepted,
    /// This reason is used with the "Accepted" condition when the Mesh was not
    /// accepted because the parametersRef field refers to
    ///
    /// * a namespaced resource but the Namespace field is not set, or
    /// * a cluster-scoped resource but the Namespace field is set, or
    /// * a nonexistent object, or
    /// * an unsupported resource or kind, or
    /// * an existing resource but the data within that resource is malformed.
    InvalidParameters,
    /// This reason is used with the "Accepted" condition when the status is
    /// "Unknown" and the requested controller has not yet made a decision
    /// about whether to accept the Mesh. It is the default Reason on a new
    /// Mesh.
    Pending,
    /// A value without a variant: one this version of the crate
    /// doesn't know, or one outside upstream's values.
    Other(String),
}
impl MeshConditionReason {
    /// The value as a string.
    pub fn as_str(&self) -> &str {
        match self {
            Self::Accepted => "Accepted",
            Self::InvalidParameters => "InvalidParameters",
            Self::Pending => "Pending",
            Self::Other(value) => value,
        }
    }
}
impl From<&str> for MeshConditionReason {
    fn from(value: &str) -> Self {
        match value {
            "Accepted" => Self::Accepted,
            "InvalidParameters" => Self::InvalidParameters,
            "Pending" => Self::Pending,
            _ => Self::Other(value.to_string()),
        }
    }
}
impl From<String> for MeshConditionReason {
    fn from(value: String) -> Self {
        match value.as_str() {
            "Accepted" => Self::Accepted,
            "InvalidParameters" => Self::InvalidParameters,
            "Pending" => Self::Pending,
            _ => Self::Other(value),
        }
    }
}
impl std::str::FromStr for MeshConditionReason {
    type Err = std::convert::Infallible;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Ok(Self::from(value))
    }
}
impl std::fmt::Display for MeshConditionReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
impl PartialEq for MeshConditionReason {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}
impl Eq for MeshConditionReason {}
impl std::hash::Hash for MeshConditionReason {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(self.as_str(), state);
    }
}
impl PartialEq<str> for MeshConditionReason {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}
impl PartialEq<&str> for MeshConditionReason {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}
impl serde::Serialize for MeshConditionReason {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}
impl<'de> serde::Deserialize<'de> for MeshConditionReason {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        <String as serde::Deserialize>::deserialize(deserializer).map(Self::from)
    }
}
impl schemars::JsonSchema for MeshConditionReason {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "MeshConditionReason".into()
    }
    fn schema_id() -> std::borrow::Cow<'static, str> {
        concat!(module_path!(), "::", "MeshConditionReason").into()
    }
    fn json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
        schemars::json_schema!({ "type" : "string" })
    }
}
/// MeshStatus is the current status for the Mesh.
#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, schemars::JsonSchema, PartialEq, Default)]
pub struct MeshStatus {
    /// Conditions is the current status from the controller for
    /// this Mesh.
    ///
    /// Controllers should prefer to publish conditions using values
    /// of MeshConditionType for the type of each Condition.
    ///
    /// Default: `{{type: "Accepted", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"},{type: "Programmed", status: "Unknown", reason:"Pending", message:"Waiting for controller", lastTransitionTime: "1970-01-01T00:00:00Z"}}`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<Condition>>,
    /// SupportedFeatures is the set of features the Mesh support.
    /// It MUST be sorted in ascending alphabetical order by the Name key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedFeatures")]
    pub supported_features: Option<Vec<SupportedFeature>>,
}
