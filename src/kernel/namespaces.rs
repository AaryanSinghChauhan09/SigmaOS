//! # Namespace Infrastructure Module
//!
//! This module provides the core namespace infrastructure for process isolation in SigmaOS.
//! It supports various namespace types including PID, IPC, Network, UTS, User, Cgroup, and Mount namespaces.
//!
//! ## Architecture
//!
//! - **KernelNamespace trait**: Generic interface for all namespace types
//! - **NamespaceRegistry**: Central registry for namespace management
//! - **Specific namespace implementations**: PID, IPC, Network, UTS, User, Cgroup, Mount
//!
//! Inspired by Linux namespaces (CLONE_NEWPID, CLONE_NEWNS, CLONE_NEWNET, CLONE_NEWUTS, CLONE_NEWUSER, CLONE_NEWCGROUP, CLONE_NEWNS)

use std::sync::atomic::{AtomicU64, AtomicU32, Ordering};
use std::string::String;
use std::collections::BTreeMap;

/// Maximum number of namespaces in the system
pub const MAX_NAMESPACES: usize = 1024;

/// Maximum processes per PID namespace
pub const MAX_PIDS_PER_NAMESPACE: u32 = 32768;

/// Unique namespace identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NamespaceId(u64);

impl NamespaceId {
    /// Create a new namespace ID
    pub fn new(id: u64) -> Self {
        NamespaceId(id)
    }

    /// Get the raw ID value
    pub fn raw(&self) -> u64 {
        self.0
    }
}

/// Types of namespaces supported by SigmaOS (for trait-based namespace system)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KernelNamespaceType {
    Pid,
    Ipc,
    Network,
    Uts,  // UTS (hostname/domainname)
    User,
    Cgroup,
    Mount,
}

impl KernelNamespaceType {
    /// Get a string representation of the namespace type
    pub fn as_str(&self) -> &'static str {
        match self {
            KernelNamespaceType::Pid => "pid",
            KernelNamespaceType::Ipc => "ipc",
            KernelNamespaceType::Network => "network",
            KernelNamespaceType::Uts => "uts",
            KernelNamespaceType::User => "user",
            KernelNamespaceType::Cgroup => "cgroup",
            KernelNamespaceType::Mount => "mount",
        }
    }
}

/// Generic namespace trait defining the interface all namespaces must implement
pub trait KernelNamespace: Send + Sync {
    /// Get the unique namespace ID
    fn namespace_id(&self) -> NamespaceId;

    /// Get the namespace type
    fn namespace_type(&self) -> KernelNamespaceType;

    /// Get the reference count (how many processes use this namespace)
    fn ref_count(&self) -> u32;

    /// Increment reference count (when a process enters this namespace)
    fn increment_ref(&self);

    /// Decrement reference count (when a process leaves this namespace)
    fn decrement_ref(&self);

    /// Check if this namespace is equal to another
    fn equals(&self, other: &dyn KernelNamespace) -> bool {
        self.namespace_id() == other.namespace_id()
    }

    /// Get namespace metadata as a string
    fn metadata(&self) -> String;
}

/// Namespace creation configuration
#[derive(Debug, Clone)]
pub struct NamespaceConfig {
    pub namespace_type: KernelNamespaceType,
    pub inherit_from: Option<NamespaceId>,
}

impl NamespaceConfig {
    /// Create a new namespace configuration
    pub fn new(namespace_type: KernelNamespaceType) -> Self {
        NamespaceConfig {
            namespace_type,
            inherit_from: None,
        }
    }

    /// Create configuration that inherits from an existing namespace
    pub fn inherit(namespace_type: KernelNamespaceType, parent_id: NamespaceId) -> Self {
        NamespaceConfig {
            namespace_type,
            inherit_from: Some(parent_id),
        }
    }
}

/// Namespace error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceError {
    InvalidNamespaceId,
    NamespaceFull,
    NamespaceTypeNotSupported,
    ProcessNotInNamespace,
    InsufficientPermissions,
    AlreadyInNamespace,
}

impl NamespaceError {
    /// Get error message
    pub fn message(&self) -> &'static str {
        match self {
            NamespaceError::InvalidNamespaceId => "Invalid namespace ID",
            NamespaceError::NamespaceFull => "Namespace is full",
            NamespaceError::NamespaceTypeNotSupported => "Namespace type not supported",
            NamespaceError::ProcessNotInNamespace => "Process not in namespace",
            NamespaceError::InsufficientPermissions => "Insufficient permissions",
            NamespaceError::AlreadyInNamespace => "Process already in this namespace type",
        }
    }
}

