// SigmaOS Virtual Filesystem (VFS)
// Capability-based filesystem with security

use crate::klib::HashMap;
use crate::security::CapabilityToken;

// Standard File Status & Access Modes inspired by Linux distros
pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_APPEND: u32 = 8;
pub const O_NONBLOCK: u32 = 4096;
pub const FD_CLOEXEC: u32 = 1;

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
        }
    }
}

/// System-wide Open File Description (OFD) inspired by Linux
/// tracks shared offset, file status flags, and description reference count.
#[derive(Debug, Clone)]
pub struct OpenFileDescription {
    pub inode_id: u64,
    pub offset: u64,
    pub flags: u32,       // e.g. O_APPEND, O_NONBLOCK
    pub ref_count: usize, // Reference count for shared file descriptions (for dup/dup2)
}

impl OpenFileDescription {
    pub fn new(inode_id: u64, flags: u32) -> Self {
        Self {
            inode_id,
            offset: 0,
            flags,
            ref_count: 1,
        }
    }
}

/// Process-Local File Descriptor
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub ofd_id: u64,      // Points to the system-wide OpenFileDescription
    pub flags: u32,       // FD-specific flags (e.g. FD_CLOEXEC)
}

impl FileDescriptor {
    pub fn new(ofd_id: u64, flags: u32) -> Self {
        Self {
            ofd_id,
            flags,
        }
    }
}

/// Virtual Filesystem
pub struct VirtualFilesystem {
    pub inodes: HashMap<u64, Inode>,
    pub next_inode_id: u64,
    pub root_inode: u64,

    // System-wide Open File Descriptions
    pub open_file_descriptions: HashMap<u64, OpenFileDescription>,
    pub next_ofd_id: u64,

    // Process-Private File Descriptors
    pub file_descriptors: HashMap<u64, FileDescriptor>,
    pub next_fd: u64,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        let mut fs = Self {
            inodes: HashMap::new(),
            next_inode_id: 1,
            root_inode: 0,
            open_file_descriptions: HashMap::new(),
            next_ofd_id: 1,
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

    pub fn open_file(&mut self, inode_id: u64, flags: u32) -> Result<u64, FsError> {
        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }

        // Allocate system-wide Open File Description (OFD)
        let ofd_id = self.next_ofd_id;
        self.next_ofd_id += 1;
        let ofd = OpenFileDescription::new(inode_id, flags);
        self.open_file_descriptions.insert(ofd_id, ofd);

        // Allocate process-private File Descriptor (FD)
        let fd = self.next_fd;
        self.next_fd += 1;
        let file_descriptor = FileDescriptor::new(ofd_id, 0);
        self.file_descriptors.insert(fd, file_descriptor);

        Ok(fd)
    }

    /// Duplicate file descriptor (dup parity)
    /// Shares the same OpenFileDescription and current file offset.
    pub fn dup_file(&mut self, fd: u64) -> Result<u64, FsError> {
        let file_descriptor = self
            .file_descriptors
            .get(&fd)
            .ok_or(FsError::InvalidFd)?
            .clone();

        let ofd = self
            .open_file_descriptions
            .get_mut(&file_descriptor.ofd_id)
            .ok_or(FsError::InvalidFd)?;

        ofd.ref_count += 1;

        let new_fd = self.next_fd;
        self.next_fd += 1;

        let new_file_descriptor = FileDescriptor::new(file_descriptor.ofd_id, file_descriptor.flags);
        self.file_descriptors.insert(new_fd, new_file_descriptor);

        Ok(new_fd)
    }

    /// Duplicate file descriptor onto a specific FD (dup2 parity)
    pub fn dup2_file(&mut self, old_fd: u64, new_fd: u64) -> Result<(), FsError> {
        if old_fd == new_fd {
            if !self.file_descriptors.contains_key(&old_fd) {
                return Err(FsError::InvalidFd);
            }
            return Ok(());
        }

        let file_descriptor = self
            .file_descriptors
            .get(&old_fd)
            .ok_or(FsError::InvalidFd)?
            .clone();

        // If new_fd is already open, close it first
        if self.file_descriptors.contains_key(&new_fd) {
            let _ = self.close_file(new_fd);
        }

        let ofd = self
            .open_file_descriptions
            .get_mut(&file_descriptor.ofd_id)
            .ok_or(FsError::InvalidFd)?;

        ofd.ref_count += 1;

        let new_file_descriptor = FileDescriptor::new(file_descriptor.ofd_id, file_descriptor.flags);
        self.file_descriptors.insert(new_fd, new_file_descriptor);

        Ok(())
    }

