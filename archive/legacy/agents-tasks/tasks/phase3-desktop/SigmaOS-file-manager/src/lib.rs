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

/// Kind of directory entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryKind {
    File,
    Directory,
    Symlink,
}

/// A file system entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub kind: EntryKind,
    pub path: std::path::PathBuf,
    pub mime_type: String,
}

impl FileEntry {
    /// Create a regular file entry.
    pub fn file(name: impl Into<String>, size: u64) -> Self {
        let name = name.into();
        Self {
            path: std::path::PathBuf::from(&name),
            name,
            size,
            kind: EntryKind::File,
            mime_type: "application/octet-stream".into(),
        }
    }

    /// Create a directory entry.
    pub fn directory(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            path: std::path::PathBuf::from(&name),
            name,
            size: 0,
            kind: EntryKind::Directory,
            mime_type: "inode/directory".into(),
        }
    }

    /// Create a symlink entry.
    pub fn symlink(name: impl Into<String>) -> Self {
        let name = name.into();
        Self {
            path: std::path::PathBuf::from(&name),
            name,
            size: 0,
            kind: EntryKind::Symlink,
            mime_type: "inode/symlink".into(),
        }
    }

    pub fn is_dir(&self) -> bool { self.kind == EntryKind::Directory }
    pub fn is_symlink(&self) -> bool { self.kind == EntryKind::Symlink }
}

/// A sidebar bookmark.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: std::path::PathBuf,
}

// ── DirectoryView ─────────────────────────────────────────────────────────────

/// In-memory view of a directory's contents.
#[derive(Debug, Default)]
pub struct DirectoryView {
    pub current_path: String,
    entries: Vec<FileEntry>,
}

impl DirectoryView {
    pub fn new() -> Self { Self::default() }

    /// Load entries for a given path.
    pub fn load(&mut self, path: impl Into<String>, entries: Vec<FileEntry>) {
        self.current_path = path.into();
        self.entries = entries;
    }

    /// Return all entries.
    pub fn entries(&self) -> &[FileEntry] {
        &self.entries
    }

    /// Filter entries by name (case-insensitive substring).
    pub fn filter(&self, pattern: &str) -> Vec<&FileEntry> {
        let lower = pattern.to_lowercase();
        self.entries
            .iter()
            .filter(|e| e.name.to_lowercase().contains(&lower))
            .collect()
    }
}

// ── SearchProvider ────────────────────────────────────────────────────────────

/// Simple in-memory search index: path → entries.
#[derive(Debug, Default)]
pub struct SearchProvider {
    index: Vec<(String, FileEntry)>,
}

impl SearchProvider {
    pub fn new() -> Self { Self::default() }

    /// Add an entry to the index.
    pub fn index(&mut self, path: impl Into<String>, entry: FileEntry) {
        self.index.push((path.into(), entry));
    }

    /// Search for entries whose name matches `pattern` (case-insensitive).
    pub fn search(&self, pattern: &str) -> Vec<&(String, FileEntry)> {
        let lower = pattern.to_lowercase();
        self.index
            .iter()
            .filter(|(_, e)| e.name.to_lowercase().contains(&lower))
            .collect()
    }
}

// ── FileManager ───────────────────────────────────────────────────────────────

/// Top-level file manager state.
#[derive(Debug)]
pub struct FileManager {
    pub root_path: String,
    pub view: DirectoryView,
    pub bookmarks: Vec<Bookmark>,
}

/// Open a file manager rooted at `path`.
pub fn open_file_manager(path: impl Into<String>) -> FileManager {
    FileManager {
        root_path: path.into(),
        view: DirectoryView::default(),
        bookmarks: Vec::new(),
    }
}
