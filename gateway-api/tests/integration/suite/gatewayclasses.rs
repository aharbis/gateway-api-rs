use kube::Api;

use self::api::{GatewayClass, GatewayClassSpec};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let class = GatewayClass {
        spec: GatewayClassSpec {
            controller_name: "example.com/gateway-api-tests".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::all(harness::client().await), class).await;
}
