// WARNING: generated file — do not edit

/// Every API version's comparison with the upstream CRD.
fn comparisons() -> Vec<Comparison> {
    vec![
        compare::<gateway_api_preview::standard::v1::BackendTlsPolicy>(Channel::Standard, "backendtlspolicies", "v1"),
        compare::<gateway_api_preview::standard::v1::GatewayClass>(Channel::Standard, "gatewayclasses", "v1"),
        compare::<gateway_api_preview::standard::v1::Gateway>(Channel::Standard, "gateways", "v1"),
        compare::<gateway_api_preview::standard::v1::GrpcRoute>(Channel::Standard, "grpcroutes", "v1"),
        compare::<gateway_api_preview::standard::v1::HttpRoute>(Channel::Standard, "httproutes", "v1"),
        compare::<gateway_api_preview::standard::v1::ListenerSet>(Channel::Standard, "listenersets", "v1"),
        compare::<gateway_api_preview::standard::v1::ReferenceGrant>(Channel::Standard, "referencegrants", "v1"),
        compare::<gateway_api_preview::standard::v1::TlsRoute>(Channel::Standard, "tlsroutes", "v1"),
        compare::<gateway_api_preview::standard::v1beta1::GatewayClass>(Channel::Standard, "gatewayclasses", "v1beta1"),
        compare::<gateway_api_preview::standard::v1beta1::Gateway>(Channel::Standard, "gateways", "v1beta1"),
        compare::<gateway_api_preview::standard::v1beta1::HttpRoute>(Channel::Standard, "httproutes", "v1beta1"),
        compare::<gateway_api_preview::standard::v1beta1::ReferenceGrant>(
            Channel::Standard,
            "referencegrants",
            "v1beta1",
        ),
        compare::<gateway_api_preview::standard::v1alpha2::TlsRoute>(Channel::Standard, "tlsroutes", "v1alpha2"),
        compare::<gateway_api_preview::standard::v1alpha3::BackendTlsPolicy>(
            Channel::Standard,
            "backendtlspolicies",
            "v1alpha3",
        ),
        compare::<gateway_api_preview::standard::v1alpha3::TlsRoute>(Channel::Standard, "tlsroutes", "v1alpha3"),
        compare::<gateway_api_preview::experimental::v1::BackendTlsPolicy>(
            Channel::Experimental,
            "backendtlspolicies",
            "v1",
        ),
        compare::<gateway_api_preview::experimental::v1::GatewayClass>(Channel::Experimental, "gatewayclasses", "v1"),
        compare::<gateway_api_preview::experimental::v1::Gateway>(Channel::Experimental, "gateways", "v1"),
        compare::<gateway_api_preview::experimental::v1::GrpcRoute>(Channel::Experimental, "grpcroutes", "v1"),
        compare::<gateway_api_preview::experimental::v1::HttpRoute>(Channel::Experimental, "httproutes", "v1"),
        compare::<gateway_api_preview::experimental::v1::ListenerSet>(Channel::Experimental, "listenersets", "v1"),
        compare::<gateway_api_preview::experimental::v1::ReferenceGrant>(
            Channel::Experimental,
            "referencegrants",
            "v1",
        ),
        compare::<gateway_api_preview::experimental::v1::TlsRoute>(Channel::Experimental, "tlsroutes", "v1"),
        compare::<gateway_api_preview::experimental::v1beta1::GatewayClass>(
            Channel::Experimental,
            "gatewayclasses",
            "v1beta1",
        ),
        compare::<gateway_api_preview::experimental::v1beta1::Gateway>(Channel::Experimental, "gateways", "v1beta1"),
        compare::<gateway_api_preview::experimental::v1beta1::HttpRoute>(
            Channel::Experimental,
            "httproutes",
            "v1beta1",
        ),
        compare::<gateway_api_preview::experimental::v1beta1::ReferenceGrant>(
            Channel::Experimental,
            "referencegrants",
            "v1beta1",
        ),
        compare::<gateway_api_preview::experimental::v1alpha2::TcpRoute>(
            Channel::Experimental,
            "tcproutes",
            "v1alpha2",
        ),
        compare::<gateway_api_preview::experimental::v1alpha2::TlsRoute>(
            Channel::Experimental,
            "tlsroutes",
            "v1alpha2",
        ),
        compare::<gateway_api_preview::experimental::v1alpha2::UdpRoute>(
            Channel::Experimental,
            "udproutes",
            "v1alpha2",
        ),
        compare::<gateway_api_preview::experimental::v1alpha3::BackendTlsPolicy>(
            Channel::Experimental,
            "backendtlspolicies",
            "v1alpha3",
        ),
        compare::<gateway_api_preview::experimental::v1alpha3::TlsRoute>(
            Channel::Experimental,
            "tlsroutes",
            "v1alpha3",
        ),
        compare::<gateway_api_preview::experimental::apisx::v1alpha1::XBackendTrafficPolicy>(
            Channel::Experimental,
            "xbackendtrafficpolicies",
            "v1alpha1",
        ),
        compare::<gateway_api_preview::experimental::apisx::v1alpha1::XMesh>(
            Channel::Experimental,
            "xmeshes",
            "v1alpha1",
        ),
    ]
}
