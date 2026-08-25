#![no_std]

extern crate alloc;
use alloc::string::String;
use alloc::vec::Vec;
use core::any::Any;

use crate::kernel::device::{Device, DeviceBinding, DeviceType, DriverError, DriverMetadata};
use crate::kernel::object::{KRef, KernelObject};
use crate::security::capability::CapabilityToken;

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

    // Linux & BSD inspired module features
    fn is_pqc_signed(&self) -> bool { true }
    fn get_module_param(&self, _param_name: &str) -> Option<String> { None }
    fn set_module_param(&mut self, _param_name: &str, _value: &str) -> Result<(), DriverError> { Ok(()) }
}

pub trait DeviceDriver: Any + Send + Sync {
    fn init(&mut self) -> Result<(), DriverError>;
    fn handle_io(&mut self, operation: u32) -> Result<u32, DriverError>;
    fn shutdown(&mut self) -> Result<(), DriverError>;
    fn metadata(&self) -> &DriverMetadata;
    fn has_capability(&self, capability: u64) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    // Linux & BSD inspired module features
    fn get_module_param(&self, _param_name: &str) -> Option<String> { None }
    fn set_module_param(&mut self, _param_name: &str, _value: &str) -> Result<(), DriverError> { Ok(()) }
}

#[derive(Debug, Clone)]
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

    pub fn find_driver<'a>(&'a self, name: &str) -> Option<&'a (dyn Driver + 'static)> {
        self.drivers
            .iter()
            .find(|d| d.driver.driver_name() == name)
            .map(|d| d.driver.as_ref())
    }

    pub fn find_driver_mut<'a>(&'a mut self, name: &str) -> Option<&'a mut (dyn Driver + 'static)> {
        self.drivers
            .iter_mut()
            .find(|d| d.driver.driver_name() == name)
            .map(|d| d.driver.as_mut())
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
        let mut bindings = Vec::new();
        for device in self.device_manager.devices() {
            for reg in &mut self.drivers {
                if !reg.loaded && reg.driver.probe(device) {
                    if let Some(driver) = reg.driver.as_driver_impl_mut() {
                        driver.init()?;
                        bindings.push((device.name().to_string(), reg.driver.driver_name().to_string()));
                        reg.loaded = true;
                        break;
                    }
                }
            }
        }
        for (dev_name, drv_name) in bindings {
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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::kernel::object::{KObject, ObjectError};

    struct MockDriver {
        owner: Option<String>,
        debug_level: String,
        base: KObject,
    }

    impl KernelObject for MockDriver {
        fn name(&self) -> &str { self.base.name() }
        fn set_name(&mut self, name: &str) { self.base.set_name(name); }
        fn parent(&self) -> Option<&dyn KernelObject> { self.base.parent() }
        fn set_parent(&mut self, parent: Option<&dyn KernelObject>) { self.base.set_parent(parent); }
        fn children(&self) -> Vec<&dyn KernelObject> { self.base.children() }
        fn add_child(&mut self, child: &dyn KernelObject) { self.base.add_child(child); }
        fn remove_child(&mut self, child_name: &str) -> Option<alloc::boxed::Box<dyn KernelObject>> { self.base.remove_child(child_name) }
        fn kref(&self) -> &KRef { self.base.kref() }
        fn as_any(&self) -> &dyn core::any::Any { self }
        fn as_any_mut(&mut self) -> &mut dyn core::any::Any { self }
        fn sysfs_attrs(&self) -> Vec<&str> { self.base.sysfs_attrs() }
        fn sysfs_show(&self, attr: &str) -> Option<String> { self.base.sysfs_show(attr) }
        fn sysfs_store(&mut self, attr: &str, value: &str) -> Result<(), ObjectError> { self.base.sysfs_store(attr, value) }
    }

    impl Driver for MockDriver {
        fn driver_name(&self) -> &str { "mock_driver" }
        fn set_owner(&mut self, owner: &str) { self.owner = Some(owner.to_string()); }
        fn owner(&self) -> Option<&str> { self.owner.as_deref() }
        fn probe(&self, _device: &dyn Device) -> bool { true }
        fn attach(&mut self, _device: &mut dyn Device) -> Result<(), DriverError> { Ok(()) }
        fn detach(&mut self, _device: &mut dyn Device) -> Result<(), DriverError> { Ok(()) }
        fn supported_devices(&self) -> Vec<DeviceType> { Vec::new() }
        fn as_driver_impl(&self) -> Option<&dyn DeviceDriver> { None }
        fn as_driver_impl_mut(&mut self) -> Option<&mut dyn DeviceDriver> { None }

        fn get_module_param(&self, param_name: &str) -> Option<String> {
            if param_name == "debug" {
                Some(self.debug_level.clone())
            } else {
                None
            }
        }

        fn set_module_param(&mut self, param_name: &str, value: &str) -> Result<(), DriverError> {
            if param_name == "debug" {
                self.debug_level = value.to_string();
                Ok(())
            } else {
                Err(DriverError::NotFound)
            }
        }
    }

    #[test]
    fn test_driver_module_params_and_pqc_signing() {
        let mut drv = MockDriver {
            owner: None,
            debug_level: "3".to_string(),
            base: KObject::new("mock_driver"),
        };

        assert!(drv.is_pqc_signed());
        assert_eq!(drv.get_module_param("debug"), Some("3".to_string()));

        assert!(drv.set_module_param("debug", "5").is_ok());
        assert_eq!(drv.get_module_param("debug"), Some("5".to_string()));
        assert_eq!(drv.get_module_param("nonexistent"), None);
    }
}
