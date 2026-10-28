// WARNING: generated file — do not edit

/// Calls `$callback!` once per channel, with each resource's root type,
/// every API version its CRD defines and the module of the version's
/// root type in the channel, e.g.
/// `$callback!(standard { httproutes: HttpRoute [v1 => v1, v1beta1 => v1beta1] ... })`.
/// Every channel is passed in every build: one behind a feature comes
/// with its `#[cfg(feature = ...)]`, for the callback to put on
/// whatever uses the channel's types.
#[macro_export]
macro_rules! apis {
    ($callback:ident) => {
        $callback!(standard {
            backendtlspolicies: BackendTlsPolicy [v1 => v1, v1alpha3 => v1alpha3]
            gatewayclasses: GatewayClass [v1 => v1, v1beta1 => v1beta1]
            gateways: Gateway [v1 => v1, v1beta1 => v1beta1]
            grpcroutes: GrpcRoute [v1 => v1]
            httproutes: HttpRoute [v1 => v1, v1beta1 => v1beta1]
            listenersets: ListenerSet [v1 => v1]
            referencegrants: ReferenceGrant [v1 => v1, v1beta1 => v1beta1]
            tlsroutes: TlsRoute [v1 => v1, v1alpha2 => v1alpha2, v1alpha3 => v1alpha3]
        });
        $callback!(#[cfg(feature = "experimental")] experimental {
            backendtlspolicies: BackendTlsPolicy [v1 => v1, v1alpha3 => v1alpha3]
            gatewayclasses: GatewayClass [v1 => v1, v1beta1 => v1beta1]
            gateways: Gateway [v1 => v1, v1beta1 => v1beta1]
            grpcroutes: GrpcRoute [v1 => v1]
            httproutes: HttpRoute [v1 => v1, v1beta1 => v1beta1]
            listenersets: ListenerSet [v1 => v1]
            referencegrants: ReferenceGrant [v1 => v1, v1beta1 => v1beta1]
            tcproutes: TcpRoute [v1alpha2 => v1alpha2]
            tlsroutes: TlsRoute [v1 => v1, v1alpha2 => v1alpha2, v1alpha3 => v1alpha3]
            udproutes: UdpRoute [v1alpha2 => v1alpha2]
            xbackendtrafficpolicies: XBackendTrafficPolicy [v1alpha1 => apisx::v1alpha1]
            xmeshes: XMesh [v1alpha1 => apisx::v1alpha1]
        });
    };
}
