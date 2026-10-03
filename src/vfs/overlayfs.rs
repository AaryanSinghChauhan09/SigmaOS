//! SigmaOS — OverlayFS Union Filesystem
//! SPDX-License-Identifier: MIT OR GPL-2.0
//! Inspired by Linux fs/overlayfs/
//!
//! Provides a union filesystem with:
//! - upper (writable) + lower (read-only) layers
//! - Copy-up: transparent copy of lower files to upper on first write
//! - Whiteout: deletion of lower-layer files
//! - Merged readdir: merged view of upper + lower, respecting whiteouts
//! - Multiple lower layers (overlay stacking)

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};
use std::string::{String, ToString};
use std::vec::Vec;

// ── Errors ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayError {
    /// File not found in any layer
    NotFound,
    /// File is a whiteout entry (deleted)
    Whiteout,
    /// Upper layer is read-only (should not happen, but defensive)
    ReadOnly,
    /// Operation not permitted
    NotPermitted,
    /// I/O error in underlying layer
    IoError(String),
}

impl core::fmt::Display for OverlayError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotFound => write!(f, "overlayfs: not found"),
            Self::Whiteout => write!(f, "overlayfs: whiteout (deleted)"),
            Self::ReadOnly => write!(f, "overlayfs: read-only"),
            Self::NotPermitted => write!(f, "overlayfs: not permitted"),
            Self::IoError(s) => write!(f, "overlayfs: I/O error: {}", s),
        }
    }
}

// ── File handle ───────────────────────────────────────────────────────────────

/// Simulated file handle returned by open operations
#[derive(Debug, Clone)]
pub struct FileHandle {
    /// Full path within the overlay
    pub path: String,
    /// Whether this handle allows writes
    pub writable: bool,
    /// Layer the file lives in (upper or lower index)
    pub layer: FileLayer,
    /// File content (simulated)
    pub content: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileLayer {
    Upper,
    Lower(usize),
}

// ── OverlayEntry ──────────────────────────────────────────────────────────────

/// Result of an overlay lookup
#[derive(Debug, Clone)]
pub struct OverlayEntry {
    /// Path relative to overlay root
    pub path: String,
    /// Present in upper layer
    pub in_upper: bool,
    /// Present in at least one lower layer
    pub in_lower: bool,
    /// This path is a whiteout marker
    pub whiteout: bool,
    /// Index of the lowest layer that has this file (if in_lower)
    pub lower_index: Option<usize>,
}

// ── Layer storage (simulated in-memory) ───────────────────────────────────────

/// A simulated filesystem layer: maps path → file bytes
#[derive(Debug, Default, Clone)]
pub struct FsLayer {
    /// path → content. A whiteout file has content `.wh.` prefix in metadata.
    pub files: HashMap<String, Vec<u8>>,
    /// Set of whiteout paths (directories whose entries are hidden)
    pub whiteouts: HashSet<String>,
}

impl FsLayer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exists(&self, path: &str) -> bool {
        self.files.contains_key(path)
    }

    pub fn is_whiteout(&self, path: &str) -> bool {
        self.whiteouts.contains(path)
    }

    pub fn read(&self, path: &str) -> Option<&Vec<u8>> {
        self.files.get(path)
    }

    pub fn write(&mut self, path: &str, data: Vec<u8>) {
        self.files.insert(path.to_string(), data);
    }

    pub fn remove(&mut self, path: &str) {
        self.files.remove(path);
        self.whiteouts.insert(path.to_string());
    }

    /// List all entries in a directory (non-recursive)
    pub fn readdir(&self, dir: &str) -> Vec<String> {
        let prefix = if dir.ends_with('/') {
            dir.to_string()
        } else {
            format!("{}/", dir)
        };
        self.files
            .keys()
            .filter(|p| {
                let rel = p.strip_prefix(&prefix).unwrap_or("");
                !rel.is_empty() && !rel.contains('/')
            })
            .cloned()
            .collect()
    }
}

// ── OverlayFs ─────────────────────────────────────────────────────────────────

/// OverlayFS mount point
pub struct OverlayFs {
    /// Upper (writable) layer
    pub upper: FsLayer,
    /// Lower layers (read-only), ordered top to bottom
    pub lowers: Vec<FsLayer>,
    /// Work directory (used for atomic copy-up; simulated here)
    pub work_dir: String,
    /// Upper dir path label
    pub upper_dir: String,
    /// Lower dir path labels
    pub lower_dirs: Vec<String>,
}

impl OverlayFs {
    /// Create and mount an OverlayFS
    pub fn new(upper_dir: &str, lower_dirs: Vec<&str>, work_dir: &str) -> Self {
        Self {
            upper: FsLayer::new(),
            lowers: lower_dirs.iter().map(|_| FsLayer::new()).collect(),
            work_dir: work_dir.to_string(),
            upper_dir: upper_dir.to_string(),
            lower_dirs: lower_dirs.iter().map(|s| s.to_string()).collect(),
        }
    }

