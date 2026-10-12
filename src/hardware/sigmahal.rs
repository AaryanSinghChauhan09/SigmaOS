// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaHAL - Hardware Abstraction Layer
//!
//! SigmaHAL provides hardware abstraction for SigmaOS, inspired by Linux HAL and systemd-hwdb.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Unknown device
    Unknown,
    /// PCI device
    PCI,
    /// USB device
    USB,
    /// Network device
    Network,
    /// Storage device
    Storage,
    /// Audio device
    Audio,
    /// Video device
    Video,
    /// Input device
    Input,
    /// Bluetooth device
    Bluetooth,
    /// Printer device
    Printer,
    /// Scanner device
    Scanner,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Unknown => write!(f, "unknown"),
            DeviceType::PCI => write!(f, "pci"),
            DeviceType::USB => write!(f, "usb"),
            DeviceType::Network => write!(f, "network"),
            DeviceType::Storage => write!(f, "storage"),
            DeviceType::Audio => write!(f, "audio"),
            DeviceType::Video => write!(f, "video"),
            DeviceType::Input => write!(f, "input"),
            DeviceType::Bluetooth => write!(f, "bluetooth"),
            DeviceType::Printer => write!(f, "printer"),
            DeviceType::Scanner => write!(f, "scanner"),
        }
    }
}

/// Device status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceStatus {
    /// Device is present
    Present,
    /// Device is absent
    Absent,
    /// Device is active
    Active,
    /// Device is inactive
    Inactive,
    /// Device has error
    Error,
}

impl fmt::Display for DeviceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceStatus::Present => write!(f, "present"),
            DeviceStatus::Absent => write!(f, "absent"),
            DeviceStatus::Active => write!(f, "active"),
            DeviceStatus::Inactive => write!(f, "inactive"),
            DeviceStatus::Error => write!(f, "error"),
        }
    }
}

/// Hardware device
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    /// Device ID
    pub device_id: String,
    /// Device type
    pub device_type: DeviceType,
    /// Device name
    pub name: String,
    /// Vendor
    pub vendor: String,
    /// Model
    pub model: String,
    /// Device status
    pub status: DeviceStatus,
    /// Device path
    pub path: String,
    /// Driver name
    pub driver: Option<String>,
    /// Properties
    pub properties: BTreeMap<String, String>,
}

impl HardwareDevice {
    /// Create a new hardware device
    pub fn new(device_id: String, device_type: DeviceType, name: String) -> Self {
        Self {
            device_id,
            device_type,
            name,
            vendor: String::new(),
            model: String::new(),
            status: DeviceStatus::Present,
            path: String::new(),
            driver: None,
            properties: BTreeMap::new(),
        }
    }

    /// Set vendor
    pub fn with_vendor(mut self, vendor: String) -> Self {
        self.vendor = vendor;
        self
    }

    /// Set model
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    /// Set status
    pub fn with_status(mut self, status: DeviceStatus) -> Self {
        self.status = status;
        self
    }

    /// Set path
    pub fn with_path(mut self, path: String) -> Self {
        self.path = path;
        self
    }

    /// Set driver
    pub fn with_driver(mut self, driver: String) -> Self {
        self.driver = Some(driver);
        self
    }

    /// Add property
    pub fn with_property(mut self, key: String, value: String) -> Self {
        self.properties.insert(key, value);
        self
    }
}

/// SigmaHAL - Hardware Abstraction Layer
pub struct SigmaHAL {
    /// Hardware devices (device_id -> device)
    devices: BTreeMap<String, HardwareDevice>,
    /// Running state
    running: bool,
}

