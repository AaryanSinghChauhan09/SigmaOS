// SPDX-License-Identifier: MIT
/// SigmaOS: Virtual File System (VFS) Layer
/// Provides unified filesystem abstraction supporting multiple filesystem types
/// Integrates with syscall dispatcher for read, write, open, close operations

use std::collections::HashMap;
use std::string::{String, ToString};
use std::vec::Vec;
use core::fmt;

pub const O_CREAT: u32 = 0o100;
pub const O_EXCL: u32 = 0o200;
pub const O_TRUNC: u32 = 0o1000;
pub const O_APPEND: u32 = 0o2000;

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    SymbolicLink,
    Symlink, // Alias for SymbolicLink
    CharacterDevice,
    BlockDevice,
    Fifo,
    Socket,
}

/// Capability token and permissions for gated access
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    FileRead,
    FileWrite,
    ProcessExec,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilityToken {
    pub permissions: Vec<Permission>,
}

impl CapabilityToken {
    pub fn new() -> Self {
        Self {
            permissions: vec![
                Permission::FileRead,
                Permission::FileWrite,
                Permission::ProcessExec,
            ],
        }
    }

    pub fn has_permission(&self, perm: Permission) -> bool {
        self.permissions.contains(&perm)
    }
}

impl Default for CapabilityToken {
    fn default() -> Self {
        Self::new()
    }
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
        let mut mode = 0u32;
        if self.owner_read { mode |= 0o400; }
        if self.owner_write { mode |= 0o200; }
        if self.owner_execute { mode |= 0o100; }
        if self.group_read { mode |= 0o040; }
        if self.group_write { mode |= 0o020; }
        if self.group_execute { mode |= 0o010; }
        if self.other_read { mode |= 0o004; }
        if self.other_write { mode |= 0o002; }
        if self.other_execute { mode |= 0o001; }
        mode
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InodePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

/// Inode - represents a file or directory on disk
#[derive(Debug, Clone)]
pub struct Inode {
    pub inode_number: u64,
    pub file_type: FileType,
    pub mode: FileMode,
    pub permissions: InodePermissions,
    pub size: u64,
    pub owner: u64,
    pub group: u64,
    pub created: u64,
    pub modified: u64,
    pub capabilities: CapabilityToken,
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
            permissions: InodePermissions {
                read: true,
                write: true,
                execute: true,
            },
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

#[derive(Debug, Clone)]
pub struct OpenFileDescriptor {
    pub fd: u64,
    pub inode_id: u64,
    pub offset: u64,
    pub flags: u32,
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
    fn init(&mut self) -> Result<(), VfsError>;
    fn read_inode(&self, inode_number: u64) -> Result<Inode, VfsError>;
    fn write_inode(&mut self, inode: &Inode) -> Result<(), VfsError>;
    fn read_data(&self, inode_number: u64, offset: u64, buffer: &mut [u8]) -> Result<usize, VfsError>;
    fn write_data(&mut self, inode_number: u64, offset: u64, data: &[u8]) -> Result<usize, VfsError>;
    fn list_dir(&self, inode_number: u64) -> Result<Vec<DirEntry>, VfsError>;
    fn lookup(&self, parent_inode: u64, name: &str) -> Result<u64, VfsError>;
    fn create(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;
    fn mkdir(&mut self, parent_inode: u64, name: &str, mode: u32) -> Result<u64, VfsError>;
    fn unlink(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;
    fn rmdir(&mut self, parent_inode: u64, name: &str) -> Result<(), VfsError>;
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
    pub filesystems: Vec<(String, u64)>, // (fs_type, block_device_id)
    pub mounts: Vec<MountPoint>,
    pub open_files: Vec<FileHandle>,
    pub next_fd: i32,
    pub inode_cache: Vec<(u64, Inode)>,
    pub inodes: HashMap<u64, Inode>,
    pub file_descriptors: HashMap<u64, OpenFileDescriptor>,
    pub root_inode: u64,
    pub next_inode_id: u64,
    next_descriptor_id: u64,
}

impl VirtualFileSystem {
    pub fn new() -> Self {
        let mut inodes = HashMap::new();
        let root_inode = Inode::new(1, FileType::Directory, 0o755);
        inodes.insert(1, root_inode);

        Self {
            filesystems: Vec::new(),
            mounts: Vec::new(),
            open_files: Vec::new(),
            next_fd: 3,
            inode_cache: Vec::new(),
            inodes,
            file_descriptors: HashMap::new(),
            root_inode: 1,
            next_inode_id: 2,
            next_descriptor_id: 1,
        }
    }

    pub fn get_inode(&self, inode_id: u64) -> Option<&Inode> {
        self.inodes.get(&inode_id)
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
        let mut inode = Inode::new(id, FileType::Symlink, 0o777);
        inode.owner = owner;
        inode.symlink_target = Some(target.to_string());
        self.inodes.insert(id, inode);
        Ok(id)
    }

    pub fn open_file(&mut self, inode_id: u64, flags: u32) -> Result<u64, FsError> {
        if !self.inodes.contains_key(&inode_id) {
            return Err(FsError::NotFound);
        }
        let desc_id = self.next_descriptor_id;
        self.next_descriptor_id += 1;

        self.file_descriptors.insert(
            desc_id,
            OpenFileDescriptor {
                fd: desc_id,
                inode_id,
                offset: 0,
                flags,
            },
        );
        Ok(desc_id)
    }

    pub fn set_xattr(&mut self, inode_id: u64, key: &str, value: &[u8]) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.xattrs.insert(key.to_string(), value.to_vec());
        Ok(())
    }

    pub fn get_xattr(&self, inode_id: u64, key: &str) -> Result<Vec<u8>, FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;
        inode
            .xattrs
            .get(key)
            .cloned()
            .ok_or(FsError::AttributeNotFound)
    }

