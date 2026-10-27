// WARNING: generated file — do not edit
//! Go package `sigs.k8s.io/gateway-api/apisx/v1alpha1`, experimental channel.

#[allow(unused_imports)]
use std::collections::BTreeMap;

#[allow(unused_imports)]
use kube::CustomResource;
#[allow(unused_imports)]
use schemars::JsonSchema;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};

// Go aliases of other packages' types
pub use crate::experimental::v1::Duration;
use crate::experimental::v1::{GatewayController, ParametersReference, SupportedFeature};
pub use crate::experimental::v1::{LocalPolicyTargetReference, PolicyStatus, SessionPersistence};

/// BackendTrafficPolicySpec define the desired state of BackendTrafficPolicy
/// Note: there is no Override or Default policy configuration.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.x-k8s.io",
    version = "v1alpha1",
    kind = "XBackendTrafficPolicy",
    root = "XBackendTrafficPolicy",
    plural = "xbackendtrafficpolicies"
)]
#[kube(namespaced)]
#[kube(status = "PolicyStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct XBackendTrafficPolicySpec {
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

/// MeshSpec defines the desired state of an XMesh.
#[derive(CustomResource, Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
#[kube(
    group = "gateway.networking.x-k8s.io",
    version = "v1alpha1",
    kind = "XMesh",
    root = "XMesh",
    plural = "xmeshes"
)]
#[kube(status = "MeshStatus")]
#[kube(derive = "Default")]
#[kube(derive = "PartialEq")]
pub struct XMeshSpec {
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

/// RequestRate expresses a rate of requests over a given period of time.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

/// RetryConstraint defines the configuration for when to retry a request.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct RetryConstraint {
    /// Budget holds the details of the retry budget configuration.
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
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "minRetryRate")]
    pub min_retry_rate: Option<RequestRate>,
}

/// BudgetDetails specifies the details of the budget configuration, like
/// the percentage of requests in the budget, and the interval between
/// checks.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct BudgetDetails {
    /// Percent defines the maximum percentage of active requests that may
    /// be made up of retries.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<i64>,
    /// Interval defines the duration in which requests will be considered
    /// for calculating the budget for retries.
    ///
    /// Support: Extended
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub interval: Option<Duration>,
}

/// MeshSpec defines the desired state of an XMesh.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
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

/// MeshStatus is the current status for the Mesh.
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq)]
pub struct MeshStatus {
    /// Conditions is the current status from the controller for
    /// this Mesh.
    ///
    /// Controllers should prefer to publish conditions using values
    /// of MeshConditionType for the type of each Condition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition>>,
    /// SupportedFeatures is the set of features the Mesh support.
    /// It MUST be sorted in ascending alphabetical order by the Name key.
    #[serde(default, skip_serializing_if = "Option::is_none", rename = "supportedFeatures")]
    pub supported_features: Option<Vec<SupportedFeature>>,
}
