use kube::Api;

use self::api::{BackendRef, UdpRoute, UdpRouteRule, UdpRouteSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let route = UdpRoute::new(
        "test",
        UdpRouteSpec::new(vec![UdpRouteRule::new(vec![BackendRef {
            port: Some(53),
            ..BackendRef::new("test-service")
        }])]),
    );

    common::crud(Api::default_namespaced(common::client().await), route).await;
}
