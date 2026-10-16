//! # debugfs — Kernel Debug Virtual Filesystem
//!
//! `debugfs` is a RAM-backed, in-kernel virtual filesystem that exposes
//! arbitrary kernel state to user-space via ordinary file operations.  It is
//! typically mounted at `/sys/kernel/debug`.
//!
//! ## Design
//! - An in-memory tree of [`DebugfsNode`]s rooted at `/`.
//! - Each node is either a **directory** or a **file** (no symlinks for now).
//! - File nodes carry a [`DebugFileOps`] trait object that kernel subsystems
//!   register to provide custom `read` / `write` / `open` behaviour.
//! - No persistence: all state is lost on unmount or reboot.
//!
//! ## Usage (kernel-internal)
//! ```no_run
//! // Register a read-only file exposing scheduler statistics.
//! debugfs_create_file("sched_stats", 0o444, &SCHED_OPS);
//! ```
//!
//! Reference: Linux `fs/debugfs/` and `Documentation/filesystems/debugfs.rst`.

#![allow(dead_code)]

extern crate alloc;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

// ---------------------------------------------------------------------------
// File-Operation Trait
// ---------------------------------------------------------------------------

/// Trait implemented by each debugfs file to supply custom I/O.
///
/// All methods default to returning `DebugfsError::NotSupported` so that
/// providers only need to override the operations they support.
pub trait DebugFileOps: Send + Sync {
    /// Called when user-space opens the file.  May initialise per-open state.
    fn open(&self) -> Result<(), DebugfsError> {
        Ok(())
    }

    /// Read up to `buf.len()` bytes of content into `buf` starting at `offset`.
    ///
    /// Returns the number of bytes actually read.
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, DebugfsError>;

    /// Write `data` into the file starting at `offset`.
    ///
    /// Returns the number of bytes consumed.
    fn write(&self, offset: u64, data: &[u8]) -> Result<usize, DebugfsError> {
        let _ = (offset, data);
        Err(DebugfsError::ReadOnly)
    }

    /// Return the current logical size in bytes (used for `stat`).
    fn size(&self) -> u64 {
        0
    }
}

// ---------------------------------------------------------------------------
// Node Types
// ---------------------------------------------------------------------------

/// Type of a debugfs node.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// A directory node; may contain child nodes.
    Directory,
    /// A regular file node backed by a [`DebugFileOps`] provider.
    File,
}

/// A single node in the debugfs tree.
pub struct DebugfsNode {
    /// Display name of this node (relative to its parent).
    pub name: String,
    /// Node type.
    pub node_type: NodeType,
    /// POSIX permission bits (e.g. `0o444` = read-only for all).
    pub mode: u16,
    /// File operations provider (only valid when `node_type == File`).
    pub ops: Option<Box<dyn DebugFileOps>>,
    /// Child nodes (only valid when `node_type == Directory`).
    pub children: Vec<DebugfsNode>,
    /// Inode number (assigned on creation).
    pub ino: u64,
}

impl fmt::Debug for DebugfsNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugfsNode")
            .field("name", &self.name)
            .field("type", &self.node_type)
            .field("mode", &format_args!("{:#o}", self.mode))
            .field("ino", &self.ino)
            .finish()
    }
}

impl DebugfsNode {
    /// Create a new directory node.
    pub fn new_dir(name: impl Into<String>, mode: u16, ino: u64) -> Self {
        DebugfsNode {
            name: name.into(),
            node_type: NodeType::Directory,
            mode,
            ops: None,
            children: Vec::new(),
            ino,
        }
    }

    /// Create a new file node backed by `ops`.
    pub fn new_file(
        name: impl Into<String>,
        mode: u16,
        ino: u64,
        ops: Box<dyn DebugFileOps>,
    ) -> Self {
        DebugfsNode {
            name: name.into(),
            node_type: NodeType::File,
            mode,
            ops: Some(ops),
            children: Vec::new(),
            ino,
        }
    }

    /// Returns `true` if this node is a directory.
    pub fn is_dir(&self) -> bool {
        self.node_type == NodeType::Directory
    }

    /// Look up a direct child by name.
    pub fn find_child(&self, name: &str) -> Option<&DebugfsNode> {
        self.children.iter().find(|c| c.name == name)
    }

