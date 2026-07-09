// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired security framework for SigmaOS
// Zero-allocation, performance-optimized security operations

/// Security context for processes
pub struct SecurityContext {
    pub uid: u32,
    pub gid: u32,
    pub euid: u32,
    pub egid: u32,
    pub fsuid: u32,
    pub fsgid: u32,
    pub capabilities: Capabilities,
    pub selinux_context: Option<String>,
    pub apparmor_profile: Option<String>,
}

impl SecurityContext {
    pub const fn new(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            euid: uid,
            egid: gid,
            fsuid: uid,
            fsgid: gid,
            capabilities: Capabilities::new(),
            selinux_context: None,
            apparmor_profile: None,
        }
    }
    
    pub fn is_root(&self) -> bool {
        self.uid == 0
    }
    
    pub fn has_capability(&self, cap: Capability) -> bool {
        if self.is_root() {
            return true;
        }
        self.capabilities.has(cap)
    }
}

/// Linux capabilities
pub struct Capabilities {
    pub effective: u64,
    pub permitted: u64,
    pub inheritable: u64,
    pub bounding: u64,
    pub ambient: u64,
}

impl Capabilities {
    pub const fn new() -> Self {
        Self {
            effective: 0,
            permitted: 0,
            inheritable: 0,
            bounding: u64::MAX,
            ambient: 0,
        }
    }
    
    pub fn has(&self, cap: Capability) -> bool {
        let bit = cap as u64;
        (self.effective & (1 << bit)) != 0
    }
    
    pub fn set(&mut self, cap: Capability) {
        let bit = cap as u64;
        self.effective |= 1 << bit;
        self.permitted |= 1 << bit;
    }
    
    pub fn clear(&mut self, cap: Capability) {
        let bit = cap as u64;
        self.effective &= !(1 << bit);
    }
}

/// Linux capability constants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Capability {
    CAP_CHOWN = 0,
    CAP_DAC_OVERRIDE = 1,
    CAP_DAC_READ_SEARCH = 2,
    CAP_FOWNER = 3,
    CAP_FSETID = 4,
    CAP_KILL = 5,
    CAP_SETGID = 6,
    CAP_SETUID = 7,
    CAP_SETPCAP = 8,
    CAP_LINUX_IMMUTABLE = 9,
    CAP_NET_BIND_SERVICE = 10,
    CAP_NET_BROADCAST = 11,
    CAP_NET_ADMIN = 12,
    CAP_NET_RAW = 13,
    CAP_IPC_LOCK = 14,
    CAP_IPC_OWNER = 15,
    CAP_SYS_MODULE = 16,
    CAP_SYS_RAWIO = 17,
    CAP_SYS_CHROOT = 18,
    CAP_SYS_PTRACE = 19,
    CAP_SYS_PACCT = 20,
    CAP_SYS_ADMIN = 21,
    CAP_SYS_BOOT = 22,
    CAP_SYS_NICE = 23,
    CAP_SYS_RESOURCE = 24,
    CAP_SYS_TIME = 25,
    CAP_SYS_TTY_CONFIG = 26,
    CAP_MKNOD = 27,
    CAP_LEASE = 28,
    CAP_AUDIT_WRITE = 29,
    CAP_AUDIT_CONTROL = 30,
    CAP_SETFCAP = 31,
    CAP_MAC_OVERRIDE = 32,
    CAP_MAC_ADMIN = 33,
    CAP_SYSLOG = 34,
    CAP_WAKE_ALARM = 35,
    CAP_BLOCK_SUSPEND = 36,
    CAP_AUDIT_READ = 37,
}

/// Access control list entry
pub struct AclEntry {
    pub tag: AclTag,
    pub qualifier: u32,
    pub permissions: AclPermissions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AclTag {
    UserObj,
    User,
    GroupObj,
    Group,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AclPermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl AclPermissions {
    pub const fn new() -> Self {
        Self {
            read: false,
            write: false,
            execute: false,
        }
    }
    
    pub fn from_mode(mode: u32) -> Self {
        Self {
            read: (mode & 0o4) != 0,
            write: (mode & 0o2) != 0,
            execute: (mode & 0o1) != 0,
        }
    }
    
    pub fn to_mode(&self) -> u32 {
        let mut mode = 0;
        if self.read { mode |= 0o4; }
        if self.write { mode |= 0o2; }
        if self.execute { mode |= 0o1; }
        mode
    }
}

/// Access control list
pub struct AccessControlList {
    pub entries: Vec<AclEntry>,
}

impl AccessControlList {
    pub const fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }
    
    pub fn add_entry(&mut self, entry: AclEntry) {
        self.entries.push(entry);
    }
    
    pub fn check_permission(&self, uid: u32, gid: u32, perm: AclPermissions) -> bool {
        for entry in &self.entries {
            match entry.tag {
                AclTag::UserObj if uid == entry.qualifier => {
                    return (entry.permissions.read || !perm.read) &&
                           (entry.permissions.write || !perm.write) &&
                           (entry.permissions.execute || !perm.execute);
                }
                AclTag::GroupObj if gid == entry.qualifier => {
                    return (entry.permissions.read || !perm.read) &&
                           (entry.permissions.write || !perm.write) &&
                           (entry.permissions.execute || !perm.execute);
                }
                AclTag::Other => {
                    return (entry.permissions.read || !perm.read) &&
                           (entry.permissions.write || !perm.write) &&
                           (entry.permissions.execute || !perm.execute);
                }
                _ => continue,
            }
        }
        false
    }
}

/// Security module interface
pub trait SecurityModule {
    /// Initialize security module
    fn init(&mut self) -> Result<(), SecurityError>;
    