/// Global namespace ID generator
#[derive(Debug)]
pub struct NamespaceIdGenerator {
    next_id: AtomicU64,
}

impl NamespaceIdGenerator {
    /// Create a new ID generator
    pub const fn new() -> Self {
        NamespaceIdGenerator {
            next_id: AtomicU64::new(1),
        }
    }

    /// Generate the next namespace ID
    pub fn next(&self) -> NamespaceId {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        NamespaceId(id)
    }
}

/// Global namespace ID generator instance
static NAMESPACE_ID_GEN: NamespaceIdGenerator = NamespaceIdGenerator::new();

/// Get the next global namespace ID
pub fn next_namespace_id() -> NamespaceId {
    NAMESPACE_ID_GEN.next()
}

// ─── PID Namespace Implementation ─────────────────────────────────────────────

/// PID namespace for process isolation
#[derive(Debug)]
pub struct PidNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    next_pid: AtomicU32,
}

impl PidNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        PidNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            next_pid: AtomicU32::new(1),
        }
    }

    pub fn allocate_pid(&self) -> u32 {
        self.next_pid.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for PidNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Pid
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("PID namespace {} (parent: {:?}, refcount: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst))
    }
}

// ─── IPC Namespace Implementation ─────────────────────────────────────────────

/// IPC namespace for IPC isolation
#[derive(Debug)]
pub struct IpcNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    message_queues: AtomicU64,
}

impl IpcNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        IpcNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            message_queues: AtomicU64::new(0),
        }
    }

    pub fn allocate_queue(&self) -> u64 {
        self.message_queues.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for IpcNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Ipc
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("IPC namespace {} (parent: {:?}, refcount: {}, queues: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.message_queues.load(Ordering::SeqCst))
    }
}

// ─── Network Namespace Implementation ─────────────────────────────────────────────

/// Network namespace for network isolation
#[derive(Debug)]
pub struct NetworkNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    interfaces: AtomicU64,
}

impl NetworkNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        NetworkNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            interfaces: AtomicU64::new(0),
        }
    }

    pub fn allocate_interface(&self) -> u64 {
        self.interfaces.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for NetworkNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Network
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("Network namespace {} (parent: {:?}, refcount: {}, interfaces: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.interfaces.load(Ordering::SeqCst))
    }
}

// ─── UTS Namespace Implementation ─────────────────────────────────────────────

/// UTS namespace for hostname/domainname isolation
#[derive(Debug)]
pub struct UtsNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    hostname: String,
    domainname: String,
}

impl UtsNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>, hostname: &str, domainname: &str) -> Self {
        UtsNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            hostname: String::from(hostname),
            domainname: String::from(domainname),
        }
    }

    pub fn set_hostname(&mut self, hostname: &str) {
        self.hostname = String::from(hostname);
    }

    pub fn set_domainname(&mut self, domainname: &str) {
        self.domainname = String::from(domainname);
    }

    pub fn get_hostname(&self) -> &str {
        &self.hostname
    }

    pub fn get_domainname(&self) -> &str {
        &self.domainname
    }
}

impl KernelNamespace for UtsNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Uts
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("UTS namespace {} (parent: {:?}, refcount: {}, hostname: {}, domainname: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.hostname, self.domainname)
    }
}

// ─── User Namespace Implementation ─────────────────────────────────────────────

/// User namespace for user/group ID isolation
#[derive(Debug)]
pub struct UserNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    uid_map: AtomicU32,
    gid_map: AtomicU32,
}

impl UserNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        UserNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            uid_map: AtomicU32::new(0),
            gid_map: AtomicU32::new(0),
        }
    }

    pub fn map_uid(&self, _uid: u32) -> u32 {
        self.uid_map.fetch_add(1, Ordering::SeqCst)
    }

    pub fn map_gid(&self, _gid: u32) -> u32 {
        self.gid_map.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for UserNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::User
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("User namespace {} (parent: {:?}, refcount: {}, uid_maps: {}, gid_maps: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.uid_map.load(Ordering::SeqCst), self.gid_map.load(Ordering::SeqCst))
    }
}

