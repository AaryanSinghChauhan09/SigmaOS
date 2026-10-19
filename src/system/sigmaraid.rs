//! SigmaRAID - RAID Manager
//!
//! Inspired by Linux mdadm, this module provides comprehensive RAID array
//! management including RAID levels 0, 1, 5, 6, 10, and array monitoring.

#![no_std]

use alloc::collections::BTreeMap;
use alloc::string::String;
use core::sync::atomic::{AtomicBool, Ordering};

/// RAID level enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RAIDLevel {
    RAID0,
    RAID1,
    RAID5,
    RAID6,
    RAID10,
    Linear,
    Unknown,
;

/// RAID array state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RAIDState {
    Active,
    Inactive,
    Degraded,
    Failed,
    Resyncing,
    Recovering,
    Unknown,
;

/// RAID device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RAIDDeviceState {
    InSync,
    Failed,
    Spare,
    Rebuilding,
    Removed,
    Unknown,
;

/// RAID device data structure
#[derive(Debug, Clone)]
pub struct RAIDDevice {
    pub device_id: String,
    pub device_path: String,
    pub state: RAIDDeviceState,
    pub size: u64,
    pub array_id: String,
;

impl RAIDDevice {
    /// Create a new RAID device
    pub fn new(device_id: String, device_path: String, size: u64, array_id: String) -> Self {
        Self {
            device_id,
            device_path,
            state: RAIDDeviceState::InSync,
            size,
            array_id,
        }
    }

    /// Set device state
    pub fn set_state(&mut self, state: RAIDDeviceState) {
        self.state = state;
    }

    /// Check if device is in sync
    pub fn is_in_sync(&self) -> bool {
        self.state == RAIDDeviceState::InSync
    }

    /// Check if device is failed
    pub fn is_failed(&self) -> bool {
        self.state == RAIDDeviceState::Failed
    }
;

/// RAID array data structure
#[derive(Debug, Clone)]
pub struct RAIDArray {
    pub array_id: String,
    pub array_name: String,
    pub raid_level: RAIDLevel,
    pub state: RAIDState,
    pub size: u64,
    pub device_count: usize,
    pub active_devices: usize,
    pub spare_devices: usize,
    pub chunk_size: u64,
;

impl RAIDArray {
    /// Create a new RAID array
    pub fn new(
        array_id: String,
        array_name: String,
        raid_level: RAIDLevel,
        chunk_size: u64,
    ) -> Self {
        Self {
            array_id,
            array_name,
            raid_level,
            state: RAIDState::Inactive,
            size: 0,
            device_count: 0,
            active_devices: 0,
            spare_devices: 0,
            chunk_size,
        }
    }

    /// Set array state
    pub fn set_state(&mut self, state: RAIDState) {
        self.state = state;
    }

    /// Update size
    pub fn update_size(&mut self, size: u64) {
        self.size = size;
    }

    /// Update device counts
    pub fn update_counts(
        &mut self,
        device_count: usize,
        active_devices: usize,
        spare_devices: usize,
    ) {
        self.device_count = device_count;
        self.active_devices = active_devices;
        self.spare_devices = spare_devices;
    }

    /// Check if array is active
    pub fn is_active(&self) -> bool {
        self.state == RAIDState::Active
    }

    /// Check if array is degraded
    pub fn is_degraded(&self) -> bool {
        self.state == RAIDState::Degraded
    }

    /// Check if array is failed
    pub fn is_failed(&self) -> bool {
        self.state == RAIDState::Failed
    }
;

/// RAID manager
pub struct SigmaRAID {
    arrays: BTreeMap<String, RAIDArray>,
    devices: BTreeMap<String, RAIDDevice>,
    running: AtomicBool,
;

impl SigmaRAID {
    /// Create a new RAID manager
    pub fn new() -> Self {
        Self {
            arrays: BTreeMap::new(),
            devices: BTreeMap::new(),
            running: AtomicBool::new(false),
        }
    }

