use kube::Api;

use self::{
    api::{UdpRoute, UdpRouteRules, UdpRouteSpec},
    common::BackendRef,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = UdpRoute {
        spec: UdpRouteSpec {
            rules: vec![UdpRouteRules {
                backend_refs: vec![BackendRef {
                    name: "test-service".into(),
                    port: Some(53),
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
