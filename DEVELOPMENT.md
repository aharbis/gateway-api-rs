# Development

## Code Generation

The API bindings are generated from upstream's Go source by
[`gateway-api-codegen`](codegen/README.md). Generation needs [Go] (the version
in `gateway-api/codegen/go.mod`) and a nightly `rustfmt`.

```console
$ make generate
```

This runs two steps:

1. **gofacts** reads the Go packages of the `sigs.k8s.io/gateway-api` version
   that `gateway-api/codegen/go.mod` pins, and the CRDs in that module. It
   writes one IR file per channel to `gateway-api/codegen/ir/`, and copies the
   CRDs to `gateway-api/tests/fixtures/crds/<channel>/`, which the integration
   tests install and `tests/schema.rs` compares the derived schemas against.
2. **The emitter** writes the Rust modules in `gateway-api/src/apis/` from the
   IR, and `cargo xtask generate` writes the `apis!` test macro in
   `gateway-api/tests/support/apis.rs`.

`gateway-api/codegen/config.yaml` says what's generated: the Go packages and
the Rust module each becomes, the channels, upstream's doc tags, the library
files and the Rust paths of Kubernetes types. All of these outputs are
generated: don't edit them by hand.

[Go]:https://go.dev/dl/

### Bumping Gateway API

Bump the pinned module, then regenerate:

```console
$ cd gateway-api/codegen
$ go get sigs.k8s.io/gateway-api@v1.6.0
$ go mod tidy
$ cd ../..
$ make generate
```

Then review the diff:

- `gateway-api/codegen/ir/` shows upstream's changes in Go terms: new and
  changed types, fields, consts and CRD versions. The diff between
  `standard.json` and `experimental.json` is the channel difference.
- `gateway-api/src/apis/` shows what they do to the crate.

Generation fails, naming what to change, when:

- upstream adds a `<gateway:...>` doc tag: add a rule for it under `tags` in
  `config.yaml`, mirroring what upstream's `tools/generator` does with it;
- a type uses a Kubernetes type the config doesn't map: add it under
  `external`;
- a CRD's version has no Go kind, or the CRDs aren't for the pinned version.

A new CRD gets a failing integration test stub in
`gateway-api/tests/integration/suite/`, to fill in. Run the tests: a new
difference from upstream's schema fails `tests/schema.rs`, to fix or to add to
its `ALLOWED` list with a reason. Finally, update the version in `README.md`.

## Testing

```console
$ make test.unit                         # Unit tests
$ make test.unit.experimental            # Unit tests with the experimental feature
$ make test.integration                  # Integration tests
$ make test.integration.experimental     # Integration tests with the experimental feature
$ make test.all                          # All of the above
```

Integration tests require Docker and [Kind](https://kind.sigs.k8s.io/) installed locally.
