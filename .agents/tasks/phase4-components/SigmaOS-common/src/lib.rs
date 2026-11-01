//! SigmaOS-common: Shared libraries and utility functions for SigmaOS components.
//!
//! This crate is the single source of truth for types and helpers used across
//! the SigmaOS component ecosystem. Depend on it via Cargo to keep behaviour
//! consistent between components.

use std::fmt;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Version information
// ---------------------------------------------------------------------------

/// Build-time version information for a SigmaOS component.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentVersion {
    /// Semantic version string (e.g. "1.0.0").
    pub version: &'static str,
    /// Short name of the component (e.g. "SigmaOS-common").
    pub name: &'static str,
}

impl fmt::Display for ComponentVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} v{}", self.name, self.version)
    }
}

// ---------------------------------------------------------------------------
// Error types
// ---------------------------------------------------------------------------

/// Top-level error type used across SigmaOS components.
#[derive(Debug)]
pub enum SigmaError {
    /// An I/O operation failed.
    Io(std::io::Error),
    /// A configuration file could not be parsed.
    Config(String),
    /// A requested resource was not found.
    NotFound(String),
    /// An unexpected internal error occurred.
    Internal(String),
}

impl fmt::Display for SigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaError::Io(e) => write!(f, "I/O error: {e}"),
            SigmaError::Config(msg) => write!(f, "Configuration error: {msg}"),
            SigmaError::NotFound(resource) => write!(f, "Not found: {resource}"),
            SigmaError::Internal(msg) => write!(f, "Internal error: {msg}"),
        }
    }
}

impl std::error::Error for SigmaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SigmaError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SigmaError {
    fn from(e: std::io::Error) -> Self {
        SigmaError::Io(e)
    }
}

/// Convenience alias for `Result<T, SigmaError>`.
pub type SigmaResult<T> = Result<T, SigmaError>;

// ---------------------------------------------------------------------------
// Path utilities
// ---------------------------------------------------------------------------

/// Return the SigmaOS user-level configuration directory.
///
/// Defaults to `~/.config/sigmaos` if `XDG_CONFIG_HOME` is not set.
pub fn config_dir() -> PathBuf {
    let base = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs_path()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join(".config")
        });
    base.join("sigmaos")
}

/// Return the SigmaOS user-level data directory.
///
/// Defaults to `~/.local/share/sigmaos` if `XDG_DATA_HOME` is not set.
pub fn data_dir() -> PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            dirs_path()
                .unwrap_or_else(|| PathBuf::from("/tmp"))
                .join(".local/share")
        });
    base.join("sigmaos")
}

fn dirs_path() -> Option<PathBuf> {
    std::env::var("HOME").ok().map(PathBuf::from)
}

/// Ensure *dir* exists, creating it (and all parents) if necessary.
pub fn ensure_dir(dir: &Path) -> SigmaResult<()> {
    std::fs::create_dir_all(dir)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// String utilities
// ---------------------------------------------------------------------------

/// Truncate *s* to at most *max_chars* Unicode scalar values, appending `…`
/// if the string was shortened.
pub fn truncate(s: &str, max_chars: usize) -> String {
    let mut chars = s.chars();
    let collected: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{collected}…")
    } else {
        collected
    }
}

/// Convert a string to title case (first letter of each word capitalised).
pub fn title_case(s: &str) -> String {
    s.split_whitespace()
        .map(|word| {
            let mut c = word.chars();
            match c.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" ")
}

// ---------------------------------------------------------------------------
// Logging helpers
// ---------------------------------------------------------------------------

/// Log severity levels used by SigmaOS components.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
        }
    }
}

/// Emit a formatted log line to stderr, prefixed with the component name.
pub fn log(component: &str, level: LogLevel, message: &str) {
    eprintln!("[{level}] {component}: {message}");
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn truncate_short_string_unchanged() {
        assert_eq!(truncate("hello", 10), "hello");
    }

    #[test]
    fn truncate_long_string_gets_ellipsis() {
        let result = truncate("hello world", 5);
        assert!(result.ends_with('…'));
        assert_eq!(result, "hello…");
    }

    #[test]
    fn title_case_converts_correctly() {
        assert_eq!(title_case("hello world"), "Hello World");
    }

    #[test]
    fn component_version_display() {
        let v = ComponentVersion { version: "1.0.0", name: "SigmaOS-common" };
        assert_eq!(v.to_string(), "SigmaOS-common v1.0.0");
    }

    #[test]
    fn sigma_error_display_io() {
        let err = SigmaError::NotFound("test-resource".to_string());
        assert!(err.to_string().contains("test-resource"));
    }
}
