// SPDX-License-Identifier: MIT
// SigmaOS - Hardware Manager
// Hardware detection and management inspired by Omarchy hw-* commands and Linux hardware systems

#![no_std]
extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Hardware type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareType {
    CPU,
    GPU,
    Audio,
    Network,
    Bluetooth,
    Storage,
}

/// Hardware device
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub id: String,
    pub name: String,
    pub hardware_type: HardwareType,
    pub vendor: String,
    pub model: String,
    pub driver: Option<String>,
}

/// Hardware manager
pub struct SigmaHardwareManager {
    devices: Vec<HardwareDevice>,
}

impl SigmaHardwareManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: HardwareDevice) {
        self.devices.push(device);
    }

    pub fn get_devices(&self) -> &[HardwareDevice] {
        &self.devices
    }

    pub fn get_device(&self, id: &str) -> Option<&HardwareDevice> {
        self.devices.iter().find(|d| d.id == id)
    }

    pub fn get_devices_by_type(&self, hardware_type: HardwareType) -> Vec<&HardwareDevice> {
        self.devices.iter().filter(|d| d.hardware_type == hardware_type).collect()
    }

    pub fn detect_asus_rog(&self) -> bool {
        self.devices.iter().any(|d| d.vendor.contains("ASUS") && d.model.contains("ROG"))
    }

    pub fn get_vendor_devices(&self, vendor: &str) -> Vec<&HardwareDevice> {
        self.devices.iter().filter(|d| d.vendor.contains(vendor)).collect()
    }
}

impl Default for SigmaHardwareManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_manager_creation() {
        let manager = SigmaHardwareManager::new();
        assert_eq!(manager.get_devices().len(), 0);
    }

    #[test]
    fn test_add_device() {
        let mut manager = SigmaHardwareManager::new();
        let device = HardwareDevice {
            id: "cpu0".to_string(),
            name: "CPU 0".to_string(),
            hardware_type: HardwareType::CPU,
            vendor: "Intel".to_string(),
            model: "i7-12700K".to_string(),
            driver: Some("intel_pstate".to_string()),
        };
        manager.add_device(device);
        assert_eq!(manager.get_devices().len(), 1);
    }

    #[test]
    fn test_devices_by_type() {
        let mut manager = SigmaHardwareManager::new();
        let cpu = HardwareDevice {
            id: "cpu0".to_string(),
            name: "CPU 0".to_string(),
            hardware_type: HardwareType::CPU,
            vendor: "Intel".to_string(),
            model: "i7-12700K".to_string(),
            driver: Some("intel_pstate".to_string()),
        };
        let gpu = HardwareDevice {
            id: "gpu0".to_string(),
            name: "GPU 0".to_string(),
            hardware_type: HardwareType::GPU,
            vendor: "NVIDIA".to_string(),
            model: "RTX 3080".to_string(),
            driver: Some("nvidia".to_string()),
        };
        manager.add_device(cpu);
        manager.add_device(gpu);
        assert_eq!(manager.get_devices_by_type(HardwareType::CPU).len(), 1);
    }

    #[test]
    fn test_detect_asus_rog() {
        let mut manager = SigmaHardwareManager::new();
        let device = HardwareDevice {
            id: "mobo0".to_string(),
            name: "Motherboard".to_string(),
            hardware_type: HardwareType::CPU,
            vendor: "ASUS".to_string(),
            model: "ROG STRIX".to_string(),
            driver: None,
        };
        manager.add_device(device);
        assert!(manager.detect_asus_rog());
    }

    #[test]
    fn test_vendor_devices() {
        let mut manager = SigmaHardwareManager::new();
        let device1 = HardwareDevice {
            id: "cpu0".to_string(),
            name: "CPU 0".to_string(),
            hardware_type: HardwareType::CPU,
            vendor: "Intel".to_string(),
            model: "i7-12700K".to_string(),
            driver: Some("intel_pstate".to_string()),
        };
        let device2 = HardwareDevice {
            id: "gpu0".to_string(),
            name: "GPU 0".to_string(),
            hardware_type: HardwareType::GPU,
            vendor: "NVIDIA".to_string(),
            model: "RTX 3080".to_string(),
            driver: Some("nvidia".to_string()),
        };
        manager.add_device(device1);
        manager.add_device(device2);
        assert_eq!(manager.get_vendor_devices("Intel").len(), 1);
    }
}
