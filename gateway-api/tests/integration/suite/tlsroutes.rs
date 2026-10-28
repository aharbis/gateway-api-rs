use kube::Api;

use self::{
    api::{TlsRoute, TlsRouteSpec},
    channel::v1::{BackendRef, TlsRouteRule},
};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TlsRoute::new(
        "test",
        TlsRouteSpec::new(
            vec!["example.com".into()],
            vec![TlsRouteRule::new(vec![BackendRef {
                port: Some(443),
                ..BackendRef::new("test-service")
            }])],
        ),
    );

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
