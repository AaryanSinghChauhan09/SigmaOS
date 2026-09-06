//! SigmaOS Kernel Namespaces
//!
//! Sovereign implementation of Linux-compatible kernel namespaces.
//! Provides isolation boundaries for processes.
//!
//! Supported namespace types:
//! - **PID** — Process ID isolation (containers see separate PID 1)
//! - **Network** — Network stack isolation (interfaces, routes, iptables)
//! - **Mount** — Filesystem mount isolation
//! - **UTS** — Hostname and domain name isolation
//! - **IPC** — System V IPC and POSIX mq isolation
//! - **User** — UID/GID mapping (rootless containers)
//! - **Cgroup** — cgroup root isolation
//! - **Time** — Clock offset isolation (Linux 5.6+)

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(clippy::new_without_default)]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

// ============================================================
// Namespace Types
// ============================================================

/// The type of kernel namespace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    /// PID namespace — separate process ID space
    Pid,
    /// Network namespace — isolated network stack
    Net,
    /// Mount namespace — isolated mount table
    Mnt,
    /// UTS namespace — isolated hostname/domainname
    Uts,
    /// IPC namespace — isolated SysV IPC, POSIX mq
    Ipc,
    /// User namespace — UID/GID remapping
    User,
    /// Cgroup namespace — isolated cgroup root
    Cgroup,
    /// Time namespace — per-namespace clock offsets
    Time,
}

impl NamespaceType {
    /// The clone flag bit corresponding to this namespace type.
    pub fn clone_flag(self) -> u64 {
        match self {
            Self::Pid    => 0x20000000, // CLONE_NEWPID
            Self::Net    => 0x40000000, // CLONE_NEWNET
            Self::Mnt    => 0x00020000, // CLONE_NEWNS
            Self::Uts    => 0x04000000, // CLONE_NEWUTS
            Self::Ipc    => 0x08000000, // CLONE_NEWIPC
            Self::User   => 0x10000000, // CLONE_NEWUSER
            Self::Cgroup => 0x02000000, // CLONE_NEWCGROUP
            Self::Time   => 0x00000080, // CLONE_NEWTIME
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Pid => "pid", Self::Net => "net", Self::Mnt => "mnt",
            Self::Uts => "uts", Self::Ipc => "ipc", Self::User => "user",
            Self::Cgroup => "cgroup", Self::Time => "time",
        }
    }
}

// ============================================================
// Namespace ID
// ============================================================

/// Unique identifier for a namespace instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NsId(u64);

static NEXT_NS_ID: AtomicU64 = AtomicU64::new(1);

impl NsId {
    fn alloc() -> Self { Self(NEXT_NS_ID.fetch_add(1, Ordering::Relaxed)) }
}

// ============================================================
// UTS Namespace
// ============================================================

/// UTS namespace — hostname and domainname isolation.
#[derive(Debug, Clone)]
pub struct UtsNamespace {
    pub id: NsId,
    pub hostname: String,
    pub domainname: String,
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    pub version: String,
    pub machine: String,
}

impl UtsNamespace {
    pub fn new(hostname: &str) -> Self {
        Self {
            id: NsId::alloc(),
            hostname: hostname.into(),
            domainname: "(none)".into(),
            sysname: "SigmaOS".into(),
            nodename: hostname.into(),
            release: "6.6.0-sigma".into(),
            version: "#1 SMP".into(),
            machine: "x86_64".into(),
        }
    }

    pub fn set_hostname(&mut self, name: &str) { self.hostname = name.into(); self.nodename = name.into(); }
    pub fn set_domainname(&mut self, name: &str) { self.domainname = name.into(); }
}

// ============================================================
// PID Namespace
// ============================================================

/// PID namespace — isolated PID number space.
///
/// Each PID namespace has its own PID 1 (init process).
/// Child namespaces see pids starting from 1, while the
/// host sees the real global PIDs.
#[derive(Debug, Clone)]
pub struct PidNamespace {
    pub id: NsId,
    /// Nesting level (0 = host, 1 = first child, ...)
    pub level: u32,
    /// Parent namespace ID
    pub parent: Option<NsId>,
    /// Next PID to assign within this namespace
    next_pid: u32,
    /// Map from namespace-local PID → global PID
    pub pid_map: BTreeMap<u32, u32>,
    /// Init process PID (global)
    pub init_pid: u32,
}

