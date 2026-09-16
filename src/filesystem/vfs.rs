// SPDX-License-Identifier: MIT
use core::fmt;
use std::collections::{BTreeMap, HashMap};
use std::string::String;
use std::string::ToString;
use std::vec::Vec;
use crate::security::{CapabilityToken, Permission};

pub const O_RDONLY: u32 = 0;
pub const O_WRONLY: u32 = 1;
pub const O_RDWR: u32 = 2;
pub const O_CREAT: u32 = 0o100;
pub const O_EXCL: u32 = 0o200;
pub const O_TRUNC: u32 = 0o1000;
pub const O_APPEND: u32 = 0o2000;

/// BSD File Flags (chflags: nodump, uchg, schg, opaque, nounlink, sappnd, uappnd)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BsdFileFlags {
    pub nodump: bool,
    pub immutable: bool,
    pub append_only: bool,
    pub opaque: bool,
    pub nounlink: bool,
    pub archived: bool,
}

impl BsdFileFlags {
    pub fn new() -> Self {
        Self::default()
    }
}

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    Symlink,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
}

/// File mode bits (permissions + BSD file flags)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct FileMode {
    // POSIX permission bits
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
    // BSD file flags (chflags)
    pub nodump: bool,       // Do not dump (backup exclusion)
    pub immutable: bool,    // File cannot be changed
    pub append_only: bool,  // File can only be appended to
    pub opaque: bool,       // Directory is opaque for union mounts
    pub nounlink: bool,     // File cannot be renamed or deleted
    pub archived: bool,     // File is archived
}

impl FileMode {
    pub fn new(mode: u32) -> Self {
        Self {
            owner_read: (mode & 0o400) != 0,
            owner_write: (mode & 0o200) != 0,
            owner_execute: (mode & 0o100) != 0,
            group_read: (mode & 0o040) != 0,
            group_write: (mode & 0o020) != 0,
            group_execute: (mode & 0o010) != 0,
            other_read: (mode & 0o004) != 0,
            other_write: (mode & 0o002) != 0,
            other_execute: (mode & 0o001) != 0,
            nodump: false,
            immutable: false,
            append_only: false,
            opaque: false,
            nounlink: false,
            archived: false,
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut mode = 0u32;
        if self.owner_read {
            mode |= 0o400;
        }
        if self.owner_write {
            mode |= 0o200;
        }
        if self.owner_execute {
            mode |= 0o100;
        }
        if self.group_read {
            mode |= 0o040;
        }
        if self.group_write {
            mode |= 0o020;
        }
        if self.group_execute {
            mode |= 0o010;
        }
        if self.other_read {
            mode |= 0o004;
        }
        if self.other_write {
            mode |= 0o002;
        }
        if self.other_execute {
            mode |= 0o001;
        }
        mode
    }
}

/// Inode - represents a file or directory on disk
#[derive(Debug, Clone)]
pub struct Inode {
    pub inode_number: u64,
    pub file_type: FileType,
    pub mode: FileMode,
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
    pub data_blocks: Vec<u64>,
    pub entries: HashMap<String, u64>, // Directory entries
}

impl Inode {
    pub fn new(inode_number: u64, file_type: FileType, mode: u32) -> Self {
        Self {
            inode_number,
            file_type,
            mode: FileMode::new(mode),
            permissions: FilePermissions::from_mode(mode as u16),
            size: 0,
            owner: 0,
            group: 0,
            created: 0,
            modified: 0,
            capabilities: CapabilityToken::new(),
            hard_links_count: 1,
            link_count: 1,
            symlink_target: None,
            xattrs: HashMap::new(),
            data: Vec::new(),
            data_blocks: Vec::new(),
            entries: HashMap::new(),
        }
    }
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub inode_number: u64,
    pub file_type: FileType,
}

/// File Descriptor for open files
#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: u64,
    pub inode_id: u64,
    pub inode_number: u64,
    pub offset: u64,
    pub position: u64,
    pub flags: u32,
    pub mode: u32,
}

pub type FileHandle = FileDescriptor;

