use kube::Api;

use self::api::{ListenerEntry, ListenerSet, ListenerSetSpec, ParentGatewayReference};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let set = ListenerSet::new(
        "test",
        ListenerSetSpec::new(
            ParentGatewayReference {
                group: Some("gateway.networking.k8s.io".into()),
                kind: Some("Gateway".into()),
                ..ParentGatewayReference::new("test-gateway")
            },
            vec![ListenerEntry::new("http", 8080, "HTTP")],
        ),
    );

    common::crud(Api::default_namespaced(common::client().await), set).await;
}