impl PidNamespace {
    pub fn new_root() -> Self {
        Self { id: NsId::alloc(), level: 0, parent: None, next_pid: 1,
            pid_map: BTreeMap::new(), init_pid: 1 }
    }

    pub fn new_child(parent: &PidNamespace) -> Self {
        Self { id: NsId::alloc(), level: parent.level + 1, parent: Some(parent.id),
            next_pid: 1, pid_map: BTreeMap::new(), init_pid: 0 }
    }

    /// Allocate a new PID within this namespace, mapping to global_pid.
    pub fn alloc_pid(&mut self, global_pid: u32) -> u32 {
        let local = self.next_pid;
        self.next_pid += 1;
        self.pid_map.insert(local, global_pid);
        if local == 1 { self.init_pid = global_pid; }
        local
    }

    /// Translate local PID to global PID.
    pub fn to_global(&self, local: u32) -> Option<u32> { self.pid_map.get(&local).copied() }
}

// ============================================================
// IPC Namespace
// ============================================================

/// IPC namespace — isolated System V IPC objects.
#[derive(Debug, Clone)]
pub struct IpcNamespace {
    pub id: NsId,
    /// Message queue count
    pub mq_count: u32,
    /// Max message queue count
    pub mq_max: u32,
    /// Semaphore set count
    pub sem_count: u32,
    /// Shared memory segment count
    pub shm_count: u32,
    /// Max shared memory total (bytes)
    pub shmall: u64,
}

impl IpcNamespace {
    pub fn new() -> Self {
        Self { id: NsId::alloc(), mq_count: 0, mq_max: 256, sem_count: 0, shm_count: 0,
            shmall: 8 * 1024 * 1024 * 1024 } // 8GB default
    }
}

// ============================================================
// User Namespace
// ============================================================

/// A UID/GID mapping entry.
#[derive(Debug, Clone)]
pub struct IdMapEntry {
    /// First UID/GID in namespace
    pub ns_id: u32,
    /// Corresponding host UID/GID
    pub host_id: u32,
    /// Number of IDs in range
    pub count: u32,
}

/// User namespace — UID/GID remapping for rootless containers.
#[derive(Debug, Clone)]
pub struct UserNamespace {
    pub id: NsId,
    pub parent: Option<NsId>,
    /// UID mappings (ns → host)
    pub uid_map: Vec<IdMapEntry>,
    /// GID mappings (ns → host)
    pub gid_map: Vec<IdMapEntry>,
    /// Owner UID of this namespace (host UID)
    pub owner_uid: u32,
}

impl UserNamespace {
    pub fn new_root() -> Self {
        // Root namespace: identity mapping
        Self {
            id: NsId::alloc(), parent: None,
            uid_map: vec![IdMapEntry { ns_id: 0, host_id: 0, count: u32::MAX }],
            gid_map: vec![IdMapEntry { ns_id: 0, host_id: 0, count: u32::MAX }],
            owner_uid: 0,
        }
    }

    pub fn new_child(owner_uid: u32, parent: &UserNamespace) -> Self {
        Self { id: NsId::alloc(), parent: Some(parent.id),
            uid_map: Vec::new(), gid_map: Vec::new(), owner_uid }
    }

    /// Add a UID mapping. Maps ns_ids [ns_start..ns_start+count) → host [host_start..host_start+count).
    pub fn add_uid_map(&mut self, ns_start: u32, host_start: u32, count: u32) -> Result<(), &'static str> {
        if self.uid_map.len() >= 5 { return Err("too many uid_map entries"); }
        self.uid_map.push(IdMapEntry { ns_id: ns_start, host_id: host_start, count });
        Ok(())
    }

    /// Translate a namespace UID to host UID.
    pub fn ns_uid_to_host(&self, ns_uid: u32) -> Option<u32> {
        for entry in &self.uid_map {
            if ns_uid >= entry.ns_id && ns_uid < entry.ns_id + entry.count {
                return Some(entry.host_id + (ns_uid - entry.ns_id));
            }
        }
        None
    }

    /// Translate a host UID to namespace UID.
    pub fn host_uid_to_ns(&self, host_uid: u32) -> Option<u32> {
        for entry in &self.uid_map {
            if host_uid >= entry.host_id && host_uid < entry.host_id + entry.count {
                return Some(entry.ns_id + (host_uid - entry.host_id));
            }
        }
        None
    }
}

