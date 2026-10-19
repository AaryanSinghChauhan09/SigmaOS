//! SigmaDriverManager - Driver Management System
//!
//! Provides comprehensive driver management inspired by Omarchy and Linux Mint.
//! Supports driver installation, updates, detection, and hardware compatibility.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Driver status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Installed,
    NotInstalled,
    UpdateAvailable,
    Incompatible,
    Blacklisted,
;

/// Driver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Graphics,
    Network,
    Audio,
    Input,
    Storage,
    USB,
    Bluetooth,
    Camera,
    Printer,
    Other,
;

/// Hardware device
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub id: String,
    pub vendor_id: String,
    pub device_id: String,
    pub name: String,
    pub device_type: DriverType,
    pub bus: String, // pci, usb, platform
    pub needs_driver: bool,
;

impl HardwareDevice {
    pub fn new(id: String, vendor_id: String, device_id: String, name: String, device_type: DriverType, bus: String) -> Self {
        Self {
            id,
            vendor_id,
            device_id,
            name,
            device_type,
            bus,
            needs_driver: true,
        }
    }

    pub fn set_needs_driver(&mut self, needs: bool) {
        self.needs_driver = needs;
    }
;

/// Driver package
#[derive(Debug, Clone)]
pub struct DriverPackage {
    pub id: String,
    pub name: String,
    pub driver_type: DriverType,
    pub version: String,
    pub vendor: String,
    pub description: String,
    pub status: DriverStatus,
    pub installed_version: Option<String>,
    pub latest_version: String,
    pub compatible_devices: Vec<String>, // device IDs
;

impl DriverPackage {
    pub fn new(id: String, name: String, driver_type: DriverType, version: String, vendor: String, description: String) -> Self {
        Self {
            id,
            name,
            driver_type,
            version,
            vendor,
            description,
            status: DriverStatus::NotInstalled,
            installed_version: None,
            latest_version: version.clone(),
            compatible_devices: Vec::new(),
        }
    }

    pub fn set_status(&mut self, status: DriverStatus) {
        self.status = status;
    }

    pub fn set_installed_version(&mut self, version: Option<String>) {
        self.installed_version = version;
    }

    pub fn add_compatible_device(&mut self, device_id: String) {
        self.compatible_devices.push(device_id);
    }

    pub fn is_compatible_with(&self, device_id: &str) -> bool {
        self.compatible_devices.contains(&device_id.to_string())
    }

    pub fn needs_update(&self) -> bool {
        self.status == DriverStatus::UpdateAvailable
    }
;

/// SigmaDriverManager - Driver management system
#[derive(Debug)]
pub struct SigmaDriverManager {
    devices: BTreeMap<String, HardwareDevice>,
    drivers: BTreeMap<String, DriverPackage>,
;

impl SigmaDriverManager {
    pub fn new() -> Self {
        Self {
            devices: BTreeMap::new(),
            drivers: BTreeMap::new(),
        }
    }

    pub fn add_device(&mut self, device: HardwareDevice) {
        self.devices.insert(device.id.clone(), device);
    }

    pub fn remove_device(&mut self, id: &str) -> bool {
        self.devices.remove(id).is_some()
    }

    pub fn get_device(&self, id: &str) -> Option<&HardwareDevice> {
        self.devices.get(id)
    }

    pub fn add_driver(&mut self, driver: DriverPackage) {
        self.drivers.insert(driver.id.clone(), driver);
    }

    pub fn remove_driver(&mut self, id: &str) -> bool {
        self.drivers.remove(id).is_some()
    }

    pub fn get_driver(&self, id: &str) -> Option<&DriverPackage> {
        self.drivers.get(id)
    }

    pub fn get_driver_mut(&mut self, id: &str) -> Option<&mut DriverPackage> {
        self.drivers.get_mut(id)
    }

    pub fn find_compatible_drivers(&self, device_id: &str) -> Vec<&DriverPackage> {
        self.drivers.values()
            .filter(|driver| driver.is_compatible_with(device_id))
            .collect()
    }

    pub fn find_drivers_for_device(&self, device: &HardwareDevice) -> Vec<&DriverPackage> {
        self.drivers.values()
            .filter(|driver| driver.driver_type == device.device_type)
            .filter(|driver| driver.is_compatible_with(&device.id))
            .collect()
    }

    pub fn list_missing_drivers(&self) -> Vec<&HardwareDevice> {
        self.devices.values()
            .filter(|device| device.needs_driver)
            .collect()
    }

    pub fn list_available_updates(&self) -> Vec<&DriverPackage> {
        self.drivers.values()
            .filter(|driver| driver.needs_update())
            .collect()
    }

