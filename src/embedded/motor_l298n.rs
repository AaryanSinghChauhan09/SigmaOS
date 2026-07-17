#![no_std]
#![no_main]

/// OOP-based L298N Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2746
/// Implements L298N dual H-bridge motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type L298NID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum L298NError { Success = 0, NotFound = 1 }

pub trait L298NDriver {
    fn id(&self) -> L298NID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleL298NDriver {
    pub id: L298NID,
    pub enabled: AtomicUsize,
}

impl SimpleL298NDriver {
    pub fn new(id: L298NID) -> Self {
        SimpleL298NDriver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl L298NDriver for SimpleL298NDriver {
    fn id(&self) -> L298NID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait L298NController {
    fn set_speed(&self, l298n_id: L298NID, motor: u8, speed: u8) -> Result<(), L298NError>;
    fn set_direction(&self, l298n_id: L298NID, motor: u8, forward: bool) -> Result<(), L298NError>;
    def stop(&self, l298n_id: L298NID, motor: u8) -> Result<(), L298NError>;
}

#[repr(C)]
pub struct SimpleL298NController {
    pub drivers: Vec<Option<Box<dyn L298NDriver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleL298NController {
    pub fn new() -> Self {
        SimpleL298NController {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl L298NController for SimpleL298NController {
    fn set_speed(&self, l298n_id: L298NID, _motor: u8, speed: u8) -> Result<(), L298NError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == l298n_id {
                    if speed > 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    } else {
                        driver.enabled.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(L298NError::NotFound)
    }
    
    fn set_direction(&self, l298n_id: L298NID, _motor: u8, _forward: bool) -> Result<(), L298NError> {
        if self.get_driver(l298n_id).is_some() {
            Ok(())
        } else {
            Err(L298NError::NotFound)
        }
    }
    
    fn stop(&self, l298n_id: L298NID, _motor: u8) -> Result<(), L298NError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == l298n_id {
                    driver.enabled.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(L298NError::NotFound)
    }
    
    fn get_driver(&self, id: L298NID) -> Option<&dyn L298NDriver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait L298NEnable {
    def set_enable(&mut self, l298n_id: L298NID, enable: bool) -> Result<(), L298NError>;
    def get_enable(&self, l298n_id: L298NID) -> Result<bool, L298NError>;
}

#[repr(C)]
pub struct SimpleL298NEnable {
    pub controller: SimpleL298NController,
}

impl SimpleL298NEnable {
    pub fn new(controller: SimpleL298NController) -> Self {
        SimpleL298NEnable { controller }
    }
}

impl L298NEnable for SimpleL298NEnable {
    fn set_enable(&mut self, _l298n_id: L298NID, _enable: bool) -> Result<(), L298NError> {
        Ok(())
    }
    
    fn get_enable(&self, l298n_id: L298NID) -> Result<bool, L298NError> {
        if self.controller.get_driver(l298n_id).is_some() {
            Ok(false)
        } else {
            Err(L298NError::NotFound)
        }
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