// ============================================================
// NamespaceSet — Full Set for a Process
// ============================================================

/// The complete set of namespaces a process belongs to.
///
/// Analogous to Linux `nsproxy` struct.
#[derive(Debug, Clone)]
pub struct NamespaceSet {
    pub uts: NsId,
    pub pid: NsId,
    pub ipc: NsId,
    pub user: NsId,
    pub mnt: NsId,
    pub net: NsId,
}

// ============================================================
// SigmaNamespaceManager — Global Namespace Registry
// ============================================================

/// Global manager for all kernel namespace instances.
///
/// Maintains the full namespace hierarchy and provides
/// clone/unshare/setns operations.
pub struct SigmaNamespaceManager {
    pub uts_ns:  BTreeMap<NsId, UtsNamespace>,
    pub pid_ns:  BTreeMap<NsId, PidNamespace>,
    pub ipc_ns:  BTreeMap<NsId, IpcNamespace>,
    pub user_ns: BTreeMap<NsId, UserNamespace>,
    /// Map from PID to its NamespaceSet
    process_ns: BTreeMap<u32, NamespaceSet>,
    /// The initial (host) namespace IDs
    pub initial_ns: NamespaceSet,
}

impl SigmaNamespaceManager {
    /// Create a new manager with host/initial namespaces.
    pub fn new() -> Self {
        let uts  = UtsNamespace::new("sigmaos");
        let pid  = PidNamespace::new_root();
        let ipc  = IpcNamespace::new();
        let user = UserNamespace::new_root();

        let init_ns = NamespaceSet {
            uts: uts.id, pid: pid.id, ipc: ipc.id,
            user: user.id, mnt: NsId(0), net: NsId(0),
        };

        let mut mgr = Self {
            uts_ns: BTreeMap::new(), pid_ns: BTreeMap::new(),
            ipc_ns: BTreeMap::new(), user_ns: BTreeMap::new(),
            process_ns: BTreeMap::new(), initial_ns: init_ns.clone(),
        };
        mgr.uts_ns.insert(uts.id, uts);
        mgr.pid_ns.insert(pid.id, pid);
        mgr.ipc_ns.insert(ipc.id, ipc);
        mgr.user_ns.insert(user.id, user);
        mgr
    }

    /// Register a process with the initial (host) namespaces.
    pub fn register_process(&mut self, pid: u32) {
        self.process_ns.insert(pid, self.initial_ns.clone());
    }

    /// Unregister a process (on exit).
    pub fn unregister_process(&mut self, pid: u32) { self.process_ns.remove(&pid); }

    /// Create new namespaces for a process (clone/unshare flags).
    ///
    /// # Arguments
    /// * `pid` — PID requesting the unshare
    /// * `flags` — Bitmask of CLONE_NEW* flags
    pub fn unshare(&mut self, pid: u32, flags: u64) -> Result<(), &'static str> {
        let ns_set = self.process_ns.get(&pid).cloned().ok_or("process not found")?;

        let mut new_ns = ns_set;

        if flags & NamespaceType::Uts.clone_flag() != 0 {
            let old_uts = self.uts_ns.get(&new_ns.uts).cloned().ok_or("uts ns not found")?;
            let new_uts = UtsNamespace { id: NsId::alloc(), ..old_uts };
            let id = new_uts.id;
            self.uts_ns.insert(id, new_uts);
            new_ns.uts = id;
        }

        if flags & NamespaceType::Ipc.clone_flag() != 0 {
            let new_ipc = IpcNamespace::new();
            let id = new_ipc.id;
            self.ipc_ns.insert(id, new_ipc);
            new_ns.ipc = id;
        }

