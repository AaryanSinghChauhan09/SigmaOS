// SigmaOS Virtual Filesystem (VFS)
// Capability-based filesystem with security and advanced Linux-inspired abstractions (xattrs, hardlinks, symlinks)

use crate::security::{CapabilityToken, Permission};
use crate::klib::HashMap;

pub const O_RDONLY: u32 = 0x0000;
pub const O_WRONLY: u32 = 0x0001;
pub const O_RDWR:   u32 = 0x0002;
pub const O_CREAT:  u32 = 0x0040;
pub const O_EXCL:   u32 = 0x0080;
pub const O_TRUNC:  u32 = 0x0200;
pub const O_APPEND: u32 = 0x0400;

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
    pub link_count: usize, // Standard Linux Inode reference link count (hard links)
    pub symlink_target: Option<String>, // Path targets for symbolic links
    pub xattrs: HashMap<String, std::vec::Vec<u8>>, // Extended attributes (e.g. user.mime_type)
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
            link_count: 1, // Starts at 1
            symlink_target: None,
            xattrs: HashMap::new(),
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
    next_inode_id: u64,
    root_inode: u64,
    file_descriptors: HashMap<u64, FileDescriptor>,
    next_fd: u64,
    pub directory_paths: HashMap<String, u64>,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        let mut fs = Self {
            inodes: HashMap::new(),
            next_inode_id: 1,
            root_inode: 0,
            file_descriptors: HashMap::new(),
            next_fd: 0,
            directory_paths: HashMap::new(),
        };

        // Create root directory
        let root = Inode::new(0, FileType::Directory, 0);
        fs.inodes.insert(0, root);
        fs.root_inode = 0;

        fs
    }

    /// Seed the filesystem with standard Linux-inspired directory hierarchies (/bin, /etc, /var, /home, /sys, /proc, /dev, /tmp)
    pub fn seed_standard_hierarchy(&mut self) -> Result<(), FsError> {
        let directories = [
            "/bin", "/etc", "/var", "/home", "/sys", "/proc", "/dev", "/tmp", "/boot", "/root",
            "/opt",
        ];

        for &dir in &directories {
            let inode_id = self.create_file(FileType::Directory, 0)?;
            self.directory_paths.insert(dir.to_string(), inode_id);
        }

        Ok(())
    }

    pub fn create_file(&mut self, file_type: FileType, owner: u64) -> Result<u64, FsError> {
        let inode_id = self.next_inode_id;
        self.next_inode_id += 1;

        let inode = Inode::new(inode_id, file_type, owner);
        self.inodes.insert(inode_id, inode);

        Ok(inode_id)
    }

    /// Creates a symbolic link (Symlink) pointing to a target filepath string
    pub fn create_symlink(&mut self, target_path: &str, owner: u64) -> Result<u64, FsError> {
        let inode_id = self.create_file(FileType::Symlink, owner)?;
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.symlink_target = Some(target_path.to_string());
        }
        Ok(inode_id)
    }

    /// Creates a hard link pointing directly to the same underlying file Inode
    pub fn create_hard_link(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.link_count += 1;
        Ok(())
    }

    /// Sets an extended attribute (xattr) on an active Inode
    pub fn set_xattr(&mut self, inode_id: u64, name: &str, value: &[u8]) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.xattrs.insert(name.to_string(), value.to_vec());
        Ok(())
    }

    /// Retrieves an extended attribute (xattr) from an active Inode
    pub fn get_xattr(&self, inode_id: u64, name: &str) -> Result<std::vec::Vec<u8>, FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;
        let val = inode.xattrs.get(name).ok_or(FsError::AttributeNotFound)?;
        Ok(val.clone())
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

    /// Read file guarded behind explicit capability token permission validation (Phase 2.1)
    pub fn read_file_gated(
        &mut self,
        fd: u64,
        buffer: &mut [u8],
        token: &CapabilityToken,
    ) -> Result<usize, FsError> {
        if !token.has_permission(Permission::FileRead) {
            return Err(FsError::PermissionDenied);
        }
        self.read_file(fd, buffer)
    }

    /// Write file guarded behind explicit capability token permission validation (Phase 2.1)
    pub fn write_file_gated(
        &mut self,
        fd: u64,
        buffer: &[u8],
        token: &CapabilityToken,
    ) -> Result<usize, FsError> {
        if !token.has_permission(Permission::FileWrite) {
            return Err(FsError::PermissionDenied);
        }
        self.write_file(fd, buffer)
    }

    /// Linux-grade link-aware file removal
    pub fn delete_file(&mut self, inode_id: u64) -> Result<(), FsError> {
        if inode_id == self.root_inode {
            return Err(FsError::PermissionDenied);
        }

        let mut should_delete = false;
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.link_count = inode.link_count.saturating_sub(1);
            if inode.link_count == 0 {
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

    pub fn get_inode(&self, inode_id: u64) -> Option<&Inode> {
        self.inodes.get(&inode_id)
    }

    pub fn list_directory(&self, inode_id: u64) -> Result<std::vec::Vec<u64>, FsError> {
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidFd,
    NotADirectory,
    IsDirectory,
    NoSpace,
    AttributeNotFound,
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
    fn test_standard_hierarchy_seeding() {
        let mut vfs = VirtualFilesystem::new();
        assert!(vfs.seed_standard_hierarchy().is_ok());

        assert!(vfs.directory_paths.contains_key("/bin"));
        assert!(vfs.directory_paths.contains_key("/etc"));
        assert!(vfs.directory_paths.contains_key("/home"));

        let bin_inode_id = vfs.directory_paths.get("/bin").unwrap();
        let bin_inode = vfs.get_inode(*bin_inode_id).unwrap();
        assert_eq!(bin_inode.file_type, FileType::Directory);
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
    fn test_gated_read_write() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        let fd = vfs.open_file(inode_id, 0).unwrap();

        let bad_token = CapabilityToken::new(); // no read or write permissions
        let read_token = CapabilityToken::new().allow_read("/var/www");
        let write_token = CapabilityToken::new().allow_write("/tmp");
        let _all_token = CapabilityToken::new()
            .allow_read("/var/www")
            .allow_write("/tmp");

        let mut buf = [0u8; 10];

        // Write should fail with bad_token and read_token, but succeed with write_token or all_token
        assert_eq!(
            vfs.write_file_gated(fd, b"gated", &bad_token),
            Err(FsError::PermissionDenied)
        );
        assert_eq!(
            vfs.write_file_gated(fd, b"gated", &read_token),
            Err(FsError::PermissionDenied)
        );
        assert!(vfs.write_file_gated(fd, b"gated", &write_token).is_ok());

        // Read should fail with bad_token and write_token, but succeed with read_token or all_token
        assert_eq!(
            vfs.read_file_gated(fd, &mut buf, &bad_token),
            Err(FsError::PermissionDenied)
        );
        assert_eq!(
            vfs.read_file_gated(fd, &mut buf, &write_token),
            Err(FsError::PermissionDenied)
        );
        assert_eq!(vfs.read_file_gated(fd, &mut buf, &read_token), Ok(5));
    }

    #[test]
    fn test_linux_hardlinks_symlinks_and_xattrs() {
        let mut vfs = VirtualFilesystem::new();

        // 1. Create a regular file with extended attribute (user.mime_type = "text/plain")
        let inode_id = vfs.create_file(FileType::Regular, 1000).unwrap();
        vfs.set_xattr(inode_id, "user.mime_type", b"text/plain").unwrap();
        assert_eq!(vfs.get_xattr(inode_id, "user.mime_type").unwrap(), b"text/plain");

        // 2. Create a symlink pointing to our file
        let symlink_id = vfs.create_symlink("/home/tc/file.txt", 1000).unwrap();
        assert_eq!(vfs.get_inode(symlink_id).unwrap().file_type, FileType::Symlink);
        assert_eq!(vfs.get_inode(symlink_id).unwrap().symlink_target.as_ref().unwrap(), "/home/tc/file.txt");

        // 3. Create a hard link -> increments link_count
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);
        vfs.create_hard_link(inode_id).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 2);

        // 4. Deleting the file first time simply decrements link_count and keeps underlying Inode alive!
        vfs.delete_file(inode_id).unwrap();
        assert!(vfs.get_inode(inode_id).is_some());
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);

        // 5. Deleting the file second time drops link_count to 0, successfully freeing the Inode from VFS!
        vfs.delete_file(inode_id).unwrap();
        assert!(vfs.get_inode(inode_id).is_none());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDescriptor {
    pub fd: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    Success = 0,
    NotFound = 1,
    PermissionDenied = 2,
    AlreadyExists = 3,
    NotADirectory = 4,
    IsADirectory = 5,
    NotEmpty = 6,
    IOError = 7,
    InvalidParameter = 8,
}

pub struct VirtualFilesystem {
    pub root: Inode,
}

impl VirtualFilesystem {
    pub fn new() -> Self {
        Self {
            root: Inode {
                id: 1,
                file_type: FileType::Directory,
                permissions: FilePermissions::all(),
                size: 0,
                owner: 0,
                group: 0,
                created: 0,
                modified: 0,
                capabilities: CapabilityToken::from_bits(!0),
                link_count: 1,
            },
        }
    }
}
    pub link_count: u32, // standard inode link count tracking hard links
}
