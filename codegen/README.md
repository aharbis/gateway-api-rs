# gateway-api-codegen

Generates a crate's Rust bindings for Kubernetes API groups from their Go
source: the same packages upstream's own Go SDK is. The `gateway-api` crate is
generated with it.

Generation has two steps:

1. **gofacts** (`gofacts/`, a Go package) reads the Go packages and CRDs that a
   crate's config names, applies the config's tag rules, and writes one IR file
   per profile. It also copies the CRDs to the crate's test fixtures.
2. **The emitter** (this Rust crate) reads the IR and writes the Rust modules.

The Go types are the only source of types. The CRDs supply each resource's
envelope (its names, scope, versions, printer columns) and, as fixtures, test
the wire shape.

## A crate's codegen directory

Each generated crate has a `codegen/` directory:

```
<crate>/codegen/
  go.mod, go.sum   pins the upstream module; replace → <repo>/codegen/gofacts
  deps.go          blank imports, so `go mod tidy` keeps the upstream module
  main.go          calls gofacts.Main()
  config.yaml
  ir/<profile>.json
```

`go run .` in that directory writes the IR (`-root` names the repository root,
which defaults to the directory `<crate>/codegen` is in). For `gateway-api`,
`cargo xtask generate` runs it and then the emitter.

Bumping the upstream version is `go get <module>@<version>` there, then
generating.

gofacts is shaped for any Go-based API group, not only Gateway API. Its tests
(`go test ./...` in `gofacts/`) include one that writes the IR of the
[Gateway API Inference Extension][Gateway API Inference Extension], pinned in
`gofacts/testdata/inference-extension/` with a config that has no profiles and
no tags.

The IR diff shows upstream's changes in Go terms, and the Rust diff
shows what they do to the crate.

[Gateway API Inference Extension]:https://github.com/kubernetes-sigs/gateway-api-inference-extension 


## Config

```yaml
module: sigs.k8s.io/gateway-api   # the Go module, pinned by go.mod
crate: gateway-api                # relative to the repository root
packages:                         # Go package (relative to the module): Rust module
  apis/v1: v1
  apisx/v1alpha1: apisx::v1alpha1
profiles:
  - name: standard
    crds: config/crd/standard     # relative to the module
    out: apis/standard            # relative to the crate's src/
  - name: experimental
    crds: config/crd/experimental
    out: apis/experimental
    feature: experimental         # gates the profile's module
tags: …                           # see below
library_files: [shared_types.go]  # types generated even where no CRD uses them
external:                         # Rust paths for other modules' Go types
  k8s.io/apimachinery/pkg/apis/meta/v1:
    Condition: k8s_openapi::apimachinery::pkg::apis::meta::v1::Condition
fixtures: tests/fixtures/crds     # relative to the crate; one directory per profile
```

Unknown keys are errors, on both sides.

- **Profiles.** A profile generates one module tree from one CRD directory. Its
  kinds and versions are exactly those its CRDs define; files that hold no
  `CustomResourceDefinition` are skipped. Its IR is `ir/<name>.json` and its
  fixtures are `<fixtures>/<name>/`. Without `profiles`, a top-level `crds` and
  `out` make one profile named `default`.
- **Library files.** Their types are generated in every package of a profile's
  groups, except those reachable only through items the profile drops.
- **External.** Every Go type from another module that a generated type uses
  must be mapped here.

Generation fails when the Go source and the CRDs disagree:

- a CRD's `*/bundle-version` annotation isn't the version `go.mod` pins;
- a CRD version has no Go kind (`+kubebuilder:object:root`) in the package for
  its group (`+groupName`) and version (the package name);
- that Go kind is marked `+kubebuilder:skipversion`.

## Tags

A tag is `<PREFIX:NAME>` or `<PREFIX:NAME=VALUE>` in a Go doc comment. A block
tag also has a closing `</PREFIX:NAME>`. Each rule names a tag and says what it
does:

```yaml
tags:
  prefix: gateway
  rules:
    - tag: experimental                 # drop every field or const that has it
      drop: item
      in: [standard]
    - tag: experimental:description     # drop the doc text between the tags
      drop: block
      in: [standard]
    - tag: util:excludeFromCRD
      drop: block
      in: all
    - tag: "{profile}:validation:Enum"  # replace a marker's value in that profile
      override: kubebuilder:validation:Enum
    - tag: validateIPAddress            # known, and only stripped
```

- `in` lists profiles or is `all`. Nothing is removed from a profile no rule
  lists.
- `override` rules must contain `{profile}`, which matches any profile's name;
  the override applies only in that profile.
- After the rules apply, every remaining tag is stripped from docs, and runs of
  blank lines collapse.
