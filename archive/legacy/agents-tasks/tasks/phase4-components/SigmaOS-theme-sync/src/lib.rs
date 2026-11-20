// SigmaOS-theme-sync
// Cross-application theme synchronisation tool.

pub mod sync;
pub mod config;

/// Initialise the theme sync engine.
///
/// Call this once at application startup to parse the unified config
/// and trigger a full cross-application theme synchronisation.
pub fn init() {
    // TODO: parse config file and invoke sync engine for all targets
}
