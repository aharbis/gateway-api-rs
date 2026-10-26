use kube::Api;

use self::api::{ReferenceGrant, ReferenceGrantFrom, ReferenceGrantSpec, ReferenceGrantTo};
use crate::harness;

#[ignore]
#[tokio::test]
async fn crud() {
    let grant = ReferenceGrant {
        spec: ReferenceGrantSpec {
            from: vec![ReferenceGrantFrom {
                group: "gateway.networking.k8s.io".into(),
                kind: "HTTPRoute".into(),
                namespace: "default".into(),
            }],
            to: vec![ReferenceGrantTo {
                group: "".into(),
                kind: "Service".into(),
                name: None,
            }],
        },
        ..Default::default()
    };

    harness::crud(Api::default_namespaced(harness::client().await), grant).await;
}
