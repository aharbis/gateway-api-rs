use kube::Api;

use self::api::{BackendTrafficPolicySpec, LocalPolicyTargetReference, XBackendTrafficPolicy};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = XBackendTrafficPolicy::new(
        "test",
        BackendTrafficPolicySpec::new(vec![LocalPolicyTargetReference::new("", "Service", "test-service")]),
    );

    common::crud(Api::default_namespaced(common::client().await), policy).await;
}