    pub fn close_file(&mut self, fd: u64) -> Result<(), FsError> {
        let file_descriptor = self
            .file_descriptors
            .get(&fd)
            .ok_or(FsError::InvalidFd)?
            .clone();

        let ofd = self
            .open_file_descriptions
            .get_mut(&file_descriptor.ofd_id)
            .ok_or(FsError::InvalidFd)?;

        ofd.ref_count -= 1;
        if ofd.ref_count == 0 {
            self.open_file_descriptions.remove(&file_descriptor.ofd_id);
        }

        self.file_descriptors.remove(&fd);
        Ok(())
    }

    pub fn read_file(&mut self, fd: u64, buffer: &mut [u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .file_descriptors
            .get(&fd)
            .ok_or(FsError::InvalidFd)?;

        let ofd = self
            .open_file_descriptions
            .get_mut(&file_descriptor.ofd_id)
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get(&ofd.inode_id)
            .ok_or(FsError::NotFound)?;

        // Check read permission
        if !inode.permissions.read {
            return Err(FsError::PermissionDenied);
        }

        // Prevent integer overflow in offset calculation
        let _new_offset = ofd
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::InvalidFd)?;

        // Simulate read
        let bytes_read = buffer.len().min(inode.size as usize);
        ofd.offset += bytes_read as u64;

        Ok(bytes_read)
    }

    pub fn write_file(&mut self, fd: u64, buffer: &[u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .file_descriptors
            .get(&fd)
            .ok_or(FsError::InvalidFd)?;

        let ofd = self
            .open_file_descriptions
            .get_mut(&file_descriptor.ofd_id)
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get_mut(&ofd.inode_id)
            .ok_or(FsError::NotFound)?;

        // Check write permission
        if !inode.permissions.write {
            return Err(FsError::PermissionDenied);
        }

        // Handle O_APPEND status flag inspired by Linux distros
        if (ofd.flags & O_APPEND) != 0 {
            ofd.offset = inode.size;
        }

        // Prevent integer overflow in size calculation
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Prevent integer overflow in offset calculation
        let _new_offset = ofd
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Simulate write
        let bytes_written = buffer.len();
        inode.size += bytes_written as u64;
        ofd.offset += bytes_written as u64;
        inode.modified = 0;

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

        // Return all inodes
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
    fn test_file_descriptor_sharing_via_dup() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        let fd1 = vfs.open_file(inode_id, 0).unwrap();
        let fd2 = vfs.dup_file(fd1).unwrap();

        // Write via fd1 should advance the offset of fd2 as they share the same OFD
        let data = b"shared offset test";
        vfs.write_file(fd1, data).unwrap();

        let ofd1_id = vfs.file_descriptors.get(&fd1).unwrap().ofd_id;
        let ofd2_id = vfs.file_descriptors.get(&fd2).unwrap().ofd_id;
        assert_eq!(ofd1_id, ofd2_id);

        let ofd = vfs.open_file_descriptions.get(&ofd1_id).unwrap();
        assert_eq!(ofd.offset, data.len() as u64);
        assert_eq!(ofd.ref_count, 2);

        vfs.close_file(fd1).unwrap();
        // ofd should still exist since fd2 is open
        assert!(vfs.open_file_descriptions.contains_key(&ofd1_id));

        vfs.close_file(fd2).unwrap();
        // ofd should be removed now
        assert!(!vfs.open_file_descriptions.contains_key(&ofd1_id));
    }

    #[test]
    fn test_dup2_overwrite() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id1 = vfs.create_file(FileType::Regular, 100).unwrap();
        let inode_id2 = vfs.create_file(FileType::Regular, 100).unwrap();

        let fd1 = vfs.open_file(inode_id1, 0).unwrap();
        let fd2 = vfs.open_file(inode_id2, 0).unwrap();

        let ofd2_id = vfs.file_descriptors.get(&fd2).unwrap().ofd_id;

        // dup2 should close fd2 and overwrite it to point to fd1's OFD
        vfs.dup2_file(fd1, fd2).unwrap();

        let ofd1_id = vfs.file_descriptors.get(&fd1).unwrap().ofd_id;
        let new_ofd2_id = vfs.file_descriptors.get(&fd2).unwrap().ofd_id;

        assert_eq!(ofd1_id, new_ofd2_id);
        // Previous OFD of fd2 should be garbage collected if ref_count became 0
        assert!(!vfs.open_file_descriptions.contains_key(&ofd2_id));
    }
}
