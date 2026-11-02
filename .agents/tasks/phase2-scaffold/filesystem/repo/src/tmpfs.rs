//! # tmpfs — Temporary Filesystem
//!
//! RAM-based temporary filesystem. All data resides in memory and is lost on
//! unmount / reboot. Inspired by Linux `fs/tmpfs/` and FreeBSD `tmpfs`.
//!
//! ## Principles
//! - Memory safety: no unsafe code; data managed by the std allocator.
//! - Single responsibility: tmpfs owns only in-memory file storage.
//! - POSIX compliance: inode types match S_IF* constants.

use std::collections::BTreeMap;
use std::string::String;
use std::vec::Vec;

use thiserror::Error;

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

/// Errors returned by tmpfs operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum TmpfsError {
    #[error("no such file or directory")]
    NotFound,
    #[error("file already exists")]
    AlreadyExists,
    #[error("not a directory")]
    NotDirectory,
    #[error("not a regular file")]
    NotRegularFile,
    #[error("directory not empty")]
    DirectoryNotEmpty,
    #[error("no space left on device")]
    OutOfSpace,
    #[error("permission denied")]
    PermissionDenied,
}

// ---------------------------------------------------------------------------
// Inode type and inode
// ---------------------------------------------------------------------------

/// Tmpfs inode type (POSIX S_IF* values in lower byte).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InodeType {
    Regular   = 1,
    Directory = 2,
    Symlink   = 3,
}

/// In-memory inode for tmpfs.
#[derive(Debug)]
pub struct TmpfsInode {
    pub ino:            u64,
    pub inode_type:     InodeType,
    pub mode:           u16,
    pub uid:            u32,
    pub gid:            u32,
    pub size:           u64,
    pub atime:          u64,
    pub mtime:          u64,
    pub ctime:          u64,
    pub nlink:          u32,
    /// Raw data for regular files.
    pub data:           Vec<u8>,
    /// Name-to-inode-number map for directories.
    pub children:       BTreeMap<String, u64>,
    /// Symlink target path.
    pub symlink_target: Option<String>,
}

impl TmpfsInode {
    /// Create a new regular-file inode.
    pub fn new_file(ino: u64, mode: u16, uid: u32, gid: u32) -> Self {
        Self {
            ino,
            inode_type:     InodeType::Regular,
            mode:           mode & 0o0777,
            uid, gid,
            size: 0, atime: 0, mtime: 0, ctime: 0,
            nlink: 1,
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: None,
        }
    }

    /// Create a new directory inode.
    pub fn new_dir(ino: u64, mode: u16, uid: u32, gid: u32) -> Self {
        Self {
            ino,
            inode_type: InodeType::Directory,
            mode:       (mode & 0o0777) | 0o040000, // S_IFDIR
            uid, gid,
            size: 0, atime: 0, mtime: 0, ctime: 0,
            nlink: 2, // `.` and `..`
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: None,
        }
    }

    /// Create a new symlink inode.
    pub fn new_symlink(ino: u64, target: String, uid: u32, gid: u32) -> Self {
        let len = target.len() as u64;
        Self {
            ino,
            inode_type:     InodeType::Symlink,
            mode:           0o120777, // S_IFLNK | 0777
            uid, gid,
            size: len, atime: 0, mtime: 0, ctime: 0,
            nlink: 1,
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: Some(target),
        }
    }

    /// Read bytes from a regular file.
    pub fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }
        let offset = offset as usize;
        if offset >= self.data.len() {
            return Ok(0);
        }
        let available = self.data.len() - offset;
        let to_read   = available.min(buf.len());
        buf[..to_read].copy_from_slice(&self.data[offset..offset + to_read]);
        Ok(to_read)
    }

    /// Write bytes into a regular file, extending it if necessary.
    pub fn write(&mut self, offset: u64, data: &[u8]) -> Result<usize, TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }
        let offset = offset as usize;
        let end    = offset + data.len();
        if end > self.data.len() {
            self.data.resize(end, 0);
        }
        self.data[offset..end].copy_from_slice(data);
        self.size = self.data.len() as u64;
        Ok(data.len())
    }

    /// Truncate or extend a regular file to `size` bytes.
    pub fn truncate(&mut self, size: u64) -> Result<(), TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }
        self.data.resize(size as usize, 0);
        self.size = size;
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// TmpfsFilesystem
// ---------------------------------------------------------------------------

