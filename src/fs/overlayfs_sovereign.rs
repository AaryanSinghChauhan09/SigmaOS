//! SigmaOS Sovereign OverlayFS
//! Implements Linux overlayfs (overlay2) union filesystem in 100% safe Rust.
//!
//! Linux overlayfs was merged in Linux 3.18 (2014) and is the default
//! Docker/Podman container storage driver (overlay2).
//! It provides a union of:
//!   - lower: read-only base layer(s)
//!   - upper: read-write writable layer
//!   - work:  internal scratch directory (opaque whiteouts)
//!   - merged: unified view

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── File Entry ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum OverlayEntryKind {
    Regular,
    Directory,
    Symlink,
    Whiteout,   // deletion marker — hides file from lower layer
    Opaque,     // directory was deleted and re-created in upper
}

#[derive(Debug, Clone)]
pub struct OverlayEntry {
    pub path: String,
    pub kind: OverlayEntryKind,
    pub content: Vec<u8>,    // file content (simplified, not chunked)
    pub symlink_target: Option<String>,
    pub size: u64,
    pub modified: bool,       // true if in upper layer
}

impl OverlayEntry {
    pub fn file(path: &str, content: &[u8]) -> Self {
        OverlayEntry {
            path: path.to_string(),
            kind: OverlayEntryKind::Regular,
            content: content.to_vec(),
            symlink_target: None,
            size: content.len() as u64,
            modified: false,
        }
    }

    pub fn dir(path: &str) -> Self {
        OverlayEntry {
            path: path.to_string(),
            kind: OverlayEntryKind::Directory,
            content: Vec::new(),
            symlink_target: None,
            size: 0,
            modified: false,
        }
    }

    pub fn whiteout(path: &str) -> Self {
        OverlayEntry {
            path: path.to_string(),
            kind: OverlayEntryKind::Whiteout,
            content: Vec::new(),
            symlink_target: None,
            size: 0,
            modified: true,
        }
    }

    pub fn symlink(path: &str, target: &str) -> Self {
        OverlayEntry {
            path: path.to_string(),
            kind: OverlayEntryKind::Symlink,
            content: Vec::new(),
            symlink_target: Some(target.to_string()),
            size: target.len() as u64,
            modified: false,
        }
    }
}

// ─── Layer ────────────────────────────────────────────────────────────────────

pub struct OverlayLayer {
    pub id: String,
    pub entries: Vec<OverlayEntry>,
    pub writable: bool,
    pub size_bytes: u64,
}

impl OverlayLayer {
    pub fn new_lower(id: &str) -> Self {
        OverlayLayer { id: id.to_string(), entries: Vec::new(), writable: false, size_bytes: 0 }
    }

    pub fn new_upper(id: &str) -> Self {
        OverlayLayer { id: id.to_string(), entries: Vec::new(), writable: true, size_bytes: 0 }
    }

    pub fn add_entry(&mut self, entry: OverlayEntry) {
        self.size_bytes = self.size_bytes.saturating_add(entry.size);
        self.entries.push(entry);
    }

    pub fn get(&self, path: &str) -> Option<&OverlayEntry> {
        self.entries.iter().find(|e| e.path == path)
    }

    pub fn get_mut(&mut self, path: &str) -> Option<&mut OverlayEntry> {
        self.entries.iter_mut().find(|e| e.path == path)
    }

    pub fn contains(&self, path: &str) -> bool {
        self.entries.iter().any(|e| e.path == path)
    }
}

// ─── Sovereign OverlayFS ──────────────────────────────────────────────────────

pub struct SovereignOverlayFs {
    pub lower_layers: Vec<OverlayLayer>,  // ordered: [topmost, ..., bottommost]
    pub upper_layer: OverlayLayer,
    pub copy_up_count: u64,   // number of copy-up operations performed
    pub write_count: u64,
    pub lookup_count: u64,
}

impl SovereignOverlayFs {
    pub fn new(upper_id: &str) -> Self {
        SovereignOverlayFs {
            lower_layers: Vec::new(),
            upper_layer: OverlayLayer::new_upper(upper_id),
            copy_up_count: 0,
            write_count: 0,
            lookup_count: 0,
        }
    }

    pub fn add_lower_layer(&mut self, layer: OverlayLayer) {
        self.lower_layers.push(layer);
    }

    /// Lookup a path in the merged view (upper first, then lower layers).
    pub fn lookup(&mut self, path: &str) -> Option<OverlayEntry> {
        self.lookup_count = self.lookup_count.saturating_add(1);

        // 1. Check upper layer first
        if let Some(entry) = self.upper_layer.get(path) {
            if entry.kind == OverlayEntryKind::Whiteout {
                return None; // Whiteout = file deleted
            }
            return Some(entry.clone());
        }

        // 2. Check lower layers top-to-bottom
        for layer in &self.lower_layers {
            if let Some(entry) = layer.get(path) {
                // Whiteout in lower masks files further down
                if entry.kind == OverlayEntryKind::Whiteout {
                    return None;
                }
                return Some(entry.clone());
            }
        }
        None
    }

