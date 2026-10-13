//! SigmaLVM - Logical Volume Manager
//!
//! Inspired by Linux LVM, this module provides comprehensive logical volume
//! management including physical volumes, volume groups, and logical volumes.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// Physical volume state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PVState {
    Available,
    InUse,
    Removed,
    Unknown,
}

/// Volume group state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VGState {
    Active,
    Inactive,
    Partial,
    Removed,
    Unknown,
}

/// Logical volume state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LVState {
    Active,
    Inactive,
    Suspended,
    Removed,
    Unknown,
}

/// Physical volume data structure
#[derive(Debug, Clone)]
pub struct PhysicalVolume {
    pub pv_id: String,
    pub pv_name: String,
    pub device_path: String,
    pub size: u64,
    pub free: u64,
    pub state: PVState,
    pub vg_name: String,
}

impl PhysicalVolume {
    /// Create a new physical volume
    pub fn new(
        pv_id: String,
        pv_name: String,
        device_path: String,
        size: u64,
        vg_name: String,
    ) -> Self {
        Self {
            pv_id,
            pv_name,
            device_path,
            size,
            free: size,
            state: PVState::Available,
            vg_name,
        }
    }

    /// Set physical volume state
    pub fn set_state(&mut self, state: PVState) {
        self.state = state;
    }

    /// Allocate space from physical volume
    pub fn allocate(&mut self, size: u64) -> bool {
        if self.free >= size {
            self.free -= size;
            true
        } else {
            false
        }
    }

    /// Free space on physical volume
    pub fn free_space(&mut self, size: u64) {
        self.free += size;
        if self.free > self.size {
            self.free = self.size;
        }
    }

    /// Get used space
    pub fn used(&self) -> u64 {
        self.size - self.free
    }
}

/// Volume group data structure
#[derive(Debug, Clone)]
pub struct VolumeGroup {
    pub vg_id: String,
    pub vg_name: String,
    pub size: u64,
    pub free: u64,
    pub state: VGState,
    pub pv_count: usize,
    pub lv_count: usize,
}

impl VolumeGroup {
    /// Create a new volume group
    pub fn new(vg_id: String, vg_name: String) -> Self {
        Self {
            vg_id,
            vg_name,
            size: 0,
            free: 0,
            state: VGState::Inactive,
            pv_count: 0,
            lv_count: 0,
        }
    }

    /// Set volume group state
    pub fn set_state(&mut self, state: VGState) {
        self.state = state;
    }

    /// Update size and free space
    pub fn update_size(&mut self, size: u64, free: u64) {
        self.size = size;
        self.free = free;
    }

    /// Update counts
    pub fn update_counts(&mut self, pv_count: usize, lv_count: usize) {
        self.pv_count = pv_count;
        self.lv_count = lv_count;
    }
}

/// Logical volume data structure
#[derive(Debug, Clone)]
pub struct LogicalVolume {
    pub lv_id: String,
    pub lv_name: String,
    pub vg_name: String,
    pub size: u64,
    pub state: LVState,
    pub filesystem: String,
    pub mount_point: String,
}

impl LogicalVolume {
    /// Create a new logical volume
    pub fn new(
        lv_id: String,
        lv_name: String,
        vg_name: String,
        size: u64,
        filesystem: String,
    ) -> Self {
        Self {
            lv_id,
            lv_name,
            vg_name,
            size,
            state: LVState::Inactive,
            filesystem,
            mount_point: String::new(),
        }
    }

    /// Set logical volume state
    pub fn set_state(&mut self, state: LVState) {
        self.state = state;
    }

    /// Set mount point
    pub fn set_mount_point(&mut self, mount_point: String) {
        self.mount_point = mount_point;
    }

    /// Check if logical volume is active
    pub fn is_active(&self) -> bool {
        self.state == LVState::Active
    }
}

/// LVM manager
pub struct SigmaLVM {
    physical_volumes: BTreeMap<String, PhysicalVolume>,
    volume_groups: BTreeMap<String, VolumeGroup>,
    logical_volumes: BTreeMap<String, LogicalVolume>,
    running: AtomicBool,
}

