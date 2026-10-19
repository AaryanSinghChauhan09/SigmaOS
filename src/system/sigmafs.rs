// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaFS - System Filesystem Manager
//!
//! SigmaFS provides system filesystem management for SigmaOS, inspired by mount/umount/fsck from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    /// ext4
    Ext4,
    /// xfs
    Xfs,
    /// btrfs
    Btrfs,
    /// FAT32
    Fat32,
    /// NTFS
    Ntfs,
;

impl fmt::Display for FsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsType::Ext4 => write!(f, "ext4"),
            FsType::Xfs => write!(f, "xfs"),
            FsType::Btrfs => write!(f, "btrfs"),
            FsType::Fat32 => write!(f, "fat32"),
            FsType::Ntfs => write!(f, "ntfs"),
        }
    }
;

/// Mount state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    /// Mounted
    Mounted,
    /// Unmounted
    Unmounted,
;

impl fmt::Display for MountState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MountState::Mounted => write!(f, "mounted"),
            MountState::Unmounted => write!(f, "unmounted"),
        }
    }
;

/// Filesystem mount
#[derive(Debug, Clone)]
pub struct FsMount {
    /// Mount ID
    pub mount_id: String,
    /// Device path
    pub device_path: String,
    /// Mount point
    pub mount_point: String,
    /// Filesystem type
    pub fs_type: FsType,
    /// Mount state
    pub state: MountState,
    /// Mount options
    pub options: String,
;

impl FsMount {
    /// Create a new filesystem mount
    pub fn new(
        mount_id: String,
        device_path: String,
        mount_point: String,
        fs_type: FsType,
    ) -> Self {
        Self {
            mount_id,
            device_path,
            mount_point,
            fs_type,
            state: MountState::Unmounted,
            options: String::new(),
        }
    }

    /// Set state
    pub fn with_state(mut self, state: MountState) -> Self {
        self.state = state;
        self
    }

    /// Set options
    pub fn with_options(mut self, options: String) -> Self {
        self.options = options;
        self
    }
;

/// SigmaFS - System Filesystem Manager
pub struct SigmaFS {
    /// Filesystem mounts (mount_id -> mount)
    mounts: BTreeMap<String, FsMount>,
    /// Running state
    running: bool,
    /// Mount counter
    mount_counter: u32,
;

impl SigmaFS {
    /// Create a new SigmaFS instance
    pub fn new() -> Self {
        Self {
            mounts: BTreeMap::new(),
            running: false,
            mount_counter: 0,
        }
    }

    /// Start the filesystem manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the filesystem manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a filesystem mount
    pub fn add_mount(&mut self, mount: FsMount) -> Result<(), String> {
        if self.mounts.contains_key(&mount.mount_id) {
            return Err(format!("Mount '{}' already exists", mount.mount_id));
        }

        self.mounts.insert(mount.mount_id.clone(), mount);
        self.mount_counter += 1;
        Ok(())
    }

    /// Remove a filesystem mount
    pub fn remove_mount(&mut self, mount_id: &str) -> Result<(), String> {
        if self.mounts.remove(mount_id).is_none() {
            return Err(format!("Mount '{}' not found", mount_id));
        }
        Ok(())
    }

    /// Mount a filesystem
    pub fn mount_fs(&mut self, mount_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFS not running".to_string());
        }

        match self.mounts.get_mut(mount_id) {
            Some(mount) => {
                mount.state = MountState::Mounted;
                Ok(())
            }
            None => Err(format!("Mount '{}' not found", mount_id)),
        }
    }

    /// Unmount a filesystem
    pub fn unmount_fs(&mut self, mount_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaFS not running".to_string());
        }

        match self.mounts.get_mut(mount_id) {
            Some(mount) => {
                mount.state = MountState::Unmounted;
                Ok(())
            }
            None => Err(format!("Mount '{}' not found", mount_id)),
        }
    }

    /// Get a filesystem mount
    pub fn get_mount(&self, mount_id: &str) -> Option<&FsMount> {
        self.mounts.get(mount_id)
    }

    /// List all mounts
    pub fn list_mounts(&self) -> Vec<&FsMount> {
        self.mounts.values().collect()
    }

