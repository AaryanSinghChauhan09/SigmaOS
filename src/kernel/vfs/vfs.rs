#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use alloc::format;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFlags {
    Read,
    Write,
    ReadWrite,
    Append,
    Create,
    Truncate,
    Exclusive,
    NonBlock,
    Direct,
    Sync,
    DSync,
    NoFollow,
    Directory,
    NoAtime,
    LargeFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilePermission {
    Read = 4,
    Write = 2,
    Execute = 1,
}

pub struct Inode {
    pub i_ino: u64,
    pub i_mode: u32,
    pub i_uid: u32,
    pub i_gid: u32,
    pub i_size: u64,
    pub i_atime: u64,
    pub i_mtime: u64,
    pub i_ctime: u64,
    pub i_nlink: u32,
    pub i_flags: u32,
    pub i_type: InodeType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InodeType {
    Regular,
    Directory,
    SymbolicLink,
    BlockDevice,
    CharacterDevice,
    FIFO,
    Socket,
}

impl Inode {
    pub fn new(ino: u64, inode_type: InodeType) -> Self {
        Inode {
            i_ino: ino,
            i_mode: 0,
            i_uid: 0,
            i_gid: 0,
            i_size: 0,
            i_atime: 0,
            i_mtime: 0,
            i_ctime: 0,
            i_nlink: 1,
            i_flags: 0,
            i_type: inode_type,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.i_type == InodeType::Directory
    }

    pub fn is_reg(&self) -> bool {
        self.i_type == InodeType::Regular
    }

    pub fn permission(&self, perm: FilePermission) -> bool {
        let mask = perm as u32;
        (self.i_mode & mask) != 0
    }
}

pub struct InodeAttr {
    pub ia_mode: u32,
    pub ia_uid: u32,
    pub ia_gid: u32,
    pub ia_size: u64,
    pub ia_atime: u64,
    pub ia_mtime: u64,
    pub ia_ctime: u64,
    pub ia_birthtime: u64,
    pub ia_nlink: u32,
    pub ia_blocks: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsError {
    InitFailed,
    MountFailed,
    OpenFailed,
    ReadFailed,
    WriteFailed,
    CapabilityDenied,
    NotFound,
    AlreadyExists,
    NotDir,
    IsDir,
    NotEmpty,
    PermissionDenied,
    InvalidArgument,
    IoError,
    OutOfMemory,
    DiskQuota,
    FileTooLarge,
    NoSpace,
    ReadOnly,
    Busy,
    Deadlock,
    Stale,
}

pub trait InodeOperations: Send + Sync {
    fn lookup(&self, dir: &Inode, name: &str) -> Result<Option<u64>, FsError>;
    fn create(&self, dir: &Inode, name: &str, mode: u32) -> Result<u64, FsError>;
    fn link(&self, dir: &Inode, name: &str, inode: u64) -> Result<(), FsError>;
    fn unlink(&self, dir: &Inode, name: &str) -> Result<(), FsError>;
    fn mkdir(&self, dir: &Inode, name: &str, mode: u32) -> Result<(), FsError>;
    fn rmdir(&self, dir: &Inode, name: &str) -> Result<(), FsError>;
    fn rename(
        &self,
        old_dir: &Inode,
        old_name: &str,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), FsError>;
    fn symlink(&self, dir: &Inode, name: &str, target: &str) -> Result<(), FsError>;
    fn readlink(&self, inode: &Inode) -> Result<String, FsError>;
    fn setattr(&self, inode: &Inode, attr: &InodeAttr) -> Result<(), FsError>;
    fn getattr(&self, inode: &Inode) -> Result<InodeAttr, FsError>;
    fn list(&self, dir: &Inode) -> Result<Vec<String>, FsError>;
    fn truncate(&self, inode: &Inode, size: u64) -> Result<(), FsError>;
}

pub trait FileOperations: Send + Sync {
    fn open(&self, inode: &Inode, flags: u32) -> Result<(), FsError>;
    fn release(&self, inode: &Inode) -> Result<(), FsError>;
    fn read(&self, inode: &Inode, buf: &mut [u8], offset: u64) -> Result<usize, FsError>;
    fn write(&self, inode: &Inode, buf: &[u8], offset: u64) -> Result<usize, FsError>;
    fn llseek(&self, inode: &Inode, offset: i64, whence: u32) -> Result<u64, FsError>;
    fn mmap(
        &self,
        inode: &Inode,
        addr: u64,
        len: usize,
        prot: u32,
        flags: u32,
    ) -> Result<(), FsError>;
    fn fsync(&self, inode: &Inode) -> Result<(), FsError>;
    fn unlocked_ioctl(&self, inode: &Inode, cmd: u32, arg: u64) -> Result<(), FsError>;
    fn compat_ioctl(&self, inode: &Inode, cmd: u32, arg: u64) -> Result<(), FsError>;
    fn poll(&self, inode: &Inode, poll_type: u32) -> Result<u32, FsError>;
}

pub struct Statfs {
    pub f_type: u64,
    pub f_bsize: u64,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_fsid: u64,
    pub f_namelen: u64,
    pub f_frsize: u64,
}

pub struct VfsMount {
    pub mnt_devname: String,
    pub mnt_dir: String,
    pub mnt_sb: usize,
    pub mnt_mountpoint: usize,
    pub mnt_parent: Option<usize>,
    pub mnt_flags: u32,
    pub mnt_root: usize,
}

impl VfsMount {
    pub fn new(devname: &str, mount_point: &str) -> Self {
        VfsMount {
            mnt_devname: devname.to_string(),
            mnt_dir: mount_point.to_string(),
            mnt_sb: 0,
            mnt_mountpoint: 0,
            mnt_parent: None,
            mnt_flags: 0,
            mnt_root: 0,
        }
    }
}

pub trait Filesystem: core::any::Any + Send + Sync {
    fn init(&mut self) -> Result<(), FsError>;
    fn mount(&mut self, device: &str, mount_point: &str) -> Result<VfsMount, FsError>;
    fn unmount(&mut self, mount: &mut VfsMount) -> Result<(), FsError>;
    fn fill_super(
        &mut self,
        sb: &mut SuperBlock,
        data: Option<&str>,
        silent: bool,
    ) -> Result<(), FsError>;
    fn sync_fs(&self, wait: bool) -> Result<(), FsError>;
    fn freeze_fs(&self) -> Result<(), FsError>;
    fn thaw_fs(&self) -> Result<(), FsError>;
    fn statfs(&self, mount: &VfsMount) -> Result<Statfs, FsError>;
    fn metadata(&self) -> &FilesystemMetadata;
}

pub trait SuperBlockOperations: Send + Sync {
    fn statfs(&self) -> Result<Statfs, FsError>;
    fn sync_fs(&self, wait: bool) -> Result<(), FsError>;
    fn freeze_fs(&self) -> Result<(), FsError>;
    fn thaw_fs(&self) -> Result<(), FsError>;
    fn remount_fs(&self, flags: u32, data: Option<&str>) -> Result<(), FsError>;
    fn umount(&self) -> Result<(), FsError>;
    fn show_options(&self) -> Result<String, FsError>;
    fn drop_inode(&self, inode: &Inode);
}

pub trait ExportOperations: Send + Sync {
    fn encode_fh(&self, inode: &Inode, max_len: usize) -> Result<Vec<u8>, FsError>;
    fn decode_fh(&self, sb: &SuperBlock, fh: &[u8]) -> Result<(u64, u32), FsError>;
    fn get_parent(&self, child: &Dentry) -> Result<usize, FsError>;
}

pub trait DentryOperations: Send + Sync {
    fn d_revalidate(&self, dentry: &Dentry, flags: u32) -> Result<bool, FsError>;
    fn d_hash(&self, dentry: &Dentry, name: &str) -> Result<u64, FsError>;
    fn d_compare(&self, dentry: &Dentry, name1: &str, name2: &str) -> bool;
}

pub struct SuperBlock {
    pub s_magic: u64,
    pub s_op: Option<&'static dyn SuperBlockOperations>,
    pub s_root: Option<usize>,
    pub s_mount: Option<usize>,
    pub s_refcount: AtomicU32,
    pub s_flags: u32,
    pub s_iflags: u32,
    pub s_mnts: Vec<VfsMount>,
    pub s_umount: Option<fn(&mut SuperBlock)>,
    pub s_d_op: Option<&'static dyn DentryOperations>,
    pub s_export_op: Option<&'static dyn ExportOperations>,
    pub s_quota_formats: Vec<u32>,
    pub s_maxbytes: u64,
    pub s_time_gran: u64,
    pub s_fs_info: Option<usize>,
}

impl SuperBlock {
    pub fn new(magic: u64) -> Self {
        SuperBlock {
            s_magic: magic,
            s_op: None,
            s_root: None,
            s_mount: None,
            s_refcount: AtomicU32::new(1),
            s_flags: 0,
            s_iflags: 0,
            s_mnts: Vec::new(),
            s_umount: None,
            s_d_op: None,
            s_export_op: None,
            s_quota_formats: Vec::new(),
            s_maxbytes: u64::MAX,
            s_time_gran: 1,
            s_fs_info: None,
        }
    }
}

pub struct Dentry {
    pub d_name: String,
    pub d_parent: Option<usize>,
    pub d_inode: Option<u64>,
    pub d_sb: Option<usize>,
    pub d_flags: Vec<String>,
    pub d_subdirs: Vec<Dentry>,
    pub d_alias: Vec<u64>,
    pub d_time: u64,
    pub d_revalidate: bool,
}

impl Dentry {
    pub fn new(name: &str) -> Self {
        Dentry {
            d_name: name.to_string(),
            d_parent: None,
            d_inode: None,
            d_sb: None,
            d_flags: Vec::new(),
            d_subdirs: Vec::new(),
            d_alias: Vec::new(),
            d_time: 0,
            d_revalidate: false,
        }
    }

