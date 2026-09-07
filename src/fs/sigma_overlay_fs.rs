#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! # SigmaOS OverlayFS
//!
//! A copy-on-write overlay filesystem for SigmaOS, inspired by Linux `overlayfs`
//! and BSD `nullfs`.
//!
//! ## Layered Design
//! ```text
//!  ┌──────────────────────────┐
//!  │  Upper layer (writable)  │  ← writes go here
//!  ├──────────────────────────┤
//!  │  Work directory          │  ← atomic rename staging
//!  ├──────────────────────────┤
//!  │  Lower layer (read-only) │  ← original content
//!  └──────────────────────────┘
//! ```
//!
//! ## Copy-on-Write (COW)
//! When a file in the lower layer is opened for writing, it is *copied up* to
//! the upper layer first.  Subsequent reads and writes operate on the upper copy.
//!
//! ## Whiteout Files
//! Deletion of a lower-layer file is recorded by creating a *whiteout* marker
//! in the upper layer.  During lookup, any path with a whiteout marker returns
//! `NotFound`.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::collections::{HashMap, HashSet};
use std::vec::Vec;
use std::string::String;

// ── Error type ────────────────────────────────────────────────────────────────

/// Errors returned by OverlayFS operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OverlayError {
    /// Path does not exist in any layer.
    NotFound(String),
    /// Path already exists.
    AlreadyExists(String),
    /// The target is a directory; a file operation was attempted.
    IsDirectory(String),
    /// The target is a file; a directory operation was attempted.
    NotDirectory(String),
    /// The path is whited-out (deleted in upper layer).
    WhiteoutExists(String),
    /// Upper layer is read-only (misconfigured mount).
    ReadOnly,
    /// General I/O error with message.
    Io(String),
}

pub type OverlayResult<T> = Result<T, OverlayError>;

// ── Inode types ───────────────────────────────────────────────────────────────

/// Inode type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeKind {
    /// Regular file.
    File,
    /// Directory.
    Directory,
    /// Symbolic link.
    Symlink,
    /// Whiteout marker (records a deletion in the upper layer).
    Whiteout,
}

/// An inode in the overlay filesystem.
///
/// In real use, `data` would be a reference to page-cache pages or an
/// in-kernel inode.  Here we represent file data as `Vec<u8>` for simulation.
#[derive(Debug, Clone)]
pub struct OverlayInode {
    /// Absolute path within the overlay namespace.
    pub path: String,
    /// Inode kind.
    pub kind: InodeKind,
    /// File content (only meaningful for `File` inodes).
    pub data: Vec<u8>,
    /// Symlink target (only meaningful for `Symlink` inodes).
    pub symlink_target: Option<String>,
    /// Extended attributes.
    pub xattrs: HashMap<String, Vec<u8>>,
    /// File permissions (simplified Unix mode, e.g. 0o644).
    pub mode: u32,
    /// Whether this inode was copied up from the lower layer.
    pub copied_up: bool,
}

impl OverlayInode {
    /// Create a new regular file inode.
    pub fn new_file(path: impl Into<String>, data: Vec<u8>, mode: u32) -> Self {
        OverlayInode {
            path: path.into(),
            kind: InodeKind::File,
            data,
            symlink_target: None,
            xattrs: HashMap::new(),
            mode,
            copied_up: false,
        }
    }

    /// Create a new directory inode.
    pub fn new_dir(path: impl Into<String>, mode: u32) -> Self {
        OverlayInode {
            path: path.into(),
            kind: InodeKind::Directory,
            data: Vec::new(),
            symlink_target: None,
            xattrs: HashMap::new(),
            mode,
            copied_up: false,
        }
    }

    /// Create a whiteout marker inode.
    pub fn new_whiteout(path: impl Into<String>) -> Self {
        OverlayInode {
            path: path.into(),
            kind: InodeKind::Whiteout,
            data: Vec::new(),
            symlink_target: None,
            xattrs: HashMap::new(),
            mode: 0,
            copied_up: false,
        }
    }

