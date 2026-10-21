// SPDX-License-Identifier: MIT
// Copyright (c) 2025 Aaryan Singh Chauhan

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Storage device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageType {
    HDD,
    SSD,
    NVMe,
    USB,
    Network,
}

/// Filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilesystemType {
    Ext4,
    Btrfs,
    XFS,
    ZFS,
    NTFS,
    FAT32,
}

/// Mount state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MountState {
    Unmounted,
    Mounted,
    ReadOnly,
}

/// Storage device
#[derive(Debug, Clone)]
pub struct StorageDevice {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub storage_type: StorageType,
    pub filesystem: Option<FilesystemType>,
    pub mount_point: Option<String>,
    pub mount_state: MountState,
}

/// Storage manager for managing storage devices
/// Inspired by Linux storage management and Omarchy setup commands
#[derive(Debug, Clone)]
pub struct SigmaStorageManager {
    devices: Vec<StorageDevice>,
}

impl SigmaStorageManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Register a storage device
    pub fn register_device(&mut self, device: StorageDevice) {
        self.devices.push(device);
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&StorageDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    /// Get all devices
    pub fn get_devices(&self) -> &[StorageDevice] {
        &self.devices
    }

    /// Mount a device
    pub fn mount_device(&mut self, id: &str, mount_point: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.mount_point = Some(mount_point.to_string());
            device.mount_state = MountState::Mounted;
            Ok(format!("Mounted device: {} at {}", device.name, mount_point))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Unmount a device
    pub fn unmount_device(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.mount_point = None;
            device.mount_state = MountState::Unmounted;
            Ok(format!("Unmounted device: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Set device as read-only
    pub fn set_readonly(&mut self, id: &str) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.mount_state = MountState::ReadOnly;
            Ok(format!("Set device as read-only: {}", device.name))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Format a device with a filesystem
    pub fn format_device(&mut self, id: &str, filesystem: FilesystemType) -> Result<String, String> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.id == id) {
            device.filesystem = Some(filesystem);
            Ok(format!("Formatted device: {} with {:?}", device.name, filesystem))
        } else {
            Err(format!("Device not found: {}", id))
        }
    }

    /// Get mounted devices
    pub fn get_mounted_devices(&self) -> Vec<&StorageDevice> {
        self.devices
            .iter()
            .filter(|d| d.mount_state == MountState::Mounted)
            .collect()
    }

    /// Get devices by type
    pub fn get_devices_by_type(&self, storage_type: StorageType) -> Vec<&StorageDevice> {
        self.devices
            .iter()
            .filter(|d| d.storage_type == storage_type)
            .collect()
    }

    /// Get total storage capacity
    pub fn get_total_capacity(&self) -> u64 {
        self.devices.iter().map(|d| d.size).sum()
    }

    /// Get available storage capacity (unmounted devices)
    pub fn get_available_capacity(&self) -> u64 {
        self.devices
            .iter()
            .filter(|d| d.mount_state == MountState::Unmounted)
            .map(|d| d.size)
            .sum()
    }
}

impl Default for SigmaStorageManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_device() {
        let mut manager = SigmaStorageManager::new();
        let device = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: Some(FilesystemType::Ext4),
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        manager.register_device(device);
        assert_eq!(manager.get_devices().len(), 1);
    }

    #[test]
    fn test_mount_device() {
        let mut manager = SigmaStorageManager::new();
        let device = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: Some(FilesystemType::Ext4),
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        manager.register_device(device);
        manager.mount_device("sda1", "/mnt/data").unwrap();
        assert_eq!(
            manager.get_device("sda1").unwrap().mount_state,
            MountState::Mounted
        );
    }

    #[test]
    fn test_unmount_device() {
        let mut manager = SigmaStorageManager::new();
        let device = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: Some(FilesystemType::Ext4),
            mount_point: Some("/mnt/data".to_string()),
            mount_state: MountState::Mounted,
        };
        manager.register_device(device);
        manager.unmount_device("sda1").unwrap();
        assert_eq!(
            manager.get_device("sda1").unwrap().mount_state,
            MountState::Unmounted
        );
    }

    #[test]
    fn test_format_device() {
        let mut manager = SigmaStorageManager::new();
        let device = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: None,
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        manager.register_device(device);
        manager.format_device("sda1", FilesystemType::Btrfs).unwrap();
        assert_eq!(
            manager.get_device("sda1").unwrap().filesystem,
            Some(FilesystemType::Btrfs)
        );
    }

    #[test]
    fn test_get_mounted_devices() {
        let mut manager = SigmaStorageManager::new();
        let device1 = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: Some(FilesystemType::Ext4),
            mount_point: Some("/mnt/data".to_string()),
            mount_state: MountState::Mounted,
        };
        let device2 = StorageDevice {
            id: "sdb1".to_string(),
            name: "Backup Drive".to_string(),
            path: "/dev/sdb1".to_string(),
            size: 2000000000000,
            storage_type: StorageType::HDD,
            filesystem: Some(FilesystemType::Btrfs),
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        manager.register_device(device1);
        manager.register_device(device2);
        assert_eq!(manager.get_mounted_devices().len(), 1);
    }

    #[test]
    fn test_get_total_capacity() {
        let mut manager = SigmaStorageManager::new();
        let device1 = StorageDevice {
            id: "sda1".to_string(),
            name: "Main Drive".to_string(),
            path: "/dev/sda1".to_string(),
            size: 1000000000000,
            storage_type: StorageType::SSD,
            filesystem: Some(FilesystemType::Ext4),
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        let device2 = StorageDevice {
            id: "sdb1".to_string(),
            name: "Backup Drive".to_string(),
            path: "/dev/sdb1".to_string(),
            size: 2000000000000,
            storage_type: StorageType::HDD,
            filesystem: Some(FilesystemType::Btrfs),
            mount_point: None,
            mount_state: MountState::Unmounted,
        };
        manager.register_device(device1);
        manager.register_device(device2);
        assert_eq!(manager.get_total_capacity(), 3000000000000);
    }
}
