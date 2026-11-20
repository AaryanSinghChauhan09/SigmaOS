//! SigmaOS unified control panel.
//!
//! Aggregates all system configuration panels into a single searchable
//! interface acting as launcher and container.

#![warn(missing_docs)]

/// A search result from the control center.
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// The panel id the result belongs to.
    pub panel_id: String,
    /// The matched setting label.
    pub label: String,
}

/// Trait for the control center shell.
pub trait ControlCenter {
    /// List all available panels.
    fn panels(&self) -> Vec<String>;
    /// Open a panel by id.
    fn open(&mut self, panel_id: &str) -> Result<(), CenterError>;
    /// Search across all panels.
    fn search(&self, query: &str) -> Vec<SearchResult>;
}

/// Errors returned by the control center.
#[derive(Debug)]
pub enum CenterError {
    /// The requested panel was not found.
    PanelNotFound,
    /// The panel failed to load.
    LoadFailed(String),
}

impl std::fmt::Display for CenterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CenterError::PanelNotFound => write!(f, "panel not found"),
            CenterError::LoadFailed(msg) => write!(f, "load failed: {msg}"),
        }
    }
}

impl std::error::Error for CenterError {}
