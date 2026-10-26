// Builders of a channel's `common` types, included into each channel's
// `builders` module with `common` bound to the channel's `common`. Bodies for
// different resources build these alike, which checks that one Rust type
// serves them all against the API server.

/// A parent ref to the Gateway `test-gateway`.
pub fn parent_ref() -> common::ParentReference {
    common::ParentReference {
        name: "test-gateway".into(),
        ..Default::default()
    }
}