    /// Start the RAID manager
    pub fn start(&mut self) {
        self.running.store(true, Ordering::SeqCst);
    }

    /// Stop the RAID manager
    pub fn stop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
    }

    /// Check if the manager is running
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::SeqCst)
    }

    /// Create a RAID array
    pub fn create_array(&mut self, array: RAIDArray) {
        self.arrays.insert(array.array_id.clone(), array);
    }

    /// Remove a RAID array
    pub fn remove_array(&mut self, array_id: &str) -> Option<RAIDArray> {
        self.arrays.remove(array_id)
    }

    /// Get a RAID array by ID
    pub fn get_array(&self, array_id: &str) -> Option<&RAIDArray> {
        self.arrays.get(array_id)
    }

    /// List all RAID arrays
    pub fn list_arrays(&self) -> Vec<&RAIDArray> {
        self.arrays.values().collect()
    }

    /// List arrays by RAID level
    pub fn list_arrays_by_level(&self, raid_level: RAIDLevel) -> Vec<&RAIDArray> {
        self.arrays
            .values()
            .filter(|a| a.raid_level == raid_level)
            .collect()
    }

    /// List arrays by state
    pub fn list_arrays_by_state(&self, state: RAIDState) -> Vec<&RAIDArray> {
        self.arrays.values().filter(|a| a.state == state).collect()
    }

    /// Activate a RAID array
    pub fn activate_array(&mut self, array_id: &str) -> bool {
        if let Some(array) = self.arrays.get_mut(array_id) {
            array.set_state(RAIDState::Active);
            true
        } else {
            false
        }
    }

    /// Deactivate a RAID array
    pub fn deactivate_array(&mut self, array_id: &str) -> bool {
        if let Some(array) = self.arrays.get_mut(array_id) {
            array.set_state(RAIDState::Inactive);
            true
        } else {
            false
        }
    }

    /// Add a device to a RAID array
    pub fn add_device(&mut self, device: RAIDDevice) {
        self.devices.insert(device.device_id.clone(), device);
    }

    /// Remove a device from a RAID array
    pub fn remove_device(&mut self, device_id: &str) -> Option<RAIDDevice> {
        self.devices.remove(device_id)
    }

