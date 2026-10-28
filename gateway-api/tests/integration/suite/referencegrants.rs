use kube::Api;

use self::api::{ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo};
use crate::common;

#[ignore]
#[tokio::test]
async fn crud() {
    let grant = ReferenceGrant::new(
        "test",
        ReferenceGrantSpec::new(
            vec![ReferenceGrantFrom::new(
                "gateway.networking.k8s.io",
                "HTTPRoute",
                "default",
            )],
            vec![ReferenceGrantTo::new("", "Service")],
        ),
    );

    common::crud(Api::default_namespaced(common::client().await), grant).await;
}
