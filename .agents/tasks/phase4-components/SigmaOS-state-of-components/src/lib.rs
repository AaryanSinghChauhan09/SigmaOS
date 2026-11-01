//! SigmaOS State of Components
//!
//! Component health dashboard for SigmaOS. Aggregates build status, test
//! coverage, and dependency health across all SigmaOS components.
//!
//! # Modules
//!
//! - [`dashboard`]: Rendering and aggregation of component health data.
//! - [`health`]: Health check definitions and per-component status evaluation.

pub mod dashboard;
pub mod health;

/// Initialize the state-of-components subsystem.
///
/// Call once at startup to load component registry and configure
/// data sources for CI, coverage, and dependency health.
pub fn init() {
    // TODO: load component registry, configure health data sources
}
