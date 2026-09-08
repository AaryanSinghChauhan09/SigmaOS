//! Linux Mint mintdrivers-inspired Driver Manager
//! 
//! This module implements a driver manager inspired by Linux Mint's mintdrivers,
//! which handles installation and management of proprietary and open-source drivers.

#![no_std]
#![allow(dead_code)]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

/// Driver type classification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    /// Open-source driver
    OpenSource,
    /// Proprietary driver
    Proprietary,
    /// Community-maintained driver
    Community,
    /// Built-in kernel driver
    BuiltIn,
}

impl DriverType {
    /// Get display name for the driver type
    pub fn display_name(&self) -> &'static str {
        match self {
            DriverType::OpenSource => "Open Source",
            DriverType::Proprietary => "Proprietary",
            DriverType::Community => "Community",
            DriverType::BuiltIn => "Built-in",
        }
    }
}

/// Driver status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    /// Driver is not installed
    NotInstalled,
    /// Driver is installed but not active
    Installed,
    /// Driver is installed and active
    Active,
    /// Driver is recommended for this hardware
    Recommended,
    /// Driver is in use
    InUse,
}

impl DriverStatus {
    /// Get display name for the status
    pub fn display_name(&self) -> &'static str {
        match self {
            DriverStatus::NotInstalled => "Not Installed",
            DriverStatus::Installed => "Installed",
            DriverStatus::Active => "Active",
            DriverStatus::Recommended => "Recommended",
            DriverStatus::InUse => "In Use",
        }
    }
}

/// Hardware device requiring a driver
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    /// Device ID
    pub device_id: String,
    /// Device name
    pub name: String,
    /// Device vendor
    pub vendor: String,
    /// Device class (e.g., "GPU", "Network", "Audio")
    pub device_class: String,
    /// PCI ID if applicable
    pub pci_id: Option<String>,
    /// USB ID if applicable
    pub usb_id: Option<String>,
}

/// Driver package information
#[derive(Debug, Clone)]
pub struct DriverPackage {
    /// Package name
    pub package_name: String,
    /// Driver name
    pub driver_name: String,
    /// Driver type
    pub driver_type: DriverType,
    /// Driver status
    pub status: DriverStatus,
    /// Supported devices
    pub supported_devices: Vec<String>,
    /// Package version
    pub version: String,
    /// Package repository
    pub repository: String,
    /// Whether driver is recommended
    pub recommended: bool,
    /// Whether driver is proprietary
    pub proprietary: bool,
    /// Driver description
    pub description: String,
}

/// Driver manager - manages system drivers
#[derive(Debug)]
pub struct MintDriverManager {
    /// Available driver packages
    pub drivers: Vec<DriverPackage>,
    /// Detected hardware devices
    pub devices: Vec<HardwareDevice>,
    /// Driver-device mappings
    pub device_driver_map: BTreeMap<String, String>, // device_id -> package_name
    /// Whether to use proprietary drivers
    pub allow_proprietary: bool,
    /// Whether to install recommended drivers automatically
    pub auto_install_recommended: bool,
}

impl MintDriverManager {
    /// Create a new Driver Manager
    pub fn new() -> Self {
        Self {
            drivers: Vec::new(),
            devices: Vec::new(),
            device_driver_map: BTreeMap::new(),
            allow_proprietary: true,
            auto_install_recommended: false,
        }
    }

    /// Add a driver package
    pub fn add_driver(&mut self, driver: DriverPackage) {
        self.drivers.push(driver);
    }

    /// Add a hardware device
    pub fn add_device(&mut self, device: HardwareDevice) {
        self.devices.push(device);
    }

    /// Get drivers for a specific device
    pub fn get_drivers_for_device(&self, device_id: &str) -> Vec<&DriverPackage> {
        self.drivers
            .iter()
            .filter(|d| d.supported_devices.contains(&device_id.to_string()))
            .collect()
    }

