use kube::Api;

use self::{
    api::{BackendTlsPolicy, BackendTlsPolicySpec, BackendTlsPolicyValidation},
    common::LocalPolicyTargetReferenceWithSectionName,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = BackendTlsPolicy {
        spec: BackendTlsPolicySpec {
            target_refs: vec![LocalPolicyTargetReferenceWithSectionName {
                group: "".into(),
                kind: "Service".into(),
                name: "test-service".into(),
                ..Default::default()
            }],
            validation: BackendTlsPolicyValidation {
                hostname: "example.com".into(),
                well_known_ca_certificates: Some("System".into()),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), policy).await;
}
