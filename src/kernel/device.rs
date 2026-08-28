extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::kernel::object::{KRef, KernelObject};
use crate::security::capability::CapabilityToken;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeviceType {
    Network,
    Block,
    Character,
    Graphics,
    Audio,
    Input,
    Storage,
    Sensor,
    Embedded,
    Other(String),
}

pub trait Device: KernelObject + Send + Sync {
    fn device_id(&self) -> u16;
    fn vendor_id(&self) -> u16;
    fn device_type(&self) -> DeviceType;
    fn set_driver(&mut self, driver_name: &str);
    fn driver_name(&self) -> Option<&str>;
    fn capabilities(&self) -> Vec<u64>;
    fn add_capability(&mut self, cap: u64);
    fn has_capability(&self, cap: u64) -> bool;
    fn as_driver(&self) -> Option<&dyn DeviceDriver>;
    fn as_driver_mut(&mut self) -> Option<&mut dyn DeviceDriver>;
}

pub trait DeviceDriver: Any + Send + Sync {
    fn init(&mut self) -> Result<(), DriverError>;
    fn handle_io(&mut self, operation: u32) -> Result<u32, DriverError>;
    fn shutdown(&mut self) -> Result<(), DriverError>;
    fn metadata(&self) -> &DriverMetadata;
    fn has_capability(&self, capability: u64) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct DriverMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub driver_type: DeviceType,
    pub capabilities: Vec<u64>,
    pub required_capabilities: Vec<u64>,
    pub linux_heritage: Option<String>,
}

impl DriverMetadata {
    pub fn new(name: &str) -> Self {
        DriverMetadata {
            name: name.to_string(),
            version: "1.0".to_string(),
            author: String::new(),
            description: String::new(),
            driver_type: DeviceType::Other(String::new()),
            capabilities: Vec::new(),
            required_capabilities: Vec::new(),
            linux_heritage: None,
        }
    }

    pub fn with_linux_heritage(mut self, heritage: &str) -> Self {
        self.linux_heritage = Some(heritage.to_string());
        self
    }

    pub fn with_capability(mut self, cap: u64) -> Self {
        self.capabilities.push(cap);
        self
    }

    pub fn requires_capability(mut self, cap: u64) -> Self {
        self.required_capabilities.push(cap);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InitFailed,
    IoFailed,
    ShutdownFailed,
    CapabilityDenied,
    AlreadyRegistered,
    NotFound,
}

pub struct DeviceBinding {
    pub device_name: String,
    pub driver_name: String,
    pub bound_at: u64,
    pub capabilities: Vec<u64>,
}

impl DeviceBinding {
    pub fn new(device_name: &str, driver_name: &str) -> Self {
        DeviceBinding {
            device_name: device_name.to_string(),
            driver_name: driver_name.to_string(),
            bound_at: 0,
            capabilities: Vec::new(),
        }
    }
}

pub struct DeviceManager {
    devices: Vec<Box<dyn Device>>,
    bindings: Vec<DeviceBinding>,
    capability_token: CapabilityToken,
}

impl DeviceManager {
    pub fn new() -> Self {
        DeviceManager {
            devices: Vec::new(),
            bindings: Vec::new(),
            capability_token: CapabilityToken::new(),
        }
    }

    pub fn register_device(&mut self, device: Box<dyn Device>) -> Result<(), DriverError> {
        self.devices.push(device);
        Ok(())
    }

    pub fn remove_device(&mut self, name: &str) -> Option<Box<dyn Device>> {
        if let Some(idx) = self.devices.iter().position(|d| d.name() == name) {
            Some(self.devices.remove(idx))
        } else {
            None
        }
    }

    pub fn find_device<'a>(&'a self, name: &str) -> Option<&'a (dyn Device + 'static)> {
        self.devices
            .iter()
            .find(|d| d.name() == name)
            .map(|d| d.as_ref())
    }

    pub fn find_device_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut (dyn Device + 'static)> {
        self.devices
            .iter_mut()
            .find(|d| d.name() == name)
            .map(|d| d.as_mut())
    }

    pub fn bind_driver(&mut self, device_name: &str, driver_name: &str) -> Result<(), DriverError> {
        if let Some(device) = self.devices.iter_mut().find(|d| d.name() == device_name) {
            let binding = DeviceBinding::new(device_name, driver_name);
            self.bindings.push(binding);
            device.set_driver(driver_name);
            Ok(())
        } else {
            Err(DriverError::InitFailed)
        }
    }

    pub fn devices(&self) -> Vec<&dyn Device> {
        self.devices.iter().map(|d| d.as_ref()).collect()
    }

    pub fn bindings(&self) -> &[DeviceBinding] {
        &self.bindings
    }
}