    /// Get recommended driver for a device
    pub fn get_recommended_driver(&self, device_id: &str) -> Option<&DriverPackage> {
        self.drivers
            .iter()
            .filter(|d| d.supported_devices.contains(&device_id.to_string()))
            .find(|d| d.recommended)
    }

    /// Get all available drivers by device class
    pub fn get_drivers_by_class(&self, device_class: &str) -> Vec<&DriverPackage> {
        self.drivers
            .iter()
            .filter(|d| {
                self.devices
                    .iter()
                    .any(|dev| dev.device_class == device_class && d.supported_devices.contains(&dev.device_id))
            })
            .collect()
    }

    /// Get proprietary drivers
    pub fn get_proprietary_drivers(&self) -> Vec<&DriverPackage> {
        self.drivers.iter().filter(|d| d.proprietary).collect()
    }

    /// Get open-source drivers
    pub fn get_open_source_drivers(&self) -> Vec<&DriverPackage> {
        self.drivers.iter().filter(|d| !d.proprietary).collect()
    }

    /// Install a driver for a device
    pub fn install_driver(&mut self, device_id: &str, package_name: &str) -> Result<(), String> {
        // Check if driver exists
        if !self.drivers.iter().any(|d| d.package_name == package_name) {
            return Err("Driver package not found".to_string());
        }

        // Check if driver supports the device
        if let Some(driver) = self.drivers.iter().find(|d| d.package_name == package_name) {
            if !driver.supported_devices.contains(&device_id.to_string()) {
                return Err("Driver does not support this device".to_string());
            }

            // Check proprietary restriction
            if driver.proprietary && !self.allow_proprietary {
                return Err("Proprietary drivers are not allowed".to_string());
            }
        }

        // Map device to driver
        self.device_driver_map.insert(device_id.to_string(), package_name.to_string());
        
        // Update driver status
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.package_name == package_name) {
            driver.status = DriverStatus::Installed;
        }

        Ok(())
    }

    /// Remove a driver for a device
    pub fn remove_driver(&mut self, device_id: &str) -> Result<(), String> {
        if let Some(package_name) = self.device_driver_map.remove(device_id) {
            // Update driver status
            if let Some(driver) = self.drivers.iter_mut().find(|d| d.package_name == package_name) {
                driver.status = DriverStatus::NotInstalled;
            }
            Ok(())
        } else {
            Err("No driver installed for this device".to_string())
        }
    }

    /// Activate a driver
    pub fn activate_driver(&mut self, package_name: &str) -> Result<(), String> {
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.package_name == package_name) {
            driver.status = DriverStatus::Active;
            Ok(())
        } else {
            Err("Driver package not found".to_string())
        }
    }

    /// Set whether to allow proprietary drivers
    pub fn set_allow_proprietary(&mut self, allow: bool) {
        self.allow_proprietary = allow;
    }

    /// Set whether to auto-install recommended drivers
    pub fn set_auto_install_recommended(&mut self, auto: bool) {
        self.auto_install_recommended = auto;
    }

    /// Auto-install recommended drivers for all devices
    pub fn auto_install_recommended_drivers(&mut self) -> Vec<String> {
        let mut installed = Vec::new();
        
        if !self.auto_install_recommended {
            return installed;
        }

        // Collect device IDs and names first to avoid borrow conflicts
        let device_list: Vec<(String, String)> = self.devices.iter().map(|d| (d.device_id.clone(), d.name.clone())).collect();

        for (device_id, device_name) in device_list {
            if let Some(driver) = self.get_recommended_driver(&device_id) {
                if driver.status == DriverStatus::NotInstalled {
                    let package_name = driver.package_name.clone();
                    if self.install_driver(&device_id, &package_name).is_ok() {
                        installed.push(format!("{} for {}", package_name, device_name));
                    }
                }
            }
        }

        installed
    }

    /// Get driver recommendations for all devices
    pub fn get_driver_recommendations(&self) -> Vec<(String, String)> {
        let mut recommendations = Vec::new();
        
        for device in &self.devices {
            if let Some(driver) = self.get_recommended_driver(&device.device_id) {
                recommendations.push((device.device_id.clone(), driver.package_name.clone()));
            }
        }

        recommendations
    }
}

