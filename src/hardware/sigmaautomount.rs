// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaAutoMount - Auto-Mount Daemon
//!
//! SigmaAutoMount provides automatic device mounting for SigmaOS, inspired by udisks and automount from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Mount option
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MountOption {
    /// Read-only mount
    ReadOnly,
    /// Read-write mount
    ReadWrite,
    /// No-execution mount
    NoExec,
    /// No-suid mount
    NoSuid,
    /// Synchronous mount
    Sync,
    /// Asynchronous mount
    Async,
}

impl fmt::Display for MountOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MountOption::ReadOnly => write!(f, "ro"),
            MountOption::ReadWrite => write!(f, "rw"),
            MountOption::NoExec => write!(f, "noexec"),
            MountOption::NoSuid => write!(f, "nosuid"),
            MountOption::Sync => write!(f, "sync"),
            MountOption::Async => write!(f, "async"),
        }
    }
}

/// Filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    /// Unknown filesystem
    Unknown,
    /// ext4
    Ext4,
    /// ext3
    Ext3,
    /// ext2
    Ext2,
    /// xfs
    Xfs,
    /// btrfs
    Btrfs,
    /// vfat
    VFAT,
    /// ntfs
    NTFS,
    /// iso9660
    ISO9660,
    /// nfs
    NFS,
    /// tmpfs
    Tmpfs,
}

impl fmt::Display for FsType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FsType::Unknown => write!(f, "unknown"),
            FsType::Ext4 => write!(f, "ext4"),
            FsType::Ext3 => write!(f, "ext3"),
            FsType::Ext2 => write!(f, "ext2"),
            FsType::Xfs => write!(f, "xfs"),
            FsType::Btrfs => write!(f, "btrfs"),
            FsType::VFAT => write!(f, "vfat"),
            FsType::NTFS => write!(f, "ntfs"),
            FsType::ISO9660 => write!(f, "iso9660"),
            FsType::NFS => write!(f, "nfs"),
            FsType::Tmpfs => write!(f, "tmpfs"),
        }
    }
}

/// Mount status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountStatus {
    /// Not mounted
    NotMounted,
    /// Mounting
    Mounting,
    /// Mounted
    Mounted,
    /// Unmounting
    Unmounting,
    /// Error
    Error,
}

impl fmt::Display for MountStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MountStatus::NotMounted => write!(f, "not_mounted"),
            MountStatus::Mounting => write!(f, "mounting"),
            MountStatus::Mounted => write!(f, "mounted"),
            MountStatus::Unmounting => write!(f, "unmounting"),
            MountStatus::Error => write!(f, "error"),
        }
    }
}

/// Mount point
#[derive(Debug, Clone)]
pub struct MountPoint {
    /// Device ID
    pub device_id: String,
    /// Device path
    pub device_path: String,
    /// Mount point path
    pub mount_point: String,
    /// Filesystem type
    pub fs_type: FsType,
    /// Mount options
    pub options: Vec<MountOption>,
    /// Mount status
    pub status: MountStatus,
    /// Auto-mount enabled
    pub auto_mount: bool,
    /// Mount timestamp
    pub mount_time: Option<u64>,
}

impl MountPoint {
    /// Create a new mount point
    pub fn new(device_id: String, device_path: String, mount_point: String, fs_type: FsType) -> Self {
        Self {
            device_id,
            device_path,
            mount_point,
            fs_type,
            options: Vec::new(),
            status: MountStatus::NotMounted,
            auto_mount: true,
            mount_time: None,
        }
    }

    /// Add mount option
    pub fn with_option(mut self, option: MountOption) -> Self {
        self.options.push(option);
        self
    }

    /// Set auto-mount
    pub fn with_auto_mount(mut self, auto: bool) -> Self {
        self.auto_mount = auto;
        self
    }

    /// Set status
    pub fn with_status(mut self, status: MountStatus) -> Self {
        self.status = status;
        self
    }
}

/// SigmaAutoMount - Auto-Mount Daemon
pub struct SigmaAutoMount {
    /// Mount points (device_id -> mount point)
    mount_points: BTreeMap<String, MountPoint>,
    /// Running state
    running: bool,
    /// Auto-mount enabled
    auto_mount_enabled: bool,
}

impl SigmaAutoMount {
    /// Create a new SigmaAutoMount instance
    pub fn new() -> Self {
        Self {
            mount_points: BTreeMap::new(),
            running: false,
            auto_mount_enabled: true,
        }
    }

    /// Start the auto-mount daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the auto-mount daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Enable auto-mount
    pub fn enable_auto_mount(&mut self) {
        self.auto_mount_enabled = true;
    }

    /// Disable auto-mount
    pub fn disable_auto_mount(&mut self) {
        self.auto_mount_enabled = false;
    }

