//! Error types shared across SigmaOS components.

use std::fmt;

/// The top-level error type for SigmaOS components.
#[derive(Debug)]
pub enum SigmaError {
    /// A configuration value was missing or invalid.
    Config(String),
    /// An I/O operation failed.
    Io(std::io::Error),
}

impl fmt::Display for SigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaError::Config(msg) => write!(f, "config error: {msg}"),
            SigmaError::Io(err) => write!(f, "io error: {err}"),
        }
    }
}

impl std::error::Error for SigmaError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            SigmaError::Io(err) => Some(err),
            _ => None,
        }
    }
}

impl From<std::io::Error> for SigmaError {
    fn from(err: std::io::Error) -> Self {
        SigmaError::Io(err)
    }
}