impl Default for MintDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_manager_creation() {
        let manager = MintDriverManager::new();
        assert_eq!(manager.drivers.len(), 0);
        assert_eq!(manager.devices.len(), 0);
    }

    #[test]
    fn test_add_driver() {
        let mut manager = MintDriverManager::new();
        
        let driver = DriverPackage {
            package_name: "nvidia-driver".to_string(),
            driver_name: "NVIDIA Driver".to_string(),
            driver_type: DriverType::Proprietary,
            status: DriverStatus::NotInstalled,
            supported_devices: vec!["gpu-001".to_string()],
            version: "535.0".to_string(),
            repository: "non-free".to_string(),
            recommended: true,
            proprietary: true,
            description: "NVIDIA proprietary driver".to_string(),
        };
        
        manager.add_driver(driver);
        assert_eq!(manager.drivers.len(), 1);
    }

    #[test]
    fn test_install_driver() {
        let mut manager = MintDriverManager::new();
        
        let device = HardwareDevice {
            device_id: "gpu-001".to_string(),
            name: "NVIDIA GPU".to_string(),
            vendor: "NVIDIA".to_string(),
            device_class: "GPU".to_string(),
            pci_id: Some("10de:1234".to_string()),
            usb_id: None,
        };
        
        let driver = DriverPackage {
            package_name: "nvidia-driver".to_string(),
            driver_name: "NVIDIA Driver".to_string(),
            driver_type: DriverType::Proprietary,
            status: DriverStatus::NotInstalled,
            supported_devices: vec!["gpu-001".to_string()],
            version: "535.0".to_string(),
            repository: "non-free".to_string(),
            recommended: true,
            proprietary: true,
            description: "NVIDIA proprietary driver".to_string(),
        };
        
        manager.add_device(device);
        manager.add_driver(driver);
        
        let result = manager.install_driver("gpu-001", "nvidia-driver");
        assert!(result.is_ok());
        assert_eq!(manager.device_driver_map.get("gpu-001"), Some(&"nvidia-driver".to_string()));
    }

    #[test]
    fn test_proprietary_restriction() {
        let mut manager = MintDriverManager::new();
        manager.set_allow_proprietary(false);
        
        let device = HardwareDevice {
            device_id: "gpu-001".to_string(),
            name: "NVIDIA GPU".to_string(),
            vendor: "NVIDIA".to_string(),
            device_class: "GPU".to_string(),
            pci_id: Some("10de:1234".to_string()),
            usb_id: None,
        };
        
        let driver = DriverPackage {
            package_name: "nvidia-driver".to_string(),
            driver_name: "NVIDIA Driver".to_string(),
            driver_type: DriverType::Proprietary,
            status: DriverStatus::NotInstalled,
            supported_devices: vec!["gpu-001".to_string()],
            version: "535.0".to_string(),
            repository: "non-free".to_string(),
            recommended: true,
            proprietary: true,
            description: "NVIDIA proprietary driver".to_string(),
        };
        
        manager.add_device(device);
        manager.add_driver(driver);
        
        let result = manager.install_driver("gpu-001", "nvidia-driver");
        assert!(result.is_err());
    }

    #[test]
    fn test_get_recommended_driver() {
        let mut manager = MintDriverManager::new();
        
        let driver = DriverPackage {
            package_name: "amdgpu-driver".to_string(),
            driver_name: "AMDGPU Driver".to_string(),
            driver_type: DriverType::OpenSource,
            status: DriverStatus::NotInstalled,
            supported_devices: vec!["gpu-002".to_string()],
            version: "1.0".to_string(),
            repository: "main".to_string(),
            recommended: true,
            proprietary: false,
            description: "AMD open-source driver".to_string(),
        };
        
        manager.add_driver(driver);
        
        let recommended = manager.get_recommended_driver("gpu-002");
        assert!(recommended.is_some());
        assert_eq!(recommended.unwrap().package_name, "amdgpu-driver");
    }
}
