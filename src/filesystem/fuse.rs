// SigmaOS Filesystem in Userspace (FUSE) Kernel Subsystem
// Implements Linux FUSE ABI (kernel /dev/fuse communication channel & dispatcher)

#![allow(dead_code)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

pub const FUSE_KERNEL_VERSION: u32 = 7;
pub const FUSE_KERNEL_MINOR_VERSION: u32 = 32;

/// FUSE Operation Codes compliant with Linux <linux/fuse.h>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FuseOpcode {
    Lookup = 1,
    Forget = 2,
    Getattr = 3,
    Setattr = 4,
    Readlink = 5,
    Symlink = 6,
    Mknod = 8,
    Mkdir = 9,
    Unlink = 10,
    Rmdir = 11,
    Rename = 12,
    Link = 13,
    Open = 14,
    Read = 15,
    Write = 16,
    Statfs = 17,
    Release = 18,
    Fsync = 20,
    Setxattr = 21,
    Getxattr = 22,
    Listxattr = 23,
    Removexattr = 24,
    Flush = 25,
    Init = 26,
    Opendir = 27,
    Readdir = 28,
    Releasedir = 29,
    Fsyncdir = 30,
    Getlk = 31,
    Setlk = 32,
    Setlkw = 33,
    Access = 34,
    Create = 35,
    Interrupt = 36,
    Bmap = 37,
    Destroy = 38,
    Ioctl = 39,
    Poll = 40,
    NotifyReply = 41,
    BatchForget = 42,
    Fallocate = 43,
    Readdirplus = 44,
    Rename2 = 45,
    Lseek = 46,
    CopyFileRange = 47,
    Unknown = 0xFFFF,
}

impl From<u32> for FuseOpcode {
    fn from(val: u32) -> Self {
        match val {
            1 => FuseOpcode::Lookup,
            2 => FuseOpcode::Forget,
            3 => FuseOpcode::Getattr,
            4 => FuseOpcode::Setattr,
            5 => FuseOpcode::Readlink,
            6 => FuseOpcode::Symlink,
            8 => FuseOpcode::Mknod,
            9 => FuseOpcode::Mkdir,
            10 => FuseOpcode::Unlink,
            11 => FuseOpcode::Rmdir,
            12 => FuseOpcode::Rename,
            13 => FuseOpcode::Link,
            14 => FuseOpcode::Open,
            15 => FuseOpcode::Read,
            16 => FuseOpcode::Write,
            17 => FuseOpcode::Statfs,
            18 => FuseOpcode::Release,
            20 => FuseOpcode::Fsync,
            25 => FuseOpcode::Flush,
            26 => FuseOpcode::Init,
            27 => FuseOpcode::Opendir,
            28 => FuseOpcode::Readdir,
            29 => FuseOpcode::Releasedir,
            34 => FuseOpcode::Access,
            35 => FuseOpcode::Create,
            _ => FuseOpcode::Unknown,
        }
    }
}

/// Linux FUSE Inbound Request Header (from kernel VFS to userspace)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuseInHeader {
    pub len: u32,
    pub opcode: FuseOpcode,
    pub unique: u64,
    pub nodeid: u64,
    pub uid: u32,
    pub gid: u32,
    pub pid: u32,
}

/// Linux FUSE Outbound Response Header (from userspace back to kernel)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FuseOutHeader {
    pub len: u32,
    pub error: i32, // 0 for success, negative errno for failure
    pub unique: u64,
}

/// FUSE Session & Channel Manager
pub struct FuseChannel {
    next_unique: u64,
    pending_requests: BTreeMap<u64, FuseInHeader>,
    is_connected: bool,
    max_write: u32,
    mount_point: alloc::string::String,
}

impl FuseChannel {
    pub fn new(mount_point: &str) -> Self {
        Self {
            next_unique: 1,
            pending_requests: BTreeMap::new(),
            is_connected: false,
            max_write: 131072, // 128 KiB
            mount_point: alloc::string::String::from(mount_point),
        }
    }

    /// Emits a new FUSE request to the /dev/fuse queue
    pub fn enqueue_request(
        &mut self,
        opcode: FuseOpcode,
        nodeid: u64,
        uid: u32,
        gid: u32,
        pid: u32,
    ) -> FuseInHeader {
        let unique = self.next_unique;
        self.next_unique += 1;

        let header = FuseInHeader {
            len: 40, // Base header size
            opcode,
            unique,
            nodeid,
            uid,
            gid,
            pid,
        };

        self.pending_requests.insert(unique, header);
        header
    }

    /// Completes a FUSE request with userspace daemon response
    pub fn complete_request(&mut self, unique: u64, error: i32) -> Result<FuseOutHeader, &'static str> {
        if self.pending_requests.remove(&unique).is_none() {
            return Err("Unknown or timed out FUSE unique identifier");
        }

        Ok(FuseOutHeader {
            len: 16,
            error,
            unique,
        })
    }

    pub fn set_connected(&mut self, connected: bool) {
        self.is_connected = connected;
    }

    pub fn is_connected(&self) -> bool {
        self.is_connected
    }

    pub fn pending_count(&self) -> usize {
        self.pending_requests.len()
    }
}
