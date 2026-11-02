// SigmaOS-theme-support
// Theme compatibility library — shared theming APIs and shims.

pub mod compat;
pub mod api;

/// Initialise the theme compatibility layer.
///
/// Call this once at application startup to wire up compatibility shims
/// and expose the unified theming API to callers.
pub fn init() {
    // TODO: detect active toolkit and apply appropriate compat shims
}
