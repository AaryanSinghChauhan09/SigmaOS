//! SigmaOS system backup utility.
//!
//! Provides rsync-based incremental snapshot management with a CLI.

#![warn(missing_docs)]

/// Type of snapshot filesystem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnapshotBackend {
    /// rsync-based snapshots on any filesystem.
    Rsync,
    /// BTRFS native snapshots.
    Btrfs,
}

/// A restore point created by the backup tool.
#[derive(Debug, Clone)]
pub struct Snapshot {
    /// Identifier for the snapshot.
    pub id: String,
    /// Timestamp of creation.
    pub created: u64,
    /// Label given by the user.
    pub label: String,
    /// Backend used to create the snapshot.
    pub backend: SnapshotBackend,
}

/// Trait for backup engines.
pub trait BackupEngine {
    /// Create a new snapshot.
    fn create(&mut self, label: &str) -> Result<Snapshot, BackupError>;
    /// Restore from an existing snapshot.
    fn restore(&self, id: &str) -> Result<(), BackupError>;
    /// List all available snapshots.
    fn list(&self) -> Result<Vec<Snapshot>, BackupError>;
}

/// Errors returned by the backup tool.
#[derive(Debug)]
pub enum BackupError {
    /// The snapshot could not be created.
    SnapshotFailed(String),
    /// The requested snapshot was not found.
    NotFound,
}

impl std::fmt::Display for BackupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BackupError::SnapshotFailed(msg) => write!(f, "snapshot failed: {msg}"),
            BackupError::NotFound => write!(f, "snapshot not found"),
        }
    }
}

impl std::error::Error for BackupError {}
