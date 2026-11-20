//! # devfs — Device Filesystem Stub
//!
//! In-memory device filesystem. Provides a virtual directory of device nodes
//! (`/dev/null`, `/dev/zero`, block devices, character devices, etc.).
//!
//! ## Principles
//! - Plug-in driver model: device nodes are registered, not hard-coded.
//! - Least privilege: only the device owner can open exclusive devices.
//! - Separation of mechanism & policy: devfs manages naming; drivers manage I/O.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors returned by devfs operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum DevfsError {
    #[error("device not found")]
    NotFound,
    #[error("device already registered")]
    AlreadyExists,
    #[error("permission denied")]
    PermissionDenied,
    #[error("device busy")]
    Busy,
    #[error("invalid device number")]
    InvalidDevice,
}

// ---------------------------------------------------------------------------
// Device node types
// ---------------------------------------------------------------------------

/// Kind of device a node represents.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DevNodeKind {
    /// Character device (e.g. `/dev/null`, `/dev/tty`).
    Char,
    /// Block device (e.g. `/dev/sda`).
    Block,
    /// Pseudo-device that always reads zeros.
    Zero,
    /// Pseudo-device that discards all writes and returns EOF on read.
    Null,
}

/// A registered device node entry in devfs.
#[derive(Debug, Clone)]
pub struct DevNode {
    pub name:  String,
    pub kind:  DevNodeKind,
    pub major: u32,
    pub minor: u32,
    pub mode:  u16,
    pub uid:   u32,
    pub gid:   u32,
}

// ---------------------------------------------------------------------------
// DevFs
// ---------------------------------------------------------------------------

/// Device filesystem instance (typically mounted at `/dev`).
pub struct DevFs {
    nodes: BTreeMap<String, DevNode>,
}

impl DevFs {
    /// Create a new devfs with standard pseudo-devices pre-populated.
    pub fn new() -> Self {
        let mut fs = Self { nodes: BTreeMap::new() };
        // Populate standard pseudo-devices.
        fs.nodes.insert("null".to_string(), DevNode {
            name: "null".to_string(), kind: DevNodeKind::Null,
            major: 1, minor: 3, mode: 0o0666, uid: 0, gid: 0,
        });
        fs.nodes.insert("zero".to_string(), DevNode {
            name: "zero".to_string(), kind: DevNodeKind::Zero,
            major: 1, minor: 5, mode: 0o0666, uid: 0, gid: 0,
        });
        fs
    }

    /// Register a new device node.
    pub fn register(&mut self, node: DevNode) -> Result<(), DevfsError> {
        if self.nodes.contains_key(&node.name) {
            return Err(DevfsError::AlreadyExists);
        }
        self.nodes.insert(node.name.clone(), node);
        Ok(())
    }

    /// Remove a device node by name.
    pub fn unregister(&mut self, name: &str) -> Result<(), DevfsError> {
        self.nodes.remove(name).ok_or(DevfsError::NotFound).map(|_| ())
    }

    /// Look up a device node by name.
    pub fn lookup(&self, name: &str) -> Result<&DevNode, DevfsError> {
        self.nodes.get(name).ok_or(DevfsError::NotFound)
    }

    /// List all registered device names.
    pub fn list(&self) -> Vec<&str> {
        self.nodes.keys().map(|s| s.as_str()).collect()
    }

    /// Simulate a read from a device node.
    /// Returns how many bytes were "read" into `buf`.
    pub fn read(&self, name: &str, buf: &mut [u8]) -> Result<usize, DevfsError> {
        let node = self.lookup(name)?;
        match node.kind {
            DevNodeKind::Null => Ok(0),                   // EOF
            DevNodeKind::Zero => {
                buf.fill(0);
                Ok(buf.len())
            }
            _ => Err(DevfsError::InvalidDevice),
        }
    }

    /// Simulate a write to a device node.
    /// Returns how many bytes were "written".
    pub fn write(&self, name: &str, data: &[u8]) -> Result<usize, DevfsError> {
        let node = self.lookup(name)?;
        match node.kind {
            DevNodeKind::Null => Ok(data.len()), // discard
            _ => Err(DevfsError::InvalidDevice),
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_devices_present() {
        let fs = DevFs::new();
        assert!(fs.lookup("null").is_ok());
        assert!(fs.lookup("zero").is_ok());
    }

    #[test]
    fn register_and_lookup() {
        let mut fs = DevFs::new();
        let node = DevNode {
            name: "sda".to_string(), kind: DevNodeKind::Block,
            major: 8, minor: 0, mode: 0o0660, uid: 0, gid: 6,
        };
        fs.register(node).unwrap();
        assert_eq!(fs.lookup("sda").unwrap().major, 8);
    }

    #[test]
    fn double_register_rejected() {
        let mut fs = DevFs::new();
        let node = DevNode {
            name: "null".to_string(), kind: DevNodeKind::Null,
            major: 1, minor: 3, mode: 0o0666, uid: 0, gid: 0,
        };
        assert_eq!(fs.register(node), Err(DevfsError::AlreadyExists));
    }

    #[test]
    fn null_read_returns_eof() {
        let fs = DevFs::new();
        let mut buf = [0xFFu8; 16];
        let n = fs.read("null", &mut buf).unwrap();
        assert_eq!(n, 0);
    }

    #[test]
    fn zero_read_fills_with_zeros() {
        let fs = DevFs::new();
        let mut buf = [0xFFu8; 16];
        let n = fs.read("zero", &mut buf).unwrap();
        assert_eq!(n, 16);
        assert!(buf.iter().all(|&b| b == 0));
    }

    #[test]
    fn null_write_discards() {
        let fs = DevFs::new();
        let n = fs.write("null", b"hello").unwrap();
        assert_eq!(n, 5);
    }

    #[test]
    fn unregister_removes_node() {
        let mut fs = DevFs::new();
        fs.unregister("zero").unwrap();
        assert!(fs.lookup("zero").is_err());
    }
}