    /// Add a mount point
    pub fn add_mount_point(&mut self, mount_point: MountPoint) -> Result<(), String> {
        if self.mount_points.contains_key(&mount_point.device_id) {
            return Err(format!("Mount point for '{}' already exists", mount_point.device_id));
        }
        self.mount_points.insert(mount_point.device_id.clone(), mount_point);
        Ok(())
    }

    /// Remove a mount point
    pub fn remove_mount_point(&mut self, device_id: &str) -> Result<(), String> {
        if self.mount_points.remove(device_id).is_none() {
            return Err(format!("Mount point for '{}' not found", device_id));
        }
        Ok(())
    }

    /// Get a mount point
    pub fn get_mount_point(&self, device_id: &str) -> Option<&MountPoint> {
        self.mount_points.get(device_id)
    }

    /// Get a mount point (mutable)
    pub fn get_mount_point_mut(&mut self, device_id: &str) -> Option<&mut MountPoint> {
        self.mount_points.get_mut(device_id)
    }

    /// List all mount points
    pub fn list_mount_points(&self) -> Vec<&MountPoint> {
        self.mount_points.values().collect()
    }

    /// List mounted mount points
    pub fn list_mounted(&self) -> Vec<&MountPoint> {
        self.mount_points.values().filter(|m| m.status == MountStatus::Mounted).collect()
    }

    /// List auto-mount enabled points
    pub fn list_auto_mount(&self) -> Vec<&MountPoint> {
        self.mount_points.values().filter(|m| m.auto_mount).collect()
    }

    /// Mount a device
    pub fn mount(&mut self, device_id: &str, current_time: u64) -> Result<(), String> {
        if !self.running {
            return Err("SigmaAutoMount not running".to_string());
        }

        match self.mount_points.get_mut(device_id) {
            Some(mount_point) => {
                mount_point.status = MountStatus::Mounting;
                // In a real implementation, this would perform the actual mount
                mount_point.status = MountStatus::Mounted;
                mount_point.mount_time = Some(current_time);
                Ok(())
            }
            None => Err(format!("Mount point for '{}' not found", device_id)),
        }
    }

    /// Unmount a device
    pub fn unmount(&mut self, device_id: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaAutoMount not running".to_string());
        }

