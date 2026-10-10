//! Driver Manager
//!
//! Hardware driver management inspired by Linux Mint's mintdrivers,
//! supporting automatic driver detection, installation, and management.

use std::collections::HashMap;

/// Driver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Nvidia,
    Amd,
    Intel,
    Broadcom,
    Wifi,
    Bluetooth,
    Printer,
    Scanner,
    Other,
}

impl DriverType {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "nvidia" => Some(DriverType::Nvidia),
            "amd" => Some(DriverType::Amd),
            "intel" => Some(DriverType::Intel),
            "broadcom" => Some(DriverType::Broadcom),
            "wifi" | "wireless" => Some(DriverType::Wifi),
            "bluetooth" => Some(DriverType::Bluetooth),
            "printer" => Some(DriverType::Printer),
            "scanner" => Some(DriverType::Scanner),
            _ => Some(DriverType::Other),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            DriverType::Nvidia => "NVIDIA",
            DriverType::Amd => "AMD",
            DriverType::Intel => "Intel",
            DriverType::Broadcom => "Broadcom",
            DriverType::Wifi => "WiFi",
            DriverType::Bluetooth => "Bluetooth",
            DriverType::Printer => "Printer",
            DriverType::Scanner => "Scanner",
            DriverType::Other => "Other",
        }
    }
}

/// Driver status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    NotInstalled,
    Installed,
    Active,
    Incompatible,
    RebootRequired,
}

impl DriverStatus {
    pub fn as_str(&self) -> &str {
        match self {
            DriverStatus::NotInstalled => "Not Installed",
            DriverStatus::Installed => "Installed",
            DriverStatus::Active => "Active",
            DriverStatus::Incompatible => "Incompatible",
            DriverStatus::RebootRequired => "Reboot Required",
        }
    }
}

/// Driver information
#[derive(Debug, Clone)]
pub struct Driver {
    pub name: String,
    pub version: String,
    pub driver_type: DriverType,
    pub status: DriverStatus,
    pub is_recommended: bool,
    pub is_open_source: bool,
    pub device_id: String,
    pub description: String,
}

impl Driver {
    pub fn new(name: String, version: String, driver_type: DriverType, device_id: String) -> Self {
        Self {
            name,
            version,
            driver_type,
            status: DriverStatus::NotInstalled,
            is_recommended: false,
            is_open_source: true,
            device_id,
            description: String::new(),
        }
    }

    pub fn set_status(&mut self, status: DriverStatus) {
        self.status = status;
    }

    pub fn set_recommended(&mut self, recommended: bool) {
        self.is_recommended = recommended;
    }

    pub fn set_open_source(&mut self, open_source: bool) {
        self.is_open_source = open_source;
    }

    pub fn set_description(&mut self, desc: String) {
        self.description = desc;
    }

    pub fn is_installed(&self) -> bool {
        self.status == DriverStatus::Installed || self.status == DriverStatus::Active
    }
}

/// Driver manager
#[derive(Debug)]
pub struct DriverManager {
    drivers: HashMap<String, Driver>,
    installed_count: u32,
}

impl DriverManager {
    pub fn new() -> Self {
        Self {
            drivers: HashMap::new(),
            installed_count: 0,
        }
    }

    /// Add a driver
    pub fn add_driver(&mut self, driver: Driver) {
        self.drivers.insert(driver.device_id.clone(), driver);
    }

    /// Get a driver
    pub fn get_driver(&self, device_id: &str) -> Option<&Driver> {
        self.drivers.get(device_id)
    }

    /// Get a driver mutably
    pub fn get_driver_mut(&mut self, device_id: &str) -> Option<&mut Driver> {
        self.drivers.get_mut(device_id)
    }

    /// List all drivers
    pub fn list_drivers(&self) -> Vec<&Driver> {
        self.drivers.values().collect()
    }

    /// List drivers by type
    pub fn list_by_type(&self, driver_type: DriverType) -> Vec<&Driver> {
        self.drivers
            .values()
            .filter(|d| d.driver_type == driver_type)
            .collect()
    }

    /// List recommended drivers
    pub fn list_recommended(&self) -> Vec<&Driver> {
        self.drivers.values().filter(|d| d.is_recommended).collect()
    }

    /// List proprietary drivers
    pub fn list_proprietary(&self) -> Vec<&Driver> {
        self.drivers
            .values()
            .filter(|d| !d.is_open_source)
            .collect()
    }

    /// List open source drivers
    pub fn list_open_source(&self) -> Vec<&Driver> {
        self.drivers.values().filter(|d| d.is_open_source).collect()
    }

    /// Install a driver
    pub fn install(&mut self, device_id: &str) -> Result<(), String> {
        let driver = self
            .get_driver_mut(device_id)
            .ok_or_else(|| format!("Driver {} not found", device_id))?;

        if driver.is_installed() {
            return Err(format!("Driver {} is already installed", device_id));
        }

        // Simulate installation
        driver.set_status(DriverStatus::Installed);
        self.installed_count += 1;

        Ok(())
    }

