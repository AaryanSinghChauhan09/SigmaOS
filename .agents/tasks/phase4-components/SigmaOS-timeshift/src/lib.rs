//! SigmaOS-timeshift: System snapshot and restore tool.
//!
//! Create and restore system snapshots using rsync or btrfs for reliable
//! system recovery. Inspired by timeshift.

pub mod snapshot;
pub mod restore;

/// Initialise the timeshift snapshot engine.
///
/// Call once at application startup to set up snapshot backends.
pub fn init() {
    // TODO: detect available backend (rsync / btrfs) and configure paths
}
