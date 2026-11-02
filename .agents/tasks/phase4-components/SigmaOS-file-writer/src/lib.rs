//! SigmaOS file writing utility library.
//!
//! Provides atomic write, buffered write, and metadata-preserving copy
//! operations for use across SigmaOS components.

#![warn(missing_docs)]

/// Options controlling a write operation.
#[derive(Debug, Clone)]
pub struct WriteOptions {
    /// Whether the write should be atomic.
    pub atomic: bool,
    /// Whether to preserve file metadata on copy.
    pub preserve_metadata: bool,
    /// File mode to apply (if supported).
    pub mode: Option<u32>,
}

impl Default for WriteOptions {
    fn default() -> Self {
        Self {
            atomic: true,
            preserve_metadata: true,
            mode: None,
        }
    }
}

/// Trait for file writers.
pub trait FileWriter {
    /// Write contents to a path with the given options.
    fn write_file(&self, path: &str, contents: &[u8], opts: &WriteOptions) -> Result<(), WriteError>;
    /// Copy a file preserving metadata per the options.
    fn copy_file(&self, from: &str, to: &str, opts: &WriteOptions) -> Result<(), WriteError>;
}

/// Errors returned by the file writer.
#[derive(Debug)]
pub enum WriteError {
    /// The target path was not writable.
    NotWritable(String),
    /// The source file was not found.
    SourceNotFound,
    /// An I/O error occurred.
    Io(String),
}

impl std::fmt::Display for WriteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WriteError::NotWritable(path) => write!(f, "not writable: {path}"),
            WriteError::SourceNotFound => write!(f, "source not found"),
            WriteError::Io(msg) => write!(f, "io error: {msg}"),
        }
    }
}

impl std::error::Error for WriteError {}