- A tag with the prefix that matches no rule is an error. A new kind of effect
  is a gofacts change; a new tag name is a config change.

Without `tags`, nothing is pruned.

## IR

The IR is the contract between gofacts and the emitter: `gofacts/ir.go` writes
it and `src/ir.rs` reads it. It's versioned by `irVersion`, which changes with
the format; the emitter rejects any other version. It's ordered and
deterministic, and committed with each crate.

It holds nothing specific to one API group: raw tags never reach it.

```json
{
  "irVersion": 1,
  "module": "sigs.k8s.io/gateway-api",
  "version": "v1.5.1",
  "profile": "standard",
  "packages": [ … ],
  "crds": [ … ]
}
```

- **Packages,** sorted by path, each with `path` (relative to the module),
  `name`, `group` (from `+groupName`), `types` and `consts`, both in source
  order (by file, then position).
- **Types:** every exported type, with `name`, `file`, `doc`, `markers` and a
  `kind`:
  - `alias` (`type X = Y`), with its right-hand side in `alias`;
  - `struct`, with `fields`, and `dropped`: the names and types of the fields
    the tag rules drop;
  - `scalar`, with its `basic` type;
  - `other`, with its `underlying` type, which the emitter rejects.

  A defined type declared from a named type (`type X v1.X`) also has
  `defined`. `enum` holds the type's `+kubebuilder:validation:Enum` values,
  `root` marks `+kubebuilder:object:root`, and `skipVersion` marks
  `+kubebuilder:skipversion`.
- **Fields:** `name`, `json` (absent for an inlined embedded struct),
  `inline`, `type`, `optional`, `enum` (the field's own marker, after
  overrides), `default` (the raw `+kubebuilder:default` value), `doc` and
  `markers`. `optional` follows controller-gen: `+optional` or
  `+kubebuilder:validation:Optional`, else `omitempty` or `omitzero`, unless
  `+required` or `+kubebuilder:validation:Required`.
- **Consts:** every const whose type is a named type of the module, with
  `name`, `type` (the type's key), `value` and `doc`, unless the tag rules drop
  it.
- **Type expressions:** exactly one of `named` (a key, `<package>.<name>`,
  where `<package>` is relative to the module for its own packages, else an
  import path), `basic`, `pointer`, `slice` or `map` (`key`, `value`).
- **CRDs,** sorted by file: `file`, `group`, `kind`, `listKind`, `plural`,
  `singular`, `scope`, `shortNames`, `categories`, and `versions` in the CRD's
  order, each with `name`, `package` (the Go package of its kind), `served`,
  `storage`, `deprecated`, `deprecationWarning`, `status` (the status
  subresource) and `printerColumns`.

## The emitter

Each profile is one module tree (`out`), with one module per Go package that
has anything to emit, named by the config's `packages`. Types are emitted in
Go source order, each a Rust type with the Go name in Rust casing: words
capitalized, runs of initialisms split (`HTTPURLRewriteFilter` is
`HttpUrlRewriteFilter`).

- **What's emitted:** the types the profile's kinds reach through their spec
  and status; the library files' types; and string types with consts (open
  sets such as condition types). Types reachable only through fields the
  profile drops are left out. Only packages of the profile's CRD groups
  re-export.
- **Aliases** of another package's type are `pub use`; aliases of unnamed
  types and named scalars that aren't enums are `pub type`. A defined type
  (`type X v1.X`) is a type of its own.
- **Structs** flatten embedded structs. Fields are the snake-cased JSON name,
  with a `_` suffix for a keyword (`type_`). Optional fields are `Option`,
  skipped when `None`. A struct derives `Default` only without required
  fields; with some, it gets a `new()` that takes them as `impl Into<T>`.
  A defaulted field's docs say its default; a field whose enum marker
  narrows its type's values lists them in its docs and its schema.
- **Enums** are emitted for string types with enum values: from their own
  marker, else their fields' markers, else their consts. They're
  `#[non_exhaustive]`, with an `Other(String)` variant, and convert from and to
  strings. A CRD-validated enum's schema lists its values; an open set's is a
  plain string. An enum implements `Default` only when every field that uses
  it has the same upstream default.
- **Integers** map exactly: `int32` is `i32`, `int` and `int64` are `i64`,
  `uint16` is `u16`.
- **Kinds:** each CRD version's spec struct derives `kube::CustomResource`,
  with the envelope's group, version, kind, plural, scope, status, short names,
  categories and printer columns. A version whose package doesn't declare the
  spec's Go type (`type HTTPRoute v1.HTTPRoute`) gets its own spec struct of
  that name, since kube needs one per version.

The Rust is built with `quote!`, printed with `prettyplease`, then formatted
with `rustfmt +nightly`.
