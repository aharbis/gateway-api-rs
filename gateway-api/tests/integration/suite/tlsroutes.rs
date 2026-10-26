use kube::Api;

use self::{
    api::{TlsRoute, TlsRouteRules, TlsRouteSpec},
    common::BackendRef,
};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TlsRoute {
        spec: TlsRouteSpec {
            hostnames: vec!["example.com".into()],
            rules: vec![TlsRouteRules {
                backend_refs: vec![BackendRef {
                    name: "test-service".into(),
                    port: Some(443),
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
