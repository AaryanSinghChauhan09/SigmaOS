// Namespaces - User, Mount, PID Namespaces
// Inspired by Linux namespaces for process isolation

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Namespace type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NamespaceType {
    User,
    Mount,
    Pid,
    Network,
    Ipc,
    Uts,
    Cgroup,
}

/// Namespace ID
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NamespaceId(pub u64);

/// User namespace
#[derive(Debug, Clone)]
pub struct UserNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub uid_map: HashMap<u32, u32>, // uid mappings
    pub gid_map: HashMap<u32, u32>, // gid mappings
}

/// Mount namespace
#[derive(Debug, Clone)]
pub struct MountNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub mount_points: Vec<MountPoint>,
}

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub source: String,
    pub target: String,
    pub filesystem_type: String,
    pub flags: u32,
}

/// PID namespace
#[derive(Debug, Clone)]
pub struct PidNamespace {
    pub id: NamespaceId,
    pub parent: Option<NamespaceId>,
    pub last_pid: AtomicU64,
    pub processes: HashMap<u64, u64>, // global pid -> namespace pid mapping
}

/// Namespace
pub struct Namespace {
    pub id: NamespaceId,
    pub namespace_type: NamespaceType,
    pub user_namespace: Option<UserNamespace>,
    pub mount_namespace: Option<MountNamespace>,
    pub pid_namespace: Option<PidNamespace>,
}

impl Namespace {
    pub fn new(id: NamespaceId, namespace_type: NamespaceType) -> Self {
        Self {
            id,
            namespace_type,
            user_namespace: None,
            mount_namespace: None,
            pid_namespace: None,
        }
    }

    /// Set user namespace
    pub fn set_user_namespace(&mut self, user_ns: UserNamespace) {
        self.user_namespace = Some(user_ns);
    }

    /// Set mount namespace
    pub fn set_mount_namespace(&mut self, mount_ns: MountNamespace) {
        self.mount_namespace = Some(mount_ns);
    }

    /// Set PID namespace
    pub fn set_pid_namespace(&mut self, pid_ns: PidNamespace) {
        self.pid_namespace = Some(pid_ns);
    }
}

/// Namespace manager
pub struct NamespaceManager {
    namespaces: HashMap<NamespaceId, Namespace>,
    next_id: AtomicU64,
    initial_namespaces: HashMap<NamespaceType, NamespaceId>,
}

impl NamespaceManager {
    pub fn new() -> Self {
        let mut manager = Self {
            namespaces: HashMap::new(),
            next_id: AtomicU64::new(1),
            initial_namespaces: HashMap::new(),
        };

        // Create initial namespaces
        let user_id = NamespaceId(1);
        let mount_id = NamespaceId(2);
        let pid_id = NamespaceId(3);

        let user_ns = Namespace::new(user_id, NamespaceType::User);
        let mount_ns = Namespace::new(mount_id, NamespaceType::Mount);
        let pid_ns = Namespace::new(pid_id, NamespaceType::Pid);

        manager.namespaces.insert(user_id, user_ns);
        manager.namespaces.insert(mount_id, mount_ns);
        manager.namespaces.insert(pid_id, pid_ns);

        manager.initial_namespaces.insert(NamespaceType::User, user_id);
        manager.initial_namespaces.insert(NamespaceType::Mount, mount_id);
        manager.initial_namespaces.insert(NamespaceType::Pid, pid_id);

        manager
    }

    /// Create a new namespace
    pub fn create_namespace(&mut self, namespace_type: NamespaceType, parent_id: Option<NamespaceId>) -> NamespaceId {
        let id = NamespaceId(self.next_id.fetch_add(1, Ordering::SeqCst));

        let mut namespace = Namespace::new(id, namespace_type);

        match namespace_type {
            NamespaceType::User => {
                let user_ns = UserNamespace {
                    id,
                    parent: parent_id,
                    uid_map: HashMap::new(),
                    gid_map: HashMap::new(),
                };
                namespace.set_user_namespace(user_ns);
            }
            NamespaceType::Mount => {
                let mount_ns = MountNamespace {
                    id,
                    parent: parent_id,
                    mount_points: Vec::new(),
                };
                namespace.set_mount_namespace(mount_ns);
            }
            NamespaceType::Pid => {
                let pid_ns = PidNamespace {
                    id,
                    parent: parent_id,
                    last_pid: AtomicU64::new(0),
                    processes: HashMap::new(),
                };
                namespace.set_pid_namespace(pid_ns);
            }
            _ => {}
        }

        self.namespaces.insert(id, namespace);
        id
    }

    /// Delete a namespace
    pub fn delete_namespace(&mut self, id: NamespaceId) -> Result<(), &'static str> {
        // Check if it's an initial namespace
        for (&ns_type, &initial_id) in &self.initial_namespaces {
            if initial_id == id {
                return Err("Cannot delete initial namespace");
            }
        }

