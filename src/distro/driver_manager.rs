// SPDX-License-Identifier: MIT
// SigmaOS - Driver Manager (inspired by Linux Mint Driver Manager)
// Inspired by Linux Mint Driver Manager (https://github.com/linuxmint/mintdrivers)
// Provides hardware driver management

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, Ordering};

/// Driver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    GPU,
    Network,
    Audio,
    Bluetooth,
    WiFi,
    Other,
}

/// Driver status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Installed,
    Available,
    Active,
    Inactive,
}

/// Hardware driver
#[derive(Debug, Clone)]
pub struct HardwareDriver {
    pub name: String,
    pub driver_type: DriverType,
    pub status: DriverStatus,
    pub version: String,
    pub recommended: bool,
    pub proprietary: bool,
}

/// Driver manager
pub struct DriverManager {
    pub drivers: BTreeMap<String, HardwareDriver>,
    pub active: AtomicBool,
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            drivers: BTreeMap::new(),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize driver manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
        self.scan_drivers();
    }

    /// Scan for drivers
    pub fn scan_drivers(&mut self) {
        // In real implementation, would scan hardware and available drivers
        // (lspci, lsusb, lshw, etc.)

        // Add some sample drivers
        self.add_driver(HardwareDriver {
            name: "amdgpu".to_string(),
            driver_type: DriverType::GPU,
            status: DriverStatus::Active,
            version: "23.0.0".to_string(),
            recommended: true,
            proprietary: false,
        });

        self.add_driver(HardwareDriver {
            name: "nvidia".to_string(),
            driver_type: DriverType::GPU,
            status: DriverStatus::Available,
            version: "535.0".to_string(),
            recommended: true,
            proprietary: true,
        });

        self.add_driver(HardwareDriver {
            name: "intel-xe".to_string(),
            driver_type: DriverType::GPU,
            status: DriverStatus::Available,
            version: "1.0.0".to_string(),
            recommended: true,
            proprietary: false,
        });

        self.add_driver(HardwareDriver {
            name: "iwlwifi".to_string(),
            driver_type: DriverType::WiFi,
            status: DriverStatus::Active,
            version: "7.0".to_string(),
            recommended: true,
            proprietary: false,
        });
    }

    /// Add driver
    pub fn add_driver(&mut self, driver: HardwareDriver) {
        self.drivers.insert(driver.name.clone(), driver);
    }

    /// Get driver
    pub fn get_driver(&self, name: &str) -> Option<&HardwareDriver> {
        self.drivers.get(name)
    }

    /// Install driver
    pub fn install_driver(&mut self, name: &str) -> Result<(), &'static str> {
        let driver = self.drivers.get_mut(name).ok_or("Driver not found")?;

        // In real implementation, would install driver:
        // apt install {name}
        // or modprobe {name}

        driver.status = DriverStatus::Installed;

        Ok(())
    }

    /// Activate driver
    pub fn activate_driver(&mut self, name: &str) -> Result<(), &'static str> {
        let driver = self.drivers.get_mut(name).ok_or("Driver not found")?;

        // In real implementation, would activate driver:
        // modprobe {name}

        driver.status = DriverStatus::Active;

        Ok(())
    }

    /// Deactivate driver
    pub fn deactivate_driver(&mut self, name: &str) -> Result<(), &'static str> {
        let driver = self.drivers.get_mut(name).ok_or("Driver not found")?;

        // In real implementation, would deactivate driver:
        // modprobe -r {name}

        driver.status = DriverStatus::Inactive;

        Ok(())
    }

    /// Get drivers by type
    pub fn get_drivers_by_type(&self, driver_type: DriverType) -> Vec<&HardwareDriver> {
        self.drivers
            .values()
            .filter(|d| d.driver_type == driver_type)
            .collect()
    }

    /// Get recommended drivers
    pub fn get_recommended_drivers(&self) -> Vec<&HardwareDriver> {
        self.drivers.values().filter(|d| d.recommended).collect()
    }

    /// Get available drivers
    pub fn get_available_drivers(&self) -> Vec<&HardwareDriver> {
        self.drivers
            .values()
            .filter(|d| d.status == DriverStatus::Available)
            .collect()
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global driver manager
static mut DRIVER_MANAGER: Option<DriverManager> = None;

/// Initialize driver manager
pub fn init_driver_manager() {
    unsafe {
        DRIVER_MANAGER = Some(DriverManager::new());
    }
}

/// Get global driver manager
pub fn get_driver_manager() -> Option<&'static mut DriverManager> {
    unsafe { DRIVER_MANAGER.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_manager_init() {
        let mut manager = DriverManager::new();
        manager.init();
        assert!(manager.is_active());
        assert!(manager.drivers.len() > 0);
    }

    #[test]
    fn test_install_driver() {
        let mut manager = DriverManager::new();
        manager.scan_drivers();
        manager.install_driver("nvidia").unwrap();
        assert_eq!(
            manager.get_driver("nvidia").unwrap().status,
            DriverStatus::Installed
        );
    }

    #[test]
    fn test_activate_driver() {
        let mut manager = DriverManager::new();
        manager.scan_drivers();
        manager.activate_driver("nvidia").unwrap();
        assert_eq!(
            manager.get_driver("nvidia").unwrap().status,
            DriverStatus::Active
        );
    }

    #[test]
    fn test_get_drivers_by_type() {
        let mut manager = DriverManager::new();
        manager.scan_drivers();
        let gpu_drivers = manager.get_drivers_by_type(DriverType::GPU);
        assert!(gpu_drivers.len() > 0);
    }

    #[test]
    fn test_get_recommended_drivers() {
        let mut manager = DriverManager::new();
        manager.scan_drivers();
        let recommended = manager.get_recommended_drivers();
        assert!(recommended.len() > 0);
    }
}
