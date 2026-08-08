#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;

use crate::filesystem::vfs::{FsError, Inode};

#[derive(Debug, Clone)]
pub struct InodeAttr {
    pub mode: u32,
    pub uid: u32,
    pub gid: u32,
    pub size: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    PermissionDenied,
    CapabilityMissing,
    PolicyViolation,
    LabelInvalid,
}

pub const CAP_CHOWN: u64 = 1 << 0;
pub const CAP_DAC_OVERRIDE: u64 = 1 << 1;
pub const CAP_DAC_READ_SEARCH: u64 = 1 << 2;
pub const CAP_FOWNER: u64 = 1 << 3;
pub const CAP_FSETID: u64 = 1 << 4;
pub const CAP_KILL: u64 = 1 << 5;
pub const CAP_SETGID: u64 = 1 << 6;
pub const CAP_SETUID: u64 = 1 << 7;
pub const CAP_SETPCAP: u64 = 1 << 8;
pub const CAP_LINUX_IMMUTABLE: u64 = 1 << 9;
pub const CAP_NET_BIND_SERVICE: u64 = 1 << 10;
pub const CAP_NET_BROADCAST: u64 = 1 << 11;
pub const CAP_NET_ADMIN: u64 = 1 << 12;
pub const CAP_NET_RAW: u64 = 1 << 13;
pub const CAP_IPC_LOCK: u64 = 1 << 14;
pub const CAP_IPC_OWNER: u64 = 1 << 15;
pub const CAP_SYS_MODULE: u64 = 1 << 16;
pub const CAP_SYS_RAWIO: u64 = 1 << 17;
pub const CAP_SYS_CHROOT: u64 = 1 << 18;
pub const CAP_SYS_PTRACE: u64 = 1 << 19;
pub const CAP_SYS_PACCT: u64 = 1 << 20;
pub const CAP_SYS_ADMIN: u64 = 1 << 21;
pub const CAP_SYS_BOOT: u64 = 1 << 22;
pub const CAP_SYS_NICE: u64 = 1 << 23;
pub const CAP_SYS_RESOURCE: u64 = 1 << 24;
pub const CAP_SYS_TIME: u64 = 1 << 25;
pub const CAP_SYS_TTY_CONFIG: u64 = 1 << 26;
pub const CAP_MKNOD: u64 = 1 << 27;
pub const CAP_LEASE: u64 = 1 << 28;
pub const CAP_AUDIT_WRITE: u64 = 1 << 29;
pub const CAP_AUDIT_CONTROL: u64 = 1 << 30;
pub const CAP_SETFCAP: u64 = 1 << 31;

pub struct CapabilitySet {
    pub inheritable: u64,
    pub permitted: u64,
    pub effective: u64,
    pub bounding: u64,
    pub ambient: u64,
}

impl CapabilitySet {
    pub fn new() -> Self {
        CapabilitySet {
            inheritable: 0,
            permitted: 0,
            effective: 0,
            bounding: 0,
            ambient: 0,
        }
    }

    pub fn allow_capability(&mut self, cap: u64) {
        self.permitted |= cap;
        self.effective |= cap;
        self.inheritable |= cap;
        self.bounding |= cap;
    }

    pub fn deny_capability(&mut self, cap: u64) {
        self.permitted &= !cap;
        self.effective &= !cap;
        self.inheritable &= !cap;
        self.bounding &= !cap;
    }

    pub fn has_capability(&self, cap: u64) -> bool {
        (self.effective & cap) != 0
    }

    pub fn is_empty(&self) -> bool {
        self.effective == 0 && self.permitted == 0
    }
}

pub struct Label {
    pub name: String,
    pub role: Option<String>,
    pub r#type: Option<String>,
    pub level: Option<String>,
}

impl Label {
    pub fn new(name: &str) -> Self {
        Label {
            name: name.to_string(),
            role: None,
            r#type: None,
            level: None,
        }
    }
}

pub struct SecurityTask {
    pub cred: CapabilitySet,
    pub secid: u64,
    pub exe_label: Option<Label>,
    pub fscreate_label: Option<Label>,
    pub keycreate_label: Option<Label>,
    pub sockcreate_label: Option<Label>,
}

