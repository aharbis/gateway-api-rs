use kube::Api;

use self::{
    api::{ListenerSet, ListenerSetListeners, ListenerSetSpec},
    common::ParentGatewayReference,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let set = ListenerSet {
        spec: ListenerSetSpec {
            parent_ref: ParentGatewayReference {
                group: Some("gateway.networking.k8s.io".into()),
                kind: Some("Gateway".into()),
                name: "test-gateway".into(),
                namespace: None,
            },
            listeners: vec![ListenerSetListeners {
                name: "http".into(),
                port: 8080,
                protocol: "HTTP".into(),
                ..Default::default()
            }],
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), set).await;
}