    pub fn hash(name: &str) -> u64 {
        let mut hash: u64 = 0;
        for byte in name.bytes() {
            hash = hash.wrapping_mul(31).wrapping_add(byte as u64);
        }
        hash
    }

    pub fn full_path(&self) -> String {
        let mut parts = Vec::new();
        let mut current = self;
        while let Some(parent_ptr) = current.d_parent {
            parts.push(current.d_name.clone());
            let parent_ref = unsafe { &*(parent_ptr as *const Dentry) };
            current = parent_ref;
        }
        parts.push(current.d_name.clone());
        parts.reverse();
        let mut path = String::from("/");
        for (i, part) in parts.iter().enumerate() {
            if i > 0 {
                path.push('/');
            }
            path.push_str(part);
        }
        path
    }
}

pub struct FilesystemMetadata {
    pub name: String,
    pub version: String,
    pub max_files: usize,
    pub max_file_size: u64,
    pub supported_features: Vec<String>,
}

impl FilesystemMetadata {
    pub fn new(name: &str) -> Self {
        FilesystemMetadata {
            name: name.to_string(),
            version: "1.0".to_string(),
            max_files: usize::MAX,
            max_file_size: u64::MAX,
            supported_features: Vec::new(),
        }
    }
}

// ==========================================
// 5. Sovereign Advanced Mount Manager (Linux-inspired)
// ==========================================

/// Standard Linux-inspired mount flags
pub const MS_RDONLY: u32 = 1;       // Mount read-only
pub const MS_NOSUID: u32 = 2;       // Ignore suid and sgid bits
pub const MS_NODEV: u32 = 4;        // Disallow access to device special files
pub const MS_NOEXEC: u32 = 8;       // Disallow program execution
pub const MS_REMOUNT: u32 = 32;     // Alter flags of a mounted FS
pub const MS_BIND: u32 = 4096;      // Create a bind mount
pub const MS_MOVE: u32 = 8192;      // Move a subtree

/// Standard Linux-inspired mount propagation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountPropagation {
    Shared,
    Private,
    Slave,
}

/// Represents an active mount entry in the mount table (fstab-inspired)
#[derive(Debug, Clone)]
pub struct ActiveMountEntry {
    pub spec_device_uuid: String,
    pub mount_point: String,
    pub fstype: String,
    pub flags: u32,
    pub propagation: MountPropagation,
    pub pass_no: u32,
}

/// Highly robust Linux-inspired Mount Manager
pub struct SovereignMountManager {
    pub active_mount_table: BTreeMap<String, ActiveMountEntry>, // maps mount point to entry
    pub fstab_config: BTreeMap<String, String>,                 // raw lines of /etc/fstab configuration
    pub automount_triggers: BTreeMap<String, String>,           // maps device UUID to auto-mount target
}

impl SovereignMountManager {
    pub fn new() -> Self {
        Self {
            active_mount_table: BTreeMap::new(),
            fstab_config: BTreeMap::new(),
            automount_triggers: BTreeMap::new(),
        }
    }

