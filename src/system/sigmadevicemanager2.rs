#![no_std]
extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Device manager for hardware device management
/// Inspired by Linux udev, lsblk, and device management
#[derive(Debug, Clone)]
pub struct SigmaDeviceManager2 {
    devices: Vec<Device>,
}

/// A hardware device
#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    name: String,
    device_type: DeviceManager2Type,
    state: DeviceManager2State,
    driver: Option<String>,
    mount_point: Option<String>,
}

/// Device type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceManager2Type {
    Block,
    Character,
    Network,
    USB,
    PCI,
}

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceManager2State {
    Connected,
    Disconnected,
    Mounted,
    Unmounted,
    Error,
}

impl SigmaDeviceManager2 {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
        }
    }

    /// Add a device
    pub fn add_device(&mut self, id: String, name: String, device_type: DeviceManager2Type, driver: Option<String>) -> Result<String, String> {
        if self.devices.iter().any(|d| d.id == id) {
            return Err(format!("Device with id {} already exists", id));
        }
        
        let device = Device {
            id: id.clone(),
            name,
            device_type,
            state: DeviceManager2State::Connected,
            driver,
            mount_point: None,
        };
        self.devices.push(device);
        Ok(id)
    }

    /// Remove a device
    pub fn remove_device(&mut self, id: &str) -> Result<(), String> {
        let pos = self.devices.iter().position(|d| d.id == id)
            .ok_or_else(|| format!("Device with id {} not found", id))?;
        self.devices.remove(pos);
        Ok(())
    }

    /// Mount a device
    pub fn mount_device(&mut self, id: &str, mount_point: String) -> Result<(), String> {
        let device = self.devices.iter_mut().find(|d| d.id == id)
            .ok_or_else(|| format!("Device with id {} not found", id))?;
        device.mount_point = Some(mount_point);
        device.state = DeviceManager2State::Mounted;
        Ok(())
    }

    /// Unmount a device
    pub fn unmount_device(&mut self, id: &str) -> Result<(), String> {
        let device = self.devices.iter_mut().find(|d| d.id == id)
            .ok_or_else(|| format!("Device with id {} not found", id))?;
        device.mount_point = None;
        device.state = DeviceManager2State::Unmounted;
        Ok(())
    }

    /// Disconnect a device
    pub fn disconnect_device(&mut self, id: &str) -> Result<(), String> {
        let device = self.devices.iter_mut().find(|d| d.id == id)
            .ok_or_else(|| format!("Device with id {} not found", id))?;
        device.state = DeviceManager2State::Disconnected;
        device.mount_point = None;
        Ok(())
    }

    /// Set device error state
    pub fn set_error_state(&mut self, id: &str) -> Result<(), String> {
        let device = self.devices.iter_mut().find(|d| d.id == id)
            .ok_or_else(|| format!("Device with id {} not found", id))?;
        device.state = DeviceManager2State::Error;
        Ok(())
    }

    /// Get all devices
    pub fn get_devices(&self) -> &[Device] {
        &self.devices
    }

    /// Get connected devices
    pub fn get_connected_devices(&self) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.state == DeviceManager2State::Connected || d.state == DeviceManager2State::Mounted).collect()
    }

    /// Get mounted devices
    pub fn get_mounted_devices(&self) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.state == DeviceManager2State::Mounted).collect()
    }

    /// Get device by ID
    pub fn get_device(&self, id: &str) -> Option<&Device> {
        self.devices.iter().find(|d| d.id == id)
    }

    /// Get devices by type
    pub fn get_devices_by_type(&self, device_type: DeviceManager2Type) -> Vec<&Device> {
        self.devices.iter().filter(|d| d.device_type == device_type).collect()
    }
}

impl Default for SigmaDeviceManager2 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_device() {
        let mut manager = SigmaDeviceManager2::new();
        let result = manager.add_device("dev1".to_string(), "sda".to_string(), DeviceManager2Type::Block, Some("ahci".to_string()));
        assert!(result.is_ok());
        assert_eq!(manager.get_devices().len(), 1);
    }

    #[test]
    fn test_duplicate_device() {
        let mut manager = SigmaDeviceManager2::new();
        manager.add_device("dev1".to_string(), "sda".to_string(), DeviceManager2Type::Block, Some("ahci".to_string())).unwrap();
        let result = manager.add_device("dev1".to_string(), "sdb".to_string(), DeviceManager2Type::Block, Some("ahci".to_string()));
        assert!(result.is_err());
    }

    #[test]
    fn test_mount_unmount_device() {
        let mut manager = SigmaDeviceManager2::new();
        manager.add_device("dev1".to_string(), "sda".to_string(), DeviceManager2Type::Block, Some("ahci".to_string())).unwrap();
        
        manager.mount_device("dev1", "/mnt/data".to_string()).unwrap();
        assert_eq!(manager.get_device("dev1").unwrap().state, DeviceManager2State::Mounted);
        assert_eq!(manager.get_device("dev1").unwrap().mount_point, Some("/mnt/data".to_string()));
        
        manager.unmount_device("dev1").unwrap();
        assert_eq!(manager.get_device("dev1").unwrap().state, DeviceManager2State::Unmounted);
    }

    #[test]
    fn test_disconnect_device() {
        let mut manager = SigmaDeviceManager2::new();
        manager.add_device("dev1".to_string(), "usb0".to_string(), DeviceManager2Type::USB, Some("usb-storage".to_string())).unwrap();
        
        manager.disconnect_device("dev1").unwrap();
        assert_eq!(manager.get_device("dev1").unwrap().state, DeviceManager2State::Disconnected);
    }

    #[test]
    fn test_get_devices_by_type() {
        let mut manager = SigmaDeviceManager2::new();
        manager.add_device("dev1".to_string(), "sda".to_string(), DeviceManager2Type::Block, Some("ahci".to_string())).unwrap();
        manager.add_device("dev2".to_string(), "eth0".to_string(), DeviceManager2Type::Network, Some("e1000".to_string())).unwrap();
        
        let block_devices = manager.get_devices_by_type(DeviceManager2Type::Block);
        assert_eq!(block_devices.len(), 1);
        assert_eq!(block_devices[0].id, "dev1");
    }
}
