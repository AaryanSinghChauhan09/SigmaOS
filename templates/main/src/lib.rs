//! # COMPONENT_NAME
//!
//! SigmaOS component: COMPONENT_NAME.
//!
//! ## Responsibilities
//!
//! <!-- TODO: describe the single responsibility of this component -->
//!
//! ## Design Principles
//!
//! This crate follows the SigmaOS engineering principles:
//! KISS, DRY, SOLID, Separation of Concerns, Modularity, Component Isolation,
//! Composition over Inheritance, Defensive Programming, Memory Safety,
//! Least Privilege, Secure Defaults, YAGNI, Boy Scout Rule, Spec-Driven Development.
//!
//! ## Safety
//!
//! Unsafe code is forbidden in this crate. Any deviation from this rule
//! requires an explicit architecture decision record (ADR) on the docs branch.

// ---------------------------------------------------------------------------
// Compiler directives
// ---------------------------------------------------------------------------

/// Forbid unsafe code across this entire crate.
/// Remove ONLY for kernel/driver components that have a justified ADR.
#![forbid(unsafe_code)]

/// Enforce documentation on all public items.
#![warn(missing_docs)]

/// Enable all Clippy lints — warnings are treated as errors in CI via `make lint`.
#![warn(clippy::all, clippy::pedantic)]

// Allow a small set of Clippy pedantic lints that generate false positives in
// systems code.
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::must_use_candidate)]

// ---------------------------------------------------------------------------
// Public modules
// ---------------------------------------------------------------------------

pub mod config;
pub mod error;

// ---------------------------------------------------------------------------
// Re-exports — keep the public API surface minimal (YAGNI)
// ---------------------------------------------------------------------------

pub use config::Config;
pub use error::{Error, Result};

// ---------------------------------------------------------------------------
// Core type
// ---------------------------------------------------------------------------

/// The primary type exported by this component.
///
/// Encapsulates all state required to operate `COMPONENT_NAME`.
/// Construct via [`ComponentName::new`]; do not access fields directly.
#[derive(Debug)]
pub struct ComponentName {
    config: Config,
}

impl ComponentName {
    /// Create a new [`ComponentName`] instance with the provided configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidConfig`] if the configuration fails validation.
    pub fn new(config: Config) -> Result<Self> {
        config.validate()?;
        Ok(Self { config })
    }

    /// Return a reference to the active configuration.
    #[must_use]
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Execute the component's primary operation.
    ///
    /// # Errors
    ///
    /// Returns an [`Error`] on failure. All errors are recoverable unless
    /// documented otherwise.
    pub fn run(&self) -> Result<()> {
        log::info!("COMPONENT_NAME: starting run");
        // TODO: implement core logic
        log::info!("COMPONENT_NAME: run complete");
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// config sub-module
// ---------------------------------------------------------------------------

/// Configuration for [`ComponentName`].
pub mod config {
    use crate::error::{Error, Result};

    /// Runtime configuration for `COMPONENT_NAME`.
    ///
    /// All fields have sensible, secure defaults (see [`Config::default`]).
    /// Override individual fields as required; never expose raw secrets here.
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    pub struct Config {
        /// Human-readable name for this instance.
        pub name: String,

        /// Enable verbose diagnostic output.
        pub verbose: bool,
    }

    impl Config {
        /// Validate configuration invariants.
        ///
        /// # Errors
        ///
        /// Returns [`Error::InvalidConfig`] if any invariant is violated.
        pub fn validate(&self) -> Result<()> {
            if self.name.is_empty() {
                return Err(Error::InvalidConfig("name must not be empty".to_owned()));
            }
            Ok(())
        }
    }

    impl Default for Config {
        fn default() -> Self {
            Self {
                name: String::from("COMPONENT_NAME"),
                verbose: false,
            }
        }
    }
}

// ---------------------------------------------------------------------------
// error sub-module
// ---------------------------------------------------------------------------

/// Error types for [`ComponentName`].
pub mod error {
    /// All errors produced by this component.
    ///
    /// Variants are non-exhaustive to allow adding new cases without breaking
    /// downstream crates (open-closed principle).
    #[derive(Debug, thiserror::Error)]
    #[non_exhaustive]
    pub enum Error {
        /// The supplied configuration is invalid.
        #[error("invalid configuration: {0}")]
        InvalidConfig(String),

        /// An I/O operation failed.
        #[error("I/O error: {0}")]
        Io(#[from] std::io::Error),

        /// A serialisation or deserialisation error.
        #[error("serialisation error: {0}")]
        Serde(#[from] serde_json::Error),

        /// An unexpected internal error. This is a bug — please report it.
        #[error("internal error: {0}")]
        Internal(String),
    }

    /// Convenience alias: all fallible operations in this crate return this type.
    pub type Result<T, E = Error> = std::result::Result<T, E>;
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Config tests
    // -----------------------------------------------------------------------

    #[test]
    fn default_config_is_valid() {
        let cfg = Config::default();
        assert!(cfg.validate().is_ok(), "default config must be valid");
    }

    #[test]
    fn empty_name_is_rejected() {
        let cfg = Config {
            name: String::new(),
            ..Config::default()
        };
        let result = cfg.validate();
        assert!(result.is_err(), "empty name should fail validation");
    }

    // -----------------------------------------------------------------------
    // ComponentName tests
    // -----------------------------------------------------------------------

    #[test]
    fn new_with_default_config_succeeds() {
        let component = ComponentName::new(Config::default());
        assert!(component.is_ok(), "construction with default config must succeed");
    }

    #[test]
    fn run_succeeds_with_valid_config() {
        let component = ComponentName::new(Config::default()).expect("valid config");
        assert!(component.run().is_ok(), "run must succeed with default config");
    }

    // -----------------------------------------------------------------------
    // Error type tests
    // -----------------------------------------------------------------------

    #[test]
    fn error_display_is_informative() {
        let err = Error::InvalidConfig("test field is wrong".to_owned());
        let msg = err.to_string();
        assert!(msg.contains("test field is wrong"), "error message must contain context");
    }

    // -----------------------------------------------------------------------
    // Arrange–Act–Assert pattern example
    // -----------------------------------------------------------------------

    #[test]
    fn config_roundtrip_via_json() {
        // Arrange
        let original = Config::default();

        // Act
        let json = serde_json::to_string(&original).expect("serialise");
        let restored: Config = serde_json::from_str(&json).expect("deserialise");

        // Assert
        assert_eq!(original.name, restored.name);
        assert_eq!(original.verbose, restored.verbose);
    }
}