pub trait MacPolicy: Send + Sync {
    fn subject(&self) -> &Label;
    fn object(&self) -> &Label;
    fn label(&self) -> &Label;
    fn cmp(&self, label1: &Label, label2: &Label) -> bool;
    fn validate_transition(&self, subject: &Label, object: &Label) -> bool;
    fn inode_permission(&self, inode: &Inode, request: u32) -> Result<(), SecurityError>;
    fn task_create(
        &self,
        parent: &SecurityTask,
        child: &mut SecurityTask,
    ) -> Result<(), SecurityError>;
    fn socket_create(&self, family: u32, type_: u32, protocol: u32) -> Result<(), SecurityError>;
    fn socket_bind(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;
    fn socket_connect(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;
    fn syscall_check(&self, syscall: u32) -> Result<(), SecurityError>;
    fn ptrace_access_check(
        &self,
        tracer: &SecurityTask,
        tracee: &SecurityTask,
    ) -> Result<(), SecurityError>;
    fn ptrace_traceme(&self, tracer: &SecurityTask) -> Result<(), SecurityError>;
    fn file_open(&self, inode: &Inode, flags: u32) -> Result<(), SecurityError>;
    fn file_exec(&self, inode: &Inode) -> Result<(), SecurityError>;
    fn inode_create(&self, dir: &Inode, name: &str, mode: u32) -> Result<(), SecurityError>;
    fn inode_link(
        &self,
        old_inode: &Inode,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;
    fn inode_unlink(&self, dir: &Inode, name: &str) -> Result<(), SecurityError>;
    fn inode_rename(
        &self,
        old_dir: &Inode,
        old_name: &str,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;
}

pub struct AvcEntry {
    pub subject: u64,
    pub object: u64,
    pub permission: u32,
    pub allowed: bool,
}

pub struct AvcCache {
    pub entries: Vec<AvcEntry>,
}

impl AvcCache {
    pub fn new() -> Self {
        AvcCache {
            entries: Vec::new(),
        }
    }

    pub fn lookup(&self, subject: u64, object: u64, permission: u32) -> Option<bool> {
        for entry in &self.entries {
            if entry.subject == subject && entry.object == object && entry.permission == permission
            {
                return Some(entry.allowed);
            }
        }
        None
    }

    pub fn add(&mut self, subject: u64, object: u64, permission: u32, allowed: bool) {
        self.entries.push(AvcEntry {
            subject,
            object,
            permission,
            allowed,
        });
    }

    pub fn flush(&mut self) {
        self.entries.clear();
    }
}

pub struct AuditEntry {
    pub timestamp: u64,
    pub subject: u64,
    pub object: u64,
    pub operation: String,
    pub result: bool,
    pub info: String,
}

pub struct AuditLog {
    pub entries: Vec<AuditEntry>,
}

impl AuditLog {
    pub fn new() -> Self {
        AuditLog {
            entries: Vec::new(),
        }
    }

    pub fn log(&mut self, subject: u64, object: u64, operation: &str, result: bool, info: &str) {
        self.entries.push(AuditEntry {
            timestamp: 0,
            subject,
            object,
            operation: operation.to_string(),
            result,
            info: info.to_string(),
        });
    }
}

pub trait LsmHook: Send + Sync {
    fn security_inode_create(
        &self,
        dir: &Inode,
        name: &str,
        mode: u32,
    ) -> Result<(), SecurityError>;
    fn security_inode_link(
        &self,
        old_inode: &Inode,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;
    fn security_inode_unlink(&self, dir: &Inode, name: &str) -> Result<(), SecurityError>;
    fn security_inode_rename(
        &self,
        old_dir: &Inode,
        old_name: &str,
        new_dir: &Inode,
        new_name: &str,
    ) -> Result<(), SecurityError>;
    fn security_inode_setattr(&self, inode: &Inode, attr: &InodeAttr) -> Result<(), SecurityError>;
    fn security_inode_getattr(&self, inode: &Inode) -> Result<(), SecurityError>;
    fn security_file_open(&self, inode: &Inode, flags: u32) -> Result<(), SecurityError>;
    fn security_file_permission(&self, inode: &Inode, mask: u32) -> Result<(), SecurityError>;
    fn security_socket_create(
        &self,
        family: u32,
        type_: u32,
        protocol: u32,
    ) -> Result<(), SecurityError>;
    fn security_socket_bind(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;
    fn security_socket_connect(&self, sock: usize, addr: &[u8]) -> Result<(), SecurityError>;
    fn security_socket_listen(&self, sock: usize, backlog: u32) -> Result<(), SecurityError>;
    fn security_socket_accept(&self, sock: usize) -> Result<(), SecurityError>;
    fn security_socket_sendmsg(&self, sock: usize) -> Result<(), SecurityError>;
    fn security_socket_recvmsg(&self, sock: usize) -> Result<(), SecurityError>;
    fn security_ptrace_access_check(&self, tracer: u64, tracee: u64) -> Result<(), SecurityError>;
    fn security_ptrace_traceme(&self, tracer: u64) -> Result<(), SecurityError>;
    fn security_capable(&self, cred: u64, cap: u64) -> Result<(), SecurityError>;
    fn security_bprm_set_uid(&self, bprm: usize) -> Result<(), SecurityError>;
    fn security_inode_mknod(&self, dir: &Inode, name: &str, mode: u32)
        -> Result<(), SecurityError>;
    fn security_inode_symlink(
        &self,
        dir: &Inode,
        name: &str,
        target: &str,
    ) -> Result<(), SecurityError>;
}