// ─── Cgroup Namespace Implementation ─────────────────────────────────────────────

/// Cgroup namespace for cgroup isolation
#[derive(Debug)]
pub struct CgroupNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    cgroups: AtomicU64,
}

impl CgroupNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        CgroupNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            cgroups: AtomicU64::new(0),
        }
    }

    pub fn create_cgroup(&self) -> u64 {
        self.cgroups.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for CgroupNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Cgroup
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("Cgroup namespace {} (parent: {:?}, refcount: {}, cgroups: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.cgroups.load(Ordering::SeqCst))
    }
}

// ─── Mount Namespace Implementation ─────────────────────────────────────────────

/// Mount namespace for mount point isolation
#[derive(Debug)]
pub struct MountNamespace {
    id: NamespaceId,
    parent_id: Option<NamespaceId>,
    ref_count: AtomicU32,
    mount_points: AtomicU64,
}

impl MountNamespace {
    pub fn new(id: NamespaceId, parent_id: Option<NamespaceId>) -> Self {
        MountNamespace {
            id,
            parent_id,
            ref_count: AtomicU32::new(0),
            mount_points: AtomicU64::new(0),
        }
    }

    pub fn add_mount(&self) -> u64 {
        self.mount_points.fetch_add(1, Ordering::SeqCst)
    }
}

impl KernelNamespace for MountNamespace {
    fn namespace_id(&self) -> NamespaceId {
        self.id
    }

    fn namespace_type(&self) -> KernelNamespaceType {
        KernelNamespaceType::Mount
    }

    fn ref_count(&self) -> u32 {
        self.ref_count.load(Ordering::SeqCst)
    }

    fn increment_ref(&self) {
        self.ref_count.fetch_add(1, Ordering::SeqCst);
    }

    fn decrement_ref(&self) {
        self.ref_count.fetch_sub(1, Ordering::SeqCst);
    }

    fn metadata(&self) -> String {
        format!("Mount namespace {} (parent: {:?}, refcount: {}, mounts: {})", self.id.0, self.parent_id, self.ref_count.load(Ordering::SeqCst), self.mount_points.load(Ordering::SeqCst))
    }
}

// ─── Namespace Registry ─────────────────────────────────────────────────────

/// Central registry for managing all namespaces
pub struct NamespaceRegistry {
    namespaces: BTreeMap<NamespaceId, Box<dyn KernelNamespace>>,
    id_generator: NamespaceIdGenerator,
}

impl NamespaceRegistry {
    pub fn new() -> Self {
        NamespaceRegistry {
            namespaces: BTreeMap::new(),
            id_generator: NamespaceIdGenerator::new(),
        }
    }

    pub fn register_namespace(&mut self, namespace: Box<dyn KernelNamespace>) -> Result<(), NamespaceError> {
        let id = namespace.namespace_id();
        if self.namespaces.contains_key(&id) {
            return Err(NamespaceError::AlreadyInNamespace);
        }
        self.namespaces.insert(id, namespace);
        Ok(())
    }

    pub fn get_namespace(&self, id: NamespaceId) -> Option<&dyn KernelNamespace> {
        self.namespaces.get(&id).map(|ns| ns.as_ref())
    }

    pub fn unregister_namespace(&mut self, id: NamespaceId) -> Result<(), NamespaceError> {
        if self.namespaces.remove(&id).is_some() {
            Ok(())
        } else {
            Err(NamespaceError::InvalidNamespaceId)
        }
    }

    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }
}

impl Default for NamespaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_id_creation() {
        let id1 = NamespaceId::new(42);
        assert_eq!(id1.raw(), 42);