    /// Check permission for operation
    fn check_permission(&self, operation: SecurityOperation) -> Result<bool, SecurityError>;
    
    /// Get security context
    fn get_context(&self) -> &SecurityContext;
    
    /// Set security context
    fn set_context(&mut self, context: SecurityContext) -> Result<(), SecurityError>;
    
    /// Audit security event
    fn audit_event(&mut self, event: SecurityEvent);
}

/// Security operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityOperation {
    FileRead,
    FileWrite,
    FileExecute,
    FileCreate,
    FileDelete,
    ProcessCreate,
    ProcessKill,
    ProcessSignal,
    NetworkBind,
    NetworkConnect,
    NetworkListen,
    CapabilityUse,
    SystemCall,
}

/// Security event for auditing
pub struct SecurityEvent {
    pub timestamp: u64,
    pub operation: SecurityOperation,
    pub subject: SecuritySubject,
    pub object: SecurityObject,
    pub result: SecurityResult,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecuritySubject {
    Process(u32),
    User(u32),
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityObject {
    File(u64),
    Process(u32),
    Network(u32),
    Capability(Capability),
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityResult {
    Allowed,
    Denied,
    Error,
}

/// Security error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SecurityError {
    PermissionDenied,
    InvalidContext,
    OperationNotPermitted,
    SecurityModuleError,
    AuditError,
    Other,
}

/// SELinux-like security context
pub struct SelinuxContext {
    pub user: String,
    pub role: String,
    pub type_: String,
    pub level: String,
}

impl SelinuxContext {
    pub fn new(user: &str, role: &str, type_: &str, level: &str) -> Self {
        Self {
            user: user.to_string(),
            role: role.to_string(),
            type_: type_.to_string(),
            level: level.to_string(),
        }
    }
    
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}:{}", self.user, self.role, self.type_, self.level)
    }
    
    pub fn from_string(s: &str) -> Option<Self> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() == 4 {
            Some(Self {
                user: parts[0].to_string(),
                role: parts[1].to_string(),
                type_: parts[2].to_string(),
                level: parts[3].to_string(),
            })
        } else {
            None
        }
    }
}

/// AppArmor-like profile
pub struct AppArmorProfile {
    pub name: String,
    pub path: String,
    pub mode: AppArmorMode,
    pub capabilities: Vec<Capability>,
    pub file_rules: Vec<FileRule>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforce,
    Complain,
    Kill,
    Unconfined,
}

pub struct FileRule {
    pub path: String,
    pub permissions: FilePermissions,
    pub audit: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub append: bool,
    pub mmap: bool,
}

impl AppArmorProfile {
    pub fn new(name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            mode: AppArmorMode::Enforce,
            capabilities: Vec::new(),
            file_rules: Vec::new(),
        }
    }
    
    pub fn add_file_rule(&mut self, rule: FileRule) {
        self.file_rules.push(rule);
    }
    
    pub fn add_capability(&mut self, cap: Capability) {
        self.capabilities.push(cap);
    }
}

/// Secure computing (seccomp) filter
pub struct SeccompFilter {
    pub mode: SeccompMode,
    pub allowed_syscalls: Vec<u32>,
    pub denied_syscalls: Vec<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SeccompMode {
    Disabled,
    Strict,
    Filter,
}

impl SeccompFilter {
    pub const fn new(mode: SeccompMode) -> Self {
        Self {
            mode,
            allowed_syscalls: Vec::new(),
            denied_syscalls: Vec::new(),
        }
    }
    
    pub fn allow_syscall(&mut self, syscall: u32) {
        self.allowed_syscalls.push(syscall);
    }
    
    pub fn deny_syscall(&mut self, syscall: u32) {
        self.denied_syscalls.push(syscall);
    }
    
    pub fn check_syscall(&self, syscall: u32) -> bool {
        match self.mode {
            SeccompMode::Disabled => true,
            SeccompMode::Strict => false,
            SeccompMode::Filter => {
                if self.denied_syscalls.contains(&syscall) {
                    false
                } else if self.allowed_syscalls.is_empty() {
                    true
                } else {
                    self.allowed_syscalls.contains(&syscall)
                }
            }
        }
    }
}

/// Key management
pub struct KeyRing {
    pub keys: Vec<Key>,
    pub max_keys: usize,
}

pub struct Key {
    pub id: u64,
    pub type_: KeyType,
    pub description: String,
    pub data: Vec<u8>,
    pub permissions: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyType {
    User,
    Session,
    Process,
    Thread,
    RequestKey,
}

impl KeyRing {
    pub const fn new(max_keys: usize) -> Self {
        Self {
            keys: Vec::new(),
            max_keys,
        }
    }
    
    pub fn add_key(&mut self, key: Key) -> Result<(), SecurityError> {
        if self.keys.len() >= self.max_keys {
            return Err(SecurityError::OperationNotPermitted);
        }
        self.keys.push(key);
        Ok(())
    }
    
    pub fn get_key(&self, id: u64) -> Option<&Key> {
        self.keys.iter().find(|k| k.id == id)
    }
    
    pub fn remove_key(&mut self, id: u64) -> bool {
        if let Some(pos) = self.keys.iter().position(|k| k.id == id) {
            self.keys.remove(pos);
            true
        } else {
            false
        }
    }
}
