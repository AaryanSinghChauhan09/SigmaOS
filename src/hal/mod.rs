//! Hardware Abstraction Layer (HAL/udev Inspiration)
//! Device discovery, properties, events, and hotplug support

#![no_std]

extern crate alloc;

use crate::klib::{Vec, String};
use alloc::vec::Vec;
use alloc::string::String;

/// Device class
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceClass {
    Audio,
    Video,
    Network,
    Storage,
    Input,
    Bluetooth,
    USB,
    PCI,
    Unknown,
}

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Added,
    Removed,
    Changed,
    Online,
    Offline,
}

/// Hardware device
#[derive(Debug, Clone)]
pub struct HardwareDevice {
    pub id: String,
    pub name: String,
    pub device_class: DeviceClass,
    pub state: DeviceState,
    pub properties: DeviceProperties,
    pub subsystem: String,
}

#[derive(Debug, Clone)]
pub struct DeviceProperties {
    pub vendor: String,
    pub model: String,
    pub serial: String,
    pub driver: String,
    pub firmware: String,
}

impl HardwareDevice {
    pub fn new(name: &str, device_class: DeviceClass) -> Self {
        Self {
            id: Self::generate_id(),
            name: name.to_string(),
            device_class,
            state: DeviceState::Added,
            properties: DeviceProperties {
                vendor: String::new(),
                model: String::new(),
                serial: String::new(),
                driver: String::new(),
                firmware: String::new(),
            },
            subsystem: String::new(),
        }
    }

    fn generate_id() -> String {
        "device_abcdef1234567890".to_string()
    }

    pub fn set_properties(&mut self, vendor: &str, model: &str, serial: &str) {
        self.properties.vendor = vendor.to_string();
        self.properties.model = model.to_string();
        self.properties.serial = serial.to_string();
    }

    pub fn set_driver(&mut self, driver: &str) {
        self.properties.driver = driver.to_string();
    }

    pub fn set_subsystem(&mut self, subsystem: &str) {
        self.subsystem = subsystem.to_string();
    }

    pub fn set_state(&mut self, state: DeviceState) {
        self.state = state;
    }
}

/// Device event
#[derive(Debug, Clone)]
pub struct DeviceEvent {
    pub device_id: String,
    pub event_type: DeviceEventType,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceEventType {
    Add,
    Remove,
    Change,
    Move,
    Bind,
    Unbind,
}

impl DeviceEvent {
    pub fn new(device_id: &str, event_type: DeviceEventType) -> Self {
        Self {
            device_id: device_id.to_string(),
            event_type,
            timestamp: 0,
        }
    }
}

/// Subsystem
#[derive(Debug, Clone)]
pub struct Subsystem {
    pub name: String,
    pub devices: Vec<String>,
}

impl Subsystem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            devices: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device_id: &str) {
        self.devices.push(device_id.to_string());
    }

    pub fn remove_device(&mut self, device_id: &str) {
        self.devices.retain(|id| id != device_id);
    }
}

/// HAL manager
pub struct HALManager {
    pub devices: Vec<HardwareDevice>,
    pub subsystems: Vec<Subsystem>,
    pub events: Vec<DeviceEvent>,
}

impl HALManager {
    pub fn new() -> Self {
        Self {
            devices: Vec::new(),
            subsystems: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_device(&mut self, device: HardwareDevice) {
        self.devices.push(device);
    }

    pub fn get_device(&mut self, id: &str) -> Option<&mut HardwareDevice> {
        self.devices.iter_mut().find(|d| d.id == id || d.name == id)
    }

    pub fn remove_device(&mut self, id: &str) -> Result<(), HALError> {
        self.devices.retain(|d| d.id != id && d.name != id);
        Ok(())
    }

    pub fn add_subsystem(&mut self, subsystem: Subsystem) {
        self.subsystems.push(subsystem);
    }

    pub fn get_subsystem(&mut self, name: &str) -> Option<&mut Subsystem> {
        self.subsystems.iter_mut().find(|s| s.name == name)
    }

    pub fn emit_event(&mut self, event: DeviceEvent) {
        self.events.push(event);
    }

    pub fn get_devices_by_class(&self, device_class: DeviceClass) -> Vec<&HardwareDevice> {
        self.devices.iter().filter(|d| d.device_class == device_class).collect()
    }

    pub fn get_devices_by_subsystem(&self, subsystem: &str) -> Vec<&HardwareDevice> {
        self.devices.iter().filter(|d| d.subsystem == subsystem).collect()
    }

    pub fn get_hal_stats(&self) -> HALStats {
        HALStats {
            total_devices: self.devices.len(),
            total_subsystems: self.subsystems.len(),
            total_events: self.events.len(),
            online_devices: self.devices.iter().filter(|d| d.state == DeviceState::Online).count(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct HALStats {
    pub total_devices: usize,
    pub total_subsystems: usize,
    pub total_events: usize,
    pub online_devices: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HALError {
    DeviceNotFound,
    SubsystemNotFound,
    EventFailed,
}

impl Default for HALManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_device() {
        let device = HardwareDevice::new("test-device", DeviceClass::USB);
        assert_eq!(device.name, "test-device");
    }

    #[test]
    fn test_device_event() {
        let event = DeviceEvent::new("device-1", DeviceEventType::Add);
        assert_eq!(event.event_type, DeviceEventType::Add);
    }

    #[test]
    fn test_subsystem() {
        let mut subsystem = Subsystem::new("usb");
        subsystem.add_device("device-1");
        assert_eq!(subsystem.devices.len(), 1);
    }

    #[test]
    fn test_hal_manager() {
        let mut manager = HALManager::new();
        let device = HardwareDevice::new("test-device", DeviceClass::USB);
        manager.add_device(device);
        assert_eq!(manager.devices.len(), 1);
    }

    #[test]
    fn test_get_devices_by_class() {
        let mut manager = HALManager::new();
        let device = HardwareDevice::new("usb-device", DeviceClass::USB);
        manager.add_device(device);
        let usb_devices = manager.get_devices_by_class(DeviceClass::USB);
        assert_eq!(usb_devices.len(), 1);
    }
}