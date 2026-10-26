use kube::Api;

use self::{
    api::{TcpRoute, TcpRouteRules, TcpRouteSpec},
    common::BackendRef,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TcpRoute {
        spec: TcpRouteSpec {
            rules: vec![TcpRouteRules {
                backend_refs: vec![BackendRef {
                    name: "test-service".into(),
                    port: Some(5432),
                    ..Default::default()
                }],
                ..Default::default()
            }],
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), route).await;
}
