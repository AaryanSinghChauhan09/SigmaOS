// Linux-inspired Mount Namespace
// Provides per-process mount namespace isolation

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mount flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MountFlags {
    pub read_only: bool,
    pub noexec: bool,
    pub nosuid: bool,
    pub nodev: bool,
    pub noatime: bool,
    pub nodiratime: bool,
    pub relatime: bool,
    pub bind: bool,
    pub remount: bool,
    pub move_mount: bool,
}

impl MountFlags {
    pub fn new() -> Self {
        Self {
            read_only: false,
            noexec: false,
            nosuid: false,
            nodev: false,
            noatime: false,
            nodiratime: false,
            relatime: false,
            bind: false,
            remount: false,
            move_mount: false,
        }
    }

    pub fn with_read_only(mut self, value: bool) -> Self {
        self.read_only = value;
        self
    }

    pub fn with_noexec(mut self, value: bool) -> Self {
        self.noexec = value;
        self
    }

    pub fn with_bind(mut self, value: bool) -> Self {
        self.bind = value;
        self
    }
}

impl Default for MountFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub source: String,
    pub target: String,
    pub filesystem_type: String,
    pub flags: MountFlags,
    pub options: String,
}

impl MountPoint {
    pub fn new(source: String, target: String, filesystem_type: String) -> Self {
        Self {
            source,
            target,
            filesystem_type,
            flags: MountFlags::new(),
            options: String::new(),
        }
    }

    pub fn with_flags(mut self, flags: MountFlags) -> Self {
        self.flags = flags;
        self
    }

    pub fn with_options(mut self, options: String) -> Self {
        self.options = options;
        self
    }
}

/// Mount namespace
#[derive(Debug, Clone)]
pub struct MountNamespace {
    pub id: u64,
    pub mounts: HashMap<String, MountPoint>,
    pub parent_id: Option<u64>,
}

impl MountNamespace {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            mounts: HashMap::new(),
            parent_id: None,
        }
    }

    pub fn with_parent(mut self, parent_id: u64) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    /// Add a mount point
    pub fn add_mount(&mut self, mount: MountPoint) -> Result<(), String> {
        if self.mounts.contains_key(&mount.target) {
            return Err(format!("Mount point {} already exists", mount.target));
        }
        self.mounts.insert(mount.target.clone(), mount);
        Ok(())
    }

    /// Remove a mount point
    pub fn remove_mount(&mut self, target: &str) -> Result<(), String> {
        match self.mounts.remove(target) {
            Some(_) => Ok(()),
            None => Err(format!("Mount point {} not found", target)),
        }
    }

    /// Get a mount point
    pub fn get_mount(&self, target: &str) -> Option<&MountPoint> {
        self.mounts.get(target)
    }

    /// List all mount points
    pub fn list_mounts(&self) -> Vec<&MountPoint> {
        self.mounts.values().collect()
    }

    /// Get mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }
}

/// Mount namespace manager for system-wide namespace management
pub struct MountNamespaceManager {
    namespaces: Arc<Mutex<HashMap<u64, MountNamespace>>>,
    next_namespace_id: Arc<Mutex<u64>>,
}

impl MountNamespaceManager {
    pub fn new() -> Self {
        Self {
            namespaces: Arc::new(Mutex::new(HashMap::new())),
            next_namespace_id: Arc::new(Mutex::new(1)),
        }
    }

    /// Create a new mount namespace
    pub fn create_namespace(&self, parent_id: Option<u64>) -> u64 {
        let mut next_id = self.next_namespace_id.lock().unwrap();
        let namespace_id = *next_id;
        *next_id += 1;
        drop(next_id);

        let namespace = match parent_id {
            Some(pid) => MountNamespace::new(namespace_id).with_parent(pid),
            None => MountNamespace::new(namespace_id),
        };

        let mut namespaces = self.namespaces.lock().unwrap();
        namespaces.insert(namespace_id, namespace);

        namespace_id
    }

