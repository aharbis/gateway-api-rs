use kube::Api;

use self::{
    api::{XBackendTrafficPolicy, XBackendTrafficPolicySpec},
    common::LocalPolicyTargetReference,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = XBackendTrafficPolicy {
        spec: XBackendTrafficPolicySpec {
            target_refs: vec![LocalPolicyTargetReference {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), policy).await;
}
