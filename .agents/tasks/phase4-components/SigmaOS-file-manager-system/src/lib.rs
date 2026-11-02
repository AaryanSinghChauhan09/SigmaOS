//! SigmaOS-file-manager-system
//!
//! Core library for the SigmaOS file manager. Provides directory traversal,
//! search integration, bookmark management, and remote filesystem abstractions.

use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Directory entry
// ---------------------------------------------------------------------------

/// Represents a single item in a directory listing.
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub path: PathBuf,
    pub name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
}

impl DirEntry {
    pub fn new(path: PathBuf, is_dir: bool, size_bytes: u64) -> Self {
        let name = path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        Self { path, name, is_dir, size_bytes }
    }
}

impl fmt::Display for DirEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let kind = if self.is_dir { "DIR " } else { "FILE" };
        write!(f, "[{}] {} ({} bytes)", kind, self.name, self.size_bytes)
    }
}

// ---------------------------------------------------------------------------
// Directory listing
// ---------------------------------------------------------------------------

/// Returns a sorted listing of `dir`, directories first.
///
/// # Errors
/// Returns an `std::io::Error` if the directory cannot be read.
pub fn list_directory(dir: &Path) -> std::io::Result<Vec<DirEntry>> {
    let mut entries = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let meta = entry.metadata()?;
        entries.push(DirEntry::new(
            entry.path(),
            meta.is_dir(),
            if meta.is_file() { meta.len() } else { 0 },
        ));
    }

    // Sort: directories first, then alphabetically by name
    entries.sort_by(|a, b| {
        b.is_dir
            .cmp(&a.is_dir)
            .then_with(|| a.name.to_lowercase().cmp(&b.name.to_lowercase()))
    });

    Ok(entries)
}

// ---------------------------------------------------------------------------
// Bookmark store
// ---------------------------------------------------------------------------

/// A simple in-memory bookmark store keyed by user-defined label.
#[derive(Debug, Default)]
pub struct BookmarkStore {
    bookmarks: HashMap<String, PathBuf>,
}

impl BookmarkStore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, label: impl Into<String>, path: impl Into<PathBuf>) {
        self.bookmarks.insert(label.into(), path.into());
    }

    pub fn remove(&mut self, label: &str) -> bool {
        self.bookmarks.remove(label).is_some()
    }

    pub fn get(&self, label: &str) -> Option<&PathBuf> {
        self.bookmarks.get(label)
    }

    pub fn all(&self) -> Vec<(&String, &PathBuf)> {
        let mut items: Vec<_> = self.bookmarks.iter().collect();
        items.sort_by_key(|(k, _)| k.as_str());
        items
    }
}

// ---------------------------------------------------------------------------
// Search
// ---------------------------------------------------------------------------

/// Simple recursive filename search (omafind integration placeholder).
///
/// Walks `root` and returns entries whose names contain `query` (case-insensitive).
pub fn search(root: &Path, query: &str) -> Vec<DirEntry> {
    let query_lower = query.to_lowercase();
    let mut results = Vec::new();
    walk(root, &query_lower, &mut results);
    results
}

fn walk(dir: &Path, query: &str, results: &mut Vec<DirEntry>) {
    let Ok(rd) = std::fs::read_dir(dir) else { return };
    for entry in rd.flatten() {
        let Ok(meta) = entry.metadata() else { continue };
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.to_lowercase().contains(query) {
            results.push(DirEntry::new(entry.path(), meta.is_dir(), if meta.is_file() { meta.len() } else { 0 }));
        }
        if meta.is_dir() {
            walk(&entry.path(), query, results);
        }
    }
}

// ---------------------------------------------------------------------------
// Tab manager
// ---------------------------------------------------------------------------

/// Manages multiple open directory tabs.
#[derive(Debug)]
pub struct TabManager {
    tabs: Vec<PathBuf>,
    active: usize,
}

impl TabManager {
    pub fn new(initial: PathBuf) -> Self {
        Self { tabs: vec![initial], active: 0 }
    }

    pub fn open_tab(&mut self, path: PathBuf) -> usize {
        self.tabs.push(path);
        self.active = self.tabs.len() - 1;
        self.active
    }

    pub fn close_tab(&mut self, index: usize) -> bool {
        if self.tabs.len() <= 1 || index >= self.tabs.len() {
            return false;
        }
        self.tabs.remove(index);
        if self.active >= self.tabs.len() {
            self.active = self.tabs.len() - 1;
        }
        true
    }

    pub fn active_path(&self) -> &Path {
        &self.tabs[self.active]
    }

    pub fn tab_count(&self) -> usize {
        self.tabs.len()
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn bookmark_roundtrip() {
        let mut store = BookmarkStore::new();
        store.add("home", PathBuf::from("/home/user"));
        assert_eq!(store.get("home"), Some(&PathBuf::from("/home/user")));
        assert!(store.remove("home"));
        assert!(store.get("home").is_none());
    }

    #[test]
    fn tab_manager_open_close() {
        let mut mgr = TabManager::new(PathBuf::from("/home"));
        let idx = mgr.open_tab(PathBuf::from("/tmp"));
        assert_eq!(idx, 1);
        assert_eq!(mgr.tab_count(), 2);
        assert!(mgr.close_tab(1));
        assert_eq!(mgr.tab_count(), 1);
        // Cannot close last tab
        assert!(!mgr.close_tab(0));
    }

    #[test]
    fn dir_entry_display() {
        let e = DirEntry::new(PathBuf::from("/tmp/foo"), true, 0);
        assert!(e.to_string().contains("DIR"));
        assert!(e.to_string().contains("foo"));
    }
}
