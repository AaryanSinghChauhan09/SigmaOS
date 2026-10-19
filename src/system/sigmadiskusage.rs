//! SigmaDiskUsage - Disk Usage Analysis System
//!
//! Provides comprehensive disk usage analysis inspired by Linux df and du utilities.
//! Supports filesystem monitoring, space calculation, and usage reporting.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Filesystem entry
#[derive(Debug, Clone)]
pub struct Filesystem {
    pub id: String,
    pub mount_point: String,
    pub device: String,
    pub total_size: u64,
    pub used_size: u64,
    pub available_size: u64,
}

impl Filesystem {
    pub fn new(id: String, mount_point: String, device: String, total_size: u64) -> Self {
        Self {
            id,
            mount_point,
            device,
            total_size,
            used_size: 0,
            available_size: total_size,
        }
    }

    pub fn with_usage(mut self, used_size: u64) -> Self {
        self.used_size = used_size;
        self.available_size = self.total_size.saturating_sub(used_size);
        self
    }

    pub fn usage_percentage(&self) -> u32 {
        if self.total_size == 0 {
            return 0;
        }
        ((self.used_size as u64) * 100 / self.total_size) as u32
    }
}

/// Directory entry
#[derive(Debug, Clone)]
pub struct Directory {
    pub id: String,
    pub path: String,
    pub size: u64,
    pub file_count: u32,
    pub directory_count: u32,
}

