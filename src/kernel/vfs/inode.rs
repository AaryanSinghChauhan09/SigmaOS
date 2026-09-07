#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

use std::string::String;
use std::vec::Vec;
use core::sync::atomic::{AtomicU32, AtomicUsize, Ordering};

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
    pub i_fop: Option<&'static dyn FileOperations>,
    pub i_op: Option<&'static dyn InodeOperations>,
    pub i_private: Option<usize>,
    pub i_mapping: Option<usize>,
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
            i_fop: None,
            i_op: None,
            i_private: None,
            i_mapping: None,
        }
    }

    pub fn is_dir(&self) -> bool {
        self.i_type == InodeType::Directory
    }

    pub fn is_reg(&self) -> bool {
        self.i_type == InodeType::Regular
    }

    pub fn is_lnk(&self) -> bool {
        self.i_type == InodeType::SymbolicLink
    }

    pub fn is_chr(&self) -> bool {
        self.i_type == InodeType::CharacterDevice
    }

    pub fn is_blk(&self) -> bool {
        self.i_type == InodeType::BlockDevice
    }

    pub fn permission(&self, perm: FilePermission) -> bool {
        let mask = perm as u32;
        (self.i_mode & mask) != 0
    }
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

#[derive(Debug, Clone)]
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