    /// Create a symbolic link inode.
    pub fn new_symlink(path: impl Into<String>, target: impl Into<String>) -> Self {
        OverlayInode {
            path: path.into(),
            kind: InodeKind::Symlink,
            data: Vec::new(),
            symlink_target: Some(target.into()),
            xattrs: HashMap::new(),
            mode: 0o777,
            copied_up: false,
        }
    }

    /// Return `true` if this inode is a whiteout marker.
    pub fn is_whiteout(&self) -> bool {
        self.kind == InodeKind::Whiteout
    }
}

// ── Layer ─────────────────────────────────────────────────────────────────────

/// A single filesystem layer.
///
/// In production this would wrap a real VFS mount; here it's an in-memory
/// `HashMap<path, OverlayInode>`.
#[derive(Debug, Default)]
pub struct Layer {
    /// Inodes keyed by canonical path.
    inodes: HashMap<String, OverlayInode>,
    /// Whether writes are permitted on this layer.
    pub writable: bool,
}

impl Layer {
    /// Create a new writable layer.
    pub fn new_writable() -> Self {
        Layer { inodes: HashMap::new(), writable: true }
    }

    /// Create a new read-only layer populated from `inodes`.
    pub fn new_readonly(inodes: HashMap<String, OverlayInode>) -> Self {
        Layer { inodes, writable: false }
    }

    /// Look up an inode by path.
    pub fn lookup(&self, path: &str) -> Option<&OverlayInode> {
        self.inodes.get(path)
    }

    /// Insert or replace an inode.
    pub fn insert(&mut self, inode: OverlayInode) {
        self.inodes.insert(inode.path.clone(), inode);
    }

    /// Remove an inode.
    pub fn remove(&mut self, path: &str) -> Option<OverlayInode> {
        self.inodes.remove(path)
    }

    /// Return all paths in this layer.
    pub fn paths(&self) -> impl Iterator<Item = &String> {
        self.inodes.keys()
    }
}

// ── OverlayMount ─────────────────────────────────────────────────────────────

/// An overlay filesystem mount.
///
/// Presents a unified view of `upper` and `lower` layers.
/// The `work` directory holds in-progress atomic operations.
#[derive(Debug)]
pub struct OverlayMount {
    /// Upper (writable) layer.
    pub upper: Layer,
    /// Lower (read-only) layer.
    pub lower: Layer,
    /// Work directory (staging for atomic operations).
    work: Layer,
    /// Mount point path (informational).
    pub mountpoint: String,
}

impl OverlayMount {
    /// Create an overlay mount with an empty upper layer over `lower`.
    pub fn new(mountpoint: impl Into<String>, lower: Layer) -> Self {
        OverlayMount {
            upper: Layer::new_writable(),
            lower,
            work: Layer::new_writable(),
            mountpoint: mountpoint.into(),
        }
    }

    // ── Lookup ────────────────────────────────────────────────────────────────

    /// Look up a path in the overlay.
    ///
    /// Resolution order:
    /// 1. Upper layer (whiteouts return `WhiteoutExists`).
    /// 2. Lower layer.
    pub fn lookup(&self, path: &str) -> OverlayResult<&OverlayInode> {
        // Check upper layer first.
        if let Some(inode) = self.upper.lookup(path) {
            if inode.is_whiteout() {
                return Err(OverlayError::WhiteoutExists(path.to_string()));
            }
            return Ok(inode);
        }
        // Fall through to lower layer.
        if let Some(inode) = self.lower.lookup(path) {
            return Ok(inode);
        }
        Err(OverlayError::NotFound(path.to_string()))
    }

    // ── Copy-up ───────────────────────────────────────────────────────────────

    /// Copy a lower-layer inode up to the upper layer.
    ///
    /// Called before any modification of a lower-layer file.
    fn copy_up(&mut self, path: &str) -> OverlayResult<()> {
        // Only copy if not already in upper.
        if self.upper.lookup(path).is_some() {
            return Ok(());
        }

        let lower_inode = self.lower.lookup(path)
            .ok_or_else(|| OverlayError::NotFound(path.to_string()))?
            .clone();

        let mut upper_inode = lower_inode;
        upper_inode.copied_up = true;

        self.upper.insert(upper_inode);
        Ok(())
    }

    // ── Create ────────────────────────────────────────────────────────────────

