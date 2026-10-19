//! SigmaNamespaces - Namespace Management System
//!
//! Provides comprehensive namespace management inspired by Linux namespaces.
//! Supports process isolation through mount, network, PID, user, UTS, IPC, and cgroup namespaces.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Namespace type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    Mount,
    Network,
    PID,
    User,
    UTS,
    IPC,
    Cgroup,
    Unknown,
;

/// Namespace state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceState {
    Active,
    Inactive,
    Destroyed,
    Unknown,
;

/// Namespace
#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: String,
    pub ns_type: NamespaceType,
    pub state: NamespaceState,
    pub processes: Vec<u32>,
    pub inode: Option<u64>,
    pub created_at: u64,
;

impl Namespace {
    pub fn new(id: String, ns_type: NamespaceType) -> Self {
        Self {
            id,
            ns_type,
            state: NamespaceState::Active,
            processes: Vec::new(),
            inode: None,
            created_at: 0,
        }
    }

    pub fn with_inode(mut self, inode: u64) -> Self {
        self.inode = Some(inode);
        self
    }

    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
    }

    pub fn remove_process(&mut self, pid: u32) -> bool {
        if let Some(pos) = self.processes.iter().position(|&p| p == pid) {
            self.processes.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn activate(&mut self) {
        self.state = NamespaceState::Active;
    }

    pub fn deactivate(&mut self) {
        self.state = NamespaceState::Inactive;
    }

    pub fn destroy(&mut self) {
        self.state = NamespaceState::Destroyed;
    }
;

/// Namespace manager
pub struct SigmaNamespaces {
    namespaces: BTreeMap<String, Namespace>,
    next_namespace_id: u64,
;

impl SigmaNamespaces {
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            next_namespace_id: 1,
        }
    }

    pub fn create_namespace(&mut self, ns_type: NamespaceType) -> String {
        let id = format!("ns-{}", self.next_namespace_id);
        self.next_namespace_id += 1;
        let namespace = Namespace::new(id.clone(), ns_type);
        self.namespaces.insert(id.clone(), namespace);
        id
    }

    pub fn get_namespace(&self, id: &str) -> Option<&Namespace> {
        self.namespaces.get(id)
    }

    pub fn get_namespace_mut(&mut self, id: &str) -> Option<&mut Namespace> {
        self.namespaces.get_mut(id)
    }

    pub fn list_namespaces(&self) -> Vec<&Namespace> {
        self.namespaces.values().collect()
    }

    pub fn list_namespaces_by_type(&self, ns_type: NamespaceType) -> Vec<&Namespace> {
        self.namespaces
            .values()
            .filter(|n| n.ns_type == ns_type)
            .collect()
    }

    pub fn list_namespaces_by_state(&self, state: NamespaceState) -> Vec<&Namespace> {
        self.namespaces
            .values()
            .filter(|n| n.state == state)
            .collect()
    }

    pub fn remove_namespace(&mut self, id: &str) -> bool {
        self.namespaces.remove(id).is_some()
    }

    pub fn add_process_to_namespace(&mut self, ns_id: &str, pid: u32) -> bool {
        if let Some(namespace) = self.namespaces.get_mut(ns_id) {
            namespace.add_process(pid);
            true
        } else {
            false
        }
    }

    pub fn remove_process_from_namespace(&mut self, ns_id: &str, pid: u32) -> bool {
        if let Some(namespace) = self.namespaces.get_mut(ns_id) {
            namespace.remove_process(pid)
        } else {
            false
        }
    }

    pub fn activate_namespace(&mut self, id: &str) -> bool {
        if let Some(namespace) = self.namespaces.get_mut(id) {
            namespace.activate();
            true
        } else {
            false
        }
    }

    pub fn deactivate_namespace(&mut self, id: &str) -> bool {
        if let Some(namespace) = self.namespaces.get_mut(id) {
            namespace.deactivate();
            true
        } else {
            false
        }
    }

    pub fn destroy_namespace(&mut self, id: &str) -> bool {
        if let Some(namespace) = self.namespaces.get_mut(id) {
            namespace.destroy();
            true
        } else {
            false
        }
    }

    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    pub fn active_namespace_count(&self) -> usize {
        self.namespaces
            .values()
            .filter(|n| n.state == NamespaceState::Active)
            .count()
    }

    pub fn get_namespaces_for_process(&self, pid: u32) -> Vec<&Namespace> {
        self.namespaces
            .values()
            .filter(|n| n.processes.contains(&pid))
            .collect()
    }
;

impl Default for SigmaNamespaces {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMANAMESPACES_INSTANCE: Option<SigmaNamespaces> = None;

pub fn init_signamespaces() {
    unsafe {
        SIGMANAMESPACES_INSTANCE = Some(SigmaNamespaces::new());
    }
;

pub fn get_signamespaces() -> &'static SigmaNamespaces {
    unsafe {
        SIGMANAMESPACES_INSTANCE
            .as_ref()
            .expect("SigmaNamespaces not initialized")
    }
;

pub fn get_signamespaces_mut() -> &'static mut SigmaNamespaces {
    unsafe {
        SIGMANAMESPACES_INSTANCE
            .as_mut()
            .expect("SigmaNamespaces not initialized")
    }
;

pub fn create_namespace(ns_type: NamespaceType) -> String {
    let manager = get_signamespaces_mut();
    manager.create_namespace(ns_type)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_creation() {
        let namespace = Namespace::new("ns-1".to_string(), NamespaceType::Network);
        assert_eq!(namespace.id, "ns-1");
        assert_eq!(namespace.ns_type, NamespaceType::Network);
        assert_eq!(namespace.state, NamespaceState::Active);
    }

    #[test]
    fn test_namespace_builder() {
        let namespace = Namespace::new("ns-1".to_string(), NamespaceType::Mount).with_inode(12345);

        assert_eq!(namespace.inode, Some(12345));
    }

    #[test]
    fn test_namespace_process_management() {
        let mut namespace = Namespace::new("ns-1".to_string(), NamespaceType::PID);
        namespace.add_process(1234);
        namespace.add_process(5678);

        assert_eq!(namespace.processes.len(), 2);
        assert!(namespace.remove_process(1234));
        assert_eq!(namespace.processes.len(), 1);
        assert!(!namespace.remove_process(9999));
    }

    #[test]
    fn test_namespace_state_transitions() {
        let mut namespace = Namespace::new("ns-1".to_string(), NamespaceType::Network);
        assert_eq!(namespace.state, NamespaceState::Active);

        namespace.deactivate();
        assert_eq!(namespace.state, NamespaceState::Inactive);

        namespace.activate();
        assert_eq!(namespace.state, NamespaceState::Active);

        namespace.destroy();
        assert_eq!(namespace.state, NamespaceState::Destroyed);
    }

    #[test]
    fn test_namespace_manager() {
        let manager = SigmaNamespaces::new();
        assert_eq!(manager.namespace_count(), 0);
    }

    #[test]
    fn test_create_namespace() {
        let mut manager = SigmaNamespaces::new();
        let id = manager.create_namespace(NamespaceType::Network);
        assert_eq!(id, "ns-1");
        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_add_remove_process_from_namespace() {
        let mut manager = SigmaNamespaces::new();
        let id = manager.create_namespace(NamespaceType::PID);

        assert!(manager.add_process_to_namespace(&id, 1234));
        assert_eq!(manager.get_namespace(&id).unwrap().processes.len(), 1);

        assert!(manager.remove_process_from_namespace(&id, 1234));
        assert_eq!(manager.get_namespace(&id).unwrap().processes.len(), 0);
    }

    #[test]
    fn test_list_namespaces_by_type() {
        let mut manager = SigmaNamespaces::new();
        manager.create_namespace(NamespaceType::Network);
        manager.create_namespace(NamespaceType::Network);
        manager.create_namespace(NamespaceType::Mount);

        let network_ns = manager.list_namespaces_by_type(NamespaceType::Network);
        assert_eq!(network_ns.len(), 2);

        let mount_ns = manager.list_namespaces_by_type(NamespaceType::Mount);
        assert_eq!(mount_ns.len(), 1);
    }

    #[test]
    fn test_list_namespaces_by_state() {
        let mut manager = SigmaNamespaces::new();
        let id1 = manager.create_namespace(NamespaceType::Network);
        let id2 = manager.create_namespace(NamespaceType::Mount);
        manager.deactivate_namespace(&id2);

        let active = manager.list_namespaces_by_state(NamespaceState::Active);
        assert_eq!(active.len(), 1);

        let inactive = manager.list_namespaces_by_state(NamespaceState::Inactive);
        assert_eq!(inactive.len(), 1);
    }

    #[test]
    fn test_activate_deactivate_destroy_namespace() {
        let mut manager = SigmaNamespaces::new();
        let id = manager.create_namespace(NamespaceType::Network);

        assert!(manager.deactivate_namespace(&id));
        assert_eq!(
            manager.get_namespace(&id).unwrap().state,
            NamespaceState::Inactive
        );

        assert!(manager.activate_namespace(&id));
        assert_eq!(
            manager.get_namespace(&id).unwrap().state,
            NamespaceState::Active
        );

        assert!(manager.destroy_namespace(&id));
        assert_eq!(
            manager.get_namespace(&id).unwrap().state,
            NamespaceState::Destroyed
        );
    }

    #[test]
    fn test_get_namespaces_for_process() {
        let mut manager = SigmaNamespaces::new();
        let id1 = manager.create_namespace(NamespaceType::Network);
        let id2 = manager.create_namespace(NamespaceType::Mount);
        let id3 = manager.create_namespace(NamespaceType::PID);

        manager.add_process_to_namespace(&id1, 1234);
        manager.add_process_to_namespace(&id2, 1234);
        manager.add_process_to_namespace(&id3, 5678);

        let ns_for_1234 = manager.get_namespaces_for_process(1234);
        assert_eq!(ns_for_1234.len(), 2);

        let ns_for_5678 = manager.get_namespaces_for_process(5678);
        assert_eq!(ns_for_5678.len(), 1);
    }

    #[test]
    fn test_active_namespace_count() {
        let mut manager = SigmaNamespaces::new();
        let id1 = manager.create_namespace(NamespaceType::Network);
        let id2 = manager.create_namespace(NamespaceType::Mount);
        manager.deactivate_namespace(&id2);

        assert_eq!(manager.active_namespace_count(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_signamespaces();
        let manager = get_signamespaces();
        assert_eq!(manager.namespace_count(), 0);

        let manager_mut = get_signamespaces_mut();
        manager_mut.create_namespace(NamespaceType::Network);
        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_create_namespace_convenience() {
        init_signamespaces();
        let id = create_namespace(NamespaceType::Network);
        assert_eq!(id, "ns-1");

        let manager = get_signamespaces();
        assert!(manager.get_namespace(&id).is_some());
    }
;
