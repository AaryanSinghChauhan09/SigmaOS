// SigmaOS Virtual Filesystem (VFS)
// Capability-based, standard Linux/BSD conforming filesystem with security, hard links, and path traversal

#[cfg(not(feature = "standalone_test"))]
use crate::security::{CapabilityToken, Permission};

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Permission {
    FileRead,
    FileWrite,
}

#[cfg(feature = "standalone_test")]
#[derive(Debug, Clone, Copy, Default)]
pub struct CapabilityToken {
    pub read_allowed: bool,
    pub write_allowed: bool,
}

#[cfg(feature = "standalone_test")]
impl CapabilityToken {
    pub fn new() -> Self {
        Self {
            read_allowed: false,
            write_allowed: false,
        }
    }
    pub fn allow_read(mut self, _path: &str) -> Self {
        self.read_allowed = true;
        self
    }
    pub fn allow_write(mut self, _path: &str) -> Self {
        self.write_allowed = true;
        self
    }
    pub fn has_permission(&self, perm: Permission) -> bool {
        match perm {
            Permission::FileRead => self.read_allowed,
            Permission::FileWrite => self.write_allowed,
        }
    }
}

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
    // Conforming Linux/BSD additions
    pub hard_links_count: u32,
    pub data: Vec<u8>,                      // File storage data
    pub entries: HashMap<String, u64>,      // Directory entries
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
            data: Vec::new(),
            entries: HashMap::new(),
        }
    }
}

// ================= Linux-style procfs & sysfs Dynamic Stats Bridge =================

pub struct ProcSysFsBridge {
    pub uptime_seconds: u64,
    pub active_processes_count: usize,
}

impl ProcSysFsBridge {
    pub fn new() -> Self {
        Self {
            uptime_seconds: 120,
            active_processes_count: 5,
        }
    }

    pub fn read_dynamic_proc_file(&self, filename: &str) -> Result<String, &'static str> {
        match filename {
            "uptime" => Ok(format!("uptime: {} seconds", self.uptime_seconds)),
            "loadavg" => Ok(format!("loadavg: 0.15 0.24 0.35 active_tasks: {}", self.active_processes_count)),
            _ => Err("procfs: File not found"),
        }
    }
}

// ================= OpenBSD chroot path containment sandbox =================

pub struct ChrootSandbox {
    pub chroot_root_path: String,
}

impl ChrootSandbox {
    pub fn new(root_path: &str) -> Self {
        Self {
            chroot_root_path: root_path.to_string(),
        }
    }

    /// Verifies path starts with jail prefix (chroot escape protection)
    pub fn sandbox_resolve_path(&self, requested_path: &str) -> Result<String, &'static str> {
        let clean_path = requested_path.to_string();
        if clean_path.contains("../") {
            return Err("chroot jail breach blocked: Directory traversal escape attempt detected!");
        }
        if !clean_path.starts_with(&self.chroot_root_path) {
            Ok(format!("{}{}", self.chroot_root_path, clean_path))
        } else {
            Ok(clean_path)
        }
    }
}

// ================= macOS FSEvents directory change notifications =================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsEventAction {
    Created,
    Modified,
    Deleted,
}

#[derive(Clone)]
pub struct FsChangeLog {
    pub path: String,
    pub action: FsEventAction,
}

pub struct FsEventsAuditor {
    pub change_logs: Vec<FsChangeLog>,
}

impl FsEventsAuditor {
    pub fn new() -> Self {
        Self { change_logs: Vec::new() }
    }

    pub fn record_filesystem_event(&mut self, path: &str, action: FsEventAction) {
        self.change_logs.push(FsChangeLog {
            path: path.to_string(),
            action,
        });
    }

