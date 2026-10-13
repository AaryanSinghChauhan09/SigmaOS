// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaStorage - Storage Manager
//!
//! SigmaStorage provides storage management functionality for SigmaOS, inspired by LVM/parted from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Volume state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VolumeState {
    /// Available
    Available,
    /// In Use
    InUse,
    /// Offline
    Offline,
}

impl fmt::Display for VolumeState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            VolumeState::Available => write!(f, "available"),
            VolumeState::InUse => write!(f, "in_use"),
            VolumeState::Offline => write!(f, "offline"),
        }
    }
}

/// Storage volume
#[derive(Debug, Clone)]
pub struct StorageVolume {
    /// Volume name
    pub name: String,
    /// Device path
    pub device_path: String,
    /// Size in bytes
    pub size: u64,
    /// State
    pub state: VolumeState,
    /// Filesystem type
    pub filesystem: String,
}

impl StorageVolume {
    /// Create a new storage volume
    pub fn new(name: String, device_path: String, size: u64, filesystem: String) -> Self {
        Self {
            name,
            device_path,
            size,
            state: VolumeState::Available,
            filesystem,
        }
    }

    /// Set state
    pub fn with_state(mut self, state: VolumeState) -> Self {
        self.state = state;
        self
    }
}

/// SigmaStorage - Storage Manager
pub struct SigmaStorage {
    /// Storage volumes (name -> volume)
    volumes: BTreeMap<String, StorageVolume>,
    /// Running state
    running: bool,
    /// Volume counter
    volume_counter: u32,
}

impl SigmaStorage {
    /// Create a new SigmaStorage instance
    pub fn new() -> Self {
        Self {
            volumes: BTreeMap::new(),
            running: false,
            volume_counter: 0,
        }
    }

    /// Start the storage manager
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the storage manager
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a storage volume
    pub fn add_volume(&mut self, volume: StorageVolume) -> Result<(), String> {
        if self.volumes.contains_key(&volume.name) {
            return Err(format!("Volume '{}' already exists", volume.name));
        }
        
        self.volumes.insert(volume.name.clone(), volume);
        self.volume_counter += 1;
        Ok(())
    }

    /// Remove a storage volume
    pub fn remove_volume(&mut self, name: &str) -> Result<(), String> {
        if self.volumes.remove(name).is_none() {
            return Err(format!("Volume '{}' not found", name));
        }
        Ok(())
    }

    /// Mount a volume
    pub fn mount_volume(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaStorage not running".to_string());
        }
        
