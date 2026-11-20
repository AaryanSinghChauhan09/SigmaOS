//! SigmaOS-upgrade-legacy
//!
//! Legacy system upgrade support tool for SigmaOS.
//! Enables upgrade paths from older or unsupported SigmaOS versions,
//! handling compatibility and migration tasks.
//!
//! # Modules
//! - [`migration`]: Upgrade path logic and data migration helpers
//! - [`compat`]: Compatibility checks for legacy SigmaOS versions

pub mod migration;
pub mod compat;

/// Initialise the legacy upgrade subsystem.
///
/// Call this once at application startup to wire up both modules
/// before invoking any migration or compatibility checks.
pub fn init() {
    // TODO: register migration paths and compat checks
}
