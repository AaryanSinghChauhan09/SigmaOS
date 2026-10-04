//! # tmpfs - Temporary Filesystem
//!
//! RAM-based temporary filesystem inspired by Linux fs/tmpfs/ and FreeBSD tmpfs.
//! All data resides in memory and is lost on unmount/reboot.

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicU64, Ordering};

/// Inode types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum InodeType {
    Regular = 1,
    Directory = 2,
    Symlink = 3,
}

/// Tmpfs inode
pub struct TmpfsInode {
    pub ino: u64,
    pub inode_type: InodeType,
    pub mode: u16,
    pub uid: u32,
    pub gid: u32,
    pub size: AtomicU64,
    pub atime: AtomicU64,
    pub mtime: AtomicU64,
    pub ctime: AtomicU64,
    pub nlink: AtomicU32,

    // Data storage
    pub data: Vec<u8>,                   // For regular files
    pub children: BTreeMap<String, u64>, // For directories (name -> ino)
    pub symlink_target: Option<String>,  // For symlinks
}

impl TmpfsInode {
    pub fn new_file(ino: u64, mode: u16, uid: u32, gid: u32) -> Self {
        let now = 0; // In production: get current timestamp

        Self {
            ino,
            inode_type: InodeType::Regular,
            mode,
            uid,
            gid,
            size: AtomicU64::new(0),
            atime: AtomicU64::new(now),
            mtime: AtomicU64::new(now),
            ctime: AtomicU64::new(now),
            nlink: AtomicU32::new(1),
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: None,
        }
    }

    pub fn new_dir(ino: u64, mode: u16, uid: u32, gid: u32) -> Self {
        let now = 0;

        Self {
            ino,
            inode_type: InodeType::Directory,
            mode: mode | 0o040000, // S_IFDIR
            uid,
            gid,
            size: AtomicU64::new(0),
            atime: AtomicU64::new(now),
            mtime: AtomicU64::new(now),
            ctime: AtomicU64::new(now),
            nlink: AtomicU32::new(2), // . and ..
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: None,
        }
    }

    pub fn new_symlink(ino: u64, target: String, uid: u32, gid: u32) -> Self {
        let now = 0;

        Self {
            ino,
            inode_type: InodeType::Symlink,
            mode: 0o120777, // S_IFLNK | 0777
            uid,
            gid,
            size: AtomicU64::new(target.len() as u64),
            atime: AtomicU64::new(now),
            mtime: AtomicU64::new(now),
            ctime: AtomicU64::new(now),
            nlink: AtomicU32::new(1),
            data: Vec::new(),
            children: BTreeMap::new(),
            symlink_target: Some(target),
        }
    }

    /// Read data from file
    pub fn read(&self, offset: u64, buf: &mut [u8]) -> Result<usize, TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }

        let offset = offset as usize;
        if offset >= self.data.len() {
            return Ok(0);
        }

        let available = self.data.len() - offset;
        let to_read = available.min(buf.len());

        buf[..to_read].copy_from_slice(&self.data[offset..offset + to_read]);

        // Update atime
        let now = 0; // In production: get timestamp
        self.atime.store(now, Ordering::Release);

        Ok(to_read)
    }

    /// Write data to file
    pub fn write(&mut self, offset: u64, data: &[u8]) -> Result<usize, TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }

        let offset = offset as usize;
        let end = offset + data.len();

        // Extend file if necessary
        if end > self.data.len() {
            self.data.resize(end, 0);
        }

        // Write data
        self.data[offset..end].copy_from_slice(data);

        // Update size and timestamps
        let new_size = self.data.len() as u64;
        self.size.store(new_size, Ordering::Release);

        let now = 0;
        self.mtime.store(now, Ordering::Release);
        self.ctime.store(now, Ordering::Release);

        Ok(data.len())
    }

    /// Truncate file
    pub fn truncate(&mut self, size: u64) -> Result<(), TmpfsError> {
        if self.inode_type != InodeType::Regular {
            return Err(TmpfsError::NotRegularFile);
        }

        self.data.resize(size as usize, 0);
        self.size.store(size, Ordering::Release);

        let now = 0;
        self.mtime.store(now, Ordering::Release);
        self.ctime.store(now, Ordering::Release);

        Ok(())
    }
}

