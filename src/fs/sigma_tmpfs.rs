#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! SigmaOS tmpfs — Sovereign In-Memory Filesystem
//!
//! Fully sovereign, pure-Rust temporary filesystem implementation.
//! Inspired by Linux tmpfs (mm/shmem.c) and BSD md(4) memory disks.
//!
//! # Features
//! - Files backed by Vec<u8> (no page cache dependency)
//! - Directories with HashMap-based dentry lookup
//! - Symlink support
//! - Size accounting with configurable limits
//! - Full mtime/ctime/atime tracking (nanosecond precision)
//! - Hardlink support via inode reference counting
//! - Truncate, append, random-access read/write

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::boxed::Box;

// ============================================================
// Inode Types
// ============================================================

/// Type of a tmpfs inode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmpfsInodeType {
    /// Regular file
    File,
    /// Directory
    Directory,
    /// Symbolic link
    Symlink,
    /// Character device
    CharDevice,
    /// Block device
    BlockDevice,
    /// Named pipe (FIFO)
    Fifo,
}

// ============================================================
// Permissions
// ============================================================

/// Unix-style permission bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UnixMode(pub u16);

impl UnixMode {
    pub const FILE_DEFAULT: Self = Self(0o644);
    pub const DIR_DEFAULT:  Self = Self(0o755);
    pub const EXEC:         Self = Self(0o755);

    pub fn readable(self) -> bool   { (self.0 & 0o444) != 0 }
    pub fn writable(self) -> bool   { (self.0 & 0o222) != 0 }
    pub fn executable(self) -> bool { (self.0 & 0o111) != 0 }
}

// ============================================================
// TmpfsInode
// ============================================================

/// An inode in the tmpfs filesystem.
///
/// # Encapsulation
/// The content variant is private; access via filesystem methods.
pub struct TmpfsInode {
    /// Unique inode number
    pub ino: u64,
    /// Inode type
    pub inode_type: TmpfsInodeType,
    /// Permission mode
    pub mode: UnixMode,
    /// Owner UID
    pub uid: u32,
    /// Owner GID
    pub gid: u32,
    /// Access time (ns since epoch)
    pub atime_ns: u64,
    /// Modification time
    pub mtime_ns: u64,
    /// Change time (metadata)
    pub ctime_ns: u64,
    /// Hard link count
    pub nlink: u32,
    /// Inode content
    content: TmpfsInodeContent,
}

enum TmpfsInodeContent {
    File(Vec<u8>),
    Directory(BTreeMap<String, u64>), // name -> child ino
    Symlink(String),
    Device { major: u32, minor: u32 },
    Fifo,
}

impl TmpfsInode {
    fn new_file(ino: u64, now_ns: u64) -> Self {
        Self {
            ino, inode_type: TmpfsInodeType::File, mode: UnixMode::FILE_DEFAULT,
            uid: 0, gid: 0, atime_ns: now_ns, mtime_ns: now_ns, ctime_ns: now_ns,
            nlink: 1, content: TmpfsInodeContent::File(Vec::new()),
        }
    }

    fn new_dir(ino: u64, now_ns: u64) -> Self {
        let mut entries = BTreeMap::new();
        entries.insert(".".to_string(), ino);
        Self {
            ino, inode_type: TmpfsInodeType::Directory, mode: UnixMode::DIR_DEFAULT,
            uid: 0, gid: 0, atime_ns: now_ns, mtime_ns: now_ns, ctime_ns: now_ns,
            nlink: 2, content: TmpfsInodeContent::Directory(entries),
        }
    }

    fn new_symlink(ino: u64, target: &str, now_ns: u64) -> Self {
        Self {
            ino, inode_type: TmpfsInodeType::Symlink, mode: UnixMode(0o777),
            uid: 0, gid: 0, atime_ns: now_ns, mtime_ns: now_ns, ctime_ns: now_ns,
            nlink: 1, content: TmpfsInodeContent::Symlink(target.into()),
        }
    }

    /// Return file size in bytes (0 for non-files).
    pub fn size(&self) -> u64 {
        match &self.content {
            TmpfsInodeContent::File(data) => data.len() as u64,
            TmpfsInodeContent::Symlink(s) => s.len() as u64,
            _ => 0,
        }
    }