    /// List mounted filesystems
    pub fn list_mounted(&self) -> Vec<&FsMount> {
        self.mounts
            .values()
            .filter(|m| m.state == MountState::Mounted)
            .collect()
    }

    /// Get mount count
    pub fn mount_count(&self) -> usize {
        self.mounts.len()
    }

    /// Get filesystem manager statistics
    pub fn fs_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_mounts".to_string(), self.mount_count().to_string());
        stats.insert(
            "mounted_count".to_string(),
            self.list_mounted().len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaFS {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaFS instance (placeholder for actual global state)
static mut SIGMAFS_INSTANCE: Option<SigmaFS> = None;

/// Initialize the global SigmaFS instance
pub fn init_sigmafs() {
    unsafe {
        SIGMAFS_INSTANCE = Some(SigmaFS::new());
    }
;

/// Get the global SigmaFS instance
pub fn get_sigmafs() -> Option<&'static mut SigmaFS> {
    unsafe { SIGMAFS_INSTANCE.as_mut() }
;

/// Convenience function to mount a filesystem
pub fn mount_filesystem(mount_id: &str) -> Result<(), String> {
    if let Some(fs) = get_sigmafs() {
        fs.mount_fs(mount_id)
    } else {
        Err("SigmaFS not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fs_type_display() {
        assert_eq!(FsType::Ext4.to_string(), "ext4");
        assert_eq!(FsType::Xfs.to_string(), "xfs");
    }

    #[test]
    fn test_mount_state_display() {
        assert_eq!(MountState::Mounted.to_string(), "mounted");
        assert_eq!(MountState::Unmounted.to_string(), "unmounted");
    }

    #[test]
    fn test_fs_mount() {
        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        )
        .with_state(MountState::Mounted)
        .with_options("rw".to_string());

        assert_eq!(mount.mount_id, "mount1");
        assert_eq!(mount.state, MountState::Mounted);
        assert_eq!(mount.options, "rw");
    }

    #[test]
    fn test_sigmafs() {
        let mut fs = SigmaFS::new();
        assert!(!fs.is_running());
        fs.start();
        assert!(fs.is_running());
        fs.stop();
        assert!(!fs.is_running());
    }

    #[test]
    fn test_sigmafs_add_mount() {
        let mut fs = SigmaFS::new();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        );

        assert!(fs.add_mount(mount).is_ok());
        assert_eq!(fs.mount_count(), 1);
    }

    #[test]
    fn test_sigmafs_mount_fs() {
        let mut fs = SigmaFS::new();
        fs.start();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        );

        fs.add_mount(mount).unwrap();
        assert!(fs.mount_fs("mount1").is_ok());

        let mount = fs.get_mount("mount1").unwrap();
        assert_eq!(mount.state, MountState::Mounted);
    }

    #[test]
    fn test_sigmafs_unmount_fs() {
        let mut fs = SigmaFS::new();
        fs.start();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        )
        .with_state(MountState::Mounted);

        fs.add_mount(mount).unwrap();
        assert!(fs.unmount_fs("mount1").is_ok());

        let mount = fs.get_mount("mount1").unwrap();
        assert_eq!(mount.state, MountState::Unmounted);
    }

    #[test]
    fn test_sigmafs_list_mounted() {
        let mut fs = SigmaFS::new();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        )
        .with_state(MountState::Mounted);

        fs.add_mount(mount).unwrap();

        let mounted = fs.list_mounted();
        assert_eq!(mounted.len(), 1);
    }

    #[test]
    fn test_sigmafs_stats() {
        let mut fs = SigmaFS::new();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        )
        .with_state(MountState::Mounted);

        fs.add_mount(mount).unwrap();

        let stats = fs.fs_stats();
        assert_eq!(stats.get("total_mounts"), Some(&"1".to_string()));
        assert_eq!(stats.get("mounted_count"), Some(&"1".to_string()));
    }

    #[test]
    fn test_mount_filesystem_convenience() {
        init_sigmafs();

        let mut fs = get_sigmafs().unwrap();
        fs.start();

        let mount = FsMount::new(
            "mount1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4,
        );

        fs.add_mount(mount).unwrap();

        let result = mount_filesystem("mount1");
        assert!(result.is_ok());
    }
;
