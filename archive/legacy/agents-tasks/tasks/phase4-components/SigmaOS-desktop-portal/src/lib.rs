//! SigmaOS XDG desktop portal implementation.
//!
//! Enables sandboxed applications to request file access, camera,
//! screenshots, and notifications through a standardized D-Bus API.

#![warn(missing_docs)]

/// A portal request kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PortalRequest {
    /// Open file(s).
    OpenFile,
    /// Save a file.
    SaveFile,
    /// Take a screenshot.
    Screenshot,
    /// Access the camera.
    Camera,
    /// Show a notification.
    Notify,
}

/// A portal access response.
#[derive(Debug, Clone)]
pub struct PortalResponse {
    /// Whether the request was granted.
    pub granted: bool,
    /// URIs returned for file requests.
    pub uris: Vec<String>,
}

/// Trait for portal backends.
pub trait PortalBackend {
    /// Handle a portal request from an application.
    fn handle(&self, app_id: &str, request: PortalRequest) -> Result<PortalResponse, PortalError>;
}

/// Errors returned by the portal.
#[derive(Debug)]
pub enum PortalError {
    /// The request was denied by the user or policy.
    Denied,
    /// The requested feature is not implemented.
    NotImplemented,
}

impl std::fmt::Display for PortalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PortalError::Denied => write!(f, "request denied"),
            PortalError::NotImplemented => write!(f, "not implemented"),
        }
    }
}

impl std::error::Error for PortalError {}
