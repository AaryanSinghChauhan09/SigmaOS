// Linux-inspired Process Namespaces for SigmaOS
// Process isolation and containerization support

#![no_std]

extern crate alloc;

use alloc::collections::HashMap;
use alloc::sync::Arc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::format;
use core::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

/// Namespace types (Linux namespace.h)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    Mount = 0x00000001,
    Uts = 0x04000000,
    Ipc = 0x08000000,
    Network = 0x40000000,
    User = 0x10000000,
    Pid = 0x20000000,
    Cgroup = 0x02000000,
    Time = 0x00000080,
}

impl NamespaceType {
    pub fn from_bits(bits: u32) -> Vec<Self> {
        let mut types = Vec::new();

        if bits & (NamespaceType::Mount as u32) != 0 {
            types.push(NamespaceType::Mount);
        }
        if bits & (NamespaceType::Uts as u32) != 0 {
            types.push(NamespaceType::Uts);
        }
        if bits & (NamespaceType::Ipc as u32) != 0 {
            types.push(NamespaceType::Ipc);
        }
        if bits & (NamespaceType::Network as u32) != 0 {
            types.push(NamespaceType::Network);
        }
        if bits & (NamespaceType::User as u32) != 0 {
            types.push(NamespaceType::User);
        }
        if bits & (NamespaceType::Pid as u32) != 0 {
            types.push(NamespaceType::Pid);
        }
        if bits & (NamespaceType::Cgroup as u32) != 0 {
            types.push(NamespaceType::Cgroup);
        }
        if bits & (NamespaceType::Time as u32) != 0 {
            types.push(NamespaceType::Time);
        }

        types
    }
}

/// Namespace identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NamespaceId {
    pub ns_type: NamespaceType,
    pub inode: u64,
}

impl NamespaceId {
    pub fn new(ns_type: NamespaceType, inode: u64) -> Self {
        NamespaceId { ns_type, inode }
    }
}

/// Namespace instance
#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: NamespaceId,
    pub parent: Option<Arc<Mutex<Namespace>>>,
    pub processes: Vec<u32>,
}

impl Namespace {
    pub fn new(ns_type: NamespaceType, inode: u64) -> Self {
        Namespace {
            id: NamespaceId::new(ns_type, inode),
            parent: None,
            processes: Vec::new(),
        }
    }

    pub fn with_parent(ns_type: NamespaceType, inode: u64, parent: Arc<Mutex<Namespace>>) -> Self {
        Namespace {
            id: NamespaceId::new(ns_type, inode),
            parent: Some(parent),
            processes: Vec::new(),
        }
    }

    pub fn add_process(&mut self, pid: u32) {
        self.processes.push(pid);
    }

    pub fn remove_process(&mut self, pid: u32) {
        self.processes.retain(|&p| p != pid);
    }

    pub fn process_count(&self) -> usize {
        self.processes.len()
    }
}

/// Namespace manager
pub struct NamespaceManager {
    namespaces: HashMap<NamespaceId, Arc<Mutex<Namespace>>>,
    next_inode: AtomicU64,
}

impl NamespaceManager {
    pub fn new() -> Self {
        NamespaceManager {
            namespaces: HashMap::new(),
            next_inode: AtomicU64::new(1),
        }
    }

    /// Create a new namespace
    pub fn create_namespace(&mut self, ns_type: NamespaceType, parent: Option<Arc<Mutex<Namespace>>>) -> Arc<Mutex<Namespace>> {
        let inode = self.next_inode.fetch_add(1, Ordering::SeqCst);

        let namespace = match parent {
            Some(p) => Namespace::with_parent(ns_type, inode, p),
            None => Namespace::new(ns_type, inode),
        };

        let ns = Arc::new(spin::Mutex::new(namespace));
        let id = ns.lock().id.clone();
        self.namespaces.insert(id, ns.clone());
        ns
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, id: &NamespaceId) -> Option<Arc<Mutex<Namespace>>> {
        self.namespaces.get(id).cloned()
    }

