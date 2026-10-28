use kube::Api;

use self::api::{GatewayClass, GatewayClassSpec};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let class = GatewayClass::new("test", GatewayClassSpec::new("example.com/gateway-api-tests"));

    common::crud(Api::all(common::client().await), class).await;
}