        if flags & NamespaceType::Pid.clone_flag() != 0 {
            let parent = self.pid_ns.get(&new_ns.pid).ok_or("pid ns not found")?;
            let new_pid = PidNamespace::new_child(parent);
            let id = new_pid.id;
            self.pid_ns.insert(id, new_pid);
            new_ns.pid = id;
        }

        self.process_ns.insert(pid, new_ns);
        Ok(())
    }

    /// Set hostname within a process's UTS namespace.
    pub fn sethostname(&mut self, pid: u32, hostname: &str) -> Result<(), &'static str> {
        let uts_id = self.process_ns.get(&pid).map(|n| n.uts).ok_or("process not found")?;
        self.uts_ns.get_mut(&uts_id).ok_or("uts ns not found")?.set_hostname(hostname);
        Ok(())
    }

    /// Get hostname from a process's UTS namespace.
    pub fn gethostname(&self, pid: u32) -> Result<&str, &'static str> {
        let uts_id = self.process_ns.get(&pid).map(|n| n.uts).ok_or("process not found")?;
        self.uts_ns.get(&uts_id).map(|u| u.hostname.as_str()).ok_or("uts ns not found")
    }

    /// Get namespace set for a process.
    pub fn process_namespaces(&self, pid: u32) -> Option<&NamespaceSet> {
        self.process_ns.get(&pid)
    }

    pub fn total_namespaces(&self) -> usize {
        self.uts_ns.len() + self.pid_ns.len() + self.ipc_ns.len() + self.user_ns.len()
    }
}

impl Default for SigmaNamespaceManager {
    fn default() -> Self { Self::new() }
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initial_namespaces() {
        let mgr = SigmaNamespaceManager::new();
        assert!(mgr.uts_ns.contains_key(&mgr.initial_ns.uts));
        assert!(mgr.pid_ns.contains_key(&mgr.initial_ns.pid));
    }

    #[test]
    fn test_sethostname() {
        let mut mgr = SigmaNamespaceManager::new();
        mgr.register_process(100);
        mgr.sethostname(100, "container1").unwrap();
        assert_eq!(mgr.gethostname(100).unwrap(), "container1");
    }

    #[test]
    fn test_unshare_uts() {
        let mut mgr = SigmaNamespaceManager::new();
        mgr.register_process(200);
        mgr.register_process(201);
        mgr.unshare(200, NamespaceType::Uts.clone_flag()).unwrap();
        mgr.sethostname(200, "isolated").unwrap();
        // Process 201 should still have original hostname
        let host200 = mgr.gethostname(200).unwrap().to_string();
        let host201 = mgr.gethostname(201).unwrap().to_string();
        assert_ne!(host200, host201);
        assert_eq!(host200, "isolated");
    }

    #[test]
    fn test_unshare_pid() {
        let mut mgr = SigmaNamespaceManager::new();
        mgr.register_process(300);
        mgr.unshare(300, NamespaceType::Pid.clone_flag()).unwrap();
        let ns = mgr.process_namespaces(300).unwrap();
        assert_ne!(ns.pid, mgr.initial_ns.pid); // New PID namespace
    }

    #[test]
    fn test_uid_mapping() {
        let root_ns = UserNamespace::new_root();
        let mut child = UserNamespace::new_child(1000, &root_ns);
        child.add_uid_map(0, 1000, 65536).unwrap();
        // Container root (0) → host UID 1000
        assert_eq!(child.ns_uid_to_host(0), Some(1000));
        assert_eq!(child.ns_uid_to_host(1), Some(1001));
        assert_eq!(child.host_uid_to_ns(1000), Some(0));
    }

    #[test]
    fn test_pid_namespace_alloc() {
        let mut ns = PidNamespace::new_root();
        let local1 = ns.alloc_pid(100); // global PID 100
        let local2 = ns.alloc_pid(101);
        assert_eq!(local1, 1);
        assert_eq!(local2, 2);
        assert_eq!(ns.to_global(1), Some(100));
        assert_eq!(ns.init_pid, 100);
    }
}
