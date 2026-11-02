//! SigmaOS display manager greeter UI.
//!
//! Provides the login screen with user selection, password entry, session
//! picker, and branding.

#![warn(missing_docs)]

/// A greeter session choice.
#[derive(Debug, Clone)]
pub struct SessionChoice {
    /// Identifier for the session.
    pub id: String,
    /// Display name.
    pub name: String,
}

/// A login attempt.
#[derive(Debug, Clone)]
pub struct LoginRequest {
    /// User selected in the greeter.
    pub username: String,
    /// Chosen session id.
    pub session: String,
}

/// Trait for the greeter controller.
pub trait Greeter {
    /// Populate the list of users to show.
    fn users(&self) -> Vec<String>;
    /// Populate the list of available sessions.
    fn sessions(&self) -> Vec<SessionChoice>;
    /// Authenticate a login request.
    fn authenticate(&mut self, request: &LoginRequest, password: &str) -> Result<(), GreeterError>;
}

/// Errors returned by the greeter.
#[derive(Debug)]
pub enum GreeterError {
    /// Authentication failed.
    AuthFailed,
    /// The selected session is not available.
    SessionUnavailable,
}

impl std::fmt::Display for GreeterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GreeterError::AuthFailed => write!(f, "authentication failed"),
            GreeterError::SessionUnavailable => write!(f, "session unavailable"),
        }
    }
}

impl std::error::Error for GreeterError {}
