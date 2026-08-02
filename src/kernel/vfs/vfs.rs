// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
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

pub struct MntOperations {}

impl MntOperations {
    pub fn compare(_mnt1: &VfsMount, _mnt2: &VfsMount) -> bool {
        false
    }
}