/// Tmpfs filesystem
pub struct TmpfsFilesystem {
    inodes: BTreeMap<u64, Arc<TmpfsInode>>,
    next_ino: AtomicU64,
    root_ino: u64,
    max_size: usize,      // Maximum filesystem size in bytes
    used_size: AtomicU64, // Current used size
}

impl TmpfsFilesystem {
    const ROOT_INO: u64 = 1;

    pub fn new(max_size: usize) -> Self {
        let mut fs = Self {
            inodes: BTreeMap::new(),
            next_ino: AtomicU64::new(Self::ROOT_INO + 1),
            root_ino: Self::ROOT_INO,
            max_size,
            used_size: AtomicU64::new(0),
        };

        // Create root directory
        let root = Arc::new(TmpfsInode::new_dir(Self::ROOT_INO, 0o755, 0, 0));
        fs.inodes.insert(Self::ROOT_INO, root);

        fs
    }

    /// Allocate new inode number
    fn alloc_ino(&self) -> u64 {
        self.next_ino.fetch_add(1, Ordering::SeqCst)
    }

    /// Lookup inode by path
    pub fn lookup(&self, path: &str) -> Result<Arc<TmpfsInode>, TmpfsError> {
        if path == "/" {
            return self.get_inode(self.root_ino);
        }

        let components: Vec<&str> = path.trim_start_matches('/').split('/').collect();
        let mut current_ino = self.root_ino;

        for component in components {
            if component.is_empty() {
                continue;
            }

            let inode = self.get_inode(current_ino)?;

            if inode.inode_type != InodeType::Directory {
                return Err(TmpfsError::NotDirectory);
            }

            current_ino = *inode.children.get(component).ok_or(TmpfsError::NotFound)?;
        }

        self.get_inode(current_ino)
    }

    /// Get inode by number
    fn get_inode(&self, ino: u64) -> Result<Arc<TmpfsInode>, TmpfsError> {
        self.inodes
            .get(&ino)
            .map(Arc::clone)
            .ok_or(TmpfsError::NotFound)
    }

    /// Create file
    pub fn create_file(
        &mut self,
        parent_path: &str,
        name: &str,
        mode: u16,
        uid: u32,
        gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent = self.lookup(parent_path)?;

        if parent.inode_type != InodeType::Directory {
            return Err(TmpfsError::NotDirectory);
        }

        // Check if already exists
        if parent.children.contains_key(name) {
            return Err(TmpfsError::AlreadyExists);
        }

        // Allocate inode
        let ino = self.alloc_ino();
        let inode = Arc::new(TmpfsInode::new_file(ino, mode, uid, gid));

        // Add to parent directory
        let parent_mut = unsafe { &mut *(Arc::as_ptr(&parent) as *mut TmpfsInode) };
        parent_mut.children.insert(name.into(), ino);
        parent_mut.nlink.fetch_add(1, Ordering::Release);

        // Insert inode
        self.inodes.insert(ino, inode);

        Ok(ino)
    }

    /// Create directory
    pub fn create_dir(
        &mut self,
        parent_path: &str,
        name: &str,
        mode: u16,
        uid: u32,
        gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent = self.lookup(parent_path)?;

        if parent.inode_type != InodeType::Directory {
            return Err(TmpfsError::NotDirectory);
        }

        if parent.children.contains_key(name) {
            return Err(TmpfsError::AlreadyExists);
        }

        let ino = self.alloc_ino();
        let inode = Arc::new(TmpfsInode::new_dir(ino, mode, uid, gid));

        let parent_mut = unsafe { &mut *(Arc::as_ptr(&parent) as *mut TmpfsInode) };
        parent_mut.children.insert(name.into(), ino);
        parent_mut.nlink.fetch_add(1, Ordering::Release);

        self.inodes.insert(ino, inode);

        Ok(ino)
    }

