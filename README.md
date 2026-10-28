[![Docs](https://img.shields.io/badge/docs-docs.rs-ff69b4.svg)](https://docs.rs/gateway-api/)
[![crates.io](https://img.shields.io/crates/v/gateway-api.svg)](https://crates.io/crates/gateway-api)
[![License](https://img.shields.io/badge/license-mit-blue.svg)](https://raw.githubusercontent.com/kube-rs/gateway-api-rs/main/LICENSE)

# Gateway API (Rust)

[Kubernetes] [Gateway API] support for [Rust].

> **Note**: Currently supports [Gateway API version v1.5.1][gwv]

[gwv]:https://github.com/kubernetes-sigs/gateway-api/releases/tag/v1.5.1
[Rust]:https://rust-lang.org
[Kubernetes]:https://kubernetes.io/
[Gateway API]:https://gateway-api.sigs.k8s.io/

## Usage

Requires a [kube-rs] [Client] to perform create, read, update and delete (CRUD)
operations on [Gateway API resources].

```rust
use gateway_api::v1::HttpRoute;
use kube::{Api, Client, ResourceExt, api::ListParams};

let client = Client::try_default().await?;
let routes: Api<HttpRoute> = Api::default_namespaced(client);
for route in routes.list(&ListParams::default()).await? {
    println!("{}", route.name_any());
}
```

[kube-rs]:https://github.com/kube-rs/kube
[Client]:https://docs.rs/kube/latest/kube/struct.Client.html
[Gateway API resources]:https://gateway-api.sigs.k8s.io/api-types/gateway/

### Modules and types

The crate is generated from upstream's Go packages, the same source as
upstream's Go SDK. Each Go package is a module (`gateway_api::v1`,
`gateway_api::v1beta1`, ...), and each Go type is a Rust type under its Go name
in Rust casing: `HTTPRouteRule` is `HttpRouteRule`. A Go alias of another
package's type is a re-export, so `v1beta1::ParentReference` is
`v1::ParentReference`.

Each API version a CRD defines has its root type in that version's module:

```rust
use gateway_api::v1::ReferenceGrant;
use gateway_api::v1beta1::ReferenceGrant as ReferenceGrantV1beta1;
```

### Building objects

Required fields have no default. A struct with required fields has a `new()`
that takes them, and struct update syntax fills in the rest:

```rust
use gateway_api::v1::{
    Gateway, GatewaySpec, HttpHeader, HttpHeaderFilter, HttpRouteFilter, HttpRouteFilterType, Listener,
};

let gateway = Gateway::new(
    "example",
    GatewaySpec::new("example-class", vec![Listener::new("http", 80, "HTTP")]),
);

let filter = HttpRouteFilter {
    request_header_modifier: Some(HttpHeaderFilter {
        set: Some(vec![HttpHeader::new("x-env", "prod")]),
        ..Default::default()
    }),
    ..HttpRouteFilter::new(HttpRouteFilterType::RequestHeaderModifier)
};
```

### Enums

Go string types with known values are enums, including the condition types and
reasons. Each has an `Other(String)` variant for values it doesn't know, and
converts from and to strings:

```rust
use gateway_api::v1::{GatewayConditionType, ProtocolType};

assert_eq!(GatewayConditionType::Programmed, "Programmed");
assert_eq!(ProtocolType::from("HTTPS"), ProtocolType::Https);
assert_eq!(
    ProtocolType::from("example.com/custom"),
    ProtocolType::Other("example.com/custom".into())
);
```

### Experimental APIs

APIs from the [experimental channel] are provided under
`gateway_api::experimental` and require the `experimental` feature.

```toml
[dependencies]
gateway-api = { version = "*", features = ["experimental"] }
```

```rust
use gateway_api::experimental::v1alpha2::TcpRoute;
use gateway_api::experimental::apisx::v1alpha1::XBackendTrafficPolicy;
```

> **Warning**: Experimental APIs may change or be removed without notice.

[experimental channel]:https://gateway-api.sigs.k8s.io/docs/concepts/versioning/#release-channels

## Development

See [DEVELOPMENT.md](DEVELOPMENT.md).

## Contributing

Contributions are welcome! For larger changes, please open an issue first.
Check the [project board][board] for unassigned tasks, and use the
[discussions board][forum] for questions.

[board]:https://github.com/orgs/kube-rs/projects/3
[forum]:https://github.com/kube-rs/gateway-api-rs/discussions

## License

Licensed under the [MIT License](LICENSE).
