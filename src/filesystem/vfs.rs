// SigmaOS Virtual Filesystem (VFS)
// Capability-based filesystem with security

use crate::security::{CapabilityToken, Permission};
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

/// BSD File Flags (inspired by FreeBSD / OpenBSD chflags(2))
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BsdFileFlags {
    pub nodump: bool,      // UF_NODUMP: Do not dump file
    pub immutable: bool,   // UF_IMMUTABLE / SF_IMMUTABLE: File cannot be modified, deleted, renamed
    pub append_only: bool, // UF_APPEND / SF_APPEND: File can only be written in append mode
    pub nounlink: bool,    // UF_NOUNLINK / SF_NOUNLINK: File cannot be removed/renamed
    pub opaque: bool,      // UF_OPAQUE: Directory is opaque when viewed through union mount
    pub archived: bool,    // SF_ARCHIVED: File is archived
}

impl BsdFileFlags {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_u32(raw: u32) -> Self {
        Self {
            nodump: (raw & 0x0001) != 0,
            immutable: (raw & 0x0002) != 0 || (raw & 0x0002_0000) != 0,
            append_only: (raw & 0x0004) != 0 || (raw & 0x0004_0000) != 0,
            opaque: (raw & 0x0008) != 0,
            nounlink: (raw & 0x0010) != 0 || (raw & 0x0010_0000) != 0,
            archived: (raw & 0x0001_0000) != 0,
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
        let owner_m = ((user_r as u8) << 2) | ((user_w as u8) << 1) | (user_x as u8);

        let group_r = (mode & mode_bits::S_IRGRP) != 0;
        let group_w = (mode & mode_bits::S_IWGRP) != 0;
        let group_x = (mode & mode_bits::S_IXGRP) != 0;
        let group_m = ((group_r as u8) << 2) | ((group_w as u8) << 1) | (group_x as u8);

        let other_r = (mode & mode_bits::S_IROTH) != 0;
        let other_w = (mode & mode_bits::S_IWOTH) != 0;
        let other_x = (mode & mode_bits::S_IXOTH) != 0;
        let other_m = ((other_r as u8) << 2) | ((other_w as u8) << 1) | (other_x as u8);

        Self {
            read: user_r,
            write: user_w,
            execute: user_x,

            user_read: user_r,
            user_write: user_w,
            user_execute: user_x,

            group_read: group_r,
            group_write: group_w,
            group_execute: group_x,

            other_read: other_r,
            other_write: other_w,
            other_execute: other_x,

            suid: (mode & mode_bits::S_ISUID) != 0,
            sgid: (mode & mode_bits::S_ISGID) != 0,
            sticky: (mode & mode_bits::S_ISVTX) != 0,

            owner_mask: owner_m,
            group_mask: group_m,
            other_mask: other_m,

            bsd_flags: BsdFileFlags::new(),
        }
    }

    pub fn to_mode(&self) -> u16 {
        let mut mode = 0u16;

        if self.suid { mode |= mode_bits::S_ISUID; }
        if self.sgid { mode |= mode_bits::S_ISGID; }
        if self.sticky { mode |= mode_bits::S_ISVTX; }

        if self.user_read { mode |= mode_bits::S_IRUSR; }
        if self.user_write { mode |= mode_bits::S_IWUSR; }
        if self.user_execute { mode |= mode_bits::S_IXUSR; }

        if self.group_read { mode |= mode_bits::S_IRGRP; }
        if self.group_write { mode |= mode_bits::S_IWGRP; }
        if self.group_execute { mode |= mode_bits::S_IXGRP; }

        if self.other_read { mode |= mode_bits::S_IROTH; }
        if self.other_write { mode |= mode_bits::S_IWOTH; }
        if self.other_execute { mode |= mode_bits::S_IXOTH; }

        mode
    }

