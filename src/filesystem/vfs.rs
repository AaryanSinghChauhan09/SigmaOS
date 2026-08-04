// SigmaOS Virtual Filesystem (VFS)
// Capability-based, standard Linux/BSD conforming filesystem with security, hard links, and path traversal

use crate::security::CapabilityToken;
use std::collections::HashMap;

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
    pub link_count: u32, // standard inode link count tracking hard links
||||||| 43be3a7e8
    // Conforming Linux/BSD additions
    pub hard_links_count: u32,
    pub data: Vec<u8>,                 // File storage data
    pub entries: HashMap<String, u64>, // Directory entries
||||||| 984d1301f
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
||||||| 43be3a7e8
            hard_links_count: 1,
            data: Vec::new(),
            entries: HashMap::new(),
||||||| 984d1301f
            link_count: 1, // default link count of 1
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
        fs.directory_paths.insert("/".to_string(), 0);

||||||| 43be3a7e8
        

||||||| 43be3a7e8
        

||||||| 43be3a7e8
        

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

||||||| 43be3a7e8
        

        // Check file offset and file size
        if file_descriptor.offset >= inode.size {
            return Ok(0);
        }

        let remaining = (inode.size - file_descriptor.offset) as usize;
        let bytes_to_read = buffer.len().min(remaining);

||||||| 43be3a7e8
        

||||||| 43be3a7e8
        

        // Prevent integer overflow in offset calculation
        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
        let _new_offset = file_descriptor
            .offset
            .checked_add(bytes_to_read as u64)
||||||| 43be3a7e8
        let new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::InvalidFd)?;

        // Simulate read (in production, actual file I/O)
        let bytes_read = buffer.len().min(inode.size as usize);
        file_descriptor.offset += bytes_read as u64;

        Ok(bytes_read)
||||||| 43be3a7e8
        
||||||| 43be3a7e8
        

        // Simulate read (in production, actual file I/O)
        let bytes_read = buffer.len().min(inode.size as usize);
        file_descriptor.offset += bytes_read as u64;

        Ok(bytes_read)

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

        // Prevent integer overflow in size calculation
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        // Prevent integer overflow in size calculation
        let _new_size = inode.size.checked_add(buffer.len() as u64)
        // If open flag O_APPEND is set, offset is moved to the end of the file before each write
        if (file_descriptor.flags & O_APPEND) != 0 {
            file_descriptor.offset = inode.size;
        }

        // Prevent integer overflow in size and offset calculation
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let _new_size = inode.size.checked_add(buffer.len() as u64)
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let _new_size = inode.size.checked_add(buffer.len() as u64)
        let _new_size = inode
            .size
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        // Prevent integer overflow in offset calculation
        let _new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        // Prevent integer overflow in offset calculation
        let _new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let _new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
        let _new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
||||||| 43be3a7e8
        let _new_offset = file_descriptor.offset.checked_add(buffer.len() as u64)
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
||||||| 43be3a7e8
        
||||||| 43be3a7e8
        

        // Simulate write (in production, actual file I/O)
        let bytes_written = buffer.len();
        inode.size += bytes_written as u64;
        file_descriptor.offset += bytes_written as u64;
        inode.modified = 0; // In production, actual timestamp

        Ok(bytes_written)

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

    pub fn create_hard_link(&mut self, source_inode_id: u64) -> Result<(), FsError> {
        if let Some(inode) = self.inodes.get_mut(&source_inode_id) {
            inode.link_count += 1;
            Ok(())
        } else {
            Err(FsError::NotFound)
        }
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
||||||| 43be3a7e8
        
||||||| 43be3a7e8
        

        if !self.inodes.contains_key(&inode_id) {

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

        let link_reached_zero = if let Some(inode) = self.inodes.get_mut(&inode_id) {
            inode.link_count = inode.link_count.saturating_sub(1);
            inode.link_count == 0
        } else {
            false
        };

        if link_reached_zero {
            self.inodes.remove(&inode_id);
        }
||||||| 43be3a7e8
        
||||||| 43be3a7e8
        

||||||| 43be3a7e8
        

        self.inodes.remove(&inode_id);

        if should_delete {
            self.inodes.remove(&inode_id);
        }
||||||| 984d1301f
        self.inodes.remove(&inode_id);
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

        // Return all inodes (in production, actual directory listing)
        Ok(self.inodes.keys().copied().collect())
||||||| 43be3a7e8
        
||||||| 43be3a7e8
        

        // Return all inodes (in production, actual directory listing)
        Ok(self.inodes.keys().copied().collect())

        // Return child inode list of the directory
        let mut list: Vec<u64> = inode.entries.values().copied().collect();
        list.sort();
        Ok(list)
    }

    // =========================================================================
    // Advanced Linux & BSD Inspired Path Traversal, O_CREAT, and Link Handling
    // =========================================================================

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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FsError {
    NotFound,
    PermissionDenied,
    InvalidFd,
    NotADirectory,
    IsDirectory,
    NoSpace,
    AlreadyExists,
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
}