    /// Pre-populate a lower layer with files (for testing / setup)
    pub fn seed_lower(&mut self, layer_idx: usize, path: &str, data: Vec<u8>) {
        if layer_idx < self.lowers.len() {
            self.lowers[layer_idx].write(path, data);
        }
    }

    /// Pre-populate the upper layer with files
    pub fn seed_upper(&mut self, path: &str, data: Vec<u8>) {
        self.upper.write(path, data);
    }
}

// ── Overlay operations ────────────────────────────────────────────────────────

/// Look up a path in the overlay: check upper first, then lower layers in order.
/// Respect whiteouts.
pub fn overlay_lookup(ofs: &OverlayFs, path: &str) -> Option<OverlayEntry> {
    // Check upper
    if ofs.upper.is_whiteout(path) {
        return Some(OverlayEntry {
            path: path.to_string(),
            in_upper: false,
            in_lower: false,
            whiteout: true,
            lower_index: None,
        });
    }
    if ofs.upper.exists(path) {
        return Some(OverlayEntry {
            path: path.to_string(),
            in_upper: true,
            in_lower: false,
            whiteout: false,
            lower_index: None,
        });
    }
    // Check lower layers in order
    for (i, lower) in ofs.lowers.iter().enumerate() {
        if lower.is_whiteout(path) {
            // Whiteout in lower: hide from further lowers
            return Some(OverlayEntry {
                path: path.to_string(),
                in_upper: false,
                in_lower: false,
                whiteout: true,
                lower_index: Some(i),
            });
        }
        if lower.exists(path) {
            return Some(OverlayEntry {
                path: path.to_string(),
                in_upper: false,
                in_lower: true,
                whiteout: false,
                lower_index: Some(i),
            });
        }
    }
    None
}

/// Open a file for reading. Returns a FileHandle with its content.
pub fn overlay_open_read(ofs: &OverlayFs, path: &str) -> Result<FileHandle, OverlayError> {
    let entry = overlay_lookup(ofs, path).ok_or(OverlayError::NotFound)?;
    if entry.whiteout {
        return Err(OverlayError::Whiteout);
    }
    let content = if entry.in_upper {
        ofs.upper.read(path).cloned().unwrap_or_default()
    } else {
        let idx = entry.lower_index.unwrap();
        ofs.lowers[idx].read(path).cloned().unwrap_or_default()
    };
    let layer = if entry.in_upper {
        FileLayer::Upper
    } else {
        FileLayer::Lower(entry.lower_index.unwrap())
    };
    Ok(FileHandle {
        path: path.to_string(),
        writable: false,
        layer,
        content,
    })
}

/// Copy-up: copy a file from its lower layer into the upper layer.
/// Returns the content that was copied.
fn copy_up(ofs: &mut OverlayFs, path: &str, lower_idx: usize) -> Result<Vec<u8>, OverlayError> {
    let data = ofs.lowers[lower_idx]
        .read(path)
        .cloned()
        .ok_or(OverlayError::NotFound)?;
    ofs.upper.write(path, data.clone());
    Ok(data)
}

/// Open a file for writing. If the file is in a lower layer, copy-up first.
pub fn overlay_open_write(ofs: &mut OverlayFs, path: &str) -> Result<FileHandle, OverlayError> {
    let entry = overlay_lookup(ofs, path);

    match entry {
        None => {
            // New file: create in upper
            ofs.upper.write(path, Vec::new());
            Ok(FileHandle {
                path: path.to_string(),
                writable: true,
                layer: FileLayer::Upper,
                content: Vec::new(),
            })
        }
        Some(e) if e.whiteout => Err(OverlayError::Whiteout),
        Some(e) if e.in_upper => {
            let content = ofs.upper.read(path).cloned().unwrap_or_default();
            Ok(FileHandle {
                path: path.to_string(),
                writable: true,
                layer: FileLayer::Upper,
                content,
            })
        }
        Some(e) => {
            // File is in lower: copy-up
            let content = copy_up(ofs, path, e.lower_index.unwrap())?;
            Ok(FileHandle {
                path: path.to_string(),
                writable: true,
                layer: FileLayer::Upper,
                content,
            })
        }
    }
}

/// Write data to a path (always goes to upper layer; copy-up if needed first).
pub fn overlay_write(ofs: &mut OverlayFs, path: &str, data: Vec<u8>) -> Result<(), OverlayError> {
    let mut fh = overlay_open_write(ofs, path)?;
    fh.content = data.clone();
    ofs.upper.write(path, data);
    Ok(())
}

