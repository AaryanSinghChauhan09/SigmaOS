// OOP-based Hardware Compatibility Matrix for SigmaOS
// Implements supported legacy, ancient (1980s/1990s), and modern hardware devices compatibility matrix.
#![no_std]

extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type DeviceID = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    GPU = 0,
    WiFi = 1,
    Printer = 2,
    Chipset = 3,
    Audio = 4,
    Storage = 5,
    LegacyBus = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportStatus {
    Supported = 0,
    Partial = 1,
    Unsupported = 2,
    Unknown = 3,
}

pub trait HardwareDevice {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn vendor_id(&self) -> u16;
    fn device_id(&self) -> u16;
    fn name(&self) -> &str;
    fn support_status(&self) -> SupportStatus;
    fn set_support_status(&mut self, status: SupportStatus);
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub id: DeviceID,
    pub device_type: DeviceType,
    pub vendor_id: u16,
    pub device_id: u16,
    pub name: String,
    pub support_status: SupportStatus,
}

impl DeviceInfo {
    pub fn new(
        id: DeviceID,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        name: &str,
    ) -> Self {
        DeviceInfo {
            id,
            device_type,
            vendor_id,
            device_id,
            name: String::from(name),
            support_status: SupportStatus::Unknown,
        }
    }
}

pub struct SimpleHardwareDevice {
    info: DeviceInfo,
}

impl SimpleHardwareDevice {
    pub fn new(
        id: DeviceID,
        device_type: DeviceType,
        vendor_id: u16,
        device_id: u16,
        name: &str,
    ) -> Self {
        SimpleHardwareDevice {
            info: DeviceInfo::new(id, device_type, vendor_id, device_id, name),
        }
    }
}

impl HardwareDevice for SimpleHardwareDevice {
    fn id(&self) -> DeviceID {
        self.info.id
    }

    fn device_type(&self) -> DeviceType {
        self.info.device_type
    }

    fn vendor_id(&self) -> u16 {
        self.info.vendor_id
    }

    fn device_id(&self) -> u16 {
        self.info.device_id
    }

    fn name(&self) -> &str {
        &self.info.name
    }

    fn support_status(&self) -> SupportStatus {
        self.info.support_status
    }

    fn set_support_status(&mut self, status: SupportStatus) {
        self.info.support_status = status;
    }
}

pub trait CompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn HardwareDevice>) -> Result<(), CompatibilityError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice>;
    fn get_device_mut(&mut self, id: DeviceID) -> Option<&mut dyn HardwareDevice>;
    fn check_compatibility(&self, vendor_id: u16, device_id: u16) -> SupportStatus;
    fn list_devices(&self) -> Vec<DeviceInfo>;
    fn list_supported_devices(&self) -> Vec<DeviceInfo>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompatibilityError {
    Success = 0,
    NotFound = 1,
    AlreadyExists = 2,
    InvalidParameter = 3,
}

pub struct SimpleCompatibilityMatrix {
    devices: Vec<Box<dyn HardwareDevice>>,
    next_id: AtomicUsize,
}

impl SimpleCompatibilityMatrix {
    pub fn new() -> Self {
        SimpleCompatibilityMatrix {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Default for SimpleCompatibilityMatrix {
    fn default() -> Self {
        Self::new()
    }
}

impl CompatibilityMatrix for SimpleCompatibilityMatrix {
    fn add_device(&mut self, device: Box<dyn HardwareDevice>) -> Result<(), CompatibilityError> {
        self.devices.push(device);
        Ok(())
    }

    fn remove_device(&mut self, id: DeviceID) -> Result<(), CompatibilityError> {
        if let Some(index) = self.devices.iter().position(|d| d.id() == id) {
            self.devices.remove(index);
            Ok(())
        } else {
            Err(CompatibilityError::NotFound)
        }
    }

    fn get_device(&self, id: DeviceID) -> Option<&dyn HardwareDevice> {
        self.devices.iter().find(|d| d.id() == id).map(|d| d.as_ref())
    }

    fn get_device_mut(&mut self, id: DeviceID) -> Option<&mut dyn HardwareDevice> {
        self.devices
            .iter_mut()
            .find(|d| d.id() == id)
            .map(|d| d.as_mut())
    }

    fn check_compatibility(&self, vendor_id: u16, device_id: u16) -> SupportStatus {
        for device in &self.devices {
            if device.vendor_id() == vendor_id && device.device_id() == device_id {
                return device.support_status();
            }
        }
        SupportStatus::Unknown
    }

    fn list_devices(&self) -> Vec<DeviceInfo> {
        self.devices
            .iter()
            .map(|d| DeviceInfo {
                id: d.id(),
                device_type: d.device_type(),
                vendor_id: d.vendor_id(),
                device_id: d.device_id(),
                name: String::from(d.name()),
                support_status: d.support_status(),
            })
            .collect()
    }

    fn list_supported_devices(&self) -> Vec<DeviceInfo> {
        self.devices
            .iter()
            .filter(|d| d.support_status() == SupportStatus::Supported)
            .map(|d| DeviceInfo {
                id: d.id(),
                device_type: d.device_type(),
                vendor_id: d.vendor_id(),
                device_id: d.device_id(),
                name: String::from(d.name()),
                support_status: d.support_status(),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compatibility_matrix() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        
        let device = SimpleHardwareDevice::new(1, DeviceType::GPU, 0x10DE, 0x2200, "NVIDIA GPU");
        matrix.add_device(Box::new(device)).unwrap();
        
        assert_eq!(matrix.check_compatibility(0x10DE, 0x2200), SupportStatus::Unknown);
        
        if let Some(dev) = matrix.get_device_mut(1) {
            dev.set_support_status(SupportStatus::Supported);
        }
        
        assert_eq!(matrix.check_compatibility(0x10DE, 0x2200), SupportStatus::Supported);
        
        let supported = matrix.list_supported_devices();
        assert_eq!(supported.len(), 1);
    }

    #[test]
    fn test_device_lifecycle() {
        let mut matrix = SimpleCompatibilityMatrix::new();
        
        let device = SimpleHardwareDevice::new(1, DeviceType::WiFi, 0x8086, 0x4221, "Intel WiFi");
        matrix.add_device(Box::new(device)).unwrap();
        
        assert!(matrix.get_device(1).is_some());
        assert!(matrix.remove_device(1).is_ok());
        assert!(matrix.get_device(1).is_none());
    }
}