        match self.mount_points.get_mut(device_id) {
            Some(mount_point) => {
                mount_point.status = MountStatus::Unmounting;
                // In a real implementation, this would perform the actual unmount
                mount_point.status = MountStatus::NotMounted;
                mount_point.mount_time = None;
                Ok(())
            }
            None => Err(format!("Mount point for '{}' not found", device_id)),
        }
    }

    /// Auto-mount devices
    pub fn auto_mount(&mut self, current_time: u64) -> Vec<String> {
        let mut errors = Vec::new();
        
        if !self.running || !self.auto_mount_enabled {
            return errors;
        }

        let to_mount: Vec<String> = self
            .mount_points
            .iter()
            .filter(|(_, mp)| mp.auto_mount && mp.status == MountStatus::NotMounted)
            .map(|(id, _)| id.clone())
            .collect();

        for device_id in to_mount {
            if let Err(e) = self.mount(&device_id, current_time) {
                errors.push(e);
            }
        }

        errors
    }

    /// Get mount point count
    pub fn mount_point_count(&self) -> usize {
        self.mount_points.len()
    }

    /// Get mounted count
    pub fn mounted_count(&self) -> usize {
        self.mount_points.values().filter(|m| m.status == MountStatus::Mounted).count()
    }

    /// Get auto-mount statistics
    pub fn auto_mount_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_mount_points".to_string(), self.mount_point_count().to_string());
        stats.insert("mounted".to_string(), self.mounted_count().to_string());
        stats.insert("auto_mount_enabled".to_string(), self.auto_mount_enabled.to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaAutoMount {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaAutoMount instance (placeholder for actual global state)
static mut SIGMAAUTOMOUNT_INSTANCE: Option<SigmaAutoMount> = None;

/// Initialize the global SigmaAutoMount instance
pub fn init_sigmaautomount() {
    unsafe {
        SIGMAAUTOMOUNT_INSTANCE = Some(SigmaAutoMount::new());
    }
}

/// Get the global SigmaAutoMount instance
pub fn get_sigmaautomount() -> Option<&'static mut SigmaAutoMount> {
    unsafe { SIGMAAUTOMOUNT_INSTANCE.as_mut() }
}

/// Convenience function to add a mount point
pub fn add_mount_point(mount_point: MountPoint) -> Result<(), String> {
    if let Some(automount) = get_sigmaautomount() {
        automount.add_mount_point(mount_point)
    } else {
        Err("SigmaAutoMount not initialized".to_string())
    }
}

/// Convenience function to mount a device
pub fn mount_device(device_id: &str, current_time: u64) -> Result<(), String> {
    if let Some(automount) = get_sigmaautomount() {
        automount.mount(device_id, current_time)
    } else {
        Err("SigmaAutoMount not initialized".to_string())
    }
}

/// Convenience function to unmount a device
pub fn unmount_device(device_id: &str) -> Result<(), String> {
    if let Some(automount) = get_sigmaautomount() {
        automount.unmount(device_id)
    } else {
        Err("SigmaAutoMount not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mount_option_display() {
        assert_eq!(MountOption::ReadOnly.to_string(), "ro");
        assert_eq!(MountOption::ReadWrite.to_string(), "rw");
    }

    #[test]
    fn test_fs_type_display() {
        assert_eq!(FsType::Ext4.to_string(), "ext4");
        assert_eq!(FsType::NTFS.to_string(), "ntfs");
    }

    #[test]
    fn test_mount_status_display() {
        assert_eq!(MountStatus::Mounted.to_string(), "mounted");
        assert_eq!(MountStatus::Error.to_string(), "error");
    }

    #[test]
    fn test_mount_point() {
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        )
        .with_option(MountOption::ReadWrite)
        .with_option(MountOption::Sync)
        .with_auto_mount(true)
        .with_status(MountStatus::Mounted);

        assert_eq!(mount_point.device_id, "sda1");
        assert_eq!(mount_point.fs_type, FsType::Ext4);
        assert_eq!(mount_point.options.len(), 2);
        assert!(mount_point.auto_mount);
    }

    #[test]
    fn test_sigmaautomount() {
        let mut automount = SigmaAutoMount::new();
        assert!(!automount.is_running());
        automount.start();
        assert!(automount.is_running());
        automount.stop();
        assert!(!automount.is_running());
    }

    #[test]
    fn test_sigmaautomount_add_mount_point() {
        let mut automount = SigmaAutoMount::new();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        assert!(automount.add_mount_point(mount_point).is_ok());
        assert_eq!(automount.mount_point_count(), 1);
    }

    #[test]
    fn test_sigmaautomount_duplicate_mount_point() {
        let mut automount = SigmaAutoMount::new();
        
        let mount_point1 = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        let mount_point2 = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data2".to_string(),
            FsType::Ext4
        );
        
        assert!(automount.add_mount_point(mount_point1).is_ok());
        assert!(automount.add_mount_point(mount_point2).is_err());
    }

    #[test]
    fn test_sigmaautomount_mount() {
        let mut automount = SigmaAutoMount::new();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        automount.add_mount_point(mount_point).unwrap();
        
        assert!(automount.mount("sda1", 1000).is_ok());
        
        let mp = automount.get_mount_point("sda1").unwrap();
        assert_eq!(mp.status, MountStatus::Mounted);
        assert_eq!(mp.mount_time, Some(1000));
    }

    #[test]
    fn test_sigmaautomount_unmount() {
        let mut automount = SigmaAutoMount::new();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        automount.add_mount_point(mount_point).unwrap();
        automount.mount("sda1", 1000).unwrap();
        
        assert!(automount.unmount("sda1").is_ok());
        
        let mp = automount.get_mount_point("sda1").unwrap();
        assert_eq!(mp.status, MountStatus::NotMounted);
        assert_eq!(mp.mount_time, None);
    }

    #[test]
    fn test_sigmaautomount_auto_mount() {
        let mut automount = SigmaAutoMount::new();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        )
        .with_auto_mount(true);
        
        automount.add_mount_point(mount_point).unwrap();
        
        let errors = automount.auto_mount(1000);
        assert!(errors.is_empty());
        
        let mp = automount.get_mount_point("sda1").unwrap();
        assert_eq!(mp.status, MountStatus::Mounted);
    }

    #[test]
    fn test_sigmaautomount_stats() {
        let mut automount = SigmaAutoMount::new();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        automount.add_mount_point(mount_point).unwrap();
        automount.mount("sda1", 1000).unwrap();
        
        let stats = automount.auto_mount_stats();
        assert_eq!(stats.get("total_mount_points"), Some(&"1".to_string()));
        assert_eq!(stats.get("mounted"), Some(&"1".to_string()));
        assert_eq!(stats.get("auto_mount_enabled"), Some(&"true".to_string()));
    }

    #[test]
    fn test_add_mount_point_convenience() {
        init_sigmaautomount();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        let result = add_mount_point(mount_point);
        assert!(result.is_ok());
    }

    #[test]
    fn test_mount_device_convenience() {
        init_sigmaautomount();
        
        let mut automount = get_sigmaautomount().unwrap();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        add_mount_point(mount_point).unwrap();
        
        let result = mount_device("sda1", 1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unmount_device_convenience() {
        init_sigmaautomount();
        
        let mut automount = get_sigmaautomount().unwrap();
        automount.start();
        
        let mount_point = MountPoint::new(
            "sda1".to_string(),
            "/dev/sda1".to_string(),
            "/mnt/data".to_string(),
            FsType::Ext4
        );
        
        add_mount_point(mount_point).unwrap();
        mount_device("sda1", 1000).unwrap();
        
        let result = unmount_device("sda1");
        assert!(result.is_ok());
    }
}
