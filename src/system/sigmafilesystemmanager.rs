//! SigmaFileSystemManager - Filesystem Management System
//!
//! Provides comprehensive filesystem management inspired by Arch, Debian, and CachyOS.
//! Supports filesystem creation, mounting, formatting, and filesystem health monitoring.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    XFS,
    Btrfs,
    ZFS,
    NTFS,
    FAT32,
    ExFAT,
    Swap,
;

/// Mount status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountStatus {
    Mounted,
    Unmounted,
    Error,
;

/// Filesystem
#[derive(Debug, Clone)]
pub struct Filesystem {
    pub id: String,
    pub device: String,
    pub mount_point: String,
    pub fs_type: FilesystemType,
    pub mount_status: MountStatus,
    pub size: u64, // in bytes
    pub used: u64,
    pub available: u64,
    pub options: String, // mount options
;

impl Filesystem {
    pub fn new(id: String, device: String, mount_point: String, fs_type: FilesystemType) -> Self {
        Self {
            id,
            device,
            mount_point,
            fs_type,
            mount_status: MountStatus::Unmounted,
            size: 0,
            used: 0,
            available: 0,
            options: "defaults".to_string(),
        }
    }

    pub fn set_mount_status(&mut self, status: MountStatus) {
        self.mount_status = status;
    }

    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }

    pub fn set_used(&mut self, used: u64) {
        self.used = used;
    }

    pub fn set_available(&mut self, available: u64) {
        self.available = available;
    }

    pub fn set_options(&mut self, options: String) {
        self.options = options;
    }

    pub fn usage_percentage(&self) -> f32 {
        if self.size > 0 {
            (self.used as f32) / (self.size as f32) * 100.0
        } else {
            0.0
        }
    }

    pub fn is_mounted(&self) -> bool {
        self.mount_status == MountStatus::Mounted
    }

    pub fn is_full(&self) -> bool {
        self.usage_percentage() > 95.0
    }
;

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    pub id: String,
    pub path: String,
    pub device: String,
    pub fs_type: FilesystemType,
    pub auto_mount: bool,
;

impl MountPoint {
    pub fn new(id: String, path: String, device: String, fs_type: FilesystemType) -> Self {
        Self {
            id,
            path,
            device,
            fs_type,
            auto_mount: true,
        }
    }

    pub fn set_auto_mount(&mut self, auto: bool) {
        self.auto_mount = auto;
    }
;

/// SigmaFileSystemManager - Filesystem management
#[derive(Debug)]
pub struct SigmaFileSystemManager {
    filesystems: BTreeMap<String, Filesystem>,
    mount_points: BTreeMap<String, MountPoint>,
;

impl SigmaFileSystemManager {
    pub fn new() -> Self {
        Self {
            filesystems: BTreeMap::new(),
            mount_points: BTreeMap::new(),
        }
    }

    pub fn add_filesystem(&mut self, fs: Filesystem) {
        self.filesystems.insert(fs.id.clone(), fs);
    }

    pub fn remove_filesystem(&mut self, id: &str) -> bool {
        self.filesystems.remove(id).is_some()
    }

    pub fn get_filesystem(&self, id: &str) -> Option<&Filesystem> {
        self.filesystems.get(id)
    }

    pub fn get_filesystem_mut(&mut self, id: &str) -> Option<&mut Filesystem> {
        self.filesystems.get_mut(id)
    }

    pub fn add_mount_point(&mut self, mount: MountPoint) {
        self.mount_points.insert(mount.id.clone(), mount);
    }

    pub fn remove_mount_point(&mut self, id: &str) -> bool {
        self.mount_points.remove(id).is_some()
    }

    pub fn get_mount_point(&self, id: &str) -> Option<&MountPoint> {
        self.mount_points.get(id)
    }

    pub fn list_mounted(&self) -> Vec<&Filesystem> {
        self.filesystems.values()
            .filter(|fs| fs.is_mounted())
            .collect()
    }

    pub fn list_by_type(&self, fs_type: FilesystemType) -> Vec<&Filesystem> {
        self.filesystems.values()
            .filter(|fs| fs.fs_type == fs_type)
            .collect()
    }

