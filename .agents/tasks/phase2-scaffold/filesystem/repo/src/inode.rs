//! # Inode Operations
//!
//! Generic inode create / delete / stat operations that sit above the concrete
//! filesystem implementations. The `InodeTable` is a filesystem-agnostic
//! in-memory registry of inodes, used by the VFS layer and unit tests.
//!
//! ## Principles
//! - Single responsibility: inode management only; no I/O policy here.
//! - Composition over inheritance: callers compose `InodeTable` with a concrete FS.
//! - POSIX compliance: stat fields follow `struct stat` semantics.

use std::collections::BTreeMap;
use std::string::String;

use thiserror::Error;

use crate::vfs::FileMode;
use crate::vfs::FileType;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors returned by inode operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum InodeError {
    #[error("inode not found")]
    NotFound,
    #[error("inode already exists for this path")]
    AlreadyExists,
    #[error("inode table is full")]
    TableFull,
    #[error("invalid inode number")]
    InvalidNumber,
    #[error("permission denied")]
    PermissionDenied,
}

// ---------------------------------------------------------------------------
// Stat structure
// ---------------------------------------------------------------------------

/// POSIX `struct stat` equivalent.
#[derive(Debug, Clone, PartialEq)]
pub struct StatInfo {
    pub ino:        u64,
    pub file_type:  FileType,
    pub mode:       FileMode,
    pub nlink:      u32,
    pub uid:        u32,
    pub gid:        u32,
    pub size:       u64,
    pub block_size: u32,
    pub blocks:     u64,
    pub atime:      u64,
    pub mtime:      u64,
    pub ctime:      u64,
}

// ---------------------------------------------------------------------------
// InodeRecord
// ---------------------------------------------------------------------------

/// One entry in the inode table.
#[derive(Debug, Clone)]
pub struct InodeRecord {
    pub ino:       u64,
    pub file_type: FileType,
    pub mode:      FileMode,
    pub uid:       u32,
    pub gid:       u32,
    pub size:      u64,
    pub nlink:     u32,
    pub atime:     u64,
    pub mtime:     u64,
    pub ctime:     u64,
    /// Optional path stored for lookup purposes.
    pub path:      Option<String>,
}

impl InodeRecord {
    /// Convert to a POSIX-style `StatInfo`.
    pub fn stat(&self) -> StatInfo {
        StatInfo {
            ino:        self.ino,
            file_type:  self.file_type,
            mode:       self.mode,
            nlink:      self.nlink,
            uid:        self.uid,
            gid:        self.gid,
            size:       self.size,
            block_size: 4096,
            blocks:     (self.size + 511) / 512, // 512-byte units like Linux
            atime:      self.atime,
            mtime:      self.mtime,
            ctime:      self.ctime,
        }
    }
}

// ---------------------------------------------------------------------------
// InodeTable
// ---------------------------------------------------------------------------

/// Filesystem-agnostic in-memory inode registry.
pub struct InodeTable {
    by_ino:  BTreeMap<u64, InodeRecord>,
    by_path: BTreeMap<String, u64>,
    next_ino: u64,
    max_inodes: u64,
}

impl InodeTable {
    /// Create a new inode table with `max_inodes` capacity.
    pub fn new(max_inodes: u64) -> Self {
        Self {
            by_ino:     BTreeMap::new(),
            by_path:    BTreeMap::new(),
            next_ino:   1,
            max_inodes,
        }
    }

    /// Allocate a new inode for `path` with `file_type` and `mode`.
    pub fn create(
        &mut self,
        path:      &str,
        file_type: FileType,
        mode:      FileMode,
        uid:       u32,
        gid:       u32,
    ) -> Result<u64, InodeError> {
        if self.by_path.contains_key(path) {
            return Err(InodeError::AlreadyExists);
        }
        if self.by_ino.len() as u64 >= self.max_inodes {
            return Err(InodeError::TableFull);
        }
        let ino = self.next_ino;
        self.next_ino += 1;
        let record = InodeRecord {
            ino, file_type, mode, uid, gid,
            size: 0, nlink: 1,
            atime: 0, mtime: 0, ctime: 0,
            path: Some(path.to_string()),
        };
        self.by_ino.insert(ino, record);
        self.by_path.insert(path.to_string(), ino);
        Ok(ino)
    }

    /// Delete an inode by inode number. Returns `NotFound` if absent.
    pub fn delete(&mut self, ino: u64) -> Result<(), InodeError> {
        let record = self.by_ino.remove(&ino).ok_or(InodeError::NotFound)?;
        if let Some(ref path) = record.path {
            self.by_path.remove(path);
        }
        Ok(())
    }

    /// Delete an inode by path.
    pub fn delete_by_path(&mut self, path: &str) -> Result<(), InodeError> {
        let ino = self.by_path.remove(path).ok_or(InodeError::NotFound)?;
        self.by_ino.remove(&ino);
        Ok(())
    }