impl SigmaHAL {
    /// Create a new SigmaHAL instance
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            running: false,
        }
    }

    /// Start the HAL daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the HAL daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a hardware device
    pub fn add_device(&mut self, device: HardwareDevice) -> Result<(), String> {
        if self.devices.contains_key(&device.device_id) {
            return Err(format!("Device '{}' already exists", device.device_id));
        }
        self.devices.insert(device.device_id.clone(), device);
        Ok(())
    }

    /// Remove a hardware device
    pub fn remove_device(&mut self, device_id: &str) -> Result<(), String> {
        if self.devices.remove(device_id).is_none() {
            return Err(format!("Device '{}' not found", device_id));
        }
        Ok(())
    }

    /// Get a device
    pub fn get_device(&self, device_id: &str) -> Option<&HardwareDevice> {
        self.devices.get(device_id)
    }

    /// Get a device (mutable)
    pub fn get_device_mut(&mut self, device_id: &str) -> Option<&mut HardwareDevice> {
        self.devices.get_mut(device_id)
    }

    /// List all devices
    pub fn list_devices(&self) -> Vec<&HardwareDevice> {
        self.devices.values().collect()
    }

    /// List devices by type
    pub fn list_devices_by_type(&self, device_type: DeviceType) -> Vec<&HardwareDevice> {
        self.devices.values().filter(|d| d.device_type == device_type).collect()
    }

    /// List active devices
    pub fn list_active_devices(&self) -> Vec<&HardwareDevice> {
        self.devices.values().filter(|d| d.status == DeviceStatus::Active).collect()
    }

    /// Update device status
    pub fn update_device_status(&mut self, device_id: &str, status: DeviceStatus) -> Result<(), String> {
        match self.devices.get_mut(device_id) {
            Some(device) => {
                device.status = status;
                Ok(())
            }
            None => Err(format!("Device '{}' not found", device_id)),
        }
    }

    /// Get device count
    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    /// Get device count by type
    pub fn device_count_by_type(&self, device_type: DeviceType) -> usize {
        self.devices.values().filter(|d| d.device_type == device_type).count()
    }

    /// Get HAL statistics
    pub fn hal_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_devices".to_string(), self.device_count().to_string());
        stats.insert("pci_devices".to_string(), self.device_count_by_type(DeviceType::PCI).to_string());
        stats.insert("usb_devices".to_string(), self.device_count_by_type(DeviceType::USB).to_string());
        stats.insert("network_devices".to_string(), self.device_count_by_type(DeviceType::Network).to_string());
        stats.insert("storage_devices".to_string(), self.device_count_by_type(DeviceType::Storage).to_string());
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaHAL {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaHAL instance (placeholder for actual global state)
static unsafe mut SIGMAHAL_INSTANCE: Option<SigmaHAL> = None;

/// Initialize the global SigmaHAL instance
pub fn init_sigmahal() {
    unsafe {
        SIGMAHAL_INSTANCE = Some(SigmaHAL::new());
    }
}

/// Get the global SigmaHAL instance
pub fn get_sigmahal() -> Option<&'static mut SigmaHAL> {
    unsafe { SIGMAHAL_INSTANCE.as_mut() }
}

/// Convenience function to add a device
pub fn add_hardware_device(device: HardwareDevice) -> Result<(), String> {
    if let Some(hal) = get_sigmahal() {
        hal.add_device(device)
    } else {
        Err("SigmaHAL not initialized".to_string())
    }
}