    /// Formats file permissions into standard Unix/BSD string representation (e.g., "-rwxr-xr-x", "-rwsr-xr-x", "drwxrwxrwt")
    pub fn to_symbolic_string(&self, is_dir: bool) -> String {
        let mut s = String::with_capacity(10);
        s.push(if is_dir { 'd' } else { '-' });

        // User
        s.push(if self.user_read { 'r' } else { '-' });
        s.push(if self.user_write { 'w' } else { '-' });
        s.push(match (self.user_execute, self.suid) {
            (true, true) => 's',
            (false, true) => 'S',
            (true, false) => 'x',
            (false, false) => '-',
        });

        // Group
        s.push(if self.group_read { 'r' } else { '-' });
        s.push(if self.group_write { 'w' } else { '-' });
        s.push(match (self.group_execute, self.sgid) {
            (true, true) => 's',
            (false, true) => 'S',
            (true, false) => 'x',
            (false, false) => '-',
        });

        // Other
        s.push(if self.other_read { 'r' } else { '-' });
        s.push(if self.other_write { 'w' } else { '-' });
        s.push(match (self.other_execute, self.sticky) {
            (true, true) => 't',
            (false, true) => 'T',
            (true, false) => 'x',
            (false, false) => '-',
        });

        s
    }

    /// Evaluates Discretionary Access Control (DAC) permission for a subject (UID, GID, supplementary GIDs)
    pub fn evaluate_dac_access(
        &self,
        subject_uid: u64,
        subject_gid: u64,
        supplementary_gids: &[u64],
        owner_uid: u64,
        group_gid: u64,
        req_read: bool,
        req_write: bool,
        req_execute: bool,
    ) -> bool {
        // Root (UID 0) bypasses standard DAC permission checks (except execution if no execute bit is set anywhere)
        if subject_uid == 0 {
            if req_execute && !self.user_execute && !self.group_execute && !self.other_execute {
                return false;
            }
            return true;
        }

        let (allow_r, allow_w, allow_x) = if subject_uid == owner_uid {
            (self.user_read, self.user_write, self.user_execute)
        } else if subject_gid == group_gid || supplementary_gids.contains(&group_gid) {
            (self.group_read, self.group_write, self.group_execute)
        } else {
            (self.other_read, self.other_write, self.other_execute)
        };

        (!req_read || allow_r) && (!req_write || allow_w) && (!req_execute || allow_x)
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
    pub link_count: u64,
    pub capabilities: CapabilityToken,
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
            link_count: 1,
            capabilities: CapabilityToken::new(),
        }
    }