/// VFS Error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    InvalidArgument,
    IsDirectory,
    NotDirectory,
    FileExists,
    OutOfSpace,
    IoError,
    BadFileDescriptor,
    NoMemory,
    TooManyOpenFiles,
    NameTooLong,
}

impl fmt::Display for VfsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotFound => write!(f, "No such file or directory"),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::InvalidArgument => write!(f, "Invalid argument"),
            Self::IsDirectory => write!(f, "Is a directory"),
            Self::NotDirectory => write!(f, "Not a directory"),
            Self::FileExists => write!(f, "File exists"),
            Self::OutOfSpace => write!(f, "No space left on device"),
            Self::IoError => write!(f, "Input/output error"),
            Self::BadFileDescriptor => write!(f, "Bad file descriptor"),
            Self::NoMemory => write!(f, "Cannot allocate memory"),
            Self::TooManyOpenFiles => write!(f, "Too many open files"),
            Self::NameTooLong => write!(f, "File name too long"),
        }
    }
}

/// Filesystem trait - implemented by ext4, NTFS, FAT32, etc.
pub trait FileSystem: Send + Sync {
    /// Initialize filesystem
    fn init(&mut self) -> Result<(), VfsError>;

    /// Read inode
    fn read_inode(&self, inode_number: u64) -> Result<Inode, VfsError>;

    /// Write inode
    fn write_inode(&mut self, inode: &Inode) -> Result<(), VfsError>;

    /// Read data from inode at offset
    fn read_data(
        &self,
        inode_number: u64,
        offset: u64,
        buffer: &mut [u8],
    ) -> Result<usize, VfsError>;

    /// Write data to inode at offset
    fn write_data(
        &mut self,
        inode_number: u64,
        offset: u64,
        data: &[u8],
    ) -> Result<usize, VfsError>;

    /// List directory entries
    fn list_dir(&self, inode_number: u64) -> Result<Vec<DirEntry>, VfsError>;

    /// Find inode by path within filesystem
    fn lookup(&self, parent_inode: u64, name: &str) -> Result<u64, VfsError>;

    /// Create new file
    fn create(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;

    /// Create new directory
    fn mkdir(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;

    /// Delete file
    fn unlink(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;

    /// Delete directory
    fn rmdir(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;

    /// Get filesystem name
    fn name(&self) -> &'static str;
}

/// Mount point - associates filesystem with path
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub path: String,
    pub fs_type: String,
}

/// Virtual File System - main VFS layer
pub struct VirtualFileSystem {
    filesystems: Vec<(String, u64)>, // (fs_type, block_device_id)
    mounts: Vec<MountPoint>,
    pub open_files: std::collections::BTreeMap<u64, FileDescriptor>,
    next_fd: u64,
    inode_cache: Vec<(u64, Inode)>,
    pub inodes: std::collections::BTreeMap<u64, Inode>,
    pub file_descriptors: std::collections::BTreeMap<u64, FileDescriptor>,
    pub root_inode: u64,
    next_inode_id: u64,
}

#[derive(Debug, Clone)]
pub struct FileDescriptor {
    pub fd: u64,
    pub inode_id: u64,
    pub offset: u64,
    pub flags: u32,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        let mut vfs = Self {
            filesystems: Vec::new(),
            mounts: Vec::new(),
            open_files: std::collections::BTreeMap::new(),
            next_fd: 3,
            inode_cache: Vec::new(),
            inodes: std::collections::BTreeMap::new(),
            file_descriptors: std::collections::BTreeMap::new(),
            root_inode: 1,
            next_inode_id: 2,
        };
        // Create root inode using actual Inode::new constructor
        let root = Inode::new(1, FileType::Directory, 0o755);
        vfs.inodes.insert(1, root);
        vfs
    }

    /// Create a node (inode) in the VFS
    pub fn create_node(
        &mut self,
        name: &str,
        file_type: FileType,
        mode: u32,
        owner: u64,
        parent_id: u64,
    ) -> Result<u64, FsError> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        let mut inode = Inode::new(id, file_type, mode);
        inode.owner = owner;
        self.inodes.insert(id, inode);
        if let Some(parent) = self.inodes.get_mut(&parent_id) {
            parent.entries.insert(name.to_string(), id);
        }
        Ok(id)
    }

    pub fn create_simple_node(&mut self, _name: String, file_type: FileType) -> u64 {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        let inode = Inode::new(id, file_type, 0o644);
        self.inodes.insert(id, inode);
        id
    }
    pub fn create_file(&mut self, file_type: FileType, owner: u64) -> Result<u64, FsError> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        let mut inode = Inode::new(id, file_type, 0o644);
        inode.owner = owner;
        self.inodes.insert(id, inode);
        Ok(id)
    }

