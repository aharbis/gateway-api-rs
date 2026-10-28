use kube::Api;

use self::{
    api::{BackendTlsPolicy, BackendTlsPolicySpec},
    channel::v1::{BackendTlsPolicyValidation, LocalPolicyTargetReferenceWithSectionName},
};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let policy = BackendTlsPolicy::new(
        "test",
        BackendTlsPolicySpec::new(
            vec![LocalPolicyTargetReferenceWithSectionName::new(
                "",
                "Service",
                "test-service",
            )],
            BackendTlsPolicyValidation {
                well_known_ca_certificates: Some("System".into()),
                ..BackendTlsPolicyValidation::new("example.com")
            },
        ),
    );

    common::crud(Api::default_namespaced(common::client().await), policy).await;
}
