// Namespaces - Linux-style process isolation
// Supports mount, UTS, IPC, network, PID, user, and cgroup namespaces

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NamespaceType {
    Mount,   // Filesystem mount points
    Uts,     // Hostname and domain name
    Ipc,     // Inter-process communication
    Network, // Network interfaces and routing
    Pid,     // Process IDs
    User,    // User and group IDs
    Cgroup,  // Cgroup root directory
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub id: u64,
    pub ns_type: NamespaceType,
    pub parent_id: Option<u64>,
    pub data: NamespaceData,
}

#[derive(Debug, Clone)]
pub enum NamespaceData {
    Mount {
        root: String,
        mounts: Vec<String>,
    },
    Uts {
        hostname: String,
        domainname: String,
    },
    Ipc {
        message_queues: Vec<u64>,
        semaphores: Vec<u64>,
        shared_memory: Vec<u64>,
    },
    Network {
        interfaces: Vec<String>,
        routing_table: Vec<String>,
    },
    Pid {
        pid_map: BTreeMap<u32, u32>, // Global to local PID mapping
    },
    User {
        uid_map: BTreeMap<u32, u32>,
        gid_map: BTreeMap<u32, u32>,
    },
    Cgroup {
        root: String,
        subsystems: Vec<String>,
    },
}

pub struct NamespaceManager {
    namespaces: BTreeMap<u64, Namespace>,
    next_id: u64,
    init_ns: Option<u64>,
}

impl NamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: BTreeMap::new(),
            next_id: 1,
            init_ns: None,
        }
    }

    /// Create a new namespace
    pub fn create_namespace(
        &mut self,
        ns_type: NamespaceType,
        parent_id: Option<u64>,
    ) -> Result<u64, &'static str> {
        let id = self.next_id;
        self.next_id += 1;

        let data = match ns_type {
            NamespaceType::Mount => NamespaceData::Mount {
                root: "/".to_string(),
                mounts: Vec::new(),
            },
            NamespaceType::Uts => NamespaceData::Uts {
                hostname: "sigmaos".to_string(),
                domainname: "localdomain".to_string(),
            },
            NamespaceType::Ipc => NamespaceData::Ipc {
                message_queues: Vec::new(),
                semaphores: Vec::new(),
                shared_memory: Vec::new(),
            },
            NamespaceType::Network => NamespaceData::Network {
                interfaces: Vec::new(),
                routing_table: Vec::new(),
            },
            NamespaceType::Pid => NamespaceData::Pid {
                pid_map: BTreeMap::new(),
            },
            NamespaceType::User => NamespaceData::User {
                uid_map: BTreeMap::new(),
                gid_map: BTreeMap::new(),
            },
            NamespaceType::Cgroup => NamespaceData::Cgroup {
                root: "/".to_string(),
                subsystems: Vec::new(),
            },
        };

        let namespace = Namespace {
            id,
            ns_type,
            parent_id,
            data,
        };

        self.namespaces.insert(id, namespace);

        // Set as init namespace if first of its type
        if self.init_ns.is_none() {
            self.init_ns = Some(id);
        }

        Ok(id)
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, id: u64) -> Option<&Namespace> {
        self.namespaces.get(&id)
    }

    /// Get a mutable namespace by ID
    pub fn get_namespace_mut(&mut self, id: u64) -> Option<&mut Namespace> {
        self.namespaces.get_mut(&id)
    }

    /// Clone a namespace
    pub fn clone_namespace(&mut self, id: u64) -> Result<u64, &'static str> {
        let source = self.namespaces.get(&id).ok_or("Namespace not found")?;

        let new_id = self.next_id;
        self.next_id += 1;

        let namespace = Namespace {
            id: new_id,
            ns_type: source.ns_type,
            parent_id: Some(id),
            data: source.data.clone(),
        };

        self.namespaces.insert(new_id, namespace);
        Ok(new_id)
    }

    /// Delete a namespace
    pub fn delete_namespace(&mut self, id: u64) -> Result<(), &'static str> {
        if self.init_ns == Some(id) {
            return Err("Cannot delete init namespace");
        }

        self.namespaces.remove(&id).ok_or("Namespace not found")?;

        Ok(())
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    /// Get init namespace ID
    pub fn init_namespace(&self) -> Option<u64> {
        self.init_ns
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
    fn test_create_namespace() {
        let mut manager = NamespaceManager::new();

        let id = manager
            .create_namespace(NamespaceType::Mount, None)
            .unwrap();
        assert_eq!(manager.namespace_count(), 1);
        assert_eq!(manager.init_namespace(), Some(id));
    }

    #[test]
    fn test_clone_namespace() {
        let mut manager = NamespaceManager::new();

        let parent_id = manager.create_namespace(NamespaceType::Uts, None).unwrap();
        let child_id = manager.clone_namespace(parent_id).unwrap();

        assert_eq!(manager.namespace_count(), 2);

        let parent = manager.get_namespace(parent_id).unwrap();
        let child = manager.get_namespace(child_id).unwrap();
        assert_eq!(child.parent_id, Some(parent_id));
    }

    #[test]
    fn test_delete_namespace() {
        let mut manager = NamespaceManager::new();

        let _init_id = manager
            .create_namespace(NamespaceType::Pid, None)
            .unwrap();
        let id = manager
            .create_namespace(NamespaceType::Network, None)
            .unwrap();
        manager.delete_namespace(id).unwrap();

        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_delete_init_namespace() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::Pid, None).unwrap();
        let result = manager.delete_namespace(id);

        assert!(result.is_err());
    }

    #[test]
    fn test_namespace_data() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::Uts, None).unwrap();
        let ns = manager.get_namespace(id).unwrap();

        if let NamespaceData::Uts { hostname, .. } = &ns.data {
            assert_eq!(hostname, "sigmaos");
        } else {
            panic!("Expected Uts namespace data");
        }
    }

    #[test]
    fn test_multiple_namespace_types() {
        let mut manager = NamespaceManager::new();

        manager
            .create_namespace(NamespaceType::Mount, None)
            .unwrap();
        manager
            .create_namespace(NamespaceType::Network, None)
            .unwrap();
        manager.create_namespace(NamespaceType::Pid, None).unwrap();

        assert_eq!(manager.namespace_count(), 3);
    }
}
