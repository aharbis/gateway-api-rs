use kube::Api;

use self::api::{GrpcRoute, GrpcRouteSpec};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = GrpcRoute {
        spec: GrpcRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            parent_refs: Some(vec![builders::parent_ref()]),
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), route).await;
}