    /// Look up an inode by inode number.
    pub fn get(&self, ino: u64) -> Result<&InodeRecord, InodeError> {
        self.by_ino.get(&ino).ok_or(InodeError::NotFound)
    }

    /// Look up an inode by path.
    pub fn get_by_path(&self, path: &str) -> Result<&InodeRecord, InodeError> {
        let ino = self.by_path.get(path).copied().ok_or(InodeError::NotFound)?;
        self.by_ino.get(&ino).ok_or(InodeError::NotFound)
    }

    /// Return POSIX stat for `ino`.
    pub fn stat(&self, ino: u64) -> Result<StatInfo, InodeError> {
        self.get(ino).map(|r| r.stat())
    }

    /// Return POSIX stat for `path`.
    pub fn stat_path(&self, path: &str) -> Result<StatInfo, InodeError> {
        self.get_by_path(path).map(|r| r.stat())
    }

    /// Update size and timestamps after a write operation.
    pub fn update_size(&mut self, ino: u64, new_size: u64, mtime: u64) -> Result<(), InodeError> {
        let record = self.by_ino.get_mut(&ino).ok_or(InodeError::NotFound)?;
        record.size  = new_size;
        record.mtime = mtime;
        record.ctime = mtime;
        Ok(())
    }

    /// Return the total number of allocated inodes.
    pub fn count(&self) -> u64 {
        self.by_ino.len() as u64
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn mode(v: u16) -> FileMode { FileMode::new(v) }

    #[test]
    fn create_and_stat_file() {
        let mut tbl = InodeTable::new(256);
        let ino = tbl.create("/etc/hosts", FileType::Regular, mode(0o644), 0, 0).unwrap();
        let st  = tbl.stat(ino).unwrap();
        assert_eq!(st.ino, ino);
        assert_eq!(st.file_type, FileType::Regular);
    }

    #[test]
    fn stat_by_path() {
        let mut tbl = InodeTable::new(256);
        tbl.create("/var/log/syslog", FileType::Regular, mode(0o640), 0, 4).unwrap();
        let st = tbl.stat_path("/var/log/syslog").unwrap();
        assert_eq!(st.gid, 4);
    }

    #[test]
    fn delete_removes_inode() {
        let mut tbl = InodeTable::new(256);
        let ino = tbl.create("/tmp/foo", FileType::Regular, mode(0o600), 1000, 1000).unwrap();
        tbl.delete(ino).unwrap();
        assert_eq!(tbl.stat(ino), Err(InodeError::NotFound));
    }

    #[test]
    fn delete_by_path() {
        let mut tbl = InodeTable::new(256);
        tbl.create("/tmp/bar", FileType::Regular, mode(0o600), 0, 0).unwrap();
        tbl.delete_by_path("/tmp/bar").unwrap();
        assert_eq!(tbl.stat_path("/tmp/bar"), Err(InodeError::NotFound));
    }

    #[test]
    fn table_full_returns_error() {
        let mut tbl = InodeTable::new(2);
        tbl.create("/a", FileType::Regular, mode(0o644), 0, 0).unwrap();
        tbl.create("/b", FileType::Regular, mode(0o644), 0, 0).unwrap();
        assert_eq!(
            tbl.create("/c", FileType::Regular, mode(0o644), 0, 0),
            Err(InodeError::TableFull)
        );
    }

    #[test]
    fn duplicate_path_returns_error() {
        let mut tbl = InodeTable::new(256);
        tbl.create("/dup", FileType::Regular, mode(0o644), 0, 0).unwrap();
        assert_eq!(
            tbl.create("/dup", FileType::Regular, mode(0o644), 0, 0),
            Err(InodeError::AlreadyExists)
        );
    }

    #[test]
    fn update_size_reflects_in_stat() {
        let mut tbl = InodeTable::new(256);
        let ino = tbl.create("/data.bin", FileType::Regular, mode(0o644), 0, 0).unwrap();
        tbl.update_size(ino, 4096, 1_000_000).unwrap();
        let st = tbl.stat(ino).unwrap();
        assert_eq!(st.size,  4096);
        assert_eq!(st.mtime, 1_000_000);
    }

    #[test]
    fn blocks_computed_correctly() {
        let mut tbl = InodeTable::new(256);
        let ino = tbl.create("/f", FileType::Regular, mode(0o644), 0, 0).unwrap();
        tbl.update_size(ino, 1, 0).unwrap();
        // 1 byte → 1 block of 512 bytes
        assert_eq!(tbl.stat(ino).unwrap().blocks, 1);
        tbl.update_size(ino, 513, 0).unwrap();
        // 513 bytes → 2 blocks of 512 bytes
        assert_eq!(tbl.stat(ino).unwrap().blocks, 2);
    }
}
