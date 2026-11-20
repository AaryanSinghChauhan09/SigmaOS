//! SigmaOS GTK+ extensions.
//!
//! Adds custom SigmaOS widgets, additional CSS variables, and themed dialog
//! overrides on top of the base GTK theme.

#![warn(missing_docs)]

/// A custom widget provided by GTK+.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CustomWidget {
    /// SigmaOS styled header bar.
    SigmaHeaderBar,
    /// SigmaOS styled dialog.
    SigmaDialog,
    /// SigmaOS sidebar.
    SigmaSidebar,
}

/// A CSS variable exposed by the extension.
#[derive(Debug, Clone)]
pub struct CssVariable {
    /// Name of the variable.
    pub name: String,
    /// Default value.
    pub value: String,
}

/// Trait for the extension loader.
pub trait ExtensionLoader {
    /// List custom widgets provided by the extension.
    fn widgets(&self) -> Vec<CustomWidget>;
    /// List CSS variables the extension defines.
    fn variables(&self) -> Vec<CssVariable>;
    /// Load the extension into a running GTK application.
    fn load(&mut self) -> Result<(), ExtensionError>;
}

/// Errors returned by the GTK+ extension.
#[derive(Debug)]
pub enum ExtensionError {
    /// The extension could not be loaded.
    LoadFailed(String),
    /// A required dependency is missing.
    DependencyMissing(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::LoadFailed(msg) => write!(f, "load failed: {msg}"),
            ExtensionError::DependencyMissing(dep) => write!(f, "dependency missing: {dep}"),
        }
    }
}

impl std::error::Error for ExtensionError {}
