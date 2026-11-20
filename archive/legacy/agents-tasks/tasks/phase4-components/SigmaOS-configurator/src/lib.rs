//! SigmaOS system configuration UI.
//!
//! Panels-based interface for display, input, network, sound, and account
//! settings.

#![warn(missing_docs)]

/// A configuration panel.
#[derive(Debug, Clone)]
pub struct Panel {
    /// Identifier for the panel.
    pub id: String,
    /// Display name shown in the sidebar.
    pub title: String,
    /// Icon name from the current theme.
    pub icon: String,
}

/// Trait for panel implementations.
pub trait ConfigPanel {
    /// Build the list of settings this panel exposes.
    fn build(&self) -> Vec<Setting>;
    /// Apply all changes made in the panel.
    fn apply(&mut self) -> Result<(), ConfiguratorError>;
}

/// A single setting within a panel.
#[derive(Debug, Clone)]
pub struct Setting {
    /// Key identifying the setting.
    pub key: String,
    /// Current value as a string.
    pub value: String,
}

/// Errors returned by the configurator.
#[derive(Debug)]
pub enum ConfiguratorError {
    /// A setting value was rejected.
    InvalidValue(String),
    /// The backend could not persist the change.
    PersistenceFailed(String),
}

impl std::fmt::Display for ConfiguratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfiguratorError::InvalidValue(msg) => write!(f, "invalid value: {msg}"),
            ConfiguratorError::PersistenceFailed(msg) => {
                write!(f, "persistence failed: {msg}")
            }
        }
    }
}

impl std::error::Error for ConfiguratorError {}