    /// Get a namespace by ID
    pub fn get_namespace(&self, namespace_id: u64) -> Option<MountNamespace> {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.get(&namespace_id).cloned()
    }

    /// Remove a namespace
    pub fn remove_namespace(&self, namespace_id: u64) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.remove(&namespace_id) {
            Some(_) => Ok(()),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Add mount to a namespace
    pub fn add_mount(&self, namespace_id: u64, mount: MountPoint) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.add_mount(mount),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Remove mount from a namespace
    pub fn remove_mount(&self, namespace_id: u64, target: &str) -> Result<(), String> {
        let mut namespaces = self.namespaces.lock().unwrap();
        match namespaces.get_mut(&namespace_id) {
            Some(namespace) => namespace.remove_mount(target),
            None => Err(format!("Namespace {} not found", namespace_id)),
        }
    }

    /// Get namespace count
    pub fn namespace_count(&self) -> usize {
        let namespaces = self.namespaces.lock().unwrap();
        namespaces.len()
    }
}

impl Default for MountNamespaceManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_flags() {
        let flags = MountFlags::new()
            .with_read_only(true)
            .with_noexec(true);

        assert!(flags.read_only);
        assert!(flags.noexec);
        assert!(!flags.bind);
    }

    #[test]
    fn test_mount_point() {
        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string());
        assert_eq!(mount.source, "/dev/sda1");
        assert_eq!(mount.target, "/mnt/data");
        assert_eq!(mount.filesystem_type, "ext4");
    }

    #[test]
    fn test_mount_point_with_flags() {
        let flags = MountFlags::new().with_read_only(true);
        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string())
            .with_flags(flags);

        assert!(mount.flags.read_only);
    }

    #[test]
    fn test_mount_namespace() {
        let namespace = MountNamespace::new(1);
        assert_eq!(namespace.id, 1);
        assert_eq!(namespace.mount_count(), 0);
    }

    #[test]
    fn test_mount_namespace_with_parent() {
        let namespace = MountNamespace::new(1).with_parent(0);
        assert_eq!(namespace.parent_id, Some(0));
    }

    #[test]
    fn test_mount_namespace_add_mount() {
        let mut namespace = MountNamespace::new(1);

        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string());
        namespace.add_mount(mount).unwrap();

        assert_eq!(namespace.mount_count(), 1);
    }

    #[test]
    fn test_mount_namespace_duplicate_mount() {
        let mut namespace = MountNamespace::new(1);

        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string());
        namespace.add_mount(mount.clone()).unwrap();
        assert!(namespace.add_mount(mount).is_err());
    }

    #[test]
    fn test_mount_namespace_remove_mount() {
        let mut namespace = MountNamespace::new(1);

        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string());
        namespace.add_mount(mount).unwrap();

        namespace.remove_mount("/mnt/data").unwrap();
        assert_eq!(namespace.mount_count(), 0);
    }

    #[test]
    fn test_mount_namespace_manager() {
        let manager = MountNamespaceManager::new();

        let namespace_id = manager.create_namespace(None);
        assert_eq!(namespace_id, 1);

        let mount = MountPoint::new("/dev/sda1".to_string(), "/mnt/data".to_string(), "ext4".to_string());
        manager.add_mount(namespace_id, mount).unwrap();

        assert_eq!(manager.namespace_count(), 1);
    }

    #[test]
    fn test_mount_namespace_manager_with_parent() {
        let manager = MountNamespaceManager::new();

        let parent_id = manager.create_namespace(None);
        let child_id = manager.create_namespace(Some(parent_id));

        let namespace = manager.get_namespace(child_id).unwrap();
        assert_eq!(namespace.parent_id, Some(parent_id));
    }

    #[test]
    fn test_mount_namespace_manager_multiple_namespaces() {
        let manager = MountNamespaceManager::new();

        let _ns_id1 = manager.create_namespace(None);
        let _ns_id2 = manager.create_namespace(None);

        assert_eq!(manager.namespace_count(), 2);
    }
}