/// Convenience function to update device status
pub fn update_device_status(device_id: &str, status: DeviceStatus) -> Result<(), String> {
    if let Some(hal) = get_sigmahal() {
        hal.update_device_status(device_id, status)
    } else {
        Err("SigmaHAL not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_type_display() {
        assert_eq!(DeviceType::PCI.to_string(), "pci");
        assert_eq!(DeviceType::USB.to_string(), "usb");
    }

    #[test]
    fn test_device_status_display() {
        assert_eq!(DeviceStatus::Active.to_string(), "active");
        assert_eq!(DeviceStatus::Error.to_string(), "error");
    }

    #[test]
    fn test_hardware_device() {
        let device = HardwareDevice::new("pci-0000:00:1f.0".to_string(), DeviceType::PCI, "Intel HD Audio".to_string())
            .with_vendor("Intel".to_string())
            .with_model("HD Audio".to_string())
            .with_status(DeviceStatus::Active)
            .with_path("/sys/devices/pci0000:00/0000:00:1f.0".to_string())
            .with_driver("snd_hda_intel".to_string())
            .with_property("class".to_string(), "audio".to_string());

        assert_eq!(device.device_id, "pci-0000:00:1f.0");
        assert_eq!(device.device_type, DeviceType::PCI);
        assert_eq!(device.vendor, "Intel");
        assert_eq!(device.driver, Some("snd_hda_intel".to_string()));
    }

    #[test]
    fn test_sigmahal() {
        let mut hal = SigmaHAL::new();
        assert!(!hal.is_running());
        hal.start();
        assert!(hal.is_running());
        hal.stop();
        assert!(!hal.is_running());
    }

    #[test]
    fn test_sigmahal_add_device() {
        let mut hal = SigmaHAL::new();
        
        let device = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device".to_string());
        assert!(hal.add_device(device).is_ok());
        assert_eq!(hal.device_count(), 1);
    }

    #[test]
    fn test_sigmahal_duplicate_device() {
        let mut hal = SigmaHAL::new();
        
        let device1 = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device".to_string());
        let device2 = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device 2".to_string());
        
        assert!(hal.add_device(device1).is_ok());
        assert!(hal.add_device(device2).is_err());
    }

    #[test]
    fn test_sigmahal_list_devices() {
        let mut hal = SigmaHAL::new();
        
        let device1 = HardwareDevice::new("device1".to_string(), DeviceType::PCI, "Device 1".to_string());
        let device2 = HardwareDevice::new("device2".to_string(), DeviceType::USB, "Device 2".to_string());
        
        hal.add_device(device1).unwrap();
        hal.add_device(device2).unwrap();
        
        let devices = hal.list_devices();
        assert_eq!(devices.len(), 2);
    }

    #[test]
    fn test_sigmahal_list_by_type() {
        let mut hal = SigmaHAL::new();
        
        let device1 = HardwareDevice::new("device1".to_string(), DeviceType::PCI, "Device 1".to_string());
        let device2 = HardwareDevice::new("device2".to_string(), DeviceType::USB, "Device 2".to_string());
        let device3 = HardwareDevice::new("device3".to_string(), DeviceType::PCI, "Device 3".to_string());
        
        hal.add_device(device1).unwrap();
        hal.add_device(device2).unwrap();
        hal.add_device(device3).unwrap();
        
        let pci_devices = hal.list_devices_by_type(DeviceType::PCI);
        assert_eq!(pci_devices.len(), 2);
    }

    #[test]
    fn test_sigmahal_update_status() {
        let mut hal = SigmaHAL::new();
        
        let device = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device".to_string());
        hal.add_device(device).unwrap();
        
        assert!(hal.update_device_status("test-device", DeviceStatus::Active).is_ok());
        
        let device = hal.get_device("test-device").unwrap();
        assert_eq!(device.status, DeviceStatus::Active);
    }

    #[test]
    fn test_sigmahal_stats() {
        let mut hal = SigmaHAL::new();
        
        let device1 = HardwareDevice::new("pci-device".to_string(), DeviceType::PCI, "PCI Device".to_string());
        let device2 = HardwareDevice::new("usb-device".to_string(), DeviceType::USB, "USB Device".to_string());
        
        hal.add_device(device1).unwrap();
        hal.add_device(device2).unwrap();
        
        let stats = hal.hal_stats();
        assert_eq!(stats.get("total_devices"), Some(&"2".to_string()));
        assert_eq!(stats.get("pci_devices"), Some(&"1".to_string()));
        assert_eq!(stats.get("usb_devices"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_hardware_device_convenience() {
        init_sigmahal();
        
        let device = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device".to_string());
        let result = add_hardware_device(device);
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_device_status_convenience() {
        init_sigmahal();
        
        let device = HardwareDevice::new("test-device".to_string(), DeviceType::Unknown, "Test Device".to_string());
        add_hardware_device(device).unwrap();
        
        let result = update_device_status("test-device", DeviceStatus::Active);
        assert!(result.is_ok());
    }
}
