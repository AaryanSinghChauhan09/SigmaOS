// SigmaOS Virtual Filesystem (VFS)
// Capability-based filesystem with security
// Enhanced with standard Linux-conforming Hard Link reference counting

use crate::security::CapabilityToken;
use std::collections::HashMap;

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    CharacterDevice,
    BlockDevice,
    Symlink,
}

/// File permissions
#[derive(Debug, Clone, Copy)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        Self {
            read,
            write,
            execute,
        }
    }

    pub fn all() -> Self {
        Self::new(true, true, true)
    }

    pub fn read_only() -> Self {
        Self::new(true, false, false)
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
    pub link_count: u32, // standard inode link count tracking hard links
    pub hard_links_count: u32, // standard Linux reference links counter
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
            link_count: 1, // default link count of 1
            hard_links_count: 1, // Default initial link
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
        let mut fs = Self {
            inodes: HashMap::new(),
            next_inode_id: 1,
            root_inode: 0,
            file_descriptors: HashMap::new(),
            next_fd: 0,
        };

        // Create root directory
        let root = Inode::new(0, FileType::Directory, 0);
        fs.inodes.insert(0, root);
        fs.root_inode = 0;

        fs
    }

    pub fn create_file(&mut self, file_type: FileType, owner: u64) -> Result<u64, FsError> {
        let inode_id = self.next_inode_id;
        self.next_inode_id += 1;

        let inode = Inode::new(inode_id, file_type, owner);
        self.inodes.insert(inode_id, inode);

        Ok(inode_id)
    }

    /// Linux-parity Hard Link creator: points a new reference to an existing inode
    pub fn link_inode(&mut self, old_inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&old_inode_id).ok_or(FsError::NotFound)?;

        // Linux FHS constraint: prevent directory hard links to avoid circular loops
        if inode.file_type == FileType::Directory {
            return Err(FsError::IsDirectory);
        }

        inode.hard_links_count += 1;
        Ok(())
    }

    /// Linux-parity Unlink handler: decrements reference links, freeing storage only when count hits 0
    pub fn unlink_inode(&mut self, inode_id: u64) -> Result<u32, FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;

        if inode.hard_links_count > 1 {
            inode.hard_links_count -= 1;
            Ok(inode.hard_links_count)
        } else {
            // Reference link hit 0, fully free the physical Inode from VFS metadata
            self.inodes.remove(&inode_id);
            Ok(0)
        }
    }

    pub fn open_file(&mut self, inode_id: u64, flags: u32) -> Result<u64, FsError> {
        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }

        let fd = self.next_fd;
        self.next_fd += 1;

        let file_descriptor = FileDescriptor::new(inode_id, flags);
        self.file_descriptors.insert(fd, file_descriptor);

        Ok(fd)
    }

    pub fn close_file(&mut self, fd: u64) -> Result<(), FsError> {
        if !self.file_descriptors.contains_key(&fd) {
            return Err(FsError::InvalidFd);
        }

        self.file_descriptors.remove(&fd);
        Ok(())
    }

    pub fn read_file(&mut self, fd: u64, buffer: &mut [u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .file_descriptors
            .get_mut(&fd)
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get(&file_descriptor.inode_id)
            .ok_or(FsError::NotFound)?;

        // Check read permission
        if !inode.permissions.read {
            return Err(FsError::PermissionDenied);
        }

        // Prevent integer overflow in offset calculation
        let _new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::InvalidFd)?;

        // Simulate read (in production, actual file I/O)
        let bytes_read = buffer.len().min(inode.size as usize);
        file_descriptor.offset += bytes_read as u64;

        Ok(bytes_read)
    }

    pub fn write_file(&mut self, fd: u64, buffer: &[u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .file_descriptors
            .get_mut(&fd)
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get_mut(&file_descriptor.inode_id)
            .ok_or(FsError::NotFound)?;

        // Check write permission
        if !inode.permissions.write {
            return Err(FsError::PermissionDenied);
        }

        // Prevent integer overflow in size calculation
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Prevent integer overflow in offset calculation
        let _new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Simulate write (in production, actual file I/O)
        let bytes_written = buffer.len();
        inode.size += bytes_written as u64;
        file_descriptor.offset += bytes_written as u64;
        inode.modified = 0; // In production, actual timestamp

        Ok(bytes_written)
    }

    pub fn create_hard_link(&mut self, source_inode_id: u64) -> Result<(), FsError> {
        if let Some(inode) = self.inodes.get_mut(&source_inode_id) {
            inode.link_count += 1;
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
    }

    pub fn delete_file(&mut self, inode_id: u64) -> Result<(), FsError> {
        if inode_id == self.root_inode {
            return Err(FsError::PermissionDenied);
        }

        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }

        let link_reached_zero = if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.link_count = inode.link_count.saturating_sub(1);
            inode.link_count == 0
        } else {
            false
        };

        if link_reached_zero {
            self.inodes.remove(&inode_id);
        }
        
        self.inodes.remove(&inode_id);

        self.inodes.remove(&inode_id);
        Ok(())
    }

    pub fn get_inode(&self, inode_id: u64) -> Option<&Inode> {
        self.inodes.get(&inode_id)
    }

    pub fn list_directory(&self, inode_id: u64) -> Result<Vec<u64>, FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;

        if inode.file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        // Return all inodes (in production, actual directory listing)
        Ok(self.inodes.keys().copied().collect())
    }
}

