use kube::Api;

use self::api::{MeshSpec, XMesh};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let mesh = XMesh::new("test", MeshSpec::new("example.com/gateway-api-tests"));

    common::crud(Api::all(common::client().await), mesh).await;
}