impl Directory {
    pub fn new(id: String, path: String) -> Self {
        Self {
            id,
            path,
            size: 0,
            file_count: 0,
            directory_count: 0,
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_counts(mut self, file_count: u32, directory_count: u32) -> Self {
        self.file_count = file_count;
        self.directory_count = directory_count;
        self
    }
}

/// Disk usage manager
pub struct SigmaDiskUsage {
    filesystems: BTreeMap<String, Filesystem>,
    directories: BTreeMap<String, Directory>,
    next_filesystem_id: u64,
    next_directory_id: u64,
}

impl SigmaDiskUsage {
    pub fn new() -> Self {
        Self {
            filesystems: BTreeMap::new(),
            directories: BTreeMap::new(),
            next_filesystem_id: 1,
            next_directory_id: 1,
        }
    }

    pub fn add_filesystem(
        &mut self,
        mount_point: String,
        device: String,
        total_size: u64,
    ) -> String {
        let id = format!("fs-{}", self.next_filesystem_id);
        self.next_filesystem_id += 1;
        let fs = Filesystem::new(id.clone(), mount_point, device, total_size);
        self.filesystems.insert(id.clone(), fs);
        id
    }

    pub fn get_filesystem(&self, id: &str) -> Option<&Filesystem> {
        self.filesystems.get(id)
    }

    pub fn get_filesystem_mut(&mut self, id: &str) -> Option<&mut Filesystem> {
        self.filesystems.get_mut(id)
    }

    pub fn list_filesystems(&self) -> Vec<&Filesystem> {
        self.filesystems.values().collect()
    }

    pub fn remove_filesystem(&mut self, id: &str) -> bool {
        self.filesystems.remove(id).is_some()
    }

    pub fn add_directory(&mut self, path: String) -> String {
        let id = format!("dir-{}", self.next_directory_id);
        self.next_directory_id += 1;
        let dir = Directory::new(id.clone(), path);
        self.directories.insert(id.clone(), dir);
        id
    }

    pub fn get_directory(&self, id: &str) -> Option<&Directory> {
        self.directories.get(id)
    }

    pub fn get_directory_mut(&mut self, id: &str) -> Option<&mut Directory> {
        self.directories.get_mut(id)
    }

    pub fn list_directories(&self) -> Vec<&Directory> {
        self.directories.values().collect()
    }

    pub fn remove_directory(&mut self, id: &str) -> bool {
        self.directories.remove(id).is_some()
    }

    pub fn filesystem_count(&self) -> usize {
        self.filesystems.len()
    }

    pub fn directory_count(&self) -> usize {
        self.directories.len()
    }

    pub fn total_used_space(&self) -> u64 {
        self.filesystems.values().map(|fs| fs.used_size).sum()
    }

    pub fn total_available_space(&self) -> u64 {
        self.filesystems.values().map(|fs| fs.available_size).sum()
    }
}

impl Default for SigmaDiskUsage {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMADISKUSAGE_INSTANCE: Option<SigmaDiskUsage> = None;

pub fn init_sigmadiskusage() {
    unsafe {
        SIGMADISKUSAGE_INSTANCE = Some(SigmaDiskUsage::new());
    }
}

pub fn get_sigmadiskusage() -> &'static SigmaDiskUsage {
    unsafe {
        SIGMADISKUSAGE_INSTANCE
            .as_ref()
            .expect("SigmaDiskUsage not initialized")
    }
}

pub fn get_sigmadiskusage_mut() -> &'static mut SigmaDiskUsage {
    unsafe {
        SIGMADISKUSAGE_INSTANCE
            .as_mut()
            .expect("SigmaDiskUsage not initialized")
    }
}

pub fn add_filesystem(mount_point: String, device: String, total_size: u64) -> String {
    let manager = get_sigmadiskusage_mut();
    manager.add_filesystem(mount_point, device, total_size)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_creation() {
        let fs = Filesystem::new(
            "fs-1".to_string(),
            "/".to_string(),
            "/dev/sda1".to_string(),
            1000000000000,
        );
        assert_eq!(fs.id, "fs-1");
        assert_eq!(fs.mount_point, "/");
        assert_eq!(fs.total_size, 1000000000000);
        assert_eq!(fs.used_size, 0);
    }

    #[test]
    fn test_filesystem_with_usage() {
        let fs = Filesystem::new(
            "fs-1".to_string(),
            "/".to_string(),
            "/dev/sda1".to_string(),
            1000000000000,
        )
        .with_usage(500000000000);
        assert_eq!(fs.used_size, 500000000000);
        assert_eq!(fs.available_size, 500000000000);
    }

    #[test]
    fn test_usage_percentage() {
        let fs = Filesystem::new(
            "fs-1".to_string(),
            "/".to_string(),
            "/dev/sda1".to_string(),
            1000000000000,
        )
        .with_usage(500000000000);
        assert_eq!(fs.usage_percentage(), 50);
    }

    #[test]
    fn test_directory_creation() {
        let dir = Directory::new("dir-1".to_string(), "/home/user".to_string());
        assert_eq!(dir.id, "dir-1");
        assert_eq!(dir.path, "/home/user");
        assert_eq!(dir.size, 0);
    }

    #[test]
    fn test_directory_with_size() {
        let dir = Directory::new("dir-1".to_string(), "/home/user".to_string())
            .with_size(1000000)
            .with_counts(100, 10);
        assert_eq!(dir.size, 1000000);
        assert_eq!(dir.file_count, 100);
        assert_eq!(dir.directory_count, 10);
    }

    #[test]
    fn test_disk_usage_manager() {
        let manager = SigmaDiskUsage::new();
        assert_eq!(manager.filesystem_count(), 0);
        assert_eq!(manager.directory_count(), 0);
    }

    #[test]
    fn test_add_filesystem() {
        let mut manager = SigmaDiskUsage::new();
        let id = manager.add_filesystem("/".to_string(), "/dev/sda1".to_string(), 1000000000000);
        assert_eq!(id, "fs-1");
        assert_eq!(manager.filesystem_count(), 1);
    }

    #[test]
    fn test_add_directory() {
        let mut manager = SigmaDiskUsage::new();
        let id = manager.add_directory("/home/user".to_string());
        assert_eq!(id, "dir-1");
        assert_eq!(manager.directory_count(), 1);
    }

    #[test]
    fn test_total_used_space() {
        let mut manager = SigmaDiskUsage::new();
        let fs1 = manager.add_filesystem("/".to_string(), "/dev/sda1".to_string(), 1000000000000);
        let fs2 =
            manager.add_filesystem("/home".to_string(), "/dev/sda2".to_string(), 500000000000);

        manager
            .get_filesystem_mut(&fs1)
            .unwrap()
            .with_usage(500000000000);
        manager
            .get_filesystem_mut(&fs2)
            .unwrap()
            .with_usage(250000000000);

        assert_eq!(manager.total_used_space(), 750000000000);
    }

    #[test]
    fn test_global_instance() {
        init_sigmadiskusage();
        let manager = get_sigmadiskusage();
        assert_eq!(manager.filesystem_count(), 0);

        let manager_mut = get_sigmadiskusage_mut();
        manager_mut.add_filesystem("/".to_string(), "/dev/sda1".to_string(), 1000000000000);
        assert_eq!(manager.filesystem_count(), 1);
    }

    #[test]
    fn test_add_filesystem_convenience() {
        init_sigmadiskusage();
        let id = add_filesystem("/".to_string(), "/dev/sda1".to_string(), 1000000000000);
        assert_eq!(id, "fs-1");

        let manager = get_sigmadiskusage();
        assert!(manager.get_filesystem(&id).is_some());
    }
}