impl SigmaLVM {
    /// Create a new LVM manager
    pub fn new() -> Self {
        Self {
            physical_volumes: BTreeMap::new(),
            volume_groups: BTreeMap::new(),
            logical_volumes: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the LVM manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the LVM manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Add a physical volume
    pub fn add_pv(&mut self, pv: PhysicalVolume) {
        self.physical_volumes.insert(pv.pv_id.clone(), pv);
    }

    /// Remove a physical volume
    pub fn remove_pv(&mut self, pv_id: &str) -> Option<PhysicalVolume> {
        self.physical_volumes.remove(pv_id)
    }

    /// Get a physical volume by ID
    pub fn get_pv(&self, pv_id: &str) -> Option<&PhysicalVolume> {
        self.physical_volumes.get(pv_id)
    }

    /// List all physical volumes
    pub fn list_pvs(&self) -> Vec<&PhysicalVolume> {
        self.physical_volumes.values().collect()
    }

    /// List physical volumes by volume group
    pub fn list_pvs_by_vg(&self, vg_name: &str) -> Vec<&PhysicalVolume> {
        self.physical_volumes
            .values()
            .filter(|pv| pv.vg_name == vg_name)
            .collect()
    }

    /// Create a volume group
    pub fn create_vg(&mut self, vg: VolumeGroup) {
        self.volume_groups.insert(vg.vg_name.clone(), vg);
    }

    /// Remove a volume group
    pub fn remove_vg(&mut self, vg_name: &str) -> Option<VolumeGroup> {
        self.volume_groups.remove(vg_name)
    }

    /// Get a volume group by name
    pub fn get_vg(&self, vg_name: &str) -> Option<&VolumeGroup> {
        self.volume_groups.get(vg_name)
    }

    /// List all volume groups
    pub fn list_vgs(&self) -> Vec<&VolumeGroup> {
        self.volume_groups.values().collect()
    }

    /// Add a physical volume to a volume group
    pub fn extend_vg(&mut self, vg_name: &str, pv_id: &str) -> bool {
        if let (Some(vg), Some(pv)) = (
            self.volume_groups.get_mut(vg_name),
            self.physical_volumes.get_mut(pv_id),
        ) {
            vg.size += pv.size;
            vg.free += pv.free;
            vg.pv_count += 1;
            pv.vg_name = vg_name.clone();
            pv.set_state(PVState::InUse);
            true
        } else {
            false
        }
    }

    /// Create a logical volume
    pub fn create_lv(&mut self, lv: LogicalVolume) {
        self.logical_volumes.insert(lv.lv_id.clone(), lv);
    }

    /// Remove a logical volume
    pub fn remove_lv(&mut self, lv_id: &str) -> Option<LogicalVolume> {
        self.logical_volumes.remove(lv_id)
    }

    /// Get a logical volume by ID
    pub fn get_lv(&self, lv_id: &str) -> Option<&LogicalVolume> {
        self.logical_volumes.get(lv_id)
    }

    /// List all logical volumes
    pub fn list_lvs(&self) -> Vec<&LogicalVolume> {
        self.logical_volumes.values().collect()
    }

    /// List logical volumes by volume group
    pub fn list_lvs_by_vg(&self, vg_name: &str) -> Vec<&LogicalVolume> {
        self.logical_volumes
            .values()
            .filter(|lv| lv.vg_name == vg_name)
            .collect()
    }

    /// Activate a logical volume
    pub fn activate_lv(&mut self, lv_id: &str) -> bool {
        if let Some(lv) = self.logical_volumes.get_mut(lv_id) {
            lv.set_state(LVState::Active);
            true
        } else {
            false
        }
    }

    /// Deactivate a logical volume
    pub fn deactivate_lv(&mut self, lv_id: &str) -> bool {
        if let Some(lv) = self.logical_volumes.get_mut(lv_id) {
            lv.set_state(LVState::Inactive);
            true
        } else {
            false
        }
    }

    /// Get counts
    pub fn pv_count(&self) -> usize {
        self.physical_volumes.len()
    }

    pub fn vg_count(&self) -> usize {
        self.volume_groups.len()
    }

    pub fn lv_count(&self) -> usize {
        self.logical_volumes.len()
    }
}

impl Default for SigmaLVM {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaLVM instance (placeholder for actual global state)
static unsafe mutable SIGMALVM_INSTANCE: Option<SigmaLVM> = None;

/// Initialize the global SigmaLVM instance
pub fn init_sigmalvm() {
    unsafe {
        SIGMALVM_INSTANCE = Some(SigmaLVM::new());
    }
}

/// Get the global SigmaLVM instance
pub fn get_sigmalvm() -> Option<&'static SigmaLVM> {
    unsafe {
        SIGMALVM_INSTANCE.as_ref()
    }
}

/// Get mutable global SigmaLVM instance
pub fn get_sigmalvm_mut() -> Option<&'static mut SigmaLVM> {
    unsafe {
        SIGMALVM_INSTANCE.as_mut()
    }
}

/// Convenience function to create a volume group
pub fn create_volume_group(vg: VolumeGroup) {
    if let Some(lvm) = get_sigmalvm_mut() {
        lvm.create_vg(vg);
    }
}

/// Convenience function to create a logical volume
pub fn create_logical_volume(lv: LogicalVolume) {
    if let Some(lvm) = get_sigmalvm_mut() {
        lvm.create_lv(lv);
    }
}

/// Convenience function to activate a logical volume
pub fn activate_logical_volume(lv_id: &str) -> bool {
    if let Some(lvm) = get_sigmalvm_mut() {
        lvm.activate_lv(lv_id)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pv_creation() {
        let pv = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "vg0".to_string(),
        );

        assert_eq!(pv.pv_id, "pv1");
        assert_eq!(pv.pv_name, "pv_sda");
        assert_eq!(pv.size, 1024 * 1024 * 1024);
        assert_eq!(pv.free, 1024 * 1024 * 1024);
        assert_eq!(pv.state, PVState::Available);
    }

    #[test]
    fn test_pv_allocation() {
        let mut pv = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "vg0".to_string(),
        );

        let allocated = pv.allocate(512 * 1024 * 1024);
        assert!(allocated);
        assert_eq!(pv.free, 512 * 1024 * 1024);
        assert_eq!(pv.used(), 512 * 1024 * 1024);
    }