    pub fn with_mode(mut self, mode: u16) -> Self {
        self.permissions = FilePermissions::from_mode(mode);
        self
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

    /// Create a hard link to an existing inode (Linux & BSD FHS standard)
    pub fn create_hard_link(&mut self, inode_id: u64) -> Result<(), FsError> {
        if let Some(inode) = self.inodes.get_mut(&inode_id) {
            if inode.file_type == FileType::Directory {
                return Err(FsError::IsDirectory);
            }
            inode.link_count += 1;
            Ok(())
        } else {
            Err(FsError::NotFound)
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

    pub fn chmod(&mut self, inode_id: u64, mode: u16) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        if inode.permissions.bsd_flags.immutable {
            return Err(FsError::ImmutableFile);
        }
        inode.permissions = FilePermissions::from_mode(mode);
        Ok(())
    }

    pub fn chown(&mut self, inode_id: u64, owner: u64, group: u64) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        if inode.permissions.bsd_flags.immutable {
            return Err(FsError::ImmutableFile);
        }
        inode.owner = owner;
        inode.group = group;
        Ok(())
    }

    pub fn chflags(&mut self, inode_id: u64, flags: u32) -> Result<(), FsError> {
        let inode = self.inodes.get_mut(&inode_id).ok_or(FsError::NotFound)?;
        inode.permissions.bsd_flags = BsdFileFlags::from_u32(flags);
        Ok(())
    }

    pub fn evaluate_access(
        &self,
        inode_id: u64,
        subject_uid: u64,
        subject_gid: u64,
        supplementary_gids: &[u64],
        req_read: bool,
        req_write: bool,
        req_execute: bool,
    ) -> Result<(), FsError> {
        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;
        if inode.permissions.evaluate_dac_access(
            subject_uid,
            subject_gid,
            supplementary_gids,
            inode.owner,
            inode.group,
            req_read,
            req_write,
            req_execute,
        ) {
            Ok(())
        } else {
            Err(FsError::PermissionDenied)
        }
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

        // Check BSD File Flags (immutable files cannot be modified)
        if inode.permissions.bsd_flags.immutable {
            return Err(FsError::ImmutableFile);
        }

        // Check write permission
        if !inode.permissions.write && !inode.permissions.user_write {
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

    pub fn delete_file(&mut self, inode_id: u64) -> Result<(), FsError> {
        if inode_id == self.root_inode {
            return Err(FsError::PermissionDenied);
        }

        let inode = self.inodes.get(&inode_id).ok_or(FsError::NotFound)?;

        // Check BSD File Flags (immutable or nounlink files cannot be deleted)
        if inode.permissions.bsd_flags.immutable || inode.permissions.bsd_flags.nounlink {
            return Err(FsError::ImmutableFile);
        }

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
    ImmutableFile,
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
    fn test_file_permissions_octal_mode_bits() {
        let perms = FilePermissions::from_mode_bits(0o4755);
        assert!(perms.suid);
        assert!(!perms.sgid);
        assert!(!perms.sticky);
        assert!(perms.allows_owner(0o4)); // read
        assert!(perms.allows_owner(0o2)); // write
        assert!(perms.allows_owner(0o1)); // execute
        assert!(perms.allows_group(0o5)); // r-x
        assert!(perms.allows_other(0o5)); // r-x
        assert_eq!(perms.to_mode_bits(), 0o4755);

        let sticky_perms = FilePermissions::from_mode_bits(0o1777);
        assert!(!sticky_perms.suid);
        assert!(!sticky_perms.sgid);
        assert!(sticky_perms.sticky);
        assert_eq!(sticky_perms.to_mode_bits(), 0o1777);
    }

    #[test]
    fn test_zero_sized_read_write_optimization() {
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
    fn test_posix_mode_bits_and_symbolic_formatting() {
        let perms = FilePermissions::from_mode(0o755);
        assert_eq!(perms.to_mode(), 0o755);
        assert_eq!(perms.to_symbolic_string(false), "-rwxr-xr-x");
        assert_eq!(perms.to_symbolic_string(true), "drwxr-xr-x");

        // SUID bit test (0o4755)
        let suid_perms = FilePermissions::from_mode(0o4755);
        assert!(suid_perms.suid);
        assert_eq!(suid_perms.to_symbolic_string(false), "-rwsr-xr-x");

        // SGID bit test (0o2775)
        let sgid_perms = FilePermissions::from_mode(0o2775);
        assert!(sgid_perms.sgid);
        assert_eq!(sgid_perms.to_symbolic_string(true), "drwxrwsr-x");

        // Sticky bit test (0o1777)
        let sticky_perms = FilePermissions::from_mode(0o1777);
        assert!(sticky_perms.sticky);
        assert_eq!(sticky_perms.to_symbolic_string(true), "drwxrwxrwt");
    }

    #[test]
    fn test_dac_evaluation_and_root_bypass() {
        // Mode 0o640: User rw-, Group r--, Other ---
        let perms = FilePermissions::from_mode(0o640);

        // Owner (UID 1000) requests read/write
        assert!(perms.evaluate_dac_access(1000, 1000, &[], 1000, 1000, true, true, false));
        // Owner requests execute -> Denied
        assert!(!perms.evaluate_dac_access(1000, 1000, &[], 1000, 1000, false, false, true));

        // Group member (GID 1000) requests read -> Allowed
        assert!(perms.evaluate_dac_access(1001, 1000, &[], 1000, 1000, true, false, false));
        // Group member requests write -> Denied
        assert!(!perms.evaluate_dac_access(1001, 1000, &[], 1000, 1000, false, true, false));

        // Other user (UID 2000, GID 2000) requests read -> Denied
        assert!(!perms.evaluate_dac_access(2000, 2000, &[], 1000, 1000, true, false, false));

        // Root user (UID 0) requests read/write -> Allowed
        assert!(perms.evaluate_dac_access(0, 0, &[], 1000, 1000, true, true, false));
    }

    #[test]
    fn test_bsd_chflags_immutable_enforcement() {
        let mut vfs = VirtualFilesystem::new();
        let inode_id = vfs.create_file(FileType::Regular, 1000).unwrap();
        let fd = vfs.open_file(inode_id, 0).unwrap();

        // Set BSD immutable flag (UF_IMMUTABLE = 0x0002)
        vfs.chflags(inode_id, 0x0002).unwrap();

        // Attempt write -> FsError::ImmutableFile
        assert_eq!(vfs.write_file(fd, b"immutable test"), Err(FsError::ImmutableFile));

        // Attempt delete -> FsError::ImmutableFile
        assert_eq!(vfs.delete_file(inode_id), Err(FsError::ImmutableFile));

        // Clear immutable flag
        vfs.chflags(inode_id, 0).unwrap();
        assert!(vfs.write_file(fd, b"immutable test").is_ok());
    }
}