    pub fn list_devices_by_type(&self, device_type: DriverType) -> Vec<&HardwareDevice> {
        self.devices.values()
            .filter(|device| device.device_type == device_type)
            .collect()
    }

    pub fn list_drivers_by_type(&self, driver_type: DriverType) -> Vec<&DriverPackage> {
        self.drivers.values()
            .filter(|driver| driver.driver_type == driver_type)
            .collect()
    }

    pub fn device_count(&self) -> usize {
        self.devices.len()
    }

    pub fn driver_count(&self) -> usize {
        self.drivers.len()
    }
;

impl Default for SigmaDriverManager {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaDriverManager instance
static mut SIGMADRIVERMANAGER_INSTANCE: Option<SigmaDriverManager> = None;

/// Initialize the global SigmaDriverManager instance
pub fn init_sigmadrivermanager() {
    unsafe {
        SIGMADRIVERMANAGER_INSTANCE = Some(SigmaDriverManager::new());
    }
;

/// Get the global SigmaDriverManager instance
pub fn get_sigmadrivermanager() -> Option<&'static SigmaDriverManager> {
    unsafe {
        SIGMADRIVERMANAGER_INSTANCE.as_ref()
    }
;

/// Get the global SigmaDriverManager instance (mutable)
pub fn get_sigmadrivermanager_mut() -> Option<&'static mut SigmaDriverManager> {
    unsafe {
        SIGMADRIVERMANAGER_INSTANCE.as_mut()
    }
;

/// Convenience function to add a device
pub fn add_device(device: HardwareDevice) -> bool {
    if let Some(manager) = get_sigmadrivermanager_mut() {
        manager.add_device(device);
        true
    } else {
        false
    }
;

/// Convenience function to add a driver
pub fn add_driver(driver: DriverPackage) -> bool {
    if let Some(manager) = get_sigmadrivermanager_mut() {
        manager.add_driver(driver);
        true
    } else {
        false
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_device_creation() {
        let device = HardwareDevice::new(
            "dev1".to_string(),
            "8086".to_string(),
            "1234".to_string(),
            "Intel Graphics".to_string(),
            DriverType::Graphics,
            "pci".to_string(),
        );
        assert_eq!(device.id, "dev1");
        assert_eq!(device.device_type, DriverType::Graphics);
        assert!(device.needs_driver);
    }

    #[test]
    fn test_driver_package_creation() {
        let driver = DriverPackage::new(
            "driver1".to_string(),
            "NVIDIA Driver".to_string(),
            DriverType::Graphics,
            "515.65".to_string(),
            "NVIDIA".to_string(),
            "NVIDIA graphics driver".to_string(),
        );
        assert_eq!(driver.id, "driver1");
        assert_eq!(driver.status, DriverStatus::NotInstalled);
    }

    #[test]
    fn test_driver_package_modification() {
        let mut driver = DriverPackage::new(
            "driver1".to_string(),
            "Test".to_string(),
            DriverType::Network,
            "1.0".to_string(),
            "Vendor".to_string(),
            "Test driver".to_string(),
        );
        driver.set_status(DriverStatus::Installed);
        driver.set_installed_version(Some("1.0".to_string()));
        driver.add_compatible_device("dev1".to_string());
        assert_eq!(driver.status, DriverStatus::Installed);
        assert!(driver.is_compatible_with("dev1"));
    }

    #[test]
    fn test_sigmadrivermanager_creation() {
        let manager = SigmaDriverManager::new();
        assert_eq!(manager.device_count(), 0);
        assert_eq!(manager.driver_count(), 0);
    }

    #[test]
    fn test_sigmadrivermanager_management() {
        let mut manager = SigmaDriverManager::new();
        let device = HardwareDevice::new(
            "dev1".to_string(),
            "1234".to_string(),
            "5678".to_string(),
            "Test".to_string(),
            DriverType::Network,
            "pci".to_string(),
        );
        manager.add_device(device);
        assert_eq!(manager.device_count(), 1);
    }

    #[test]
    fn test_compatible_driver_finding() {
        let mut manager = SigmaDriverManager::new();
        let device = HardwareDevice::new(
            "dev1".to_string(),
            "1234".to_string(),
            "5678".to_string(),
            "Test".to_string(),
            DriverType::Network,
            "pci".to_string(),
        );
        manager.add_device(device);

        let mut driver = DriverPackage::new(
            "driver1".to_string(),
            "Test Driver".to_string(),
            DriverType::Network,
            "1.0".to_string(),
            "Vendor".to_string(),
            "Test".to_string(),
        );
        driver.add_compatible_device("dev1".to_string());
        manager.add_driver(driver);

        let compatible = manager.find_compatible_drivers("dev1");
        assert_eq!(compatible.len(), 1);
    }
;
