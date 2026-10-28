use k8s_openapi::{
    apimachinery::pkg::apis::meta::v1::{Condition, Time},
    jiff::Timestamp,
};
use kube::{
    Api, ResourceExt,
    api::{DeleteParams, Patch, PatchParams, PostParams},
};
use serde_json::json;

use self::{
    api::{Gateway, GatewaySpec, GatewayStatus, Listener, ListenerStatus},
    channel::v1::{
        AddressType, GatewayConditionReason, GatewayConditionType, GatewayStatusAddress, ListenerConditionReason,
        ListenerConditionType,
    },
};
use crate::common;

fn gateway() -> Gateway {
    Gateway::new(
        "test",
        GatewaySpec::new("test-gateway-class", vec![Listener::new("http", 80, "HTTP")]),
    )
}

#[ignore]
#[tokio::test]
async fn crud() {
    common::crud(Api::default_namespaced(common::client().await), gateway()).await;
}

#[ignore]
#[tokio::test]
async fn status() {
    let api: Api<Gateway> = Api::default_namespaced(common::client().await);
    let mut gw = gateway();
    gw.metadata.name = None;
    gw.metadata.generate_name = Some("test-".into());
    let created = api
        .create(&PostParams::default(), &gw)
        .await
        .expect("failed to create Gateway");
    let name = created.name_any();
    assert!(created.metadata.uid.is_some());
    assert_eq!(created.spec.gateway_class_name, "test-gateway-class");

    let condition = |type_: &str, reason: &str| Condition {
        last_transition_time: Time(Timestamp::now()),
        message: "testing gateway".into(),
        observed_generation: Some(1),
        reason: reason.into(),
        status: "True".into(),
        type_: type_.into(),
    };
    let status = GatewayStatus {
        addresses: Some(vec![GatewayStatusAddress {
            type_: Some(AddressType::IpAddress),
            ..GatewayStatusAddress::new("192.0.2.1")
        }]),
        listeners: Some(vec![ListenerStatus {
            supported_kinds: Some(vec![]),
            ..ListenerStatus::new(
                "http",
                0,
                vec![condition(
                    ListenerConditionType::Programmed.as_str(),
                    ListenerConditionReason::Programmed.as_str(),
                )],
            )
        }]),
        conditions: Some(vec![condition(
            GatewayConditionType::Programmed.as_str(),
            GatewayConditionReason::Programmed.as_str(),
        )]),
        ..Default::default()
    };

    let patched = api
        .patch_status(
            &name,
            &PatchParams::default(),
            &Patch::Merge(json!({ "status": status })),
        )
        .await;
    api.delete(&name, &DeleteParams::default())
        .await
        .expect("failed to delete Gateway");

    let s = patched
        .expect("failed to patch Gateway status")
        .status
        .expect("status should be set");
    let addresses = s.addresses.expect("addresses should be set");
    assert_eq!(addresses.len(), 1);
    assert_eq!(addresses[0].value, "192.0.2.1");
    assert_eq!(addresses[0].type_, Some(AddressType::IpAddress));
    let listeners = s.listeners.expect("listeners should be set");
    assert_eq!(listeners.len(), 1);
    assert_eq!(listeners[0].name, "http");
    assert_eq!(listeners[0].attached_routes, 0);
    let conditions = s.conditions.expect("conditions should be set");
    assert_eq!(conditions.len(), 1);
    assert_eq!(
        GatewayConditionReason::from(conditions[0].reason.as_str()),
        GatewayConditionReason::Programmed
    );
    assert_eq!(conditions[0].status, "True");
}
