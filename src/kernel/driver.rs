#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::kernel::device::{Device, DeviceBinding, DeviceType, DriverError, DriverMetadata};
use crate::kernel::object::{KRef, KernelObject};
use crate::security::CapabilityToken;

pub trait Driver: KernelObject + Send + Sync {
    fn driver_name(&self) -> &str;
    fn set_owner(&mut self, owner: &str);
    fn owner(&self) -> Option<&str>;
    fn probe(&self, device: &dyn Device) -> bool;
    fn attach(&mut self, device: &mut dyn Device) -> Result<(), DriverError>;
    fn detach(&mut self, device: &mut dyn Device) -> Result<(), DriverError>;
    fn supported_devices(&self) -> Vec<DeviceType>;
    fn as_driver_impl(&self) -> Option<&dyn DeviceDriver>;
    fn as_driver_impl_mut(&mut self) -> Option<&mut dyn DeviceDriver>;
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

pub struct DriverRegistration {
    pub driver: Box<dyn Driver>,
    pub priority: u32,
    pub builtin: bool,
    pub loaded: bool,
}

pub struct DriverRegistry {
    drivers: Vec<DriverRegistration>,
    device_manager: crate::kernel::device::DeviceManager,
    bus_list: Vec<String>,
}

impl DriverRegistry {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        DriverRegistry {
            drivers: Vec::new(),
            device_manager: crate::kernel::device::DeviceManager::new(),
            bus_list: Vec::new(),
        }
    }

    pub fn register_driver(
        &mut self,
        driver: Box<dyn Driver>,
        priority: u32,
    ) -> Result<(), DriverError> {
        let name = driver.driver_name().to_string();
        for reg in &self.drivers {
            if reg.driver.driver_name() == name {
                return Err(DriverError::AlreadyRegistered);
            }
        }
        self.drivers.push(DriverRegistration {
            driver,
            priority,
            builtin: false,
            loaded: false,
        });
        self.drivers.sort_by(|a, b| b.priority.cmp(&a.priority));
        Ok(())
    }

    pub fn register_builtin_driver(&mut self, driver: Box<dyn Driver>) -> Result<(), DriverError> {
        let name = driver.driver_name().to_string();
        for reg in &self.drivers {
            if reg.driver.driver_name() == name {
                return Err(DriverError::AlreadyRegistered);
            }
        }
        self.drivers.push(DriverRegistration {
            driver,
            priority: 100,
            builtin: true,
            loaded: false,
        });
        Ok(())
    }

    pub fn unregister_driver(&mut self, name: &str) -> Option<Box<dyn Driver>> {
        if let Some(idx) = self
            .drivers
            .iter()
            .position(|d| d.driver.driver_name() == name)
        {
            let reg = self.drivers.remove(idx);
            Some(reg.driver)
        } else {
            None
        }
    }

    pub fn find_driver(&self, name: &str) -> Option<&dyn Driver> {
        self.drivers
            .iter()
            .find(|d| d.driver.driver_name() == name)
            .map(|d| d.driver.as_ref())
    }

    pub fn find_driver_mut(&mut self, name: &str) -> Option<&mut dyn Driver> {
        for reg in self.drivers.iter_mut() {
            if reg.driver.driver_name() == name {
                return Some(reg.driver.as_mut());
            }
        }
        None
    }

    pub fn register_device(&mut self, device: Box<dyn Device>) -> Result<(), DriverError> {
        self.device_manager.register_device(device)
    }

    pub fn bind_device_driver(
        &mut self,
        device_name: &str,
        driver_name: &str,
    ) -> Result<(), DriverError> {
        self.device_manager.bind_driver(device_name, driver_name)
    }

    pub fn probe_and_bind(&mut self) -> Result<(), DriverError> {
        let mut bindings_to_make = Vec::new();

        for device in self.device_manager.devices() {
            for reg in &mut self.drivers {
                if !reg.loaded && reg.driver.probe(device) {
                    if let Some(driver) = reg.driver.as_driver_impl_mut() {
                        driver.init()?;
                        bindings_to_make.push((device.name().to_string(), reg.driver.driver_name().to_string()));
                        reg.loaded = true;
                        break;
                    }
                }
            }
        }

        for (dev_name, drv_name) in bindings_to_make {
            self.device_manager.bind_driver(&dev_name, &drv_name)?;
        }

        Ok(())
    }

    pub fn init_all(&mut self) -> Result<(), DriverError> {
        for reg in &mut self.drivers {
            if let Some(driver) = reg.driver.as_driver_impl_mut() {
                driver.init()?;
                reg.loaded = true;
            }
        }
        Ok(())
    }

    pub fn shutdown_all(&mut self) -> Result<(), DriverError> {
        for reg in self.drivers.iter_mut().rev() {
            if let Some(driver) = reg.driver.as_driver_impl_mut() {
                driver.shutdown()?;
                reg.loaded = false;
            }
        }
        Ok(())
    }

    pub fn list_drivers(&self) -> Vec<&dyn Driver> {
        self.drivers.iter().map(|d| d.driver.as_ref()).collect()
    }

    pub fn list_devices(&self) -> Vec<&dyn Device> {
        self.device_manager.devices()
    }

    pub fn list_bindings(&self) -> &[DeviceBinding] {
        self.device_manager.bindings()
    }

    pub fn add_bus(&mut self, bus_name: &str) {
        if !self.bus_list.contains(&bus_name.to_string()) {
            self.bus_list.push(bus_name.to_string());
        }
    }

    pub fn buses(&self) -> &[String] {
        &self.bus_list
    }
}
