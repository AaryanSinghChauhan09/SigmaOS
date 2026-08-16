/// OOP-based Driver Framework for SigmaOS
/// Based on Roadmap Item 1: Driver framework

extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};
use alloc::vec::Vec;
use alloc::boxed::Box;

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Block = 0,
    Char = 1,
    Network = 2,
    Storage = 3,
    Input = 4,
}

#[repr(usize)]
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
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    ProbeFailed = 3,
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

    pub fn init(&mut self) -> Result<(), DriverError> {
        Ok(())
    }

    pub fn probe(&mut self) -> Result<bool, DriverError> {
        Ok(true)
    }

    pub fn shutdown(&mut self) -> Result<(), DriverError> {
        Ok(())
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
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Active as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Unloaded as usize, Ordering::SeqCst);
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
}

impl DriverFramework for SimpleDriverFramework {
    fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<DriverID, DriverError> {
        let id = driver.id();
        self.drivers.push(Some(driver));
        Ok(id)
    }
    fn load_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for i in 0..self.drivers.len() {
            if let Some(ref mut driver) = self.drivers[i] {
                if driver.id() == id {
                    return driver.load();
                }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for i in 0..self.drivers.len() {
            if let Some(ref mut driver) = self.drivers[i] {
                if driver.id() == id {
                    return driver.unload();
                }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for i in 0..self.drivers.len() {
            if let Some(ref driver) = self.drivers[i] {
                if driver.id() == id {
                    return Some(driver.as_ref());
                }
            }
        }
        None
    }
}
