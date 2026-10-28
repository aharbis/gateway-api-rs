// TLSRoute `v1alpha2`, where `hostnames` is optional.

use kube::Api;

use self::api::{BackendRef, TlsRoute, TlsRouteRule, TlsRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TlsRoute::new(
        "test",
        TlsRouteSpec {
            hostnames: Some(vec!["example.com".into()]),
            ..TlsRouteSpec::new(vec![TlsRouteRule::new(vec![BackendRef {
                port: Some(443),
                ..BackendRef::new("test-service")
            }])])
        },
    );

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
