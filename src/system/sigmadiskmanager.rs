//! SigmaDiskManager - Disk Management System
//!
//! Provides comprehensive disk management inspired by Linux disk utilities.
//! Supports disk discovery, partitioning, and storage device control.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Disk type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskType {
    HDD,
    SSD,
    NVMe,
    USB,
    Unknown,
}

/// Disk state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiskState {
    Online,
    Offline,
    Error,
    Unknown,
}

/// Disk
#[derive(Debug, Clone)]
pub struct Disk {
    pub id: String,
    pub name: String,
    pub state: DiskState,
    pub device: String,
    pub size: u64,
    pub disk_type: DiskType,
    pub model: String,
    pub serial: String,
}

impl Disk {
    pub fn new(id: String, name: String, device: String, size: u64, disk_type: DiskType) -> Self {
        Self {
            id,
            name,
            state: DiskState::Online,
            device,
            size,
            disk_type,
            model: String::new(),
            serial: String::new(),
        }
    }

    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn with_serial(mut self, serial: String) -> Self {
        self.serial = serial;
        self
    }

    pub fn set_offline(&mut self) {
        self.state = DiskState::Offline;
    }

    pub fn set_online(&mut self) {
        self.state = DiskState::Online;
    }
}

/// Disk manager
pub struct SigmaDiskManager {
    disks: BTreeMap<String, Disk>,
    next_disk_id: u64,
}

impl SigmaDiskManager {
    pub fn new() -> Self {
        Self {
            disks: BTreeMap::new(),
            next_disk_id: 1,
        }
    }

    pub fn add_disk(&mut self, name: String, device: String, size: u64, disk_type: DiskType) -> String {
        let id = format!("disk-{}", self.next_disk_id);
        self.next_disk_id += 1;
        let disk = Disk::new(id.clone(), name, device, size, disk_type);
        self.disks.insert(id.clone(), disk);
        id
    }

    pub fn get_disk(&self, id: &str) -> Option<&Disk> {
        self.disks.get(id)
    }

    pub fn get_disk_mut(&mut self, id: &str) -> Option<&mut Disk> {
        self.disks.get_mut(id)
    }

    pub fn list_disks(&self) -> Vec<&Disk> {
        self.disks.values().collect()
    }

    pub fn list_disks_by_type(&self, disk_type: DiskType) -> Vec<&Disk> {
        self.disks.values().filter(|d| d.disk_type == disk_type).collect()
    }

    pub fn remove_disk(&mut self, id: &str) -> bool {
        self.disks.remove(id).is_some()
    }

    pub fn disk_count(&self) -> usize {
        self.disks.len()
    }

    pub fn total_capacity(&self) -> u64 {
        self.disks.values().map(|d| d.size).sum()
    }
}

impl Default for SigmaDiskManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMADISKMANAGER_INSTANCE: Option<SigmaDiskManager> = None;

pub fn init_sigmadiskmanager() {
    unsafe {
        SIGMADISKMANAGER_INSTANCE = Some(SigmaDiskManager::new());
    }
}

pub fn get_sigmadiskmanager() -> &'static SigmaDiskManager {
    unsafe {
        SIGMADISKMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaDiskManager not initialized")
    }
}

pub fn get_sigmadiskmanager_mut() -> &'static mut SigmaDiskManager {
    unsafe {
        SIGMADISKMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaDiskManager not initialized")
    }
}

pub fn add_disk(name: String, device: String, size: u64, disk_type: DiskType) -> String {
    let manager = get_sigmadiskmanager_mut();
    manager.add_disk(name, device, size, disk_type)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_creation() {
        let disk = Disk::new("disk-1".to_string(), "sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        assert_eq!(disk.id, "disk-1");
        assert_eq!(disk.name, "sda");
        assert_eq!(disk.size, 1000000000000);
        assert_eq!(disk.disk_type, DiskType::SSD);
    }

    #[test]
    fn test_disk_state() {
        let mut disk = Disk::new("disk-1".to_string(), "sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        assert_eq!(disk.state, DiskState::Online);

        disk.set_offline();
        assert_eq!(disk.state, DiskState::Offline);

        disk.set_online();
        assert_eq!(disk.state, DiskState::Online);
    }

    #[test]
    fn test_disk_with_model() {
        let disk = Disk::new("disk-1".to_string(), "sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD)
            .with_model("Samsung EVO".to_string())
            .with_serial("ABC123".to_string());
        assert_eq!(disk.model, "Samsung EVO");
        assert_eq!(disk.serial, "ABC123");
    }

    #[test]
    fn test_disk_manager() {
        let manager = SigmaDiskManager::new();
        assert_eq!(manager.disk_count(), 0);
    }

    #[test]
    fn test_add_disk() {
        let mut manager = SigmaDiskManager::new();
        let id = manager.add_disk("sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        assert_eq!(id, "disk-1");
        assert_eq!(manager.disk_count(), 1);
    }

    #[test]
    fn test_list_disks_by_type() {
        let mut manager = SigmaDiskManager::new();
        manager.add_disk("sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        manager.add_disk("sdb".to_string(), "/dev/sdb".to_string(), 2000000000000, DiskType::HDD);
        manager.add_disk("nvme0n1".to_string(), "/dev/nvme0n1".to_string(), 500000000000, DiskType::NVMe);

        let ssds = manager.list_disks_by_type(DiskType::SSD);
        assert_eq!(ssds.len(), 1);
    }

    #[test]
    fn test_total_capacity() {
        let mut manager = SigmaDiskManager::new();
        manager.add_disk("sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        manager.add_disk("sdb".to_string(), "/dev/sdb".to_string(), 2000000000000, DiskType::HDD);

        assert_eq!(manager.total_capacity(), 3000000000000);
    }

    #[test]
    fn test_global_instance() {
        init_sigmadiskmanager();
        let manager = get_sigmadiskmanager();
        assert_eq!(manager.disk_count(), 0);

        let manager_mut = get_sigmadiskmanager_mut();
        manager_mut.add_disk("sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        assert_eq!(manager.disk_count(), 1);
    }

    #[test]
    fn test_add_disk_convenience() {
        init_sigmadiskmanager();
        let id = add_disk("sda".to_string(), "/dev/sda".to_string(), 1000000000000, DiskType::SSD);
        assert_eq!(id, "disk-1");

        let manager = get_sigmadiskmanager();
        assert!(manager.get_disk(&id).is_some());
    }
}
