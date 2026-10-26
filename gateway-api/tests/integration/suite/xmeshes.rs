use kube::Api;

use self::api::{XMesh, XMeshSpec};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let mesh = XMesh {
        spec: XMeshSpec {
            controller_name: "example.com/gateway-api-tests".into(),
            ..Default::default()
        },
        ..Default::default()
    };

    harness::crud(Api::all(harness::client().await), mesh).await;
}