    /// Remove a namespace
    pub fn remove_namespace(&mut self, id: &NamespaceId) -> Result<(), String> {
        let ns = self.namespaces.get(id)
            .ok_or_else(|| format!("Namespace not found: {:?}", id))?;

        let ns_guard = ns.lock();
        if ns_guard.process_count() > 0 {
            return Err(format!("Namespace has {} processes, cannot remove", ns_guard.process_count()));
        }

        self.namespaces.remove(id);
        Ok(())
    }

    /// Get all namespaces of a specific type
    pub fn get_namespaces_by_type(&self, ns_type: NamespaceType) -> Vec<Arc<Mutex<Namespace>>> {
        self.namespaces.values()
            .filter(|ns| ns.lock().id.ns_type == ns_type)
            .cloned()
            .collect()
    }

    /// Get total namespace count
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }
}

impl Default for NamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace_type_from_bits() {
        let bits = (NamespaceType::Mount as u32) | (NamespaceType::Network as u32);
        let types = NamespaceType::from_bits(bits);

        assert_eq!(types.len(), 2);
        assert!(types.contains(&NamespaceType::Mount));
        assert!(types.contains(&NamespaceType::Network));
    }

    #[test]
    fn test_namespace_id() {
        let id = NamespaceId::new(NamespaceType::Pid, 100);
        assert_eq!(id.ns_type, NamespaceType::Pid);
        assert_eq!(id.inode, 100);
    }

    #[test]
    fn test_namespace_creation() {
        let ns = Namespace::new(NamespaceType::Mount, 1);
        assert_eq!(ns.id.ns_type, NamespaceType::Mount);
        assert_eq!(ns.id.inode, 1);
        assert!(ns.parent.is_none());
    }

    #[test]
    fn test_namespace_with_parent() {
        let parent = Arc::new(spin::Mutex::new(Namespace::new(NamespaceType::Mount, 1)));
        let child = Namespace::with_parent(NamespaceType::Mount, 2, parent.clone());
        
        assert!(child.parent.is_some());
    }

    #[test]
    fn test_namespace_add_remove_process() {
        let mut ns = Namespace::new(NamespaceType::Pid, 1);
        
        ns.add_process(100);
        assert_eq!(ns.process_count(), 1);
        
        ns.remove_process(100);
        assert_eq!(ns.process_count(), 0);
    }

    #[test]
    fn test_namespace_manager_create() {
        let mut manager = NamespaceManager::new();
        
        let ns = manager.create_namespace(NamespaceType::Mount, None);
        assert_eq!(manager.namespace_count(), 1);
        
        let ns_guard = ns.lock();
        assert_eq!(ns_guard.id.inode, 1);
    }

    #[test]
    fn test_namespace_manager_with_parent() {
        let mut manager = NamespaceManager::new();
        
        let parent = manager.create_namespace(NamespaceType::Mount, None);
        let child = manager.create_namespace(NamespaceType::Mount, Some(parent));
        
        assert_eq!(manager.namespace_count(), 2);
    }

    #[test]
    fn test_namespace_manager_remove() {
        let mut manager = NamespaceManager::new();
        
        let ns = manager.create_namespace(NamespaceType::Mount, None);
        let id = ns.lock().id.clone();
        
        manager.remove_namespace(&id).unwrap();
        assert_eq!(manager.namespace_count(), 0);
    }

    #[test]
    fn test_namespace_manager_remove_with_processes() {
        let mut manager = NamespaceManager::new();
        
        let ns = manager.create_namespace(NamespaceType::Mount, None);
        {
            let mut ns_guard = ns.lock();
            ns_guard.add_process(100);
        }
        
        let id = ns.lock().id.clone();
        let result = manager.remove_namespace(&id);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_namespace_manager_get_by_type() {
        let mut manager = NamespaceManager::new();
        
        manager.create_namespace(NamespaceType::Mount, None);
        manager.create_namespace(NamespaceType::Network, None);
        manager.create_namespace(NamespaceType::Mount, None);
        
        let mount_ns = manager.get_namespaces_by_type(NamespaceType::Mount);
        assert_eq!(mount_ns.len(), 2);
    }
}