impl Default for VirtualFilesystem {
    fn default() -> Self {
        Self::new()
    }
}

/// Filesystem errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidFd,
    NotADirectory,
    IsDirectory,
    NoSpace,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vfs_creation() {
        let vfs = VirtualFilesystem::new();
        assert!(vfs.inodes.contains_key(&0));
    }

    #[test]
    fn test_hard_links_and_unlink() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);

        // Create hard link (link_count = 2)
        vfs.create_hard_link(inode_id).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 2);

        // First deletion (link_count = 1, file should NOT be removed)
        vfs.delete_file(inode_id).unwrap();
        assert!(vfs.inodes.contains_key(&inode_id));
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);

        // Second deletion (link_count = 0, file should be removed)
        vfs.delete_file(inode_id).unwrap();
        assert!(!vfs.inodes.contains_key(&inode_id));
    }

    #[test]
    fn test_create_file() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        assert!(vfs.inodes.contains_key(&inode_id));
        assert_eq!(vfs.get_inode(inode_id).unwrap().hard_links_count, 1);
    }

    #[test]
    fn test_open_close_file() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        let fd = vfs.open_file(inode_id, 0).unwrap();
        assert!(vfs.close_file(fd).is_ok());
    }

    #[test]
    fn test_read_write() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        let fd = vfs.open_file(inode_id, 0).unwrap();

        let data = b"test data";
        let written = vfs.write_file(fd, data).unwrap();
        assert_eq!(written, data.len());
    }

    #[test]
    fn test_linux_hard_links_flow() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 101).unwrap();

        // Link same inode twice (creating hard links)
        assert!(vfs.link_inode(inode_id).is_ok());
        assert_eq!(vfs.get_inode(inode_id).unwrap().hard_links_count, 2);

        // Attempting to hard link directory should fail (avoid loops)
        assert!(vfs.link_inode(0).is_err()); // Root is directory

        // Unlink first hard link
        let count = vfs.unlink_inode(inode_id).unwrap();
        assert_eq!(count, 1);
        assert!(vfs.inodes.contains_key(&inode_id)); // Inode still exists

        // Unlink second hard link
        let count = vfs.unlink_inode(inode_id).unwrap();
        assert_eq!(count, 0);
        assert!(!vfs.inodes.contains_key(&inode_id)); // Inode fully freed
    }
}
