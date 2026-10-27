//! Which kopium types are one upstream Go type, and which types a non-root API
//! version declares itself. The CRDs don't name their types, so `derive`
//! builds this from the Go source. See `docs/specs/go-source-generation.md`.

/// The shared types table.
pub struct Table {
    /// In source order, which is the order types are generated in.
    pub shared_types: Vec<SharedType>,
    /// The types non-root API versions declare themselves.
    pub per_version: Vec<PerVersion>,
}

/// The types one non-root API version of a resource declares itself in Go,
/// rather than as aliases of the root version's: a defined type
/// (`type X v1.X`) or a struct of its own. Named by their kopium names.
pub struct PerVersion {
    pub resource: String,
    pub version: String,
    pub types: Vec<String>,
}

/// Where a shared type's one definition is: `common`, `k8s`, or a resource's
/// name.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Placement {
    /// The channel's `common` module.
    Common,
    /// `k8s-openapi`, which defines the Kubernetes types kopium doesn't
    /// recognise. Nothing is generated.
    K8s,
    /// The resource's root version module, for a type only it uses.
    Resource(String),
}

/// An upstream Go type that kopium generates at more than one path, or that
/// upstream shares across CRDs.
#[derive(Clone)]
pub struct SharedType {
    /// The Go type's name.
    pub go: String,
    /// The Rust name, from the Go name.
    pub rust: String,
    pub placement: Placement,
    /// Every kopium name the type replaces, in any channel or API version.
    pub kopium: Vec<String>,
}