    /// Look up a direct child mutably by name.
    pub fn find_child_mut(&mut self, name: &str) -> Option<&mut DebugfsNode> {
        self.children.iter_mut().find(|c| c.name == name)
    }

    /// Add a child node.  Returns `DebugfsError::Exists` if a child with the
    /// same name already exists.
    pub fn add_child(&mut self, child: DebugfsNode) -> Result<(), DebugfsError> {
        if self.find_child(&child.name).is_some() {
            return Err(DebugfsError::Exists);
        }
        self.children.push(child);
        Ok(())
    }

    /// Remove a child by name.  Returns `DebugfsError::NotFound` if not found.
    pub fn remove_child(&mut self, name: &str) -> Result<(), DebugfsError> {
        let pos = self
            .children
            .iter()
            .position(|c| c.name == name)
            .ok_or(DebugfsError::NotFound)?;
        self.children.remove(pos);
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// Filesystem Root
// ---------------------------------------------------------------------------

/// Global debugfs instance (typically mounted at `/sys/kernel/debug`).
pub struct DebugFilesystem {
    /// Root directory node.
    pub root: DebugfsNode,
    /// Inode counter — incremented on each node creation.
    next_ino: u64,
}

impl DebugFilesystem {
    /// Initialise a new, empty debugfs instance.
    pub fn new() -> Self {
        DebugFilesystem {
            root: DebugfsNode::new_dir("/", 0o755, 1),
            next_ino: 2,
        }
    }

    /// Allocate a fresh inode number.
    fn alloc_ino(&mut self) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        ino
    }

    /// Navigate to a directory node given a slash-separated path.
    ///
    /// Returns a mutable reference to the node or `DebugfsError::NotFound`.
    pub fn lookup_dir_mut(&mut self, path: &str) -> Result<&mut DebugfsNode, DebugfsError> {
        let parts: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        let mut node = &mut self.root;
        for part in parts {
            if !node.is_dir() {
                return Err(DebugfsError::NotDir);
            }
            node = node.find_child_mut(part).ok_or(DebugfsError::NotFound)?;
        }
        Ok(node)
    }

    /// Create intermediate directories along `path` (like `mkdir -p`).
    pub fn mkdir_p(&mut self, path: &str) -> Result<(), DebugfsError> {
        let parts: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        // We have to re-navigate from root for each component to satisfy borrow rules.
        for depth in 0..parts.len() {
            let ino = self.alloc_ino();
            let component_path = parts[..=depth].join("/");
            let parent_path = if depth == 0 {
                String::new()
            } else {
                parts[..depth].join("/")
            };
            let name = parts[depth];
            let parent = self.lookup_dir_mut(&parent_path)?;
            if parent.find_child(name).is_none() {
                parent.add_child(DebugfsNode::new_dir(name, 0o755, ino))?;
            }
            let _ = component_path;
        }
        Ok(())
    }

    /// Register a file at `parent_path/name` backed by `ops`.
    pub fn create_file(
        &mut self,
        parent_path: &str,
        name: &str,
        mode: u16,
        ops: Box<dyn DebugFileOps>,
    ) -> Result<(), DebugfsError> {
        let ino = self.alloc_ino();
        let dir = self.lookup_dir_mut(parent_path)?;
        dir.add_child(DebugfsNode::new_file(name, mode, ino, ops))
    }

    /// Remove a file or empty directory at `parent_path/name`.
    pub fn remove(&mut self, parent_path: &str, name: &str) -> Result<(), DebugfsError> {
        let dir = self.lookup_dir_mut(parent_path)?;
        dir.remove_child(name)
    }

    /// Read from the file at `path`.
    pub fn read_file(
        &self,
        path: &str,
        offset: u64,
        buf: &mut [u8],
    ) -> Result<usize, DebugfsError> {
        let node = self.lookup_node(path)?;
        match &node.ops {
            Some(ops) => ops.read(offset, buf),
            None => Err(DebugfsError::IsDir),
        }
    }

    /// Write to the file at `path`.
    pub fn write_file(&self, path: &str, offset: u64, data: &[u8]) -> Result<usize, DebugfsError> {
        let node = self.lookup_node(path)?;
        match &node.ops {
            Some(ops) => ops.write(offset, data),
            None => Err(DebugfsError::IsDir),
        }
    }

    /// Immutable lookup helper.
    fn lookup_node(&self, path: &str) -> Result<&DebugfsNode, DebugfsError> {
        let parts: Vec<&str> = path
            .trim_matches('/')
            .split('/')
            .filter(|s| !s.is_empty())
            .collect();
        let mut node = &self.root;
        for part in parts {
            node = node.find_child(part).ok_or(DebugfsError::NotFound)?;
        }
        Ok(node)
    }
}

impl Default for DebugFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Built-in Debug File Providers
// ---------------------------------------------------------------------------

/// A simple static-string debug file (read-only).
pub struct StaticDebugFile {
    content: &'static str,
}

impl StaticDebugFile {
    pub fn new(content: &'static str) -> Self {
        StaticDebugFile { content }
    }
}

impl DebugFileOps for StaticDebugFile {
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, DebugfsError> {
        let bytes = self.content.as_bytes();
        let start = offset as usize;
        if start >= bytes.len() {
            return Ok(0);
        }
        let available = &bytes[start..];
        let n = buf.len().min(available.len());
        buf[..n].copy_from_slice(&available[..n]);
        Ok(n)
    }