/// In-memory filesystem instance.
pub struct TmpfsFilesystem {
    inodes:    BTreeMap<u64, TmpfsInode>,
    next_ino:  u64,
    root_ino:  u64,
    max_size:  u64,
    used_size: u64,
}

impl TmpfsFilesystem {
    const ROOT_INO: u64 = 1;

    /// Create a new tmpfs with `max_size` byte capacity.
    pub fn new(max_size: u64) -> Self {
        let mut inodes = BTreeMap::new();
        inodes.insert(Self::ROOT_INO, TmpfsInode::new_dir(Self::ROOT_INO, 0o755, 0, 0));
        Self {
            inodes,
            next_ino:  Self::ROOT_INO + 1,
            root_ino:  Self::ROOT_INO,
            max_size,
            used_size: 0,
        }
    }

    fn alloc_ino(&mut self) -> u64 {
        let ino = self.next_ino;
        self.next_ino += 1;
        ino
    }

    /// Look up an inode by absolute path.
    pub fn lookup(&self, path: &str) -> Result<&TmpfsInode, TmpfsError> {
        if path == "/" {
            return self.inodes.get(&self.root_ino).ok_or(TmpfsError::NotFound);
        }
        let mut current_ino = self.root_ino;
        for component in path.trim_start_matches('/').split('/') {
            if component.is_empty() { continue; }
            let inode = self.inodes.get(&current_ino).ok_or(TmpfsError::NotFound)?;
            if inode.inode_type != InodeType::Directory {
                return Err(TmpfsError::NotDirectory);
            }
            current_ino = *inode.children.get(component).ok_or(TmpfsError::NotFound)?;
        }
        self.inodes.get(&current_ino).ok_or(TmpfsError::NotFound)
    }

    /// Mutable lookup helper used by mutating operations.
    fn lookup_ino(&self, path: &str) -> Result<u64, TmpfsError> {
        if path == "/" { return Ok(self.root_ino); }
        let mut current_ino = self.root_ino;
        for component in path.trim_start_matches('/').split('/') {
            if component.is_empty() { continue; }
            let inode = self.inodes.get(&current_ino).ok_or(TmpfsError::NotFound)?;
            current_ino = *inode.children.get(component).ok_or(TmpfsError::NotFound)?;
        }
        Ok(current_ino)
    }

    /// Create a regular file under `parent_path`.
    pub fn create_file(
        &mut self, parent_path: &str, name: &str, mode: u16, uid: u32, gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent_ino = self.lookup_ino(parent_path)?;
        {
            let parent = self.inodes.get(&parent_ino).ok_or(TmpfsError::NotFound)?;
            if parent.inode_type != InodeType::Directory { return Err(TmpfsError::NotDirectory); }
            if parent.children.contains_key(name) { return Err(TmpfsError::AlreadyExists); }
        }
        let ino = self.alloc_ino();
        self.inodes.insert(ino, TmpfsInode::new_file(ino, mode, uid, gid));
        let parent = self.inodes.get_mut(&parent_ino).unwrap();
        parent.children.insert(name.to_string(), ino);
        Ok(ino)
    }

    /// Create a directory under `parent_path`.
    pub fn create_dir(
        &mut self, parent_path: &str, name: &str, mode: u16, uid: u32, gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent_ino = self.lookup_ino(parent_path)?;
        {
            let parent = self.inodes.get(&parent_ino).ok_or(TmpfsError::NotFound)?;
            if parent.inode_type != InodeType::Directory { return Err(TmpfsError::NotDirectory); }
            if parent.children.contains_key(name) { return Err(TmpfsError::AlreadyExists); }
        }
        let ino = self.alloc_ino();
        self.inodes.insert(ino, TmpfsInode::new_dir(ino, mode, uid, gid));
        let parent = self.inodes.get_mut(&parent_ino).unwrap();
        parent.children.insert(name.to_string(), ino);
        Ok(ino)
    }