/// Delete a file. If in upper: remove directly.
/// If in lower: create a whiteout entry in upper to hide it.
pub fn overlay_unlink(ofs: &mut OverlayFs, path: &str) -> Result<(), OverlayError> {
    let entry = overlay_lookup(ofs, path).ok_or(OverlayError::NotFound)?;
    if entry.whiteout {
        return Err(OverlayError::NotFound);
    }
    if entry.in_upper {
        ofs.upper.files.remove(path);
    } else {
        // Create whiteout in upper
        ofs.upper.whiteouts.insert(path.to_string());
    }
    Ok(())
}

/// Read directory: merge upper + lower, remove whiteout entries.
pub fn overlay_readdir(ofs: &OverlayFs, dir: &str) -> Vec<String> {
    // Collect whiteouts from upper
    let mut whiteouts: HashSet<String> = ofs.upper.whiteouts.clone();

    let mut entries: HashSet<String> = HashSet::new();

    // Upper entries (excluding whiteout paths themselves)
    for path in ofs.upper.readdir(dir) {
        if !ofs.upper.is_whiteout(&path) {
            entries.insert(path);
        }
    }

    // Lower entries, skipping whiteouts
    for lower in &ofs.lowers {
        // Accumulate lower layer whiteouts
        for wh in &lower.whiteouts {
            whiteouts.insert(wh.clone());
        }
        for path in lower.readdir(dir) {
            if !whiteouts.contains(&path) {
                entries.insert(path);
            }
        }
    }

    let mut result: Vec<String> = entries.into_iter().collect();
    result.sort();
    result
}

/// Mount convenience function: creates an OverlayFs
pub fn overlay_mount(upper: &str, lower: &str, work: &str) -> OverlayFs {
    OverlayFs::new(upper, vec![lower], work)
}

// ── Unit tests ────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_overlay() -> OverlayFs {
        let mut ofs = OverlayFs::new("/upper", vec!["/lower"], "/work");
        ofs.seed_lower(0, "/etc/config", b"lower config".to_vec());
        ofs.seed_lower(0, "/etc/hosts", b"127.0.0.1 localhost".to_vec());
        ofs
    }

    #[test]
    fn test_read_from_lower() {
        let ofs = make_overlay();
        let fh = overlay_open_read(&ofs, "/etc/config").unwrap();
        assert_eq!(fh.content, b"lower config");
        assert_eq!(fh.layer, FileLayer::Lower(0));
    }

    #[test]
    fn test_copy_up_on_write() {
        let mut ofs = make_overlay();
        let fh = overlay_open_write(&mut ofs, "/etc/config").unwrap();
        assert_eq!(fh.layer, FileLayer::Upper);
        // Content should have been copied up
        assert_eq!(fh.content, b"lower config");
        // Now upper has the file
        assert!(ofs.upper.exists("/etc/config"));
    }

    #[test]
    fn test_whiteout_on_unlink() {
        let mut ofs = make_overlay();
        overlay_unlink(&mut ofs, "/etc/config").unwrap();
        // File should be hidden
        let res = overlay_open_read(&ofs, "/etc/config");
        assert_eq!(res, Err(OverlayError::Whiteout));
    }

    #[test]
    fn test_merged_readdir() {
        let mut ofs = make_overlay();
        // Add a file in upper
        ofs.seed_upper("/etc/resolv.conf", b"nameserver 8.8.8.8".to_vec());
        // Delete one lower file via whiteout
        overlay_unlink(&mut ofs, "/etc/hosts").unwrap();

        let entries = overlay_readdir(&ofs, "/etc");
        assert!(
            entries.contains(&"/etc/resolv.conf".to_string()),
            "upper file missing"
        );
        assert!(
            entries.contains(&"/etc/config".to_string()),
            "lower file missing"
        );
        assert!(
            !entries.contains(&"/etc/hosts".to_string()),
            "whiteout not respected"
        );
    }

    #[test]
    fn test_write_new_file_in_upper() {
        let mut ofs = make_overlay();
        overlay_write(&mut ofs, "/etc/new_file", b"brand new".to_vec()).unwrap();
        let fh = overlay_open_read(&ofs, "/etc/new_file").unwrap();
        assert_eq!(fh.content, b"brand new");
        assert_eq!(fh.layer, FileLayer::Upper);
    }

    #[test]
    fn test_not_found() {
        let ofs = make_overlay();
        assert_eq!(
            overlay_open_read(&ofs, "/does/not/exist"),
            Err(OverlayError::NotFound)
        );
    }

    #[test]
    fn test_upper_overrides_lower() {
        let mut ofs = make_overlay();
        // Write a different version to upper
        overlay_write(&mut ofs, "/etc/config", b"upper config".to_vec()).unwrap();
        let fh = overlay_open_read(&ofs, "/etc/config").unwrap();
        assert_eq!(fh.content, b"upper config");
        assert_eq!(fh.layer, FileLayer::Upper);
    }
}