    /// Parses a standard Linux /etc/fstab entry line (e.g. "UUID=1234-ABCD /mnt/data ext4 rw,nosuid,nodev 0 2")
    pub fn parse_and_register_fstab_entry(&mut self, line: &str) -> Result<(), &'static str> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 4 {
            return Err("Invalid fstab entry: expected at least 4 standard fields");
        }

        let device_uuid = parts[0].to_string();
        let mount_point = parts[1].to_string();
        let fstype = parts[2].to_string();
        let options = parts[3];

        let mut flags = 0u32;
        for opt in options.split(',') {
            match opt {
                "ro" | "rdonly" => flags |= MS_RDONLY,
                "nosuid" => flags |= MS_NOSUID,
                "nodev" => flags |= MS_NODEV,
                "noexec" => flags |= MS_NOEXEC,
                _ => {}
            }
        }

        let entry = ActiveMountEntry {
            spec_device_uuid: device_uuid.clone(),
            mount_point: mount_point.clone(),
            fstype,
            flags,
            propagation: MountPropagation::Private,
            pass_no: parts.get(5).and_then(|&s| s.parse().ok()).unwrap_or(0),
        };

        self.active_mount_table.insert(mount_point, entry);
        self.fstab_config.insert(device_uuid, line.to_string());
        Ok(())
    }

