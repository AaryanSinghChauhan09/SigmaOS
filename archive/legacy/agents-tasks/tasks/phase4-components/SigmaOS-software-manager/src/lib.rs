//! SigmaOS Software Manager
//!
//! Software center and app store for SigmaOS. Provides modules for browsing
//! the application catalog and managing package installation/removal.
//!
//! # Modules
//!
//! - [`catalog`]: Application discovery, search, categories, ratings, and screenshots.
//! - [`install`]: Package installation and removal operations.

pub mod catalog;
pub mod install;

/// Initialize the software manager subsystem.
///
/// Call once at application startup to set up internal state,
/// load configuration, and prepare the catalog backend.
pub fn init() {
    // TODO: load config, initialize catalog backend, set up install backend
}