        match self.volumes.get_mut(name) {
            Some(volume) => {
                volume.state = VolumeState::InUse;
                Ok(())
            }
            None => Err(format!("Volume '{}' not found", name)),
        }
    }

    /// Unmount a volume
    pub fn unmount_volume(&mut self, name: &str) -> Result<(), String> {
        if !self.running {
            return Err("SigmaStorage not running".to_string());
        }
        
        match self.volumes.get_mut(name) {
            Some(volume) => {
                volume.state = VolumeState::Available;
                Ok(())
            }
            None => Err(format!("Volume '{}' not found", name)),
        }
    }

    /// Get a storage volume
    pub fn get_volume(&self, name: &str) -> Option<&StorageVolume> {
        self.volumes.get(name)
    }

    /// List all volumes
    pub fn list_volumes(&self) -> Vec<&StorageVolume> {
        self.volumes.values().collect()
    }

    /// List volumes by state
    pub fn list_by_state(&self, state: VolumeState) -> Vec<&StorageVolume> {
        self.volumes.values()
            .filter(|v| v.state == state)
            .collect()
    }

    /// Get volume count
    pub fn volume_count(&self) -> usize {
        self.volumes.len()
    }

    /// Get storage manager statistics
    pub fn storage_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_volumes".to_string(), self.volume_count().to_string());
        stats.insert("available_volumes".to_string(), self.list_by_state(VolumeState::Available).len().to_string());
        stats.insert("in_use_volumes".to_string(), self.list_by_state(VolumeState::InUse).len().to_string());
        stats.insert("offline_volumes".to_string(), self.list_by_state(VolumeState::Offline).len().to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaStorage {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaStorage instance (placeholder for actual global state)
static unsafe mutable SIGMASTORAGE_INSTANCE: Option<SigmaStorage> = None;

/// Initialize the global SigmaStorage instance
pub fn init_sigmastorage() {
    unsafe {
        SIGMASTORAGE_INSTANCE = Some(SigmaStorage::new());
    }
}

/// Get the global SigmaStorage instance
pub fn get_sigmastorage() -> Option<&'static mut SigmaStorage> {
    unsafe { SIGMASTORAGE_INSTANCE.as_mut() }
}

/// Convenience function to mount a volume
pub fn mount_storage_volume(name: &str) -> Result<(), String> {
    if let Some(storage) = get_sigmastorage() {
        storage.mount_volume(name)
    } else {
        Err("SigmaStorage not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_volume_state_display() {
        assert_eq!(VolumeState::Available.to_string(), "available");
        assert_eq!(VolumeState::InUse.to_string(), "in_use");
    }

    #[test]
    fn test_storage_volume() {
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        )
        .with_state(VolumeState::InUse);

        assert_eq!(volume.name, "vol1");
        assert_eq!(volume.state, VolumeState::InUse);
    }

    #[test]
    fn test_sigmastorage() {
        let mut storage = SigmaStorage::new();
        assert!(!storage.is_running());
        storage.start();
        assert!(storage.is_running());
        storage.stop();
        assert!(!storage.is_running());
    }

    #[test]
    fn test_sigmastorage_add_volume() {
        let mut storage = SigmaStorage::new();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        );
        
        assert!(storage.add_volume(volume).is_ok());
        assert_eq!(storage.volume_count(), 1);
    }

    #[test]
    fn test_sigmastorage_mount_volume() {
        let mut storage = SigmaStorage::new();
        storage.start();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        );
        
        storage.add_volume(volume).unwrap();
        assert!(storage.mount_volume("vol1").is_ok());
        
        let vol = storage.get_volume("vol1").unwrap();
        assert_eq!(vol.state, VolumeState::InUse);
    }

    #[test]
    fn test_sigmastorage_unmount_volume() {
        let mut storage = SigmaStorage::new();
        storage.start();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        )
        .with_state(VolumeState::InUse);
        
        storage.add_volume(volume).unwrap();
        assert!(storage.unmount_volume("vol1").is_ok());
        
        let vol = storage.get_volume("vol1").unwrap();
        assert_eq!(vol.state, VolumeState::Available);
    }

    #[test]
    fn test_sigmastorage_list_by_state() {
        let mut storage = SigmaStorage::new();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        )
        .with_state(VolumeState::InUse);
        
        storage.add_volume(volume).unwrap();
        
        let volumes = storage.list_by_state(VolumeState::InUse);
        assert_eq!(volumes.len(), 1);
    }

    #[test]
    fn test_sigmastorage_stats() {
        let mut storage = SigmaStorage::new();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        )
        .with_state(VolumeState::InUse);
        
        storage.add_volume(volume).unwrap();
        
        let stats = storage.storage_stats();
        assert_eq!(stats.get("total_volumes"), Some(&"1".to_string()));
        assert_eq!(stats.get("in_use_volumes"), Some(&"1".to_string()));
    }

    #[test]
    fn test_mount_storage_volume_convenience() {
        init_sigmastorage();
        
        let mut storage = get_sigmastorage().unwrap();
        storage.start();
        
        let volume = StorageVolume::new(
            "vol1".to_string(),
            "/dev/sda1".to_string(),
            1024 * 1024 * 1024,
            "ext4".to_string()
        );
        
        storage.add_volume(volume).unwrap();
        
        let result = mount_storage_volume("vol1");
        assert!(result.is_ok());
    }
}
