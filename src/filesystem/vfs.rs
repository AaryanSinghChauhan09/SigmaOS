// SigmaOS Virtual Filesystem (VFS)
// Capability-based, standard Linux/BSD conforming filesystem with security, hard links, and path traversal

use crate::security::{CapabilityToken, Permission};
// Note: Using klib HashMap instead of std::collections::HashMap
use crate::klib::hashmap::HashMap;

// Standard POSIX / Linux / BSD open flags
pub const O_RDONLY: u32 = 0x0000;
pub const O_WRONLY: u32 = 0x0001;
pub const O_RDWR: u32 = 0x0002;
pub const O_CREAT: u32 = 0x0040;
pub const O_EXCL: u32 = 0x0080;
pub const O_TRUNC: u32 = 0x0200;
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
    // Conforming Linux/BSD additions
    pub hard_links_count: u32,
    pub link_count: u32,
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
            hard_links_count: 1,
            link_count: 1,
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
    inodes: HashMap<u64, Inode>,
    next_inode_id: u64,
    root_inode: u64,
    file_descriptors: HashMap<u64, FileDescriptor>,
    next_fd: u64,
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
        inode.hard_links_count = inode.link_count;
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

        // Check file offset and file size
        if file_descriptor.offset >= inode.size {
            return Ok(0);
        }

        let remaining = (inode.size - file_descriptor.offset) as usize;
        let bytes_to_read = buffer.len().min(remaining);

        // Prevent integer overflow in offset calculation
        let _new_offset = file_descriptor
            .offset
            .checked_add(bytes_to_read as u64)
            .ok_or(FsError::InvalidFd)?;

        // Read the actual bytes from storage data
        let start = file_descriptor.offset as usize;
        let end = start + bytes_to_read;
        buffer[..bytes_to_read].copy_from_slice(&inode.data[start..end]);

        file_descriptor.offset += bytes_to_read as u64;
        Ok(bytes_to_read)
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

        // If open flag O_APPEND is set, offset is moved to the end of the file before each write
        if (file_descriptor.flags & O_APPEND) != 0 {
            file_descriptor.offset = inode.size;
        }

        // Prevent integer overflow in size and offset calculation
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Resize storage data buffer if offset + written bytes exceeds size (handling holes)
        if new_offset > inode.size {
            inode.data.resize(new_offset as usize, 0);
            inode.size = new_offset;
        }

        // Write the actual bytes into file storage data
        let start = file_descriptor.offset as usize;
        let end = start + buffer.len();
        inode.data[start..end].copy_from_slice(buffer);

        file_descriptor.offset = new_offset;
        inode.modified = 1716000000; // Simulated timestamp

        Ok(buffer.len())
    }

    /// Read file guarded behind explicit capability token permission validation (Phase 2.1)
    pub fn read_file_gated(&mut self, fd: u64, buffer: &mut [u8], token: &CapabilityToken) -> Result<usize, FsError> {
        if !token.has_permission(Permission::FileRead) {
            return Err(FsError::PermissionDenied);
        }
        self.read_file(fd, buffer)
    }

    /// Write file guarded behind explicit capability token permission validation (Phase 2.1)
    pub fn write_file_gated(&mut self, fd: u64, buffer: &[u8], token: &CapabilityToken) -> Result<usize, FsError> {
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
            if inode.link_count > 1 {
                inode.link_count -= 1;
                inode.hard_links_count = inode.link_count;
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

    pub fn get_inode(&self, inode_id: u64) -> Option<&Inode> {
        self.inodes.get(&inode_id)
    }

    pub fn list_directory(&self, inode_id: u64) -> Result<Vec<u64>, FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;

        if inode.file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        // Return child inode list of the directory
        let mut list: Vec<u64> = inode.entries.values().copied().collect();
        list.sort();
        Ok(list)
    }

    // Advanced Linux & BSD Inspired Path Traversal, O_CREAT, and Link Handling

    /// Normalizes and canonicalizes a path into its standard absolute Linux/BSD POSIX path format.
    /// Handles '.', '..', redundant slashes, and relative path resolution.
    pub fn canonicalize_path(&self, current_dir: &str, path: &str) -> String {
        let absolute = if path.starts_with('/') {
            path.to_string()
        } else {
            let base = if current_dir.ends_with('/') {
                current_dir.to_string()
            } else {
                format!("{}/", current_dir)
            };
            format!("{}{}", base, path)
        };

        let mut stack: Vec<&str> = Vec::new();
        for component in absolute.split('/') {
            match component {
                "" | "." => continue,
                ".." => {
                    stack.pop();
                }
                c => stack.push(c),
            }
        }

        if stack.is_empty() {
            "/".to_string()
        } else {
            format!("/{}", stack.join("/"))
        }
    }

    /// Traverses and resolves a path name (e.g. "/var/log/syslog") to its Inode ID
    pub fn resolve_path(&self, path: &str) -> Result<u64, FsError> {
        if path.is_empty() {
            return Err(FsError::NotFound);
        }

        let mut current_inode_id = self.root_inode;
        let components: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

        for component in components {
            let inode = self
                .inodes
                .get(&current_inode_id)
                .ok_or(FsError::NotFound)?;
            if inode.file_type != FileType::Directory {
                return Err(FsError::NotADirectory);
            }
            if let Some(&next_id) = inode.entries.get(component) {
                current_inode_id = next_id;
            } else {
                return Err(FsError::NotFound);
            }
        }

        Ok(current_inode_id)
    }

    /// Open path with creation, exclusion, truncation, and append logic matching POSIX
    pub fn open_path(&mut self, path: &str, flags: u32, owner: u64) -> Result<u64, FsError> {
        // Resolve parent and target component
        let path_str = path.to_string();
        let mut parts: Vec<&str> = path_str.split('/').filter(|s| !s.is_empty()).collect();

        let filename = parts.pop().ok_or(FsError::NotFound)?;
        let mut parent_inode_id = self.root_inode;

        if !parts.is_empty() {
            let mut parent_path = String::new();
            for part in parts {
                parent_path.push('/');
                parent_path.push_str(part);
            }
            parent_inode_id = self.resolve_path(&parent_path)?;
        }

        let parent_inode = self.inodes.get(&parent_inode_id).ok_or(FsError::NotFound)?;
        if parent_inode.file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }

        let target_inode_id = parent_inode.entries.get(filename).copied();

        let inode_id = match target_inode_id {
            Some(id) => {
                if (flags & O_CREAT) != 0 && (flags & O_EXCL) != 0 {
                    return Err(FsError::AlreadyExists);
                }
                id
            }
            None => {
                if (flags & O_CREAT) != 0 {
                    let new_id = self.create_file(FileType::Regular, owner)?;
                    // Link into parent directory
                    let parent = self
                        .inodes
                        .get_mut(&parent_inode_id)
                        .ok_or(FsError::NotFound)?;
                    parent.entries.insert(filename.to_string(), new_id);
                    new_id
                } else {
                    return Err(FsError::NotFound);
                }
            }
        };

        // Apply O_TRUNC if write permission is allowed
        if (flags & O_TRUNC) != 0 {
            let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
            if !inode.permissions.write {
                return Err(FsError::PermissionDenied);
            }
            inode.data.clear();
            inode.size = 0;
        }

        self.open_file(inode_id, flags)
    }

    /// Link target path to link path (POSIX link/hard link logic)
    pub fn link_inode(&mut self, target_path: &str, link_path: &str) -> Result<(), FsError> {
        let target_id = self.resolve_path(target_path)?;

        let link_str = link_path.to_string();
        let mut parts: Vec<&str> = link_str.split('/').filter(|s| !s.is_empty()).collect();
        let filename = parts.pop().ok_or(FsError::NotFound)?;
        let mut parent_inode_id = self.root_inode;

        if !parts.is_empty() {
            let mut parent_path = String::new();
            for part in parts {
                parent_path.push('/');
                parent_path.push_str(part);
            }
            parent_inode_id = self.resolve_path(&parent_path)?;
        }

        // Verify parent directory
        let parent = self.inodes.get(&parent_inode_id).ok_or(FsError::NotFound)?;
        if parent.file_type != FileType::Directory {
            return Err(FsError::NotADirectory);
        }
        if parent.entries.contains_key(filename) {
            return Err(FsError::AlreadyExists);
        }

        // Perform hard-linking
        let parent_mut = self
            .inodes
            .get_mut(&parent_inode_id)
            .ok_or(FsError::NotFound)?;
        parent_mut.entries.insert(filename.to_string(), target_id);

        let target_inode = self.inodes.get_mut(&target_id).ok_or(FsError::NotFound)?;
        target_inode.hard_links_count += 1;

        Ok(())
    }

    /// Unlinks (deletes directory entry) and decrements link count
    pub fn unlink_inode(&mut self, path: &str) -> Result<(), FsError> {
        let path_str = path.to_string();
        let mut parts: Vec<&str> = path_str.split('/').filter(|s| !s.is_empty()).collect();
        let filename = parts.pop().ok_or(FsError::NotFound)?;
        let mut parent_inode_id = self.root_inode;

        if !parts.is_empty() {
            let mut parent_path = String::new();
            for part in parts {
                parent_path.push('/');
                parent_path.push_str(part);
            }
            parent_inode_id = self.resolve_path(&parent_path)?;
        }

        let parent = self
            .inodes
            .get_mut(&parent_inode_id)
            .ok_or(FsError::NotFound)?;
        let target_id = parent.entries.remove(filename).ok_or(FsError::NotFound)?;

        self.delete_file(target_id)
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
    AlreadyExists,
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
    fn test_gated_read_write() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 100).unwrap();
        let fd = vfs.open_file(inode_id, 0).unwrap();

        let bad_token = CapabilityToken::new(); // no read or write permissions
        let read_token = CapabilityToken::new().allow_read("/var/www");
        let write_token = CapabilityToken::new().allow_write("/tmp");
        let _all_token = CapabilityToken::new().allow_read("/var/www").allow_write("/tmp");

        let mut buf = [0u8; 10];

        // Write should fail with bad_token and read_token, but succeed with write_token or all_token
        assert_eq!(vfs.write_file_gated(fd, b"gated", &bad_token), Err(FsError::PermissionDenied));
        assert_eq!(vfs.write_file_gated(fd, b"gated", &read_token), Err(FsError::PermissionDenied));
        assert!(vfs.write_file_gated(fd, b"gated", &write_token).is_ok());

        // Re-open file to reset offset to 0 for reading
        let read_fd = vfs.open_file(inode_id, 0).unwrap();

        // Read should fail with bad_token and write_token, but succeed with read_token or all_token
        assert_eq!(vfs.read_file_gated(read_fd, &mut buf, &bad_token), Err(FsError::PermissionDenied));
        assert_eq!(vfs.read_file_gated(read_fd, &mut buf, &write_token), Err(FsError::PermissionDenied));
        assert_eq!(vfs.read_file_gated(read_fd, &mut buf, &read_token), Ok(5));
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

    #[test]
    fn test_canonicalize_path() {
        let vfs = VirtualFilesystem::new();
        assert_eq!(vfs.canonicalize_path("/var/log", "syslog"), "/var/log/syslog");
        assert_eq!(vfs.canonicalize_path("/var/log", "../mail/../log/./syslog"), "/var/log/syslog");
        assert_eq!(vfs.canonicalize_path("/home/user", "/usr/bin/../../etc/passwd"), "/etc/passwd");
        assert_eq!(vfs.canonicalize_path("/home/user", ".."), "/home");
    }
}