    /// Implements Bind Mount operations (MS_BIND) and Namespace Propagation settings
    pub fn execute_bind_mount(&mut self, source_dir: &str, target_dir: &str, propagation: MountPropagation) -> Result<(), &'static str> {
        if source_dir.is_empty() || target_dir.is_empty() {
            return Err("Source and target paths cannot be empty");
        }

        let entry = ActiveMountEntry {
            spec_device_uuid: format!("BIND_SRC:{}", source_dir),
            mount_point: target_dir.to_string(),
            fstype: "bind".to_string(),
            flags: MS_BIND,
            propagation,
            pass_no: 0,
        };

        self.active_mount_table.insert(target_dir.to_string(), entry);
        Ok(())
    }

    /// Automount (autofs) lookup on-demand triggers
    pub fn handle_automount_lookup(&mut self, accessed_path: &str) -> Option<String> {
        for (trigger_path, dev_uuid) in &self.automount_triggers {
            if accessed_path.starts_with(trigger_path.as_str()) {
                // Auto-mount triggered! Create active entry
                let entry = ActiveMountEntry {
                    spec_device_uuid: dev_uuid.clone(),
                    mount_point: trigger_path.clone(),
                    fstype: "auto".to_string(),
                    flags: 0,
                    propagation: MountPropagation::Shared,
                    pass_no: 0,
                };
                self.active_mount_table.insert(trigger_path.clone(), entry);
                return Some(format!("Auto-mount triggered successfully for path '{}' mapping to UUID '{}'", accessed_path, dev_uuid));
            }
        }
        None
    }
}

impl Default for SovereignMountManager {
    fn default() -> Self {
        Self::new()
    }
}

pub struct MntOperations {}

impl MntOperations {
    pub fn compare(_mnt1: &VfsMount, _mnt2: &VfsMount) -> bool {
        false
    }
}

#[cfg(test)]
mod mount_tests {
    use super::*;

    #[test]
    fn test_fstab_parsing() {
        let mut manager = SovereignMountManager::new();
        let fstab_line = "UUID=6789-EFGH /mnt/test ext4 rw,nosuid,nodev 0 2";
        assert!(manager.parse_and_register_fstab_entry(fstab_line).is_ok());

        let entry = manager.active_mount_table.get("/mnt/test").unwrap();
        assert_eq!(entry.spec_device_uuid, "UUID=6789-EFGH");
        assert_eq!(entry.fstype, "ext4");
        assert_eq!(entry.pass_no, 2);

        // Flags should contain NOSUID and NODEV
        assert_ne!(entry.flags & MS_NOSUID, 0);
        assert_ne!(entry.flags & MS_NODEV, 0);
        assert_eq!(entry.flags & MS_RDONLY, 0);
    }

    #[test]
    fn test_bind_mount() {
        let mut manager = SovereignMountManager::new();
        assert!(manager.execute_bind_mount("/home/user", "/mnt/home", MountPropagation::Shared).is_ok());

        let entry = manager.active_mount_table.get("/mnt/home").unwrap();
        assert_eq!(entry.spec_device_uuid, "BIND_SRC:/home/user");
        assert_eq!(entry.fstype, "bind");
        assert_eq!(entry.flags, MS_BIND);
        assert_eq!(entry.propagation, MountPropagation::Shared);
    }

    #[test]
    fn test_automount() {
        let mut manager = SovereignMountManager::new();
        manager.automount_triggers.insert("/media/usb".to_string(), "UUID=USB-9999".to_string());

        // Prior to lookup, no active mount entry exists
        assert!(!manager.active_mount_table.contains_key("/media/usb"));

        // Trigger lookup
        let res = manager.handle_automount_lookup("/media/usb/photos/pic.jpg");
        assert!(res.is_some());
        assert!(res.unwrap().contains("UUID=USB-9999"));

        // Active mount entry should now exist
        assert!(manager.active_mount_table.contains_key("/media/usb"));
    }
}
