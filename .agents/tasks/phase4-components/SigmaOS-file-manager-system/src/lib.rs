//! SigmaOS system-level file management.
//!
//! Provides mount point handling, file operation daemons, and extended
//! metadata indexing.

#![warn(missing_docs)]

/// A mounted filesystem.
#[derive(Debug, Clone)]
pub struct Mount {
    /// Device path or label.
    pub device: String,
    /// Mount point path.
    pub mount_point: String,
    /// Filesystem type (e.g. ext4).
    pub fs_type: String,
}

/// A file operation in progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileOp {
    /// Copying files.
    Copy,
    /// Moving files.
    Move,
    /// Deleting files.
    Delete,
}

/// Trait for the file manager backend.
pub trait FileManagerBackend {
    /// List all current mounts.
    fn mounts(&self) -> Result<Vec<Mount>, FileError>;
    /// Mount a device by path.
    fn mount(&mut self, device: &str) -> Result<Mount, FileError>;
    /// Perform a file operation.
    fn operate(&mut self, op: FileOp, sources: &[String], dest: &str) -> Result<(), FileError>;
}

/// Errors returned by the file manager system.
#[derive(Debug)]
pub enum FileError {
    /// The device could not be mounted.
    MountFailed(String),
    /// The file operation failed.
    OperationFailed(String),
    /// The path was not found.
    NotFound,
}

impl std::fmt::Display for FileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileError::MountFailed(msg) => write!(f, "mount failed: {msg}"),
            FileError::OperationFailed(msg) => write!(f, "operation failed: {msg}"),
            FileError::NotFound => write!(f, "not found"),
        }
    }
}

impl std::error::Error for FileError {}