        if self.namespaces.remove(&id).is_some() {
            Ok(())
        } else {
            Err("Namespace not found")
        }
    }

    /// Get a namespace
    pub fn get_namespace(&self, id: NamespaceId) -> Option<&Namespace> {
        self.namespaces.get(&id)
    }

    /// Get mutable namespace
    pub fn get_namespace_mut(&mut self, id: NamespaceId) -> Option<&mut Namespace> {
        self.namespaces.get_mut(&id)
    }

    /// Get initial namespace by type
    pub fn get_initial_namespace(&self, namespace_type: NamespaceType) -> Option<NamespaceId> {
        self.initial_namespaces.get(&namespace_type).copied()
    }

    /// Add UID mapping to user namespace
    pub fn add_uid_mapping(&mut self, namespace_id: NamespaceId, inside_uid: u32, outside_uid: u32) -> Result<(), &'static str> {
        let namespace = self.namespaces.get_mut(&namespace_id).ok_or("Namespace not found")?;

        if let Some(ref mut user_ns) = namespace.user_namespace {
            user_ns.uid_map.insert(inside_uid, outside_uid);
            Ok(())
        } else {
            Err("Not a user namespace")
        }
    }

    /// Add GID mapping to user namespace
    pub fn add_gid_mapping(&mut self, namespace_id: NamespaceId, inside_gid: u32, outside_gid: u32) -> Result<(), &'static str> {
        let namespace = self.namespaces.get_mut(&namespace_id).ok_or("Namespace not found")?;

        if let Some(ref mut user_ns) = namespace.user_namespace {
            user_ns.gid_map.insert(inside_gid, outside_gid);
            Ok(())
        } else {
            Err("Not a user namespace")
        }
    }

    /// Add mount point to mount namespace
    pub fn add_mount_point(&mut self, namespace_id: NamespaceId, mount_point: MountPoint) -> Result<(), &'static str> {
        let namespace = self.namespaces.get_mut(&namespace_id).ok_or("Namespace not found")?;

        if let Some(ref mut mount_ns) = namespace.mount_namespace {
            mount_ns.mount_points.push(mount_point);
            Ok(())
        } else {
            Err("Not a mount namespace")
        }
    }

    /// Allocate PID in PID namespace
    pub fn allocate_pid(&mut self, namespace_id: NamespaceId, global_pid: u64) -> Result<u64, &'static str> {
        let namespace = self.namespaces.get_mut(&namespace_id).ok_or("Namespace not found")?;

        if let Some(ref mut pid_ns) = namespace.pid_namespace {
            let local_pid = pid_ns.last_pid.fetch_add(1, Ordering::SeqCst) + 1;
            pid_ns.processes.insert(global_pid, local_pid);
            Ok(local_pid)
        } else {
            Err("Not a PID namespace")
        }
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        self.namespaces.len()
    }

    /// List all namespaces
    pub fn list_namespaces(&self) -> Vec<NamespaceId> {
        self.namespaces.keys().copied().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_namespace() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::User, None);
        assert_eq!(id.0, 4);
        assert_eq!(manager.namespace_count(), 4);
    }

    #[test]
    fn test_delete_namespace() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::User, None);
        assert!(manager.delete_namespace(id).is_ok());
        assert_eq!(manager.namespace_count(), 3);
    }

    #[test]
    fn test_delete_initial() {
        let mut manager = NamespaceManager::new();

        let initial = manager.get_initial_namespace(NamespaceType::User).unwrap();
        assert!(manager.delete_namespace(initial).is_err());
    }

    #[test]
    fn test_add_uid_mapping() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::User, None);
        assert!(manager.add_uid_mapping(id, 1000, 0).is_ok());
    }

    #[test]
    fn test_add_mount_point() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::Mount, None);
        
        let mount_point = MountPoint {
            source: "/dev/sda1".to_string(),
            target: "/mnt/data".to_string(),
            filesystem_type: "ext4".to_string(),
            flags: 0,
        };

        assert!(manager.add_mount_point(id, mount_point).is_ok());
    }

    #[test]
    fn test_allocate_pid() {
        let mut manager = NamespaceManager::new();

        let id = manager.create_namespace(NamespaceType::Pid, None);
        let local_pid = manager.allocate_pid(id, 1234).unwrap();

        assert_eq!(local_pid, 1);
    }

    #[test]
    fn test_get_initial_namespace() {
        let manager = NamespaceManager::new();

        let user_id = manager.get_initial_namespace(NamespaceType::User);
        assert!(user_id.is_some());
        assert_eq!(user_id.unwrap().0, 1);
    }

    #[test]
    fn test_namespace_types() {
        let mut manager = NamespaceManager::new();

        let user_id = manager.create_namespace(NamespaceType::User, None);
        let mount_id = manager.create_namespace(NamespaceType::Mount, None);
        let pid_id = manager.create_namespace(NamespaceType::Pid, None);

        let user_ns = manager.get_namespace(user_id).unwrap();
        assert_eq!(user_ns.namespace_type, NamespaceType::User);

        let mount_ns = manager.get_namespace(mount_id).unwrap();
        assert_eq!(mount_ns.namespace_type, NamespaceType::Mount);

        let pid_ns = manager.get_namespace(pid_id).unwrap();
        assert_eq!(pid_ns.namespace_type, NamespaceType::Pid);
    }
}
