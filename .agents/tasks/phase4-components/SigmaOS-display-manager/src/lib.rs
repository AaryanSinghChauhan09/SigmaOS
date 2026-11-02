//! SigmaOS display and login manager.
//!
//! Handles X11/Wayland session startup, user authentication, and greeter
//! orchestration.

#![warn(missing_docs)]

/// A login session.
#[derive(Debug, Clone)]
pub struct Session {
    /// Identifier for the session type (e.g. "sigma-x11").
    pub id: String,
    /// Display name shown in the greeter.
    pub name: String,
    /// Command to launch the session.
    pub exec: String,
    /// Whether this is a Wayland session.
    pub wayland: bool,
}

/// A user that can log in.
#[derive(Debug, Clone)]
pub struct User {
    /// Numeric uid.
    pub uid: u32,
    /// Login name.
    pub name: String,
    /// Real name for display.
    pub real_name: String,
}

/// Trait for the display manager backend.
pub trait DisplayManager {
    /// List users that can log in.
    fn users(&self) -> Result<Vec<User>, DisplayError>;
    /// List available sessions.
    fn sessions(&self) -> Result<Vec<Session>, DisplayError>;
    /// Start a session for a user.
    fn login(&mut self, user: &User, session: &Session) -> Result<(), DisplayError>;
}

/// Errors returned by the display manager.
#[derive(Debug)]
pub enum DisplayError {
    /// Authentication failed.
    AuthFailed,
    /// The requested session is not available.
    SessionNotFound,
    /// The display server could not start.
    DisplayServerFailed(String),
}

impl std::fmt::Display for DisplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DisplayError::AuthFailed => write!(f, "authentication failed"),
            DisplayError::SessionNotFound => write!(f, "session not found"),
            DisplayError::DisplayServerFailed(msg) => {
                write!(f, "display server failed: {msg}")
            }
        }
    }
}

impl std::error::Error for DisplayError {}