    /// Get a device by ID
    pub fn get_device(&self, device_id: &str) -> Option<&RAIDDevice> {
        self.devices.get(device_id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&RAIDDevice> {
        self.devices.values().collect()
    }

    /// List devices by array
    pub fn list_devices_by_array(&self, array_id: &str) -> Vec<&RAIDDevice> {
        self.devices
            .values()
            .filter(|d| d.array_id == array_id)
            .collect()
    }

    /// List devices by state
    pub fn list_devices_by_state(&self, state: RAIDDeviceState) -> Vec<&RAIDDevice> {
        self.devices.values().filter(|d| d.state == state).collect()
    }

    /// Mark a device as failed
    pub fn mark_device_failed(&mut self, device_id: &str) -> bool {
        if let Some(device) = self.devices.get_mut(device_id) {
            device.set_state(RAIDDeviceState::Failed);
            true
        } else {
            false
        }
    }

    /// Replace a failed device
    pub fn replace_device(&mut self, old_device_id: &str, new_device: RAIDDevice) -> bool {
        if self.devices.remove(old_device_id).is_some() {
            self.devices
                .insert(new_device.device_id.clone(), new_device);
            true
        } else {
            false
        }
    }

    /// Get counts
    pub fn array_count(&self) -> usize {
        self.arrays.len()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }
;

impl Default for SigmaRAID {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaRAID instance (placeholder for actual global state)
static mut SIGMARAD_INSTANCE: Option<SigmaRAID> = None;

/// Initialize the global SigmaRAID instance
pub fn init_sigmaraid() {
    unsafe {
        SIGMARAD_INSTANCE = Some(SigmaRAID::new());
    }
;

/// Get the global SigmaRAID instance
pub fn get_sigmaraid() -> Option<&'static SigmaRAID> {
    unsafe { SIGMARAD_INSTANCE.as_ref() }
;

/// Get mutable global SigmaRAID instance
pub fn get_sigmaraid_mut() -> Option<&'static mut SigmaRAID> {
    unsafe { SIGMARAD_INSTANCE.as_mut() }
;

/// Convenience function to create a RAID array
pub fn create_raid_array(array: RAIDArray) {
    if let Some(raid) = get_sigmaraid_mut() {
        raid.create_array(array);
    }
;

/// Convenience function to activate a RAID array
pub fn activate_raid_array(array_id: &str) -> bool {
    if let Some(raid) = get_sigmaraid_mut() {
        raid.activate_array(array_id)
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raid_device_creation() {
        let device = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        assert_eq!(device.device_id, "dev1");
        assert_eq!(device.device_path, "/dev/sda");
        assert_eq!(device.size, 1024 * 1024 * 1024);
        assert_eq!(device.state, RAIDDeviceState::InSync);
    }

    #[test]
    fn test_raid_device_state() {
        let mut device = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        device.set_state(RAIDDeviceState::Failed);
        assert!(device.is_failed());
        assert!(!device.is_in_sync());
    }

    #[test]
    fn test_raid_array_creation() {
        let array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );

        assert_eq!(array.array_id, "array1");
        assert_eq!(array.array_name, "md0");
        assert_eq!(array.raid_level, RAIDLevel::RAID1);
        assert_eq!(array.state, RAIDState::Inactive);
    }

    #[test]
    fn test_raid_array_update() {
        let mut array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );

        array.update_size(1024 * 1024 * 1024);
        array.update_counts(2, 2, 0);

        assert_eq!(array.size, 1024 * 1024 * 1024);
        assert_eq!(array.device_count, 2);
        assert_eq!(array.active_devices, 2);
    }

    #[test]
    fn test_raid_array_activation() {
        let mut array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );

        array.set_state(RAIDState::Active);
        assert!(array.is_active());

        array.set_state(RAIDState::Degraded);
        assert!(array.is_degraded());

        array.set_state(RAIDState::Failed);
        assert!(array.is_failed());
    }

    #[test]
    fn test_raid_manager_creation() {
        let raid = SigmaRAID::new();
        assert!(!raid.is_running());
        assert_eq!(raid.array_count(), 0);
        assert_eq!(raid.device_count(), 0);
    }

    #[test]
    fn test_raid_manager_start_stop() {
        let mut raid = SigmaRAID::new();
        raid.start();
        assert!(raid.is_running());
        raid.stop();
        assert!(!raid.is_running());
    }

    #[test]
    fn test_raid_manager_create_array() {
        let mut raid = SigmaRAID::new();
        let array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );

        raid.create_array(array);
        assert_eq!(raid.array_count(), 1);
        assert!(raid.get_array("array1").is_some());
    }

    #[test]
    fn test_raid_manager_activate_array() {
        let mut raid = SigmaRAID::new();
        let array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );

        raid.create_array(array);
        let activated = raid.activate_array("array1");
        assert!(activated);

        let array = raid.get_array("array1").unwrap();
        assert!(array.is_active());
    }

    #[test]
    fn test_raid_manager_add_device() {
        let mut raid = SigmaRAID::new();
        let device = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        raid.add_device(device);
        assert_eq!(raid.device_count(), 1);
        assert!(raid.get_device("dev1").is_some());
    }

    #[test]
    fn test_raid_manager_mark_device_failed() {
        let mut raid = SigmaRAID::new();
        let device = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        raid.add_device(device);
        let marked = raid.mark_device_failed("dev1");
        assert!(marked);

        let device = raid.get_device("dev1").unwrap();
        assert!(device.is_failed());
    }

    #[test]
    fn test_raid_manager_replace_device() {
        let mut raid = SigmaRAID::new();
        let device = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        raid.add_device(device);

        let new_device = RAIDDevice::new(
            "dev2".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );

        let replaced = raid.replace_device("dev1", new_device);
        assert!(replaced);

        assert!(raid.get_device("dev1").is_none());
        assert!(raid.get_device("dev2").is_some());
    }

    #[test]
    fn test_raid_manager_list_arrays_by_level() {
        let mut raid = SigmaRAID::new();

        raid.create_array(RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        ));

        raid.create_array(RAIDArray::new(
            "array2".to_string(),
            "md1".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        ));

        raid.create_array(RAIDArray::new(
            "array3".to_string(),
            "md2".to_string(),
            RAIDLevel::RAID5,
            512 * 1024,
        ));

        let raid1_arrays = raid.list_arrays_by_level(RAIDLevel::RAID1);
        assert_eq!(raid1_arrays.len(), 2);

        let raid5_arrays = raid.list_arrays_by_level(RAIDLevel::RAID5);
        assert_eq!(raid5_arrays.len(), 1);
    }

    #[test]
    fn test_raid_manager_list_arrays_by_state() {
        let mut raid = SigmaRAID::new();

        let mut array1 = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );
        array1.set_state(RAIDState::Active);
        raid.create_array(array1);

        let array2 = RAIDArray::new(
            "array2".to_string(),
            "md1".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );
        raid.create_array(array2);

        let active_arrays = raid.list_arrays_by_state(RAIDState::Active);
        assert_eq!(active_arrays.len(), 1);

        let inactive_arrays = raid.list_arrays_by_state(RAIDState::Inactive);
        assert_eq!(inactive_arrays.len(), 1);
    }