    /// Activate a driver
    pub fn activate(&mut self, device_id: &str) -> Result<(), String> {
        let driver = self
            .get_driver_mut(device_id)
            .ok_or_else(|| format!("Driver {} not found", device_id))?;

        if !driver.is_installed() {
            return Err(format!("Driver {} is not installed", device_id));
        }

        driver.set_status(DriverStatus::Active);

        Ok(())
    }

    /// Remove a driver
    pub fn remove(&mut self, device_id: &str) -> Result<(), String> {
        let driver = self
            .get_driver_mut(device_id)
            .ok_or_else(|| format!("Driver {} not found", device_id))?;

        if !driver.is_installed() {
            return Err(format!("Driver {} is not installed", device_id));
        }

        // Simulate removal
        driver.set_status(DriverStatus::NotInstalled);
        self.installed_count -= 1;

        Ok(())
    }

    /// Detect hardware and suggest drivers (simulated)
    pub fn detect_hardware(&mut self) -> Vec<&Driver> {
        // Simulate hardware detection
        let detected_drivers = vec![
            (
                "0000:01:00.0",
                "NVIDIA GeForce RTX 3080",
                DriverType::Nvidia,
            ),
            ("0000:00:02.0", "Intel UHD Graphics 630", DriverType::Intel),
            ("0000:02:00.0", "Intel Wi-Fi 6 AX200", DriverType::Wifi),
        ];

        for (device_id, name, driver_type) in detected_drivers {
            if !self.drivers.contains_key(device_id) {
                let mut driver = Driver::new(
                    name.to_string(),
                    "535.104.05".to_string(),
                    driver_type,
                    device_id.to_string(),
                );
                driver.set_recommended(true);
                driver.set_description(format!("Recommended driver for {}", name));
                self.add_driver(driver);
            }
        }

        self.list_drivers()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> DriverStatistics {
        let total_drivers = self.drivers.len();
        let installed_drivers = self.drivers.values().filter(|d| d.is_installed()).count();
        let recommended_drivers = self.drivers.values().filter(|d| d.is_recommended).count();
        let proprietary_drivers = self.drivers.values().filter(|d| !d.is_open_source).count();

        DriverStatistics {
            total_drivers,
            installed_drivers,
            recommended_drivers,
            proprietary_drivers,
            open_source_drivers: total_drivers - proprietary_drivers,
        }
    }
}

impl Default for DriverManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Driver statistics
#[derive(Debug, Clone)]
pub struct DriverStatistics {
    pub total_drivers: usize,
    pub installed_drivers: usize,
    pub recommended_drivers: usize,
    pub proprietary_drivers: usize,
    pub open_source_drivers: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_type_from_str() {
        assert_eq!(DriverType::from_str("nvidia"), Some(DriverType::Nvidia));
        assert_eq!(DriverType::from_str("wifi"), Some(DriverType::Wifi));
    }

    #[test]
    fn test_driver_creation() {
        let driver = Driver::new(
            "NVIDIA Driver".to_string(),
            "535.104.05".to_string(),
            DriverType::Nvidia,
            "0000:01:00.0".to_string(),
        );
        assert_eq!(driver.name, "NVIDIA Driver");
        assert_eq!(driver.driver_type, DriverType::Nvidia);
    }

    #[test]
    fn test_driver_manager_creation() {
        let manager = DriverManager::new();
        assert_eq!(manager.list_drivers().len(), 0);
    }

    #[test]
    fn test_add_driver() {
        let mut manager = DriverManager::new();
        let driver = Driver::new(
            "Test Driver".to_string(),
            "1.0".to_string(),
            DriverType::Other,
            "0000:00:00.0".to_string(),
        );
        manager.add_driver(driver);
        assert_eq!(manager.list_drivers().len(), 1);
    }

    #[test]
    fn test_install_driver() {
        let mut manager = DriverManager::new();
        let driver = Driver::new(
            "Test Driver".to_string(),
            "1.0".to_string(),
            DriverType::Other,
            "0000:00:00.0".to_string(),
        );
        manager.add_driver(driver);
        assert!(manager.install("0000:00:00.0").is_ok());
        assert!(manager.get_driver("0000:00:00.0").unwrap().is_installed());
    }

    #[test]
    fn test_activate_driver() {
        let mut manager = DriverManager::new();
        let mut driver = Driver::new(
            "Test Driver".to_string(),
            "1.0".to_string(),
            DriverType::Other,
            "0000:00:00.0".to_string(),
        );
        driver.set_status(DriverStatus::Installed);
        manager.add_driver(driver);
        assert!(manager.activate("0000:00:00.0").is_ok());
    }

    #[test]
    fn test_detect_hardware() {
        let mut manager = DriverManager::new();
        let drivers = manager.detect_hardware();
        assert!(drivers.len() > 0);
    }

    #[test]
    fn test_statistics() {
        let mut manager = DriverManager::new();
        manager.detect_hardware();
        let stats = manager.get_statistics();
        assert!(stats.total_drivers > 0);
    }
}