    /// Create a symlink under `parent_path`.
    pub fn create_symlink(
        &mut self, parent_path: &str, name: &str, target: String, uid: u32, gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent_ino = self.lookup_ino(parent_path)?;
        {
            let parent = self.inodes.get(&parent_ino).ok_or(TmpfsError::NotFound)?;
            if parent.inode_type != InodeType::Directory { return Err(TmpfsError::NotDirectory); }
            if parent.children.contains_key(name) { return Err(TmpfsError::AlreadyExists); }
        }
        let ino = self.alloc_ino();
        self.inodes.insert(ino, TmpfsInode::new_symlink(ino, target, uid, gid));
        let parent = self.inodes.get_mut(&parent_ino).unwrap();
        parent.children.insert(name.to_string(), ino);
        Ok(ino)
    }

    /// Remove a file or empty directory under `parent_path`.
    pub fn unlink(&mut self, parent_path: &str, name: &str) -> Result<(), TmpfsError> {
        let parent_ino = self.lookup_ino(parent_path)?;
        let child_ino = {
            let parent = self.inodes.get(&parent_ino).ok_or(TmpfsError::NotFound)?;
            if parent.inode_type != InodeType::Directory { return Err(TmpfsError::NotDirectory); }
            *parent.children.get(name).ok_or(TmpfsError::NotFound)?
        };
        {
            let child = self.inodes.get(&child_ino).ok_or(TmpfsError::NotFound)?;
            if child.inode_type == InodeType::Directory && !child.children.is_empty() {
                return Err(TmpfsError::DirectoryNotEmpty);
            }
        }
        let parent = self.inodes.get_mut(&parent_ino).unwrap();
        parent.children.remove(name);
        self.inodes.remove(&child_ino);
        Ok(())
    }

    /// Return filesystem statistics.
    pub fn statfs(&self) -> TmpfsStats {
        TmpfsStats {
            total_size:   self.max_size,
            used_size:    self.used_size,
            free_size:    self.max_size.saturating_sub(self.used_size),
            total_inodes: self.inodes.len() as u64,
        }
    }
}

/// Tmpfs filesystem-level statistics.
#[derive(Debug, Clone, Copy)]
pub struct TmpfsStats {
    pub total_size:   u64,
    pub used_size:    u64,
    pub free_size:    u64,
    pub total_inodes: u64,
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn root_is_directory() {
        let fs = TmpfsFilesystem::new(1024 * 1024);
        let root = fs.lookup("/").unwrap();
        assert_eq!(root.inode_type, InodeType::Directory);
    }

    #[test]
    fn create_and_lookup_file() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        let ino = fs.create_file("/", "hello.txt", 0o644, 0, 0).unwrap();
        let node = fs.lookup("/hello.txt").unwrap();
        assert_eq!(node.ino, ino);
        assert_eq!(node.inode_type, InodeType::Regular);
    }

    #[test]
    fn create_and_lookup_dir() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        fs.create_dir("/", "etc", 0o755, 0, 0).unwrap();
        let node = fs.lookup("/etc").unwrap();
        assert_eq!(node.inode_type, InodeType::Directory);
    }

    #[test]
    fn read_write_roundtrip() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        let ino = fs.create_file("/", "data.bin", 0o644, 0, 0).unwrap();
        let node = fs.inodes.get_mut(&ino).unwrap();
        let payload = b"SigmaOS tmpfs";
        node.write(0, payload).unwrap();
        let mut buf = [0u8; 64];
        let n = node.read(0, &mut buf).unwrap();
        assert_eq!(n, payload.len());
        assert_eq!(&buf[..n], payload);
    }

    #[test]
    fn unlink_removes_file() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        fs.create_file("/", "tmp.txt", 0o644, 0, 0).unwrap();
        fs.unlink("/", "tmp.txt").unwrap();
        assert!(fs.lookup("/tmp.txt").is_err());
    }

    #[test]
    fn unlink_nonempty_dir_fails() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        fs.create_dir("/", "nonempty", 0o755, 0, 0).unwrap();
        fs.create_file("/nonempty", "child.txt", 0o644, 0, 0).unwrap();
        assert_eq!(fs.unlink("/", "nonempty"), Err(TmpfsError::DirectoryNotEmpty));
    }
}