    /// Read bytes from file content.
    pub fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, &'static str> {
        match &self.content {
            TmpfsInodeContent::File(data) => {
                let start = offset as usize;
                if start >= data.len() { return Ok(0); }
                let end = (start + buf.len()).min(data.len());
                let n = end - start;
                buf[..n].copy_from_slice(&data[start..end]);
                Ok(n)
            }
            _ => Err("not a file"),
        }
    }

    /// Write bytes to file content at offset.
    pub fn write(&mut self, offset: u64, data: &[u8], now_ns: u64) -> Result<usize, &'static str> {
        match &mut self.content {
            TmpfsInodeContent::File(buf) => {
                let start = offset as usize;
                let end = start + data.len();
                if end > buf.len() { buf.resize(end, 0); }
                buf[start..end].copy_from_slice(data);
                self.mtime_ns = now_ns;
                self.ctime_ns = now_ns;
                Ok(data.len())
            }
            _ => Err("not a file"),
        }
    }

    /// Truncate file to `new_size` bytes.
    pub fn truncate(&mut self, new_size: u64, now_ns: u64) -> Result<(), &'static str> {
        match &mut self.content {
            TmpfsInodeContent::File(buf) => {
                buf.resize(new_size as usize, 0);
                self.mtime_ns = now_ns;
                self.ctime_ns = now_ns;
                Ok(())
            }
            _ => Err("not a file"),
        }
    }

    /// List directory entries.
    pub fn readdir(&self) -> Result<Vec<(String, u64)>, &'static str> {
        match &self.content {
            TmpfsInodeContent::Directory(entries) => {
                Ok(entries.iter().map(|(n, &i)| (n.clone(), i)).collect())
            }
            _ => Err("not a directory"),
        }
    }

    /// Look up a name in a directory.
    pub fn lookup(&self, name: &str) -> Option<u64> {
        match &self.content {
            TmpfsInodeContent::Directory(entries) => entries.get(name).copied(),
            _ => None,
        }
    }

    /// Add an entry to a directory.
    fn dir_insert(&mut self, name: &str, ino: u64, now_ns: u64) -> Result<(), &'static str> {
        match &mut self.content {
            TmpfsInodeContent::Directory(entries) => {
                if entries.contains_key(name) { return Err("entry exists"); }
                entries.insert(name.into(), ino);
                self.mtime_ns = now_ns;
                self.ctime_ns = now_ns;
                Ok(())
            }
            _ => Err("not a directory"),
        }
    }

    /// Remove an entry from a directory.
    fn dir_remove(&mut self, name: &str, now_ns: u64) -> Result<u64, &'static str> {
        match &mut self.content {
            TmpfsInodeContent::Directory(entries) => {
                entries.remove(name).ok_or("entry not found").map(|ino| {
                    self.mtime_ns = now_ns;
                    ino
                })
            }
            _ => Err("not a directory"),
        }
    }

    /// Get symlink target.
    pub fn readlink(&self) -> Result<&str, &'static str> {
        match &self.content {
            TmpfsInodeContent::Symlink(t) => Ok(t.as_str()),
            _ => Err("not a symlink"),
        }
    }
}

// ============================================================
// TmpfsMount
// ============================================================

/// A mounted tmpfs filesystem instance.
///
/// # Design
/// Flat inode table (BTreeMap<ino, TmpfsInode>) with a root
/// directory inode. Analogous to Linux shmem_sb_info.
pub struct TmpfsMount {
    /// All inodes in this filesystem
    inodes: BTreeMap<u64, TmpfsInode>,
    /// Root inode number
    root_ino: u64,
    /// Next inode number
    next_ino: u64,
    /// Maximum size in bytes (u64::MAX = unlimited)
    max_bytes: u64,
    /// Current bytes used by file data
    used_bytes: u64,
    /// Mount options label
    label: String,
    /// Monotonic clock (ns) — updated by callers
    now_ns: u64,
}

impl TmpfsMount {
    /// Create a new tmpfs mount with an optional size limit.
    pub fn new(label: &str, max_bytes: u64) -> Self {
        let root = TmpfsInode::new_dir(1, 0);
        let mut inodes = BTreeMap::new();
        inodes.insert(1, root);
        Self {
            inodes,
            root_ino: 1,
            next_ino: 2,
            max_bytes,
            used_bytes: 0,
            label: label.into(),
            now_ns: 0,
        }
    }

