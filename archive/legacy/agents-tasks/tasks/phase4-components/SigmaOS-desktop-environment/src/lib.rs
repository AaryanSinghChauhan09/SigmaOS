//! SigmaOS desktop environment core.
//!
//! Provides the compositor, window manager, panel, system tray, and session
//! management.

#![warn(missing_docs)]

/// A window managed by the environment.
#[derive(Debug, Clone)]
pub struct Window {
    /// X11 or Wayland window id.
    pub id: u64,
    /// Window title.
    pub title: String,
    /// Application identifier owning the window.
    pub app_id: String,
}

/// Workspace state within a monitor.
#[derive(Debug, Clone)]
pub struct Workspace {
    /// Index of the workspace.
    pub index: u32,
    /// Windows currently on the workspace.
    pub windows: Vec<Window>,
}

/// Trait for the window manager backend.
pub trait WindowManager {
    /// Open a window on the current workspace.
    fn add_window(&mut self, window: Window) -> Result<(), DesktopError>;
    /// Close a window by id.
    fn close_window(&mut self, id: u64) -> Result<(), DesktopError>;
    /// Switch to a workspace by index.
    fn switch_workspace(&mut self, index: u32) -> Result<(), DesktopError>;
}

/// Errors returned by the desktop environment.
#[derive(Debug)]
pub enum DesktopError {
    /// The window was not found.
    WindowNotFound,
    /// The workspace index is out of range.
    InvalidWorkspace,
    /// The compositor is not available.
    CompositorUnavailable,
}

impl std::fmt::Display for DesktopError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DesktopError::WindowNotFound => write!(f, "window not found"),
            DesktopError::InvalidWorkspace => write!(f, "invalid workspace"),
            DesktopError::CompositorUnavailable => write!(f, "compositor unavailable"),
        }
    }
}

impl std::error::Error for DesktopError {}