    /// Create a new file at `path` with `data` and `mode`.
    ///
    /// Returns `AlreadyExists` if the path is occupied in any visible layer.
    pub fn create(&mut self, path: impl Into<String>, data: Vec<u8>, mode: u32)
        -> OverlayResult<()>
    {
        let path = path.into();
        match self.lookup(&path) {
            Ok(_) => return Err(OverlayError::AlreadyExists(path)),
            Err(OverlayError::NotFound(_)) | Err(OverlayError::WhiteoutExists(_)) => {}
            Err(e) => return Err(e),
        }

        let inode = OverlayInode::new_file(&path, data, mode);
        self.upper.insert(inode);
        Ok(())
    }

    /// Create a directory at `path`.
    pub fn mkdir(&mut self, path: impl Into<String>, mode: u32) -> OverlayResult<()> {
        let path = path.into();
        match self.lookup(&path) {
            Ok(_) => return Err(OverlayError::AlreadyExists(path)),
            Err(OverlayError::NotFound(_)) | Err(OverlayError::WhiteoutExists(_)) => {}
            Err(e) => return Err(e),
        }
        self.upper.insert(OverlayInode::new_dir(&path, mode));
        Ok(())
    }

    // ── Write ─────────────────────────────────────────────────────────────────

    /// Write `data` to the file at `path`.
    ///
    /// Performs a copy-up if the file is only in the lower layer.
    pub fn write(&mut self, path: &str, data: Vec<u8>) -> OverlayResult<()> {
        // Ensure existence.
        let _ = self.lookup(path).map_err(|e| e)?;

        // Copy-up if needed.
        self.copy_up(path)?;

        let inode = self.upper.inodes.get_mut(path)
            .ok_or_else(|| OverlayError::Io("copy-up failed".to_string()))?;

        if inode.kind != InodeKind::File {
            return Err(OverlayError::IsDirectory(path.to_string()));
        }
        inode.data = data;
        Ok(())
    }

    /// Read the contents of the file at `path`.
    pub fn read(&self, path: &str) -> OverlayResult<&[u8]> {
        let inode = self.lookup(path)?;
        if inode.kind != InodeKind::File {
            return Err(OverlayError::IsDirectory(path.to_string()));
        }
        Ok(&inode.data)
    }

    // ── Unlink ────────────────────────────────────────────────────────────────

    /// Delete a file or empty directory at `path`.
    ///
    /// If the file exists only in the lower layer, a whiteout is created in
    /// the upper layer.  If it exists in the upper layer, it is removed
    /// directly (a whiteout is still placed if the lower layer also has the path).
    pub fn unlink(&mut self, path: &str) -> OverlayResult<()> {
        // Verify the path exists.
        let _ = self.lookup(path)?;

        let in_upper = self.upper.lookup(path).is_some();
        let in_lower = self.lower.lookup(path).is_some();

        if in_upper {
            self.upper.remove(path);
        }

        if in_lower {
            // Place a whiteout so the lower entry is hidden.
            self.upper.insert(OverlayInode::new_whiteout(path));
        }

        Ok(())
    }

    // ── Rename ────────────────────────────────────────────────────────────────

    /// Rename `old_path` to `new_path`.
    ///
    /// Uses the work directory for atomic staging (simplified).
    pub fn rename(&mut self, old_path: &str, new_path: &str) -> OverlayResult<()> {
        // Copy-up the source if needed.
        self.copy_up(old_path)?;

        // Remove existing destination if present.
        if self.lookup(new_path).is_ok() {
            self.unlink(new_path)?;
        }

        // Stage in work directory.
        let mut inode = self.upper.remove(old_path)
            .ok_or_else(|| OverlayError::NotFound(old_path.to_string()))?;

        inode.path = new_path.to_string();
        self.work.insert(inode.clone());

        // Commit from work to upper.
        self.upper.insert(inode);
        self.work.remove(new_path);

        // Whiteout old path if it exists in lower.
        if self.lower.lookup(old_path).is_some() {
            self.upper.insert(OverlayInode::new_whiteout(old_path));
        }

        Ok(())
    }

    // ── Readdir ───────────────────────────────────────────────────────────────