    pub fn list_full(&self) -> Vec<&Filesystem> {
        self.filesystems.values()
            .filter(|fs| fs.is_full())
            .collect()
    }

    pub fn list_auto_mount(&self) -> Vec<&MountPoint> {
        self.mount_points.values()
            .filter(|mount| mount.auto_mount)
            .collect()
    }

    pub fn total_size(&self) -> u64 {
        self.filesystems.values()
            .map(|fs| fs.size)
            .sum()
    }

    pub fn total_used(&self) -> u64 {
        self.filesystems.values()
            .map(|fs| fs.used)
            .sum()
    }

    pub fn filesystem_count(&self) -> usize {
        self.filesystems.len()
    }

    pub fn mount_point_count(&self) -> usize {
        self.mount_points.len()
    }
;

impl Default for SigmaFileSystemManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaFileSystemManager instance
static mut SIGMAFILESYSTEMMANAGER_INSTANCE: Option<SigmaFileSystemManager> = None;

/// Initialize the global SigmaFileSystemManager instance
pub fn init_sigmafilesystemmanager() {
    unsafe {
        SIGMAFILESYSTEMMANAGER_INSTANCE = Some(SigmaFileSystemManager::new());
    }
;

/// Get the global SigmaFileSystemManager instance
pub fn get_sigmafilesystemmanager() -> Option<&'static SigmaFileSystemManager> {
    unsafe {
        SIGMAFILESYSTEMMANAGER_INSTANCE.as_ref()
    }
;

/// Get the global SigmaFileSystemManager instance (mutable)
pub fn get_sigmafilesystemmanager_mut() -> Option<&'static mut SigmaFileSystemManager> {
    unsafe {
        SIGMAFILESYSTEMMANAGER_INSTANCE.as_mut()
    }
;

/// Convenience function to add a filesystem
pub fn add_filesystem(fs: Filesystem) -> bool {
    if let Some(manager) = get_sigmafilesystemmanager_mut() {
        manager.add_filesystem(fs);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filesystem_creation() {
        let fs = Filesystem::new(
            "fs1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            FilesystemType::Ext4,
        );
        assert_eq!(fs.id, "fs1");
        assert_eq!(fs.fs_type, FilesystemType::Ext4);
    }

    #[test]
    fn test_filesystem_modification() {
        let mut fs = Filesystem::new(
            "fs1".to_string(),
            "/dev/sda2".to_string(),
            "/home".to_string(),
            FilesystemType::XFS,
        );
        fs.set_mount_status(MountStatus::Mounted);
        fs.set_size(1000000000);
        fs.set_used(500000000);
        assert!(fs.is_mounted());
        assert_eq!(fs.usage_percentage(), 50.0);
    }

    #[test]
    fn test_mount_point_creation() {
        let mount = MountPoint::new(
            "mount1".to_string(),
            "/mnt/data".to_string(),
            "/dev/sdb1".to_string(),
            FilesystemType::Btrfs,
        );
        assert_eq!(mount.id, "mount1");
        assert!(mount.auto_mount);
    }

    #[test]
    fn test_sigmafilesystemmanager_creation() {
        let manager = SigmaFileSystemManager::new();
        assert_eq!(manager.filesystem_count(), 0);
        assert_eq!(manager.mount_point_count(), 0);
    }

    #[test]
    fn test_sigmafilesystemmanager_management() {
        let mut manager = SigmaFileSystemManager::new();
        let fs = Filesystem::new(
            "fs1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            FilesystemType::Ext4,
        );
        manager.add_filesystem(fs);
        assert_eq!(manager.filesystem_count(), 1);
    }

    #[test]
    fn test_full_detection() {
        let mut manager = SigmaFileSystemManager::new();
        let mut fs = Filesystem::new(
            "fs1".to_string(),
            "/dev/sda1".to_string(),
            "/".to_string(),
            FilesystemType::Ext4,
        );
        fs.set_size(1000);
        fs.set_used(960);
        manager.add_filesystem(fs);
        let full = manager.list_full();
        assert_eq!(full.len(), 1);
    }
;
