//! SigmaOS settings daemon.
//!
//! Applies user preferences for keyboard, mouse, display, power, and
//! accessibility at login and on change.

#![warn(missing_docs)]

/// A category of settings the daemon manages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    /// Keyboard settings.
    Keyboard,
    /// Mouse and touchpad settings.
    Mouse,
    /// Display and monitor settings.
    Display,
    /// Power management settings.
    Power,
    /// Accessibility settings.
    Accessibility,
}

/// A setting change request.
#[derive(Debug, Clone)]
pub struct SettingsChange {
    /// Category the change belongs to.
    pub category: SettingsCategory,
    /// Key being changed.
    pub key: String,
    /// New value.
    pub value: String,
}

/// Trait for the settings daemon backend.
pub trait SettingsDaemon {
    /// Apply a setting change immediately.
    fn apply(&mut self, change: &SettingsChange) -> Result<(), SettingsError>;
    /// Reload all settings from disk.
    fn reload(&mut self) -> Result<(), SettingsError>;
}

/// Errors returned by the settings daemon.
#[derive(Debug)]
pub enum SettingsError {
    /// The setting key was not recognized.
    UnknownKey(String),
    /// The value could not be applied.
    ApplyFailed(String),
}

impl std::fmt::Display for SettingsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SettingsError::UnknownKey(key) => write!(f, "unknown key: {key}"),
            SettingsError::ApplyFailed(msg) => write!(f, "apply failed: {msg}"),
        }
    }
}

impl std::error::Error for SettingsError {}