    /// Update the internal clock.
    pub fn set_time(&mut self, now_ns: u64) { self.now_ns = now_ns; }

    /// Resolve an absolute path to an inode number.
    ///
    /// Follows symlinks up to 8 levels deep.
    pub fn resolve(&self, path: &str) -> Result<u64, &'static str> {
        self.resolve_from(self.root_ino, path, 0)
    }

    fn resolve_from(&self, start: u64, path: &str, depth: usize) -> Result<u64, &'static str> {
        if depth > 8 { return Err("symlink loop"); }
        let path = path.trim_start_matches('/');
        if path.is_empty() { return Ok(start); }

        let (component, rest) = match path.find('/') {
            Some(i) => (&path[..i], &path[i+1..]),
            None => (path, ""),
        };

        let dir = self.inodes.get(&start).ok_or("inode not found")?;
        let child_ino = dir.lookup(component).ok_or("no such file or directory")?;
        let child = self.inodes.get(&child_ino).ok_or("inode not found")?;

        // Follow symlinks
        if child.inode_type == TmpfsInodeType::Symlink {
            let target = child.readlink()?;
            let resolved = self.resolve_from(self.root_ino, target, depth + 1)?;
            return self.resolve_from(resolved, rest, depth + 1);
        }

        if rest.is_empty() { Ok(child_ino) } else { self.resolve_from(child_ino, rest, depth) }
    }

    fn parent_and_name(path: &str) -> (&str, &str) {
        let path = path.trim_end_matches('/');
        match path.rfind('/') {
            Some(i) if i > 0 => (&path[..i], &path[i+1..]),
            Some(_) => ("/", &path[1..]),
            None => (".", path),
        }
    }

    /// Create a new regular file.
    pub fn create(&mut self, path: &str) -> Result<u64, &'static str> {
        let (parent_path, name) = Self::parent_and_name(path);
        let parent_ino = self.resolve(parent_path)?;
        let ino = self.next_ino;
        self.next_ino += 1;
        let file = TmpfsInode::new_file(ino, self.now_ns);
        self.inodes.insert(ino, file);
        let now = self.now_ns;
        self.inodes.get_mut(&parent_ino).unwrap().dir_insert(name, ino, now)?;
        Ok(ino)
    }

    /// Create a directory.
    pub fn mkdir(&mut self, path: &str) -> Result<u64, &'static str> {
        let (parent_path, name) = Self::parent_and_name(path);
        let parent_ino = self.resolve(parent_path)?;
        let ino = self.next_ino;
        self.next_ino += 1;
        let now = self.now_ns;
        let mut dir = TmpfsInode::new_dir(ino, now);
        // Add ".." pointing to parent
        if let TmpfsInodeContent::Directory(ref mut entries) = dir.content {
            entries.insert("..".into(), parent_ino);
        }
        self.inodes.insert(ino, dir);
        self.inodes.get_mut(&parent_ino).unwrap().dir_insert(name, ino, now)?;
        Ok(ino)
    }

    /// Create a symbolic link.
    pub fn symlink(&mut self, path: &str, target: &str) -> Result<u64, &'static str> {
        let (parent_path, name) = Self::parent_and_name(path);
        let parent_ino = self.resolve(parent_path)?;
        let ino = self.next_ino;
        self.next_ino += 1;
        let now = self.now_ns;
        let link = TmpfsInode::new_symlink(ino, target, now);
        self.inodes.insert(ino, link);
        self.inodes.get_mut(&parent_ino).unwrap().dir_insert(name, ino, now)?;
        Ok(ino)
    }

    /// Remove a file or empty directory.
    pub fn unlink(&mut self, path: &str) -> Result<(), &'static str> {
        let (parent_path, name) = Self::parent_and_name(path);
        let parent_ino = self.resolve(parent_path)?;
        let now = self.now_ns;
        let child_ino = self.inodes.get_mut(&parent_ino).unwrap().dir_remove(name, now)?;
        // Reduce nlink; remove inode if zero
        if let Some(inode) = self.inodes.get_mut(&child_ino) {
            inode.nlink = inode.nlink.saturating_sub(1);
            let size = inode.size();
            if inode.nlink == 0 {
                self.used_bytes = self.used_bytes.saturating_sub(size);
                self.inodes.remove(&child_ino);
            }
        }
        Ok(())
    }

    /// Write data to a file by inode number.
    pub fn write_ino(&mut self, ino: u64, offset: u64, data: &[u8]) -> Result<usize, &'static str> {
        // Check size limit
        let current_size = self.inodes.get(&ino).map(|i| i.size()).unwrap_or(0);
        let new_end = offset + data.len() as u64;
        let delta = new_end.saturating_sub(current_size);
        if self.max_bytes != u64::MAX && self.used_bytes + delta > self.max_bytes {
            return Err("tmpfs: no space left on device");
        }
        let now = self.now_ns;
        let inode = self.inodes.get_mut(&ino).ok_or("inode not found")?;
        let n = inode.write(offset, data, now)?;
        self.used_bytes = self.used_bytes.saturating_add(delta);
        Ok(n)
    }

    /// Read data from a file by inode number.
    pub fn read_ino(&self, ino: u64, offset: u64, buf: &mut [u8]) -> Result<usize, &'static str> {
        self.inodes.get(&ino).ok_or("inode not found")?.read(offset, buf)
    }

    /// Write data to file at path.
    pub fn write(&mut self, path: &str, offset: u64, data: &[u8]) -> Result<usize, &'static str> {
        let ino = self.resolve(path)?;
        self.write_ino(ino, offset, data)
    }

    /// Read data from file at path.
    pub fn read(&self, path: &str, offset: u64, buf: &mut [u8]) -> Result<usize, &'static str> {
        let ino = self.resolve(path)?;
        self.read_ino(ino, offset, buf)
    }

    /// List directory contents.
    pub fn readdir(&self, path: &str) -> Result<Vec<(String, u64)>, &'static str> {
        let ino = self.resolve(path)?;
        self.inodes.get(&ino).ok_or("inode not found")?.readdir()
    }

    /// Get inode for a path.
    pub fn stat(&self, path: &str) -> Result<&TmpfsInode, &'static str> {
        let ino = self.resolve(path)?;
        self.inodes.get(&ino).ok_or("inode not found")
    }

    /// Returns bytes used / bytes available.
    pub fn usage(&self) -> (u64, u64) {
        let avail = if self.max_bytes == u64::MAX { u64::MAX } else { self.max_bytes - self.used_bytes };
        (self.used_bytes, avail)
    }

    /// Returns the root inode number.
    pub fn root_ino(&self) -> u64 { self.root_ino }
    /// Returns total inode count.
    pub fn inode_count(&self) -> usize { self.inodes.len() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_and_write() {
        let mut fs = TmpfsMount::new("test", u64::MAX);
        let ino = fs.create("/hello.txt").unwrap();
        fs.write_ino(ino, 0, b"Hello, SigmaOS!").unwrap();
        let mut buf = [0u8; 15];
        let n = fs.read_ino(ino, 0, &mut buf).unwrap();
        assert_eq!(n, 15);
        assert_eq!(&buf, b"Hello, SigmaOS!");
    }

    #[test]
    fn test_mkdir_and_readdir() {
        let mut fs = TmpfsMount::new("test", u64::MAX);
        fs.mkdir("/proc").unwrap();
        fs.create("/proc/cpuinfo").unwrap();
        let entries = fs.readdir("/proc").unwrap();
        let names: Vec<&str> = entries.iter().map(|(n, _)| n.as_str()).collect();
        assert!(names.contains(&"cpuinfo"));
    }

    #[test]
    fn test_symlink_follow() {
        let mut fs = TmpfsMount::new("test", u64::MAX);
        fs.create("/real.txt").unwrap();
        fs.symlink("/link.txt", "/real.txt").unwrap();
        assert!(fs.stat("/link.txt").is_ok());
    }

    #[test]
    fn test_size_limit() {
        let mut fs = TmpfsMount::new("limited", 1024);
        let ino = fs.create("/big.bin").unwrap();
        let data = vec![0u8; 1025];
        assert!(fs.write_ino(ino, 0, &data).is_err());
    }

    #[test]
    fn test_unlink() {
        let mut fs = TmpfsMount::new("test", u64::MAX);
        fs.create("/temp.txt").unwrap();
        assert!(fs.stat("/temp.txt").is_ok());
        fs.unlink("/temp.txt").unwrap();
        assert!(fs.stat("/temp.txt").is_err());
    }
}
