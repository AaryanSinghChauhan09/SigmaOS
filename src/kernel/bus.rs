#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::kernel::object::{KernelObject, KRef};
use crate::security::CapabilityToken;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BusError {
    DeviceNotFound,
    DriverNotFound,
    ProbeFailed,
    AlreadyBound,
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
    pub capabilities: Vec<u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    InitFailed,
    IoFailed,
    ShutdownFailed,
    CapabilityDenied,
}

pub trait Bus: KernelObject + Send + Sync {
    fn bus_type(&self) -> &str;
    fn match_device(&self, device: &dyn DeviceDriver) -> bool;
    fn probe(&self, device: &mut dyn DeviceDriver) -> Result<(), BusError>;
    fn remove(&self, device: &mut dyn DeviceDriver) -> Result<(), BusError>;
    fn devices(&self) -> Vec<&dyn DeviceDriver>;
    fn drivers(&self) -> Vec<&dyn DeviceDriver>;
    fn bind_driver(&mut self, device: &mut dyn DeviceDriver, driver: &mut dyn DeviceDriver) -> Result<(), BusError>;
}

pub struct PciBus {
    base: super::KObject,
    devices: Vec<Box<dyn DeviceDriver>>,
    drivers: Vec<Box<dyn DeviceDriver>>,
    capability: CapabilityToken,
}

impl PciBus {
    pub fn new() -> Self {
        PciBus {
            base: super::KObject::new("pci"),
            devices: Vec::new(),
            drivers: Vec::new(),
            capability: CapabilityToken::new(),
        }
    }

    pub fn register_device(&mut self, device: Box<dyn DeviceDriver>) {
        self.devices.push(device);
    }

    pub fn register_driver(&mut self, driver: Box<dyn DeviceDriver>) {
        self.drivers.push(driver);
    }
}

impl KernelObject for PciBus {
    fn name(&self) -> &str {
        self.base.name()
    }

    fn set_name(&mut self, name: &str) {
        self.base.set_name(name);
    }

    fn parent(&self) -> Option<&dyn KernelObject> {
        None
    }

    fn set_parent(&mut self, _parent: Option<&dyn KernelObject>) {}

    fn children(&self) -> Vec<&dyn KernelObject> {
        Vec::new()
    }

    fn add_child(&mut self, _child: &dyn KernelObject) {}

    fn remove_child(&mut self, _child_name: &str) -> Option<Box<dyn KernelObject>> {
        None
    }

    fn kref(&self) -> &KRef {
        self.base.kref()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn sysfs_attrs(&self) -> Vec<&str> {
        vec!["vendor", "device", "class", "irq"]
    }

    fn sysfs_show(&self, attr: &str) -> Option<String> {
        match attr {
            "vendor" => Some(format!("{:04x}", 0x8086)),
            "device" => Some(format!("{:04x}", 0x1234)),
            _ => None,
        }
    }

    fn sysfs_store(&mut self, _attr: &str, _value: &str) -> Result<(), crate::kernel::object::ObjectError> {
        Err(crate::kernel::object::ObjectError::CapabilityDenied)
    }
}

impl Bus for PciBus {
    fn bus_type(&self) -> &str {
        "pci"
    }

    fn match_device(&self, _device: &dyn DeviceDriver) -> bool {
        true
    }

    fn probe(&self, _device: &mut dyn DeviceDriver) -> Result<(), BusError> {
        Ok(())
    }

    fn remove(&self, _device: &mut dyn DeviceDriver) -> Result<(), BusError> {
        Ok(())
    }

    fn devices(&self) -> Vec<&dyn DeviceDriver> {
        Vec::new()
    }

    fn drivers(&self) -> Vec<&dyn DeviceDriver> {
        Vec::new()
    }

    fn bind_driver(&mut self, _device: &mut dyn DeviceDriver, _driver: &mut dyn DeviceDriver) -> Result<(), BusError> {
        Ok(())
    }
}

pub trait UsableBus: Bus {
    fn init(&mut self) -> Result<(), BusError>;
    fn rescan(&mut self) -> Result<(), BusError>;
}