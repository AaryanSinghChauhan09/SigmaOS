// SPDX-License-Identifier: MIT
// SigmaOS: Virtual File System (VFS) Layer
// Provides unified filesystem abstraction supporting multiple filesystem types
// Integrates with syscall dispatcher for read, write, open, close operations

use core::fmt;
use std::string::String;
use std::vec::Vec;

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
}

/// File mode bits (permissions)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileMode {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
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
        }
    }

    pub fn to_u32(&self) -> u32 {
        let mut flags = 0u32;
        if self.nodump { flags |= 0x0001; }
        if self.immutable { flags |= 0x0002; }
        if self.append_only { flags |= 0x0004; }
        if self.opaque { flags |= 0x0008; }
        if self.nounlink { flags |= 0x0010; }
        if self.archived { flags |= 0x0001_0000; }
        flags
    }
}

/// POSIX Mode Bits constants (Linux & BSD standard permissions)
pub mod mode_bits {
    pub const S_ISUID: u16 = 0o4000; // Set-user-ID on execution
    pub const S_ISGID: u16 = 0o2000; // Set-group-ID on execution
    pub const S_ISVTX: u16 = 0o1000; // Sticky bit (restricted deletion)

    pub const S_IRUSR: u16 = 0o0400; // User read
    pub const S_IWUSR: u16 = 0o0200; // User write
    pub const S_IXUSR: u16 = 0o0100; // User execute

    pub const S_IRGRP: u16 = 0o0040; // Group read
    pub const S_IWGRP: u16 = 0o0020; // Group write
    pub const S_IXGRP: u16 = 0o0010; // Group execute

    pub const S_IROTH: u16 = 0o0004; // Other read
    pub const S_IWOTH: u16 = 0o0002; // Other write
    pub const S_IXOTH: u16 = 0o0001; // Other execute

    pub const S_IRWXU: u16 = 0o0700; // User read, write, execute
    pub const S_IRWXG: u16 = 0o0070; // Group read, write, execute
    pub const S_IRWXO: u16 = 0o0007; // Other read, write, execute
}

/// Comprehensive File Permissions combining Linux POSIX Mode Bits and BSD File Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub read: bool,      // Legacy backward compatibility flag (reflects owner read)
    pub write: bool,     // Legacy backward compatibility flag (reflects owner write)
    pub execute: bool,   // Legacy backward compatibility flag (reflects owner execute)

    pub user_read: bool,
    pub user_write: bool,
    pub user_execute: bool,

    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,

    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,

    pub suid: bool,      // SUID bit (set-user-ID)
    pub sgid: bool,      // SGID bit (set-group-ID)
    pub sticky: bool,    // Sticky bit

    pub owner_mask: u8,
    pub group_mask: u8,
    pub other_mask: u8,

    pub bsd_flags: BsdFileFlags,
}

impl FilePermissions {
    pub fn new(read: bool, write: bool, execute: bool) -> Self {
        let mask = ((read as u8) << 2) | ((write as u8) << 1) | (execute as u8);
        Self {
            read,
            write,
            execute,
            user_read: read,
            user_write: write,
            user_execute: execute,
            group_read: read,
            group_write: false,
            group_execute: execute,
            other_read: read,
            other_write: false,
            other_execute: execute,
            suid: false,
            sgid: false,
            sticky: false,
            owner_mask: mask,
            group_mask: (read as u8) << 2 | (execute as u8),
            other_mask: (read as u8) << 2 | (execute as u8),
            bsd_flags: BsdFileFlags::new(),
        }
    }

    pub fn from_mode_bits(mode: u32) -> Self {
        let suid = (mode & 0o4000) != 0;
        let sgid = (mode & 0o2000) != 0;
        let sticky = (mode & 0o1000) != 0;
        let owner_mask = ((mode >> 6) & 0o7) as u8;
        let group_mask = ((mode >> 3) & 0o7) as u8;
        let other_mask = (mode & 0o7) as u8;

        Self {
            read: (owner_mask & 0o4) != 0,
            write: (owner_mask & 0o2) != 0,
            execute: (owner_mask & 0o1) != 0,
            user_read: (owner_mask & 0o4) != 0,
            user_write: (owner_mask & 0o2) != 0,
            user_execute: (owner_mask & 0o1) != 0,
            group_read: (group_mask & 0o4) != 0,
            group_write: (group_mask & 0o2) != 0,
            group_execute: (group_mask & 0o1) != 0,
            other_read: (other_mask & 0o4) != 0,
            other_write: (other_mask & 0o2) != 0,
            other_execute: (other_mask & 0o1) != 0,
            suid,
            sgid,
            sticky,
            owner_mask,
            group_mask,
            other_mask,
            bsd_flags: BsdFileFlags::new(),
        }
    }

