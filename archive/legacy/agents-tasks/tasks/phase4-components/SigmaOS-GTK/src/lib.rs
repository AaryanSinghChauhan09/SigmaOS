//! SigmaOS GTK theme and widget library.
//!
//! Provides SigmaOS's visual identity across GTK3 and GTK4 applications
//! with Sass sources, compiled CSS, and custom widget overrides.

#![warn(missing_docs)]

/// A theme variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    /// Light variant.
    Light,
    /// Dark variant.
    Dark,
}

/// A compiled theme.
#[derive(Debug, Clone)]
pub struct Theme {
    /// Name of the theme.
    pub name: String,
    /// Variant of the theme.
    pub variant: ThemeVariant,
    /// Path to the compiled GTK3 CSS.
    pub gtk3_css: String,
    /// Path to the compiled GTK4 CSS.
    pub gtk4_css: String,
}

/// Trait for theme build backends.
pub trait ThemeBuilder {
    /// Compile Sass sources into a theme.
    fn build(&self, variant: ThemeVariant) -> Result<Theme, ThemeError>;
    /// Install a compiled theme to the user themes directory.
    fn install(&self, theme: &Theme) -> Result<(), ThemeError>;
}

/// Errors returned by the theme builder.
#[derive(Debug)]
pub enum ThemeError {
    /// The Sass compilation failed.
    CompileFailed(String),
    /// The installation target was not writable.
    InstallFailed(String),
}

impl std::fmt::Display for ThemeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThemeError::CompileFailed(msg) => write!(f, "compile failed: {msg}"),
            ThemeError::InstallFailed(msg) => write!(f, "install failed: {msg}"),
        }
    }
}

impl std::error::Error for ThemeError {}
