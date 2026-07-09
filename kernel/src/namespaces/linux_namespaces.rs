// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// Linux-inspired namespaces for SigmaOS
// Zero-allocation, performance-optimized namespace isolation

/// Namespace types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType {
    Mount,
    UTS,
    IPC,
    Network,
    PID,
    User,
    Cgroup,
    Time,
}

/// Namespace identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamespaceId(pub u64);

impl NamespaceId {
    pub const fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Namespace trait
pub trait Namespace {
    /// Get namespace type
    fn namespace_type(&self) -> NamespaceType;
    
    /// Get namespace ID
    fn id(&self) -> NamespaceId;
    
    /// Clone namespace
    fn clone_namespace(&self) -> Result<Box<dyn Namespace>, NamespaceError>;
    
    /// Get namespace owner
    fn owner(&self) -> u32;
    
    /// Check if namespace is empty
    fn is_empty(&self) -> bool;
}

/// Mount namespace
pub struct MountNamespace {
    pub id: NamespaceId,
    pub root: String,
    pub mount_points: Vec<MountPoint>,
    pub owner: u32,
}

impl MountNamespace {
    pub const fn new(id: NamespaceId, root: String) -> Self {
        Self {
            id,
            root,
            mount_points: Vec::new(),
            owner: 0,
        }
    }
}

/// UTS namespace (hostname and domain name)
pub struct UtsNamespace {
    pub id: NamespaceId,
    pub hostname: String,
    pub domainname: String,
    pub owner: u32,
}

impl UtsNamespace {
    pub const fn new(id: NamespaceId) -> Self {
        Self {
            id,
            hostname: String::new(),
            domainname: String::new(),
            owner: 0,
        }
    }
    
    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = hostname.to_string();
    }
    
    pub fn set_domainname(&mut self, domainname: &str) {
        self.domainname = domainname.to_string();
    }
}

/// IPC namespace
pub struct IpcNamespace {
    pub id: NamespaceId,
    pub message_queues: Vec<u64>,
    pub semaphores: Vec<u64>,
    pub shared_memory: Vec<u64>,
    pub owner: u32,
}

impl IpcNamespace {
    pub const fn new(id: NamespaceId) -> Self {
        Self {
            id,
            message_queues: Vec::new(),
            semaphores: Vec::new(),
            shared_memory: Vec::new(),
            owner: 0,
        }
    }
}

/// Network namespace
pub struct NetworkNamespace {
    pub id: NamespaceId,
    pub interfaces: Vec<String>,
    pub routes: Vec<Route>,
    pub firewall_rules: Vec<FirewallRule>,
    pub owner: u32,
}

impl NetworkNamespace {
    pub const fn new(id: NamespaceId) -> Self {
        Self {
            id,
            interfaces: Vec::new(),
            routes: Vec::new(),
            firewall_rules: Vec::new(),
            owner: 0,
        }
    }
}

pub struct Route {
    pub destination: String,
    pub gateway: String,
    pub interface: String,
    pub metric: u32,
}

pub struct FirewallRule {
    pub source: String,
    pub destination: String,
    pub action: FirewallAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Accept,
    Drop,
    Reject,
}

/// PID namespace
pub struct PidNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub child_pid: u32,
    pub processes: Vec<u32>,
    pub owner: u32,
}

impl PidNamespace {
    pub const fn new(id: NamespaceId, parent: Option<NamespaceId>) -> Self {
        Self {
            id,
            parent,
            child_pid: 1,
            processes: Vec::new(),
            owner: 0,
        }
    }
    
    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
    }
}

/// User namespace
pub struct UserNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub uid_map: Vec<UidMap>,
    pub gid_map: Vec<GidMap>,
    pub owner: u32,
}

impl UserNamespace {
    pub const fn new(id: NamespaceId, parent: Option<NamespaceId>) -> Self {
        Self {
            id,
            parent,
            uid_map: Vec::new(),
            gid_map: Vec::new(),
            owner: 0,
        }
    }
    
    pub fn add_uid_map(&mut self, map: UidMap) {
        self.uid_map.push(map);
    }
    
    pub fn add_gid_map(&mut self, map: GidMap) {
        self.gid_map.push(map);
    }
}

pub struct UidMap {
    pub container_first: u32,
    pub host_first: u32,
    pub count: u32,
}

pub struct GidMap {
    pub container_first: u32,
    pub host_first: u32,
    pub count: u32,
}

/// Cgroup namespace
pub struct CgroupNamespace {
    pub id: NamespaceId,
    pub root: String,
    pub owner: u32,
}

impl CgroupNamespace {
    pub const fn new(id: NamespaceId, root: String) -> Self {
        Self {
            id,
            root,
            owner: 0,
        }
    }
}

/// Time namespace
pub struct TimeNamespace {
    pub id: NamespaceId,
    pub offset: TimeOffset,
    pub owner: u32,
}

impl TimeNamespace {
    pub const fn new(id: NamespaceId) -> Self {
        Self {
            id,
            offset: TimeOffset::new(),
            owner: 0,
        }
    }
}

pub struct TimeOffset {
    pub clock_monotonic: i64,
    pub clock_boottime: i64,
}

impl TimeOffset {
    pub const fn new() -> Self {
        Self {
            clock_monotonic: 0,
            clock_boottime: 0,
        }
    }
}

/// Namespace manager
pub trait NamespaceManager {
    /// Initialize namespace manager
    fn init(&mut self) -> Result<(), NamespaceError>;
    
    /// Create namespace
    fn create_namespace(&mut self, ns_type: NamespaceType) -> Result<NamespaceId, NamespaceError>;
    
    /// Clone namespace
    fn clone_namespace(&mut self, ns_id: NamespaceId) -> Result<NamespaceId, NamespaceError>;
    
    /// Destroy namespace
    fn destroy_namespace(&mut self, ns_id: NamespaceId) -> Result<(), NamespaceError>;
    
    /// Get namespace
    fn get_namespace(&self, ns_id: NamespaceId) -> Option<&dyn Namespace>;
    
    /// List namespaces
    fn list_namespaces(&self, ns_type: NamespaceType) -> Vec<NamespaceId>;
}

/// Namespace error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceError {
    NamespaceNotFound,
    NamespaceExists,
    PermissionDenied,
    InvalidType,
    CloneFailed,
    ResourceLimit,
    Other,
}

/// Namespace flags
pub mod flags {
    pub const CLONE_NEWNS: u64 = 0x00020000;   // New mount namespace
    pub const CLONE_NEWUTS: u64 = 0x04000000;  // New UTS namespace
    pub const CLONE_NEWIPC: u64 = 0x08000000;  // New IPC namespace
    pub const CLONE_NEWNET: u64 = 0x40000000; // New network namespace
    pub const CLONE_NEWPID: u64 = 0x20000000; // New PID namespace
    pub const CLONE_NEWUSER: u64 = 0x10000000; // New user namespace
    pub const CLONE_NEWCGROUP: u64 = 0x02000000; // New cgroup namespace
    pub const CLONE_NEWTIME: u64 = 0x00000080; // New time namespace
}

/// Namespace operations
pub struct NamespaceOps {
    pub unshare: u64,
    pub clone: u64,
}

impl NamespaceOps {
    pub const fn new() -> Self {
        Self {
            unshare: 0,
            clone: 0,
        }
    }
    
    pub fn set_unshare(&mut self, flags: u64) {
        self.unshare = flags;
    }
    
    pub fn set_clone(&mut self, flags: u64) {
        self.clone = flags;
    }
}
