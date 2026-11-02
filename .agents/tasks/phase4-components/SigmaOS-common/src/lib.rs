//! SigmaOS common library.
//!
//! Shared types, error handling, configuration parsing, and helpers used
//! across all SigmaOS components.

#![warn(missing_docs)]

pub mod config;
pub mod error;

pub use config::Config;
pub use error::SigmaError;

/// Returns the SigmaOS version string baked into the build.
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
