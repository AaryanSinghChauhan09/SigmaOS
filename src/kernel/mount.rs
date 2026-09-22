// Linux-inspired mount namespace and filesystem mounting
// Filesystem mount management for SigmaOS

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// mount flags (Linux mount.h)
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
        MountFlags {
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
}

impl Default for MountFlags {
    fn default() -> Self {
        Self::new()
    }
}

/// mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub source: String,
    pub target: String,
    pub filesystem_type: String,
    pub flags: MountFlags,
    pub options: String,
    pub mounted: bool,
}

impl MountPoint {
    pub fn new(source: String, target: String, filesystem_type: String, flags: MountFlags, options: String) -> Self {
        MountPoint {
            source,
            target,
            filesystem_type,
            flags,
            options,
            mounted: false,
        }
    }

    pub fn mount(&mut self) {
        self.mounted = true;
    }

    pub fn unmount(&mut self) {
        self.mounted = false;
    }
}

/// mount namespace
pub struct MountNamespace {
    mounts: HashMap<String, Arc<Mutex<MountPoint>>>,
    root: Option<Arc<Mutex<MountPoint>>>,
}

impl MountNamespace {
    pub fn new() -> Self {
        let mut namespace = MountNamespace {
            mounts: HashMap::new(),
            root: None,
        };

        // Create root mount
        let root_mount = Arc::new(Mutex::new(MountPoint::new(
            "none".to_string(),
            "/".to_string(),
            "rootfs".to_string(),
            MountFlags::new(),
            "".to_string(),
        )));
        root_mount.lock().unwrap().mount();
        namespace.root = Some(root_mount.clone());
        namespace.mounts.insert("/".to_string(), root_mount);

        namespace
    }

    /// Mount a filesystem
    pub fn mount(&mut self, source: String, target: String, filesystem_type: String, flags: MountFlags, options: String) -> Result<(), String> {
        if self.mounts.contains_key(&target) {
            return Err(format!("Mount point already exists: {}", target));
        }

        let mount_point = Arc::new(Mutex::new(MountPoint::new(
            source, target.clone(), filesystem_type, flags, options,
        )));
        mount_point.lock().unwrap().mount();

        self.mounts.insert(target, mount_point);
        Ok(())
    }

    /// Unmount a filesystem
    pub fn unmount(&mut self, target: &str) -> Result<(), String> {
        let mount_point = self.mounts.remove(target)
            .ok_or_else(|| format!("Mount point not found: {}", target))?;

        let mut mount_guard = mount_point.lock().unwrap();
        mount_guard.unmount();

        Ok(())
    }

    /// Get a mount point
    pub fn get_mount(&self, target: &str) -> Option<Arc<Mutex<MountPoint>>> {
        self.mounts.get(target).cloned()
    }

    /// List all mounts
    pub fn list_mounts(&self) -> Vec<Arc<Mutex<MountPoint>>> {
        self.mounts.values().cloned().collect()
    }

    /// Get mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }

    /// Get root mount
    pub fn get_root(&self) -> Option<Arc<Mutex<MountPoint>>> {
        self.root.clone()
    }
}

impl Default for MountNamespace {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_flags() {
        let flags = MountFlags::new();
        assert!(!flags.read_only);
        assert!(!flags.bind);
    }

    #[test]
    fn test_mount_point_creation() {
        let mount_point = MountPoint::new(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        );

        assert_eq!(mount_point.source, "/dev/sda1");
        assert_eq!(mount_point.target, "/mnt/data");
        assert!(!mount_point.mounted);
    }

    #[test]
    fn test_mount_point_mount_unmount() {
        let mut mount_point = MountPoint::new(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        );

        mount_point.mount();
        assert!(mount_point.mounted);

        mount_point.unmount();
        assert!(!mount_point.mounted);
    }

    #[test]
    fn test_mount_namespace_creation() {
        let namespace = MountNamespace::new();
        assert_eq!(namespace.mount_count(), 1);
        assert!(namespace.get_root().is_some());
    }

    #[test]
    fn test_mount_namespace_mount() {
        let mut namespace = MountNamespace::new();

        namespace.mount(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        ).unwrap();

        assert_eq!(namespace.mount_count(), 2);
    }

    #[test]
    fn test_mount_namespace_mount_duplicate() {
        let mut namespace = MountNamespace::new();

        namespace.mount(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        ).unwrap();

        let result = namespace.mount(
            "/dev/sda2".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_mount_namespace_unmount() {
        let mut namespace = MountNamespace::new();

        namespace.mount(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        ).unwrap();

        namespace.unmount("/mnt/data").unwrap();
        assert_eq!(namespace.mount_count(), 1);
    }

    #[test]
    fn test_mount_namespace_unmount_nonexistent() {
        let mut namespace = MountNamespace::new();

        let result = namespace.unmount("/mnt/data");
        assert!(result.is_err());
    }

    #[test]
    fn test_mount_namespace_get_mount() {
        let mut namespace = MountNamespace::new();

        namespace.mount(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        ).unwrap();

        let mount = namespace.get_mount("/mnt/data");
        assert!(mount.is_some());
    }

    #[test]
    fn test_mount_namespace_list_mounts() {
        let mut namespace = MountNamespace::new();

        namespace.mount(
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            "ext4".to_string(),
            MountFlags::new(),
            "".to_string(),
        ).unwrap();

        let mounts = namespace.list_mounts();
        assert_eq!(mounts.len(), 2);
    }
}
