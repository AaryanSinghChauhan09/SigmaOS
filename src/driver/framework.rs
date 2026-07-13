#![no_std]
#![no_main]

/// OOP-based Driver Framework for SigmaOS
/// Based on Roadmap Item 1: Driver framework

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DriverID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverType { Block = 0, Char = 1, Network = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverState { Unloaded = 0, Loaded = 1, Active = 2 }

pub trait Driver {
    fn id(&self) -> DriverID;
    fn driver_type(&self) -> DriverType;
    fn state(&self) -> DriverState;
    fn load(&mut self) -> Result<(), DriverError>;
    fn unload(&mut self) -> Result<(), DriverError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DriverError { Success = 0, LoadFailed = 1, UnloadFailed = 2 }

#[repr(C)]
pub struct SimpleDriver {
    pub id: DriverID,
    pub driver_type: DriverType,
    pub state: AtomicUsize,
}

impl SimpleDriver {
    pub fn new(id: DriverID, driver_type: DriverType) -> Self {
        SimpleDriver { id, driver_type, state: AtomicUsize::new(DriverState::Unloaded as usize) }
    }
}

impl Driver for SimpleDriver {
    fn id(&self) -> DriverID { self.id }
    fn driver_type(&self) -> DriverType { self.driver_type }
    fn state(&self) -> DriverState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn load(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Loaded as usize, Ordering::SeqCst);
        Ok(())
    }
    fn unload(&mut self) -> Result<(), DriverError> {
        self.state.store(DriverState::Unloaded as usize, Ordering::SeqCst);
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

impl SimpleDriverFramework {
    pub fn new() -> Self { SimpleDriverFramework { drivers: Vec::new(), next_id: AtomicUsize::new(1) } }
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
                if driver.id() == id { return driver.load(); }
            }
        }
        Err(DriverError::LoadFailed)
    }
    fn unload_driver(&mut self, id: DriverID) -> Result<(), DriverError> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == id { return driver.unload(); }
            }
        }
        Err(DriverError::UnloadFailed)
    }
    fn get_driver(&self, id: DriverID) -> Option<&dyn Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

struct Vec<T> { data: *mut T, len: usize, capacity: usize }

impl<T> Vec<T> {
    fn new() -> Self { Vec { data: core::ptr::null_mut(), len: 0, capacity: 0 } }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity { self.grow(); }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 { 4 } else { self.capacity * 2 };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len { core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1); }
            if self.capacity > 0 { free(self.data as *mut u8); }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

extern "C" { fn alloc(size: usize) -> *mut u8; fn free(ptr: *mut u8); }
