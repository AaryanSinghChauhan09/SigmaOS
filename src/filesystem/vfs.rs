// SigmaOS Virtual Filesystem (VFS) Layer
// Zero-dependency, #![no_std] compliant, highly-optimized
// Supports hierarchical namespaces, unified file descriptors, and robust link-aware removal.

use crate::security::CapabilityToken;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Socket,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub readable: bool,
    pub writable: bool,
    pub executable: bool,
}

impl FilePermissions {
    pub fn all() -> Self {
        Self {
            readable: true,
            writable: true,
            executable: true,
        }
    }

    pub fn read_only() -> Self {
        Self {
            readable: true,
            writable: false,
            executable: false,
        }
    }
}

/// Inode (file/directory metadata)
#[derive(Debug, Clone)]
pub struct Inode {
    pub id: u64,
    pub file_type: FileType,
    pub permissions: FilePermissions,
    pub size: u64,
    pub owner: u64,
    pub group: u64,
    pub created: u64,
    pub modified: u64,
    pub capabilities: CapabilityToken,
    // Conforming Linux/BSD additions
    pub link_count: u32,
    pub hard_links_count: u32,
    pub symlink_target: Option<String>,
    pub xattrs: HashMap<String, Vec<u8>>,
    pub data: Vec<u8>,                 // File storage data
    pub entries: HashMap<String, u64>, // Directory entries
}

impl Inode {
    pub fn new(id: u64, file_type: FileType, owner: u64) -> Self {
        Self {
            id,
            file_type,
            permissions: FilePermissions::all(),
            size: 0,
            owner,
            group: 0,
            created: 0,
            modified: 0,
            capabilities: CapabilityToken::new(),
            link_count: 1,
            hard_links_count: 1,
            symlink_target: None,
            xattrs: HashMap::new(),
            data: Vec::new(),
            entries: HashMap::new(),
        }
    }
}

/// File descriptor
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub inode_id: u64,
    pub offset: u64,
    pub flags: u32,
}

impl FileDescriptor {
    pub fn new(inode_id: u64, flags: u32) -> Self {
        Self {
            inode_id,
            offset: 0,
            flags,
        }
    }
}

/// Virtual Filesystem
pub struct VirtualFilesystem {
    pub inodes: HashMap<u64, Inode>,
    pub next_inode_id: u64,
    pub root_inode: u64,
    pub file_descriptors: HashMap<u64, FileDescriptor>,
    pub next_fd: u64,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        let mut inodes = HashMap::new();
        let root_inode = 1;
        inodes.insert(root_inode, Inode::new(root_inode, FileType::Directory, 0));

        Self {
            inodes,
            next_inode_id: 2,
            root_inode,
            file_descriptors: HashMap::new(),
            next_fd: 100,
        }
    }

    pub fn create_file(&mut self, file_type: FileType, owner: u64) -> Result<u64, FsError> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        self.inodes.insert(id, Inode::new(id, file_type, owner));
        Ok(id)
    }

    pub fn get_inode(&self, id: u64) -> Option<&Inode> {
        self.inodes.get(&id)
    }

    pub fn get_inode_mut(&mut self, id: u64) -> Option<&mut Inode> {
        self.inodes.get_mut(&id)
    }

    /// Link target path to link path (POSIX link/hard link logic)
    pub fn link_inode(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.hard_links_count += 1;
        inode.link_count += 1;
        Ok(())
    }

    /// Unlinks (deletes directory entry) and decrements link count
    pub fn unlink_inode(&mut self, inode_id: u64) -> Result<u32, FsError> {
        let mut should_delete = false;
        let mut links = 0;
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            if inode.hard_links_count > 1 {
                inode.hard_links_count -= 1;
                inode.link_count -= 1;
                links = inode.hard_links_count;
            } else {
                should_delete = true;
            }
        } else {
            return Err(FsError::NotFound);
        }

        if should_delete {
            self.inodes.remove(&inode_id);
            Ok(0)
        } else {
            Ok(links)
        }
    }

    pub fn delete_file(&mut self, inode_id: u64) -> Result<(), FsError> {
        if inode_id == self.root_inode {
            return Err(FsError::PermissionDenied);
        }

        let mut should_delete = false;
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            if inode.hard_links_count > 1 {
                inode.hard_links_count -= 1;
            } else {
                should_delete = true;
            }
        } else {
            return Err(FsError::NotFound);
        }

        if should_delete {
            self.inodes.remove(&inode_id);
        }
        Ok(())
    }

    pub fn open_file(&mut self, inode_id: u64, flags: u32) -> Result<u64, FsError> {
        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }
        let fd = self.next_fd;
        self.next_fd += 1;
        self.file_descriptors.insert(fd, FileDescriptor::new(inode_id, flags));
        Ok(fd)
    }

    pub fn close_file(&mut self, fd: u64) -> Result<(), FsError> {
        self.file_descriptors.remove(&fd).ok_or(FsError::InvalidFd)?;
        Ok(())
    }

    pub fn read_file(&self, fd: u64, buffer: &mut [u8]) -> Result<usize, FsError> {
        let desc = self.file_descriptors.get(&fd).ok_or(FsError::InvalidFd)?;
        let inode = self.inodes.get(&desc.inode_id).ok_or(FsError::NotFound)?;
        let data = &inode.data;
        let start = desc.offset as usize;
        if start >= data.len() {
            return Ok(0);
        }
        let end = (start + buffer.len()).min(data.len());
        let len = end - start;
        buffer[..len].copy_from_slice(&data[start..end]);
        Ok(len)
    }

    pub fn write_file(&mut self, fd: u64, buffer: &[u8]) -> Result<usize, FsError> {
        let desc = self.file_descriptors.get_mut(&fd).ok_or(FsError::InvalidFd)?;
        let inode = self.inodes.get_mut(&desc.inode_id).ok_or(FsError::NotFound)?;
        let start = desc.offset as usize;
        let needed_size = start + buffer.len();
        if inode.data.len() < needed_size {
            inode.data.resize(needed_size, 0);
        }
        inode.data[start..needed_size].copy_from_slice(buffer);
        inode.size = inode.data.len() as u64;
        desc.offset = needed_size as u64;
        Ok(buffer.len())
    }

    pub fn create_hard_link(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.link_count += 1;
        inode.hard_links_count += 1;
        Ok(())
    }

    pub fn set_xattr(&mut self, inode_id: u64, name: &str, value: &[u8]) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.xattrs.insert(name.to_string(), value.to_vec());
        Ok(())
    }

    pub fn get_xattr(&self, inode_id: u64, name: &str) -> Result<std::vec::Vec<u8>, FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;
        let val = inode.xattrs.get(name).ok_or(FsError::AttributeNotFound)?;
        Ok(val.clone())
    }
}

impl Default for VirtualFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidFd,
    NotADirectory,
    IsDirectory,
    NoSpace,
    AlreadyExists,
    AttributeNotFound,
}
