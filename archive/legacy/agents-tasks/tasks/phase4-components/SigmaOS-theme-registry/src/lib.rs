// SigmaOS-theme-registry
// Central theme registry and metadata store.

pub mod registry;
pub mod metadata;

/// Initialise the theme registry.
///
/// Call this once at application startup to load the on-disk registry
/// and make metadata available for queries.
pub fn init() {
    // TODO: read registry from disk and build in-memory index
}