    pub fn create_hard_link(&mut self, inode_id: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.link_count += 1;
        inode.hard_links_count = inode.link_count;
        Ok(())
    }

    pub fn mount(&mut self, path: String, fs_type: String) -> Result<(), VfsError> {
        if !self.filesystems.iter().any(|(ft, _)| ft == &fs_type) {
            return Err(VfsError::NotFound);
        }
        if self.mounts.iter().any(|m| m.path == path) {
            return Err(VfsError::FileExists);
        }
        self.mounts.push(MountPoint { path, fs_type });
        Ok(())
    }

    pub fn unmount(&mut self, path: &str) -> Result<(), VfsError> {
        if let Some(pos) = self.mounts.iter().position(|m| m.path == path) {
            self.mounts.remove(pos);
            Ok(())
        } else {
            Err(VfsError::NotFound)
        }
    }

    pub fn open(&mut self, path: &str, flags: u32, mode: u32) -> Result<i32, VfsError> {
        if path.len() > 4096 {
            return Err(VfsError::NameTooLong);
        }
        if self.open_files.len() >= 1024 {
            return Err(VfsError::TooManyOpenFiles);
        }
        let fd = self.next_fd;
        self.next_fd += 1;

        let handle = FileHandle {
            fd,
            inode_number: 0,
            position: 0,
            flags,
            mode,
        };

        self.open_files.push(handle);
        Ok(fd)
    }