    pub fn create_symlink(&mut self, target: &str, owner: u64) -> Result<u64, FsError> {
        let id = self.next_inode_id;
        self.next_inode_id += 1;
        let mut inode = Inode::new(id, FileType::SymbolicLink, 0o777);
        inode.owner = owner;
        inode.symlink_target = Some(target.to_string());
        self.inodes.insert(id, inode);
        Ok(id)
    }

    pub fn set_xattr(&mut self, inode_id: u64, key: &str, value: &[u8]) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.xattrs.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    pub fn get_xattr(&self, inode_id: u64, key: &str) -> Option<Vec<u8>> {
        let inode = self.inodes.get(&inode_id)?;
        inode.xattrs.get(key).cloned()
    }

    pub fn get_inode(&self, id: u64) -> Option<&Inode> {
        self.inodes.get(&id)
    }

    pub fn open_file(&mut self, inode_id: u64, flags: u32) -> Result<u64, FsError> {
        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }
        let fd = self.next_fd;
        self.next_fd += 1;
        let handle = FileDescriptor {
            fd,
            inode_id,
            inode_number: inode_id,
            offset: 0,
            position: 0,
            flags,
            mode: 0o644,
        };
        self.open_files.insert(fd, handle.clone());
        self.file_descriptors.insert(fd, handle);
        Ok(fd)
    }


    /// Creates a hard link pointing directly to the same underlying file Inode
    pub fn create_hard_link(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.link_count += 1;
        inode.hard_links_count = inode.link_count;
        Ok(())
    }


    /// Mount filesystem at path
    pub fn mount(&mut self, path: String, fs_type: String) -> Result<(), VfsError> {
        // Verify filesystem is registered
        if !self.filesystems.iter().any(|(ft, _)| ft == &fs_type) {
            return Err(VfsError::NotFound);
        }

        // Check if path is already mounted
        if self.mounts.iter().any(|m| m.path == path) {
            return Err(VfsError::FileExists);
        }

        self.mounts.push(MountPoint { path, fs_type });
        Ok(())
    }

    /// Unmount filesystem
    pub fn unmount(&mut self, path: &str) -> Result<(), VfsError> {
        if let Some(pos) = self.mounts.iter().position(|m| m.path == path) {
            self.mounts.remove(pos);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    /// Open file - returns file descriptor
        pub fn open(&mut self, path: &str, flags: u32, mode: u32) -> Result<i32, VfsError> {
        if path.len() > 4096 {
            return Err(VfsError::NameTooLong);
        }

        if self.open_files.len() >= 1024 {
            return Err(VfsError::TooManyOpenFiles);
        }

        let fd = self.next_fd;
        self.next_fd += 1;

        let handle = FileDescriptor {
            fd,
            inode_id: 0,
            inode_number: 0,
            offset: 0,
            position: 0,
            flags,
            mode,
        };

        self.open_files.insert(fd, handle.clone());
        self.file_descriptors.insert(fd, handle);
        Ok(fd as i32)
    }

    pub fn close(&mut self, fd: i32) -> Result<(), VfsError> { self.close_file(fd as u64).map_err(|_| VfsError::NotFound) }
    pub fn close_file(&mut self, fd: u64) -> Result<(), FsError> {
        if let Some(pos) = self.open_files.iter().position(|fh| fh.fd == (fd as i32)) {
            self.open_files.remove(pos);
            Ok(())
        } else {
            Err(FsError::InvalidFd)
        }
    }

    pub fn close(&mut self, fd: i32) -> Result<(), FsError> {
        self.close_file(fd as u64)
    }

    pub fn read_file(&mut self, fd: u64, buffer: &mut [u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .open_files
            .iter_mut()
            .find(|fh| fh.fd == (fd as i32))
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get(&file_descriptor.inode_number)
            .ok_or(FsError::NotFound)?;

        // Check read permission
        if !inode.mode.owner_read {
            return Err(FsError::PermissionDenied);
        }

        // Check file offset and file size
        if file_descriptor.offset >= inode.size {
            return Ok(0);
        }

        let remaining = (inode.size - file_descriptor.position) as usize;
        let bytes_to_read = buffer.len().min(remaining);

        let start = file_descriptor.position as usize;
        let end = start + bytes_to_read;
        if end <= inode.data.len() {
            buffer[..bytes_to_read].copy_from_slice(&inode.data[start..end]);
        }

        file_descriptor.position += bytes_to_read as u64;
        Ok(bytes_to_read)
    }

    pub fn write_file(&mut self, fd: u64, buffer: &[u8]) -> Result<usize, FsError> {
        let file_descriptor = self
            .open_files
            .iter_mut()
            .find(|fh| fh.fd == (fd as i32))
            .ok_or(FsError::InvalidFd)?;

        let inode = self
            .inodes
            .get_mut(&file_descriptor.inode_number)
            .ok_or(FsError::NotFound)?;

        // Check write permission
        if !inode.mode.owner_write {
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

    /// Read from file descriptor
    pub fn read(&mut self, fd: i32, buffer: &mut [u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.get_mut(&(fd as u64)) {
            let bytes_read = buffer.len().min(512);
            handle.position += bytes_read as u64;
            handle.offset = handle.position;
            Ok(bytes_read)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Write to file descriptor
    pub fn write(&mut self, fd: i32, data: &[u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.get_mut(&(fd as u64)) {
            let bytes_written = data.len();
            handle.position += bytes_written as u64;
            handle.offset = handle.position;
            Ok(bytes_written)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
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
                if (flags & O_CREAT) == 0 {
                    return Err(FsError::NotFound);
                }
                self.create_file(FileType::Regular, owner)?
            }
        };

        if (flags & O_TRUNC) != 0 {
            if let Some(inode) = self.inodes.get_mut(&inode_id) {
                inode.data.clear();
                inode.size = 0;
            }
        }

        let fd = self.next_fd;
        self.next_fd += 1;
        let handle = FileDescriptor {
            fd,
            inode_id,
            inode_number: inode_id,
            offset: 0,
            position: 0,
            flags,
            mode: 0o644,
        };
        self.open_files.insert(fd, handle.clone());
        self.file_descriptors.insert(fd, handle);

        Ok(fd)
    }

    /// Reposition read/write file offset
    pub fn lseek(&mut self, fd: u64, offset: i64, whence: u32) -> Result<u64, VfsError> {
        if let Some(handle) = self.open_files.get_mut(&fd) {
            match whence {
                0 => {
                    // SEEK_SET
                    if offset < 0 {
                        return Err(VfsError::InvalidArgument);
                    }
                    handle.position = offset as u64;
                }
                1 => {
                    // SEEK_CUR
                    if offset < 0 && (offset.abs() as u64) > handle.position {
                        return Err(VfsError::InvalidArgument);
                    }
                    handle.position = ((handle.position as i64) + offset) as u64;
                }
                2 => {
                    // SEEK_END
                    if let Some(inode) = self.inodes.get(&handle.inode_id) {
                        let new_pos = inode.size as i64 + offset;
                        if new_pos < 0 {
                            return Err(VfsError::InvalidArgument);
                        }
                        handle.position = new_pos as u64;
                    } else {
                        return Err(VfsError::InvalidArgument);
                    }
                }
                _ => return Err(VfsError::InvalidArgument),
            }
            Ok(handle.position)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Get file statistics
    pub fn stat(&self, _path: &str) -> Result<Inode, VfsError> {
        // Stub implementation - would query actual filesystem
        Ok(Inode::new(0, FileType::Regular, 0o644))
    }

    /// List directory contents
    pub fn readdir(&self, _path: &str) -> Result<Vec<DirEntry>, VfsError> {
        // Stub implementation - would query actual filesystem
        Ok(Vec::new())
    }

    /// Get number of open files
    pub fn open_file_count(&self) -> usize {
        self.open_files.len()
    }

    /// Get mount points
    pub fn get_mounts(&self) -> &[MountPoint] {
        &self.mounts
    }
}

impl Default for VirtualFileSystem {
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
        let vfs = VirtualFileSystem::new();
        assert_eq!(vfs.open_file_count(), 0);
        assert_eq!(vfs.next_fd, 3);
    }

    #[test]
    fn test_file_open_close() {
        let mut vfs = VirtualFileSystem::new();
        let fd = vfs.open("/test.txt", 0, 0o644).unwrap();
        assert!(fd >= 3);
        assert_eq!(vfs.open_file_count(), 1);

        vfs.close(fd).unwrap();
        assert_eq!(vfs.open_file_count(), 0);
    }

    #[test]
    fn test_bad_fd() {
        let mut vfs = VirtualFileSystem::new();
        let result = vfs.close(999);
        assert_eq!(result, Err(VfsError::BadFileDescriptor));
    }

    #[test]
    fn test_file_mode() {
        let mode = FileMode::new(0o755);
        assert!(mode.owner_read);
        assert!(mode.owner_write);
        assert!(mode.owner_execute);
        assert!(mode.group_read);
        assert!(mode.group_execute);
        assert!(mode.other_read);
        assert!(mode.other_execute);
    }

    #[test]
    fn test_seek_operations() {
        let mut vfs = VirtualFileSystem::new();
        let fd = vfs.open("/test.txt", 0, 0o644).unwrap();

        // SEEK_SET
        let pos = vfs.seek(fd, 100, 0).unwrap();
        assert_eq!(pos, 100);

        // SEEK_CUR
        let pos = vfs.seek(fd, 50, 1).unwrap();
        assert_eq!(pos, 150);

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

        // Re-open file to reset offset to 0 for reading
        let read_fd = vfs.open_file(inode_inode_number: id, 0).unwrap();

        // Read should fail with bad_token and write_token, but succeed with read_token or all_token
        assert_eq!(
            vfs.read_file_gated(read_fd, &mut buf, &bad_token),
            Err(FsError::PermissionDenied)
        );
        assert_eq!(
            vfs.read_file_gated(read_fd, &mut buf, &write_token),
            Err(FsError::PermissionDenied)
        );
        assert_eq!(vfs.read_file_gated(read_fd, &mut buf, &read_token), Ok(5));
    }

    #[test]
    fn test_linux_hardlinks_symlinks_and_xattrs() {
        let mut vfs = VirtualFilesystem::new();

        // 1. Create a regular file with extended attribute (user.mime_type = "text/plain")
        let inode_id = vfs.create_file(FileType::Regular, 1000).unwrap();
        vfs.set_xattr(inode_inode_number: id, "user.mime_type", b"text/plain")
            .unwrap();
        assert_eq!(
            vfs.get_xattr(inode_inode_number: id, "user.mime_type").unwrap(),
            b"text/plain"
        );

        // 2. Create a symlink pointing to our file
        let symlink_id = vfs.create_symlink("/home/tc/file.txt", 1000).unwrap();
        assert_eq!(
            vfs.get_inode(symlink_id).unwrap().file_type,
            FileType::Symlink
        );
        assert_eq!(
            vfs.get_inode(symlink_id)
                .unwrap()
                .symlink_target
                .as_ref()
                .unwrap(),
            "/home/tc/file.txt"
        );

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
        assert_eq!(
            vfs.canonicalize_path("/var/log", "syslog"),
            "/var/log/syslog"
        );
        assert_eq!(
            vfs.canonicalize_path("/var/log", "../mail/../log/./syslog"),
            "/var/log/syslog"
        );
        assert_eq!(
            vfs.canonicalize_path("/home/user", "/usr/bin/../../etc/passwd"),
            "/etc/passwd"
        );
        assert_eq!(vfs.canonicalize_path("/home/user", ".."), "/home");
    }
}

pub type VirtualFilesystem = VirtualFileSystem;