    #[test]
    fn test_pv_free_space() {
        let mut pv = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "vg0".to_string(),
        );

        pv.allocate(512 * 1024 * 1024);
        pv.free_space(256 * 1024 * 1024);
        assert_eq!(pv.free, 768 * 1024 * 1024);
    }

    #[test]
    fn test_vg_creation() {
        let vg = VolumeGroup::new("vg1".to_string(), "vg0".to_string());

        assert_eq!(vg.vg_id, "vg1");
        assert_eq!(vg.vg_name, "vg0");
        assert_eq!(vg.size, 0);
        assert_eq!(vg.free, 0);
        assert_eq!(vg.state, VGState::Inactive);
    }

    #[test]
    fn test_vg_update() {
        let mut vg = VolumeGroup::new("vg1".to_string(), "vg0".to_string());

        vg.update_size(1024 * 1024 * 1024, 512 * 1024 * 1024);
        vg.update_counts(2, 3);

        assert_eq!(vg.size, 1024 * 1024 * 1024);
        assert_eq!(vg.free, 512 * 1024 * 1024);
        assert_eq!(vg.pv_count, 2);
        assert_eq!(vg.lv_count, 3);
    }

    #[test]
    fn test_lv_creation() {
        let lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );

        assert_eq!(lv.lv_id, "lv1");
        assert_eq!(lv.lv_name, "lv_root");
        assert_eq!(lv.vg_name, "vg0");
        assert_eq!(lv.size, 512 * 1024 * 1024);
        assert_eq!(lv.state, LVState::Inactive);
    }

    #[test]
    fn test_lv_activation() {
        let mut lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );

        lv.set_state(LVState::Active);
        assert!(lv.is_active());

        lv.set_state(LVState::Inactive);
        assert!(!lv.is_active());
    }

    #[test]
    fn test_lvm_creation() {
        let lvm = SigmaLVM::new();
        assert!(!lvm.is_running());
        assert_eq!(lvm.pv_count(), 0);
        assert_eq!(lvm.vg_count(), 0);
        assert_eq!(lvm.lv_count(), 0);
    }

    #[test]
    fn test_lvm_start_stop() {
        let mut lvm = SigmaLVM::new();
        lvm.start();
        assert!(lvm.is_running());
        lvm.stop();
        assert!(!lvm.is_running());
    }

    #[test]
    fn test_lvm_add_pv() {
        let mut lvm = SigmaLVM::new();
        let pv = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "vg0".to_string(),
        );

        lvm.add_pv(pv);
        assert_eq!(lvm.pv_count(), 1);
        assert!(lvm.get_pv("pv1").is_some());
    }

    #[test]
    fn test_lvm_create_vg() {
        let mut lvm = SigmaLVM::new();
        let vg = VolumeGroup::new("vg1".to_string(), "vg0".to_string());

        lvm.create_vg(vg);
        assert_eq!(lvm.vg_count(), 1);
        assert!(lvm.get_vg("vg0").is_some());
    }

    #[test]
    fn test_lvm_extend_vg() {
        let mut lvm = SigmaLVM::new();

        let mut vg = VolumeGroup::new("vg1".to_string(), "vg0".to_string());
        vg.update_size(512 * 1024 * 1024, 512 * 1024 * 1024);
        lvm.create_vg(vg);

        let pv = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            512 * 1024 * 1024,
            "".to_string(),
        );
        lvm.add_pv(pv);

        let extended = lvm.extend_vg("vg0", "pv1");
        assert!(extended);

        let vg = lvm.get_vg("vg0").unwrap();
        assert_eq!(vg.size, 1024 * 1024 * 1024);
        assert_eq!(vg.pv_count, 1);
    }

    #[test]
    fn test_lvm_create_lv() {
        let mut lvm = SigmaLVM::new();
        let lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );

        lvm.create_lv(lv);
        assert_eq!(lvm.lv_count(), 1);
        assert!(lvm.get_lv("lv1").is_some());
    }

    #[test]
    fn test_lvm_activate_lv() {
        let mut lvm = SigmaLVM::new();
        let lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );

        lvm.create_lv(lv);
        let activated = lvm.activate_lv("lv1");
        assert!(activated);

        let lv = lvm.get_lv("lv1").unwrap();
        assert!(lv.is_active());
    }

    #[test]
    fn test_lvm_list_pvs_by_vg() {
        let mut lvm = SigmaLVM::new();

        let mut pv1 = PhysicalVolume::new(
            "pv1".to_string(),
            "pv_sda".to_string(),
            "/dev/sda".to_string(),
            512 * 1024 * 1024,
            "vg0".to_string(),
        );
        lvm.add_pv(pv1);

        let mut pv2 = PhysicalVolume::new(
            "pv2".to_string(),
            "pv_sdb".to_string(),
            "/dev/sdb".to_string(),
            512 * 1024 * 1024,
            "vg1".to_string(),
        );
        lvm.add_pv(pv2);

        let vg0_pvs = lvm.list_pvs_by_vg("vg0");
        assert_eq!(vg0_pvs.len(), 1);

        let vg1_pvs = lvm.list_pvs_by_vg("vg1");
        assert_eq!(vg1_pvs.len(), 1);
    }

    #[test]
    fn test_lvm_list_lvs_by_vg() {
        let mut lvm = SigmaLVM::new();

        lvm.create_lv(LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        ));

        lvm.create_lv(LogicalVolume::new(
            "lv2".to_string(),
            "lv_home".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        ));

        lvm.create_lv(LogicalVolume::new(
            "lv3".to_string(),
            "lv_swap".to_string(),
            "vg1".to_string(),
            512 * 1024 * 1024,
            "swap".to_string(),
        ));

        let vg0_lvs = lvm.list_lvs_by_vg("vg0");
        assert_eq!(vg0_lvs.len(), 2);

        let vg1_lvs = lvm.list_lvs_by_vg("vg1");
        assert_eq!(vg1_lvs.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmalvm();

        let lvm = get_sigmalvm();
        assert!(lvm.is_some());

        let mut lvm_mut = get_sigmalvm_mut();
        assert!(lvm_mut.is_some());

        if let Some(lvm) = lvm_mut {
            lvm.start();
            assert!(lvm.is_running());
        }
    }

    #[test]
    fn test_convenience_create_vg() {
        init_sigmalvm();

        let vg = VolumeGroup::new("vg1".to_string(), "vg0".to_string());
        create_volume_group(vg);

        let lvm = get_sigmalvm().unwrap();
        assert_eq!(lvm.vg_count(), 1);
    }

    #[test]
    fn test_convenience_create_lv() {
        init_sigmalvm();

        let lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );
        create_logical_volume(lv);

        let lvm = get_sigmalvm().unwrap();
        assert_eq!(lvm.lv_count(), 1);
    }

    #[test]
    fn test_convenience_activate_lv() {
        init_sigmalvm();

        let lv = LogicalVolume::new(
            "lv1".to_string(),
            "lv_root".to_string(),
            "vg0".to_string(),
            512 * 1024 * 1024,
            "ext4".to_string(),
        );
        create_logical_volume(lv);

        let activated = activate_logical_volume("lv1");
        assert!(activated);

        let lvm = get_sigmalvm().unwrap();
        let lv = lvm.get_lv("lv1").unwrap();
        assert!(lv.is_active());
    }
}