    pub fn close(&mut self, fd: i32) -> Result<(), VfsError> {
        if let Some(pos) = self.open_files.iter().position(|h| h.fd == fd) {
            self.open_files.remove(pos);
            Ok(())
        } else {
            Err(VfsError::BadFileDescriptor)
        }
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

        if !inode.permissions.read {
            return Err(FsError::PermissionDenied);
        }

        if file_descriptor.offset >= inode.size {
            return Ok(0);
        }

        let remaining = (inode.size - file_descriptor.offset) as usize;
        let bytes_to_read = buffer.len().min(remaining);

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

        if !inode.permissions.write {
            return Err(FsError::PermissionDenied);
        }

        if (file_descriptor.flags & O_APPEND) != 0 {
            file_descriptor.offset = inode.size;
        }

        let new_offset = file_descriptor
            .offset
            .checked_add(buffer.len() as u64)
            .ok_or(FsError::NoSpace)?;

        if new_offset > inode.size {
            inode.data.resize(new_offset as usize, 0);
            inode.size = new_offset;
        }

        let start = file_descriptor.offset as usize;
        let end = start + buffer.len();
        inode.data[start..end].copy_from_slice(buffer);

        file_descriptor.offset = new_offset;
        inode.modified = 1716000000;

        Ok(buffer.len())
    }

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

    pub fn seek(&mut self, fd: i32, offset: i64, whence: i32) -> Result<u64, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            match whence {
                0 => handle.position = offset as u64, // SEEK_SET
                1 => handle.position = (handle.position as i64 + offset) as u64, // SEEK_CUR
                _ => return Err(VfsError::InvalidArgument),
            }
            Ok(handle.position)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    pub fn read(&mut self, fd: i32, buffer: &mut [u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            let bytes_read = buffer.len().min(512);
            handle.position += bytes_read as u64;
            Ok(bytes_read)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

    pub fn write(&mut self, fd: i32, data: &[u8]) -> Result<usize, VfsError> {
        if let Some(handle) = self.open_files.iter_mut().find(|h| h.fd == fd) {
            let bytes_written = data.len().min(512);
            handle.position += bytes_written as u64;
            Ok(bytes_written)
        } else {
            Err(VfsError::BadFileDescriptor)
        }
    }

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

    pub fn open_path(&mut self, path: &str, flags: u32, owner: u64) -> Result<u64, FsError> {
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

        Ok(inode_id)
    }

    pub fn stat(&self, _path: &str) -> Result<Inode, VfsError> {
        Ok(Inode::new(0, FileType::Regular, 0o644))
    }

    pub fn readdir(&self, _path: &str) -> Result<Vec<DirEntry>, VfsError> {
        Ok(Vec::new())
    }

    pub fn open_file_count(&self) -> usize {
        self.open_files.len()
    }

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
    fn test_linux_hardlinks_symlinks_and_xattrs() {
        let mut vfs = VirtualFileSystem::new();

        let inode_id = vfs.create_file(FileType::Regular, 1000).unwrap();
        vfs.set_xattr(inode_id, "user.mime_type", b"text/plain").unwrap();
        assert_eq!(vfs.get_xattr(inode_id, "user.mime_type").unwrap(), b"text/plain");

        let symlink_id = vfs.create_symlink("/home/tc/file.txt", 1000).unwrap();
        assert_eq!(vfs.get_inode(symlink_id).unwrap().file_type, FileType::Symlink);
        assert_eq!(vfs.get_inode(symlink_id).unwrap().symlink_target.as_ref().unwrap(), "/home/tc/file.txt");

        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);
        vfs.create_hard_link(inode_id).unwrap();
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 2);

        vfs.delete_file(inode_id).unwrap();
        assert!(vfs.get_inode(inode_id).is_some());
        assert_eq!(vfs.get_inode(inode_id).unwrap().link_count, 1);

        vfs.delete_file(inode_id).unwrap();
        assert!(vfs.get_inode(inode_id).is_none());
    }

    #[test]
    fn test_canonicalize_path() {
        let vfs = VirtualFileSystem::new();
        assert_eq!(vfs.canonicalize_path("/var/log", "syslog"), "/var/log/syslog");
        assert_eq!(vfs.canonicalize_path("/var/log", "../mail/../log/./syslog"), "/var/log/syslog");
        assert_eq!(vfs.canonicalize_path("/home/user", "/usr/bin/../../etc/passwd"), "/etc/passwd");
        assert_eq!(vfs.canonicalize_path("/home/user", ".."), "/home");
    }
}
