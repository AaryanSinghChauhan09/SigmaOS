/// OOP-based Driver Framework for SigmaOS
/// Based on Roadmap Item 1: Driver framework

extern crate alloc;

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    Success = 0,
    LoadFailed = 1,
    UnloadFailed = 2,
    ProbeFailed = 3,
}

pub trait NetworkDriver: Driver {
    fn send_packet(&mut self, packet: &[u8]) -> Result<(), DriverError>;
    fn receive_packet(&mut self, buf: &mut [u8]) -> Result<usize, DriverError>;
}

pub trait GraphicsDriver: Driver {
    fn set_resolution(&mut self, width: u32, height: u32) -> Result<(), DriverError>;
    fn flip_buffers(&mut self) -> Result<(), DriverError>;
}

pub trait InputDriver: Driver {
    fn poll_events(&mut self) -> Result<usize, DriverError>;
}

// Concrete Driver Classes (OOP Implementation)

pub struct SimpleStorageDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleStorageDriver {
    pub fn new(id: DriverID) -> Self {
        SimpleStorageDriver {
            id,
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

impl Driver for SimpleStorageDriver {
    fn id(&self) -> DriverID {
        self.id
    }
    fn driver_type(&self) -> DriverType {
        DriverType::Storage
    }
    fn state(&self) -> DriverState {
        unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) }
    }
    fn set_state(&self, state: DriverState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
    fn init(&mut self) -> Result<(), DriverError> {
        Ok(())
    }
    fn probe(&mut self) -> Result<bool, DriverError> {
        Ok(true)
    }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state
            .store(DriverState::Active as usize, Ordering::SeqCst);
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
