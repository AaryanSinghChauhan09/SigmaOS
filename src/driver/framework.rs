#![no_std]
#![no_main]

use core::mem;
use core::sync::atomic::{AtomicUsize, Ordering};

extern crate alloc;
use alloc::boxed::Box;
use crate::klib_vec::Vec;

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverState {
    Unloaded = 0,
    Loaded = 1,
    Active = 2,
}

pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
    fn dependencies(&self) -> &[DriverType] {
        &[]
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
}

#[repr(C)]
pub struct SimpleDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleDriver {
    pub fn new(id: DriverID, driver_type: DriverType) -> Self {
        SimpleDriver {
            id,
            driver_type,
            state: AtomicUsize::new(DriverState::Unloaded as usize),
        }
    }

    pub fn set_state(&self, state: DriverState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

impl Driver for SimpleDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        self.driver_type
    }
    fn state(&self) -> DriverState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst) as u32) }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.set_state(DriverState::Active);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.set_state(DriverState::Unloaded);
        Ok(())
    }
}

pub trait DriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError>;
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError>;
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver>;
}

#[allow(dead_code)]
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

    fn verify_dependencies(&self, driver: &dyn Driver) -> bool {
        for &dep in driver.dependencies() {
            let mut dep_found = false;
            for option_d in self.drivers.iter() {
                if let Some(ref d) = *option_d {
                    // Check if matching driver is loaded
                    let dt = d.driver_type() as usize;
                    let dep_t = dep as usize;
                    if dt == dep_t && d.state() == DriverState::Active {
                        dep_found = true;
                        break;
                    }
                }
            }
            if !dep_found {
                return false;
            }
        }
        true
    }
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}
