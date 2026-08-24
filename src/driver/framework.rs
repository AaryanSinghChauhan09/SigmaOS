// ==========================================
// Basic Driver Framework Implementation
// ==========================================

#![no_std]

extern crate alloc;

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::sync::atomic::AtomicUsize;

pub type DriverID = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriverType {
    Char,
    Block,
    Net,
    Gpu,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriverState {
    Unloaded,
    Active,
    Failed,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum DriverError {
    LoadFailed,
    UnloadFailed,
    NotFound,
}

pub trait Driver {
    fn id(&self) -> DriverID;
    fn name(&self) -> &str;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
}

pub struct SimpleStorageDriver {
    id: DriverID,
    driver_type: DriverType,
    state: DriverState,
}

impl SimpleStorageDriver {
    pub fn new(id: DriverID, driver_type: DriverType) -> Self {
        Self {
            id,
            driver_type,
            state: DriverState::Unloaded,
        }
    }
}

impl Driver for SimpleStorageDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn name(&self) -> &str {
        "SimpleStorageDriver"
    }
    fn driver_type(&self) -> DriverType {
        self.driver_type
    }
    fn state(&self) -> DriverState {
        self.state
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state = DriverState::Active;
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state = DriverState::Unloaded;
        Ok(())
    }
}

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
}

pub struct SimpleDriverFramework {
    drivers: Vec<Option<Box<dyn Driver>>>,
    next_id: AtomicUsize,
}

impl Default for SimpleDriverFramework {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleDriverFramework {
    pub fn new() -> Self {
        SimpleDriverFramework {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in self.drivers.iter_mut() {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in self.drivers.iter() {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}

// ==========================================
// Unit Tests
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_driver_framework_lifecycle() {
        let mut framework = SimpleDriverFramework::new();
        let driver = Box::new(SimpleDriver::new(101, DriverType::Block));

        let reg_id = framework.register_driver(driver).unwrap();
        assert_eq!(reg_id, 101);

        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Unloaded);

        framework.load_driver(101).unwrap();
        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Active);

        framework.unload_driver(101).unwrap();
        assert_eq!(framework.get_driver(101).unwrap().state(), DriverState::Unloaded);
    }
}