    pub fn to_mode_bits(&self) -> u32 {
        let mut mode = 0u32;
        if self.suid { mode |= 0o4000; }
        if self.sgid { mode |= 0o2000; }
        if self.sticky { mode |= 0o1000; }
        mode |= ((self.owner_mask as u32) & 0o7) << 6;
        mode |= ((self.group_mask as u32) & 0o7) << 3;
        mode |= (self.other_mask as u32) & 0o7;
        mode
    }

    pub fn allows_owner(&self, req_mask: u8) -> bool {
        (self.owner_mask & req_mask) == req_mask
    }

    pub fn allows_group(&self, req_mask: u8) -> bool {
        (self.group_mask & req_mask) == req_mask
    }

    pub fn allows_other(&self, req_mask: u8) -> bool {
        (self.other_mask & req_mask) == req_mask
    }

    pub fn all() -> Self {
        Self::from_mode(0o777)
    }

    pub fn read_only() -> Self {
        Self::from_mode(0o444)
    }

    pub fn from_mode(mode: u16) -> Self {
        let user_r = (mode & mode_bits::S_IRUSR) != 0;
        let user_w = (mode & mode_bits::S_IWUSR) != 0;
        let user_x = (mode & mode_bits::S_IXUSR) != 0;

        Self {
            read: user_r,
            write: user_w,
            execute: user_x,

            user_read: user_r,
            user_write: user_w,
            user_execute: user_x,

            group_read: (mode & mode_bits::S_IRGRP) != 0,
            group_write: (mode & mode_bits::S_IWGRP) != 0,
            group_execute: (mode & mode_bits::S_IXGRP) != 0,

            other_read: (mode & mode_bits::S_IROTH) != 0,
            other_write: (mode & mode_bits::S_IWOTH) != 0,
            other_execute: (mode & mode_bits::S_IXOTH) != 0,

            suid: (mode & mode_bits::S_ISUID) != 0,
            sgid: (mode & mode_bits::S_ISGID) != 0,
            sticky: (mode & mode_bits::S_ISVTX) != 0,

            owner_mask: ((mode >> 6) & 0o7) as u8,
            group_mask: ((mode >> 3) & 0o7) as u8,
            other_mask: (mode & 0o7) as u8,

            bsd_flags: BsdFileFlags::new(),
        }
    }

    pub fn to_posix_mode(&self) -> u32 {
        let mut mode = 0u32;
        if self.user_read {
            mode |= 0o400;
        }
        if self.user_write {
            mode |= 0o200;
        }
        if self.user_execute {
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
    pub fn new(inode_number: u64, file_type: FileType, mode: u32) -> Self {
        Self {
            inode_number,
            file_type,
            mode: FileMode::new(mode),
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

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub inode_number: u64,
    pub file_type: FileType,
}

/// File handle for open files
#[derive(Debug, Clone)]
pub struct FileHandle {
    pub fd: i32,
    pub inode_number: u64,
    pub position: u64,
    pub flags: u32,
    pub mode: u32,
}

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
    open_files: Vec<FileHandle>,
    next_fd: i32,
    inode_cache: Vec<(u64, Inode)>,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        Self {
            filesystems: Vec::new(),
            mounts: Vec::new(),
            open_files: Vec::new(),
            next_fd: 3, // 0, 1, 2 are stdin, stdout, stderr
            inode_cache: Vec::new(),
        }
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

        // For now, create a stub file handle
        let fd = self.next_fd;
        self.next_fd += 1;

        let handle = FileHandle {
            fd,
            inode_number: 0, // Would be populated by actual filesystem
            position: 0,
            flags,
            mode,
        };

        self.open_files.push(handle);
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
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Read from file descriptor
    pub fn read(&mut self, fd: i32, buffer: &mut [u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            // Stub implementation - would read from actual filesystem
            let bytes_read = buffer.len().min(512); // Limit read size
            handle.position += bytes_read as u64;
            Ok(bytes_read)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    /// Write to file descriptor
    pub fn write(&mut self, fd: i32, data: &[u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            // Stub implementation - would write to actual filesystem
            let bytes_written = data.len().min(512); // Limit write size
            handle.position += bytes_written as u64;
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
                self.create_node(filename, FileType::Regular, 0o644, owner, parent_inode_id)?
            }
        };

        if (flags & O_TRUNC) != 0 {
            if let Some(node) = self.inodes.get_mut(&inode_id) {
                node.size = 0;
            }
        }

        let fd = self.next_fd;
        self.next_fd += 1;
        self.open_files.insert(
            fd,
            FileDescriptor {
                inode_id,
                position: 0,
                flags,
            },
        );

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
        let read_fd = vfs.open_file(inode_id, 0).unwrap();

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
        vfs.set_xattr(inode_id, "user.mime_type", b"text/plain")
            .unwrap();
        assert_eq!(
            vfs.get_xattr(inode_id, "user.mime_type").unwrap(),
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