    #[test]
    fn test_raid_manager_list_devices_by_array() {
        let mut raid = SigmaRAID::new();

        raid.add_device(RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        ));

        raid.add_device(RAIDDevice::new(
            "dev2".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        ));

        raid.add_device(RAIDDevice::new(
            "dev3".to_string(),
            "/dev/sdc".to_string(),
            1024 * 1024 * 1024,
            "array2".to_string(),
        ));

        let array1_devices = raid.list_devices_by_array("array1");
        assert_eq!(array1_devices.len(), 2);

        let array2_devices = raid.list_devices_by_array("array2");
        assert_eq!(array2_devices.len(), 1);
    }

    #[test]
    fn test_raid_manager_list_devices_by_state() {
        let mut raid = SigmaRAID::new();

        let mut device1 = RAIDDevice::new(
            "dev1".to_string(),
            "/dev/sda".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );
        device1.set_state(RAIDDeviceState::Failed);
        raid.add_device(device1);

        let device2 = RAIDDevice::new(
            "dev2".to_string(),
            "/dev/sdb".to_string(),
            1024 * 1024 * 1024,
            "array1".to_string(),
        );
        raid.add_device(device2);

        let failed_devices = raid.list_devices_by_state(RAIDDeviceState::Failed);
        assert_eq!(failed_devices.len(), 1);

        let sync_devices = raid.list_devices_by_state(RAIDDeviceState::InSync);
        assert_eq!(sync_devices.len(), 1);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaraid();

        let raid = get_sigmaraid();
        assert!(raid.is_some());

        let mut raid_mut = get_sigmaraid_mut();
        assert!(raid_mut.is_some());

        if let Some(raid) = raid_mut {
            raid.start();
            assert!(raid.is_running());
        }
    }

    #[test]
    fn test_convenience_create_array() {
        init_sigmaraid();

        let array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );
        create_raid_array(array);

        let raid = get_sigmaraid().unwrap();
        assert_eq!(raid.array_count(), 1);
    }

    #[test]
    fn test_convenience_activate_array() {
        init_sigmaraid();

        let array = RAIDArray::new(
            "array1".to_string(),
            "md0".to_string(),
            RAIDLevel::RAID1,
            512 * 1024,
        );
        create_raid_array(array);

        let activated = activate_raid_array("array1");
        assert!(activated);

        let raid = get_sigmaraid().unwrap();
        let array = raid.get_array("array1").unwrap();
        assert!(array.is_active());
    }
;
