use kube::Api;

use self::api::{BackendRef, TcpRoute, TcpRouteRule, TcpRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = TcpRoute::new(
        "test",
        TcpRouteSpec::new(vec![TcpRouteRule::new(vec![BackendRef {
            port: Some(5432),
            ..BackendRef::new("test-service")
        }])]),
    );

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
