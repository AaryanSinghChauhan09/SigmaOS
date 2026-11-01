//! SigmaOS File Manager — directory browser, search, and thumbnails.

pub mod browser;
pub mod search;
pub mod thumbnails;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum FmError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Path not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Thumbnail error: {0}")]
    Thumbnail(String),
}

pub type Result<T> = std::result::Result<T, FmError>;

/// A file system entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub path: std::path::PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub mime_type: String,
    pub symlink: bool,
}

/// A sidebar bookmark.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: std::path::PathBuf,
}