    pub fn filter_events_by_prefix(&self, prefix: &str) -> Vec<FsChangeLog> {
        let mut matched = Vec::new();
        for log in &self.change_logs {
            if log.path.starts_with(prefix) {
                matched.push(log.clone());
            }
        }
        matched
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
        inode.hard_links_count += 1;
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

    pub fn read_file_gated(&mut self, fd: u64, buffer: &mut [u8], gate: &CapabilityToken) -> Result<usize, FsError> {
        if !gate.has_permission(Permission::FileRead) {
            return Err(FsError::PermissionDenied);
        }
        self.read_file(fd, buffer)
    }

    pub fn write_file_gated(&mut self, fd: u64, buffer: &[u8], gate: &CapabilityToken) -> Result<usize, FsError> {
        if !gate.has_permission(Permission::FileWrite) {
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
            let inode = self.inodes.get(&current_inode_id).ok_or(FsError::NotFound)?;
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
                    let parent = self.inodes.get_mut(&parent_inode_id).ok_or(FsError::NotFound)?;
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
        let parent_mut = self.inodes.get_mut(&parent_inode_id).ok_or(FsError::NotFound)?;
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

        let parent = self.inodes.get_mut(&parent_inode_id).ok_or(FsError::NotFound)?;
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

        let data = b"secured content";
        let empty_gate = CapabilityToken::new();

        // Write without permission -> fail
        assert!(vfs.write_file_gated(fd, data, &empty_gate).is_err());

        // Write with permission -> success
        let write_gate = CapabilityToken::new().allow_write("/home");
        let written = vfs.write_file_gated(fd, data, &write_gate).unwrap();
        assert_eq!(written, data.len());

        // Read should fail with bad_token and write_token, but succeed with read_token or all_token
        assert_eq!(vfs.read_file_gated(fd, &mut buf, &bad_token), Err(FsError::PermissionDenied));
        assert_eq!(vfs.read_file_gated(fd, &mut buf, &write_token), Err(FsError::PermissionDenied));

        // Reset file offset to beginning before reading
        vfs.file_descriptors.get_mut(&fd).unwrap().offset = 0;
        assert_eq!(vfs.read_file_gated(fd, &mut buf, &read_token), Ok(5));
    }

        // Read with permission -> success
        let read_gate = CapabilityToken::new().allow_read("/var/www");
        let read_bytes = vfs.read_file_gated(fd, &mut buf, &read_gate).unwrap();
        assert_eq!(read_bytes, data.len());
    }

    #[test]
    fn test_vfs_proc_and_sysfs_bridge() {
        let bridge = ProcSysFsBridge::new();
        let uptime = bridge.read_dynamic_proc_file("uptime").unwrap();
        assert_eq!(uptime, "uptime: 120 seconds");

        let loadavg = bridge.read_dynamic_proc_file("loadavg").unwrap();
        assert!(loadavg.contains("active_tasks: 5"));

        assert!(bridge.read_dynamic_proc_file("nonexistent").is_err());
    }

    #[test]
    fn test_vfs_chroot_sandbox_confinement() {
        let sandbox = ChrootSandbox::new("/jail/app1");

        let path1 = sandbox.sandbox_resolve_path("/var/log/syslog").unwrap();
        assert_eq!(path1, "/jail/app1/var/log/syslog");

        // Allowed relative transition
        let path2 = sandbox.sandbox_resolve_path("/jail/app1/tmp").unwrap();
        assert_eq!(path2, "/jail/app1/tmp");

        // Malicious chroot escape block
        assert!(sandbox.sandbox_resolve_path("/jail/app1/../../etc/passwd").is_err());
    }

    #[test]
    fn test_vfs_fsevents_auditor() {
        let mut auditor = FsEventsAuditor::new();
        auditor.record_filesystem_event("/home/user/document.txt", FsEventAction::Created);
        auditor.record_filesystem_event("/home/user/document.txt", FsEventAction::Modified);
        auditor.record_filesystem_event("/etc/resolv.conf", FsEventAction::Modified);

        let user_events = auditor.filter_events_by_prefix("/home/user");
        assert_eq!(user_events.len(), 2);
        assert_eq!(user_events[0].action, FsEventAction::Created);
    }
}