    /// Return the merged directory listing for `dir_path`.
    ///
    /// Whiteout entries are excluded from the result.
    pub fn readdir(&self, dir_path: &str) -> OverlayResult<Vec<String>> {
        // Verify the directory exists.
        match self.lookup(dir_path) {
            Ok(inode) if inode.kind == InodeKind::Directory => {}
            Ok(_) => return Err(OverlayError::NotDirectory(dir_path.to_string())),
            Err(e) => return Err(e),
        }

        let prefix = if dir_path.ends_with('/') {
            dir_path.to_string()
        } else {
            format!("{}/", dir_path)
        };

        // Collect all paths under this directory from both layers.
        let mut seen: HashSet<String> = HashSet::new();

        for layer in [&self.upper, &self.lower] {
            for path in layer.paths() {
                if path.starts_with(&prefix) {
                    // Only direct children.
                    let rest = &path[prefix.len()..];
                    if !rest.contains('/') {
                        seen.insert(path.clone());
                    }
                }
            }
        }

        // Remove whited-out entries.
        let result: Vec<String> = seen
            .into_iter()
            .filter(|p| {
                self.upper.lookup(p)
                    .map(|i| !i.is_whiteout())
                    .unwrap_or(true)
            })
            .collect();

        Ok(result)
    }

    /// Return `true` if `path` has a whiteout in the upper layer.
    pub fn is_whiteout(&self, path: &str) -> bool {
        self.upper.lookup(path)
            .map(|i| i.is_whiteout())
            .unwrap_or(false)
    }
}

// ── Unit Tests ───────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn make_lower() -> Layer {
        let mut inodes = HashMap::new();
        inodes.insert("/".to_string(), OverlayInode::new_dir("/", 0o755));
        inodes.insert("/etc".to_string(), OverlayInode::new_dir("/etc", 0o755));
        inodes.insert(
            "/etc/hostname".to_string(),
            OverlayInode::new_file("/etc/hostname", b"sigma-os\n".to_vec(), 0o644),
        );
        Layer::new_readonly(inodes)
    }

    #[test]
    fn test_lookup_lower() {
        let mount = OverlayMount::new("/mnt", make_lower());
        let inode = mount.lookup("/etc/hostname").unwrap();
        assert_eq!(inode.data, b"sigma-os\n");
    }

    #[test]
    fn test_copy_up_on_write() {
        let mut mount = OverlayMount::new("/mnt", make_lower());
        mount.write("/etc/hostname", b"new-hostname\n".to_vec()).unwrap();

        // Upper layer now has the file.
        let upper = mount.upper.lookup("/etc/hostname").unwrap();
        assert!(upper.copied_up);
        assert_eq!(upper.data, b"new-hostname\n");

        // Lower unchanged.
        let lower = mount.lower.lookup("/etc/hostname").unwrap();
        assert_eq!(lower.data, b"sigma-os\n");
    }

    #[test]
    fn test_unlink_creates_whiteout() {
        let mut mount = OverlayMount::new("/mnt", make_lower());
        mount.unlink("/etc/hostname").unwrap();

        assert!(mount.is_whiteout("/etc/hostname"));
        assert_eq!(
            mount.lookup("/etc/hostname"),
            Err(OverlayError::WhiteoutExists("/etc/hostname".to_string()))
        );
    }

    #[test]
    fn test_create_file() {
        let mut mount = OverlayMount::new("/mnt", make_lower());
        mount.create("/etc/motd", b"Welcome to SigmaOS\n".to_vec(), 0o644).unwrap();
        let inode = mount.lookup("/etc/motd").unwrap();
        assert_eq!(inode.data, b"Welcome to SigmaOS\n");
    }

    #[test]
    fn test_rename() {
        let mut mount = OverlayMount::new("/mnt", make_lower());
        mount.rename("/etc/hostname", "/etc/hostname.bak").unwrap();

        // Old path should be whited out.
        assert!(mount.is_whiteout("/etc/hostname"));
        // New path should exist.
        let inode = mount.lookup("/etc/hostname.bak").unwrap();
        assert_eq!(inode.data, b"sigma-os\n");
    }
}
