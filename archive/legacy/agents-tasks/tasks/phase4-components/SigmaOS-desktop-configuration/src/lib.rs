//! SigmaOS desktop configuration profiles.
//!
//! Manages themes, panel layouts, applet sets, and workspace arrangements.
//! Allows saving and applying complete desktop configuration snapshots.

#![warn(missing_docs)]

/// A saved desktop profile.
#[derive(Debug, Clone)]
pub struct Profile {
    /// Name of the profile.
    pub name: String,
    /// Theme to apply.
    pub theme: String,
    /// Ordered panel layout descriptor.
    pub panels: Vec<String>,
    /// Enabled applet identifiers.
    pub applets: Vec<String>,
}

/// Trait for the profile manager.
pub trait ProfileManager {
    /// Save the current desktop state as a named profile.
    fn save(&mut self, name: &str) -> Result<Profile, ProfileError>;
    /// Apply a saved profile by name.
    fn apply(&mut self, name: &str) -> Result<(), ProfileError>;
    /// List all saved profiles.
    fn list(&self) -> Result<Vec<String>, ProfileError>;
}

/// Errors returned by the profile manager.
#[derive(Debug)]
pub enum ProfileError {
    /// The named profile does not exist.
    NotFound(String),
    /// The profile could not be written.
    WriteFailed(String),
}

impl std::fmt::Display for ProfileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProfileError::NotFound(name) => write!(f, "profile not found: {name}"),
            ProfileError::WriteFailed(msg) => write!(f, "write failed: {msg}"),
        }
    }
}

impl std::error::Error for ProfileError {}