    /// Create symlink
    pub fn create_symlink(
        &mut self,
        parent_path: &str,
        name: &str,
        target: String,
        uid: u32,
        gid: u32,
    ) -> Result<u64, TmpfsError> {
        let parent = self.lookup(parent_path)?;

        if parent.inode_type != InodeType::Directory {
            return Err(TmpfsError::NotDirectory);
        }

        if parent.children.contains_key(name) {
            return Err(TmpfsError::AlreadyExists);
        }

        let ino = self.alloc_ino();
        let inode = Arc::new(TmpfsInode::new_symlink(ino, target, uid, gid));

        let parent_mut = unsafe { &mut *(Arc::as_ptr(&parent) as *mut TmpfsInode) };
        parent_mut.children.insert(name.into(), ino);

        self.inodes.insert(ino, inode);

        Ok(ino)
    }

    /// Unlink (delete) file or directory
    pub fn unlink(&mut self, parent_path: &str, name: &str) -> Result<(), TmpfsError> {
        let parent = self.lookup(parent_path)?;

        if parent.inode_type != InodeType::Directory {
            return Err(TmpfsError::NotDirectory);
        }

        let ino = parent
            .children
            .get(name)
            .copied()
            .ok_or(TmpfsError::NotFound)?;

        let inode = self.get_inode(ino)?;

        // Cannot unlink non-empty directory
        if inode.inode_type == InodeType::Directory && !inode.children.is_empty() {
            return Err(TmpfsError::DirectoryNotEmpty);
        }

        // Decrease link count
        let nlink = inode.nlink.fetch_sub(1, Ordering::Release) - 1;

        // Remove from parent
        let parent_mut = unsafe { &mut *(Arc::as_ptr(&parent) as *mut TmpfsInode) };
        parent_mut.children.remove(name);

        // Remove inode if no more links
        if nlink == 0 {
            self.inodes.remove(&ino);
        }

        Ok(())
    }

    /// Get filesystem statistics
    pub fn statfs(&self) -> TmpfsStats {
        let used = self.used_size.load(Ordering::Acquire);

        TmpfsStats {
            total_size: self.max_size as u64,
            used_size: used,
            free_size: (self.max_size as u64).saturating_sub(used),
            total_inodes: self.inodes.len() as u64,
        }
    }
}

/// Tmpfs statistics
#[derive(Debug, Clone, Copy)]
pub struct TmpfsStats {
    pub total_size: u64,
    pub used_size: u64,
    pub free_size: u64,
    pub total_inodes: u64,
}

/// Tmpfs errors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TmpfsError {
    NotFound,
    AlreadyExists,
    NotDirectory,
    NotRegularFile,
    DirectoryNotEmpty,
    OutOfSpace,
    PermissionDenied,
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tmpfs_creation() {
        let fs = TmpfsFilesystem::new(1024 * 1024); // 1 MB
        let root = fs.lookup("/").unwrap();
        assert_eq!(root.inode_type, InodeType::Directory);
    }

    #[test]
    fn test_file_creation() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        let ino = fs.create_file("/", "test.txt", 0o644, 0, 0).unwrap();

        let file = fs.get_inode(ino).unwrap();
        assert_eq!(file.inode_type, InodeType::Regular);
    }

    #[test]
    fn test_directory_creation() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        fs.create_dir("/", "testdir", 0o755, 0, 0).unwrap();

        let dir = fs.lookup("/testdir").unwrap();
        assert_eq!(dir.inode_type, InodeType::Directory);
    }

    #[test]
    fn test_file_read_write() {
        let mut fs = TmpfsFilesystem::new(1024 * 1024);
        let ino = fs.create_file("/", "test.txt", 0o644, 0, 0).unwrap();

        let inode = fs.get_inode(ino).unwrap();
        let inode_mut = unsafe { &mut *(Arc::as_ptr(&inode) as *mut TmpfsInode) };

        let data = b"Hello, tmpfs!";
        inode_mut.write(0, data).unwrap();

        let mut buf = [0u8; 100];
        let len = inode.read(0, &mut buf).unwrap();

        assert_eq!(len, data.len());
        assert_eq!(&buf[..len], data);
    }
}