        let id2 = NamespaceId::new(42);
        assert_eq!(id1, id2);
    }

    #[test]
    fn test_namespace_id_generator() {
        let id1 = next_namespace_id();
        let id2 = next_namespace_id();

        assert!(id1.raw() < id2.raw());
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_namespace_type_as_str() {
        assert_eq!(KernelNamespaceType::Pid.as_str(), "pid");
        assert_eq!(KernelNamespaceType::Ipc.as_str(), "ipc");
        assert_eq!(KernelNamespaceType::Network.as_str(), "network");
    }

    #[test]
    fn test_namespace_config() {
        let config = NamespaceConfig::new(KernelNamespaceType::Pid);
        assert_eq!(config.namespace_type, KernelNamespaceType::Pid);
        assert_eq!(config.inherit_from, None);

        let parent_id = NamespaceId::new(1);
        let config = NamespaceConfig::inherit(KernelNamespaceType::Pid, parent_id);
        assert_eq!(config.inherit_from, Some(parent_id));
    }

    #[test]
    fn test_namespace_error_messages() {
        assert!(!NamespaceError::InvalidNamespaceId.message().is_empty());
        assert!(!NamespaceError::NamespaceFull.message().is_empty());
    }

    #[test]
    fn test_pid_namespace() {
        let id = NamespaceId::new(1);
        let pid_ns = PidNamespace::new(id, None);
        
        assert_eq!(pid_ns.namespace_id(), id);
        assert_eq!(pid_ns.namespace_type(), KernelNamespaceType::Pid);
        assert_eq!(pid_ns.ref_count(), 0);
        
        pid_ns.increment_ref();
        assert_eq!(pid_ns.ref_count(), 1);
        
        let pid1 = pid_ns.allocate_pid();
        let pid2 = pid_ns.allocate_pid();
        assert!(pid1 < pid2);
    }

    #[test]
    fn test_ipc_namespace() {
        let id = NamespaceId::new(2);
        let ipc_ns = IpcNamespace::new(id, None);
        
        assert_eq!(ipc_ns.namespace_type(), KernelNamespaceType::Ipc);
        
        ipc_ns.increment_ref();
        assert_eq!(ipc_ns.ref_count(), 1);
        
        let q1 = ipc_ns.allocate_queue();
        let q2 = ipc_ns.allocate_queue();
        assert!(q1 < q2);
    }

    #[test]
    fn test_network_namespace() {
        let id = NamespaceId::new(3);
        let net_ns = NetworkNamespace::new(id, None);
        
        assert_eq!(net_ns.namespace_type(), KernelNamespaceType::Network);
        
        let if1 = net_ns.allocate_interface();
        let if2 = net_ns.allocate_interface();
        assert!(if1 < if2);
    }

    #[test]
    fn test_uts_namespace() {
        let id = NamespaceId::new(4);
        let mut uts_ns = UtsNamespace::new(id, None, "sigmaos", "local");
        
        assert_eq!(uts_ns.namespace_type(), KernelNamespaceType::Uts);
        assert_eq!(uts_ns.get_hostname(), "sigmaos");
        assert_eq!(uts_ns.get_domainname(), "local");
        
        uts_ns.set_hostname("newhost");
        assert_eq!(uts_ns.get_hostname(), "newhost");
    }

    #[test]
    fn test_user_namespace() {
        let id = NamespaceId::new(5);
        let user_ns = UserNamespace::new(id, None);
        
        assert_eq!(user_ns.namespace_type(), KernelNamespaceType::User);
        
        let uid1 = user_ns.map_uid(1000);
        let uid2 = user_ns.map_uid(1001);
        assert!(uid1 < uid2);
    }

    #[test]
    fn test_cgroup_namespace() {
        let id = NamespaceId::new(6);
        let cgroup_ns = CgroupNamespace::new(id, None);
        
        assert_eq!(cgroup_ns.namespace_type(), KernelNamespaceType::Cgroup);
        
        let cg1 = cgroup_ns.create_cgroup();
        let cg2 = cgroup_ns.create_cgroup();
        assert!(cg1 < cg2);
    }

    #[test]
    fn test_mount_namespace() {
        let id = NamespaceId::new(7);
        let mount_ns = MountNamespace::new(id, None);
        
        assert_eq!(mount_ns.namespace_type(), KernelNamespaceType::Mount);
        
        let m1 = mount_ns.add_mount();
        let m2 = mount_ns.add_mount();
        assert!(m1 < m2);
    }

    #[test]
    fn test_namespace_registry() {
        let mut registry = NamespaceRegistry::new();
        
        let id1 = NamespaceId::new(1);
        let pid_ns = Box::new(PidNamespace::new(id1, None)) as Box<dyn KernelNamespace>;
        
        assert!(registry.register_namespace(pid_ns).is_ok());
        assert_eq!(registry.namespace_count(), 1);
        
        assert!(registry.get_namespace(id1).is_some());
        assert!(registry.unregister_namespace(id1).is_ok());
        assert_eq!(registry.namespace_count(), 0);
    }
}