    fn size(&self) -> u64 {
        self.content.len() as u64
    }
}

/// A debug file backed by a dynamic closure that generates content on read.
pub struct DynamicDebugFile<F>
where
    F: Fn() -> Vec<u8> + Send + Sync,
{
    generator: F,
}

impl<F: Fn() -> Vec<u8> + Send + Sync> DynamicDebugFile<F> {
    pub fn new(generator: F) -> Self {
        DynamicDebugFile { generator }
    }
}

impl<F: Fn() -> Vec<u8> + Send + Sync> DebugFileOps for DynamicDebugFile<F> {
    fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, DebugfsError> {
        let data = (self.generator)();
        let start = offset as usize;
        if start >= data.len() {
            return Ok(0);
        }
        let available = &data[start..];
        let n = buf.len().min(available.len());
        buf[..n].copy_from_slice(&available[..n]);
        Ok(n)
    }
}

// ---------------------------------------------------------------------------
// Error Type
// ---------------------------------------------------------------------------

/// Errors produced by the debugfs subsystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DebugfsError {
    /// The path does not exist.
    NotFound,
    /// A node with that name already exists.
    Exists,
    /// The target node is a directory, not a file.
    IsDir,
    /// The target node is a file, not a directory.
    NotDir,
    /// The file is read-only.
    ReadOnly,
    /// The operation is not supported by this file's ops.
    NotSupported,
    /// An underlying I/O error.
    IoError,
}

impl fmt::Display for DebugfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DebugfsError::NotFound => write!(f, "no such file or directory"),
            DebugfsError::Exists => write!(f, "file exists"),
            DebugfsError::IsDir => write!(f, "is a directory"),
            DebugfsError::NotDir => write!(f, "not a directory"),
            DebugfsError::ReadOnly => write!(f, "read-only filesystem"),
            DebugfsError::NotSupported => write!(f, "operation not supported"),
            DebugfsError::IoError => write!(f, "I/O error"),
        }
    }
}

// ---------------------------------------------------------------------------
// Convenience helpers (mirrors Linux's debugfs_create_* API)
// ---------------------------------------------------------------------------

/// Create a read-only static file at `parent_path/name` in `fs`.
pub fn debugfs_create_file_static(
    fs: &mut DebugFilesystem,
    parent_path: &str,
    name: &str,
    content: &'static str,
) -> Result<(), DebugfsError> {
    fs.create_file(
        parent_path,
        name,
        0o444,
        Box::new(StaticDebugFile::new(content)),
    )
}

/// Create a read-only dynamic file at `parent_path/name` in `fs`.
pub fn debugfs_create_file_dynamic<F>(
    fs: &mut DebugFilesystem,
    parent_path: &str,
    name: &str,
    generator: F,
) -> Result<(), DebugfsError>
where
    F: Fn() -> Vec<u8> + Send + Sync + 'static,
{
    fs.create_file(
        parent_path,
        name,
        0o444,
        Box::new(DynamicDebugFile::new(generator)),
    )
}
