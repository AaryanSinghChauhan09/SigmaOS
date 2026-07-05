// SPDX-License-Identifier: GPL-2.0-or-later
// SigmaOS Common Error Handling

use std::fmt;

/// SigmaOS error types
#[derive(Debug, Clone)]
pub enum SigmaError {
    /// Configuration error
    Config(String),
    /// System error
    System(String),
    /// Network error
    Network(String),
    /// Database error
    Database(String),
    /// AI error
    AI(String),
    /// File I/O error
    Io(String),
    /// Validation error
    Validation(String),
    /// Not found error
    NotFound(String),
    /// Permission error
    Permission(String),
    /// Generic error
    Generic(String),
}

impl fmt::Display for SigmaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SigmaError::Config(msg) => write!(f, "Configuration error: {}", msg),
            SigmaError::System(msg) => write!(f, "System error: {}", msg),
            SigmaError::Network(msg) => write!(f, "Network error: {}", msg),
            SigmaError::Database(msg) => write!(f, "Database error: {}", msg),
            SigmaError::AI(msg) => write!(f, "AI error: {}", msg),
            SigmaError::Io(msg) => write!(f, "I/O error: {}", msg),
            SigmaError::Validation(msg) => write!(f, "Validation error: {}", msg),
            SigmaError::NotFound(msg) => write!(f, "Not found: {}", msg),
            SigmaError::Permission(msg) => write!(f, "Permission denied: {}", msg),
            SigmaError::Generic(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for SigmaError {}

impl From<std::io::Error> for SigmaError {
    fn from(err: std::io::Error) -> Self {
        SigmaError::Io(err.to_string())
    }
}

impl From<serde_json::Error> for SigmaError {
    fn from(err: serde_json::Error) -> Self {
        SigmaError::Config(err.to_string())
    }
}

/// Result type alias for SigmaOS
pub type SigmaResult<T> = Result<T, SigmaError>;

/// Error context helper
pub trait ErrorContext<T> {
    fn context(self, context: &str) -> SigmaResult<T>;
}

impl<T> ErrorContext<T> for Result<T, SigmaError> {
    fn context(self, context: &str) -> SigmaResult<T> {
        self.map_err(|e| SigmaError::Generic(format!("{}: {}", context, e)))
    }
}