    /// Write a file — performs copy-up if file exists only in lower layers.
    pub fn write(&mut self, path: &str, content: &[u8]) -> bool {
        self.write_count = self.write_count.saturating_add(1);

        // If file exists in lower but not upper, copy-up first
        if !self.upper_layer.contains(path) {
            let lower_entry = self.lower_layers.iter()
                .find_map(|l| l.get(path))
                .cloned();

            if let Some(mut entry) = lower_entry {
                // Copy-up: clone entry to upper layer
                entry.content = content.to_vec();
                entry.size = content.len() as u64;
                entry.modified = true;
                self.upper_layer.add_entry(entry);
                self.copy_up_count = self.copy_up_count.saturating_add(1);
                return true;
            }
        }

        // Create/update in upper layer
        if let Some(entry) = self.upper_layer.get_mut(path) {
            entry.content = content.to_vec();
            entry.size = content.len() as u64;
            entry.modified = true;
            true
        } else {
            let mut entry = OverlayEntry::file(path, content);
            entry.modified = true;
            self.upper_layer.add_entry(entry);
            true
        }
    }

    /// Delete a file — creates a whiteout in the upper layer.
    pub fn delete(&mut self, path: &str) -> bool {
        // Remove from upper if present
        if let Some(idx) = self.upper_layer.entries.iter().position(|e| e.path == path) {
            self.upper_layer.entries.remove(idx);
        }

        // If it exists in lower, create a whiteout
        let in_lower = self.lower_layers.iter().any(|l| l.contains(path));
        if in_lower {
            self.upper_layer.add_entry(OverlayEntry::whiteout(path));
        }
        true
    }

    /// List all visible paths in the merged view.
    pub fn list_merged(&mut self) -> Vec<String> {
        let mut paths: Vec<String> = Vec::new();

        // Collect from lower layers (bottom-to-top for proper override)
        for layer in self.lower_layers.iter().rev() {
            for entry in &layer.entries {
                if entry.kind != OverlayEntryKind::Whiteout
                    && !paths.iter().any(|p| p == &entry.path)
                {
                    paths.push(entry.path.clone());
                }
            }
        }

        // Apply upper layer (additions, deletions, modifications)
        for entry in &self.upper_layer.entries {
            if entry.kind == OverlayEntryKind::Whiteout {
                // Remove from list
                paths.retain(|p| p != &entry.path);
            } else if !paths.iter().any(|p| p == &entry.path) {
                paths.push(entry.path.clone());
            }
        }

        paths.sort();
        paths
    }

    pub fn upper_diff_count(&self) -> usize {
        self.upper_layer.entries.iter().filter(|e| e.modified).count()
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    fn setup_overlay() -> SovereignOverlayFs {
        let mut lower = OverlayLayer::new_lower("base");
        lower.add_entry(OverlayEntry::file("/etc/hosts", b"127.0.0.1 localhost"));
        lower.add_entry(OverlayEntry::file("/etc/passwd", b"root:x:0:0:::/bin/sh"));
        lower.add_entry(OverlayEntry::dir("/usr/bin"));
        lower.add_entry(OverlayEntry::file("/usr/bin/ls", b"ELF..."));

        let mut overlay = SovereignOverlayFs::new("container-upper");
        overlay.add_lower_layer(lower);
        overlay
    }

    #[test]
    fn test_overlay_lookup_lower_file() {
        let mut overlay = setup_overlay();
        let entry = overlay.lookup("/etc/hosts").unwrap();
        assert_eq!(entry.content, b"127.0.0.1 localhost");
    }

    #[test]
    fn test_overlay_write_triggers_copy_up() {
        let mut overlay = setup_overlay();
        overlay.write("/etc/hosts", b"127.0.0.1 myhost");
        assert_eq!(overlay.copy_up_count, 1);
        let entry = overlay.lookup("/etc/hosts").unwrap();
        assert_eq!(entry.content, b"127.0.0.1 myhost");
    }

    #[test]
    fn test_overlay_delete_creates_whiteout() {
        let mut overlay = setup_overlay();
        overlay.delete("/etc/passwd");
        assert!(overlay.lookup("/etc/passwd").is_none()); // Whiteout hides it
        // Whiteout present in upper
        assert!(overlay.upper_layer.entries.iter().any(|e| e.path == "/etc/passwd" && e.kind == OverlayEntryKind::Whiteout));
    }

    #[test]
    fn test_overlay_new_file_in_upper() {
        let mut overlay = setup_overlay();
        overlay.write("/app/config.json", b"{\"key\": \"value\"}");
        assert_eq!(overlay.copy_up_count, 0); // No copy-up — new file
        let entry = overlay.lookup("/app/config.json").unwrap();
        assert_eq!(entry.content, b"{\"key\": \"value\"}");
    }

    #[test]
    fn test_overlay_merged_listing() {
        let mut overlay = setup_overlay();
        overlay.write("/etc/sigma.conf", b"sigma=1"); // new in upper
        overlay.delete("/etc/passwd");               // whiteout
        let merged = overlay.list_merged();
        assert!(merged.contains(&"/etc/hosts".to_string()));
        assert!(merged.contains(&"/etc/sigma.conf".to_string()));
        assert!(!merged.contains(&"/etc/passwd".to_string())); // whited out
    }

    #[test]
    fn test_overlay_upper_diff() {
        let mut overlay = setup_overlay();
        overlay.write("/etc/hosts", b"modified");
        overlay.write("/new/file", b"new");
        assert_eq!(overlay.upper_diff_count(), 2);
    }
}
