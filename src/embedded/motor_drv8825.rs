#![no_std]
#![no_main]

/// OOP-based DRV8825 Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2766
/// Implements DRV8825 stepper motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DRV8825ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DRV8825Error { Success = 0, NotFound = 1 }

pub trait DRV8825Driver {
    fn id(&self) -> DRV8825ID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDRV8825Driver {
    pub id: DRV8825ID,
    pub enabled: AtomicUsize,
}

impl SimpleDRV8825Driver {
    pub fn new(id: DRV8825ID) -> Self {
        SimpleDRV8825Driver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl DRV8825Driver for SimpleDRV8825Driver {
    fn id(&self) -> DRV8825ID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait DRV8825Controller {
    fn step(&self, drv_id: DRV8825ID, steps: i32) -> Result<(), DRV8825Error>;
    def set_microstep(&mut self, drv_id: DRV8825ID, mode: u8) -> Result<(), DRV8825Error>;
    def set_direction(&mut self, drv_id: DRV8825ID, forward: bool) -> Result<(), DRV8825Error>;
}

#[repr(C)]
pub struct SimpleDRV8825Controller {
    pub drivers: Vec<Option<Box<dyn DRV8825Driver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDRV8825Controller {
    pub fn new() -> Self {
        SimpleDRV8825Controller {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DRV8825Controller for SimpleDRV8825Controller {
    fn step(&self, drv_id: DRV8825ID, steps: i32) -> Result<(), DRV8825Error> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == drv_id {
                    if steps != 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(DRV8825Error::NotFound)
    }
    
    fn set_microstep(&mut self, _drv_id: DRV8825ID, _mode: u8) -> Result<(), DRV8825Error> {
        Ok(())
    }
    
    fn set_direction(&mut self, _drv_id: DRV8825ID, _forward: bool) -> Result<(), DRV8825Error> {
        Ok(())
    }
    
    fn get_driver(&self, id: DRV8825ID) -> Option<&dyn DRV8825Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait DRV8825Enable {
    def set_enable(&mut self, drv_id: DRV8825ID, enable: bool) -> Result<(), DRV8825Error>;
    def get_enable(&self, drv_id: DRV8825ID) -> Result<bool, DRV8825Error>;
}

#[repr(C)]
pub struct SimpleDRV8825Enable {
    pub controller: SimpleDRV8825Controller,
}

impl SimpleDRV8825Enable {
    pub fn new(controller: SimpleDRV8825Controller) -> Self {
        SimpleDRV8825Enable { controller }
    }
}

impl DRV8825Enable for SimpleDRV8825Enable {
    fn set_enable(&mut self, _drv_id: DRV8825ID, _enable: bool) -> Result<(), DRV8825Error> {
        Ok(())
    }
    
    fn get_enable(&self, drv_id: DRV8825ID) -> Result<bool, DRV8825Error> {
        if self.controller.get_driver(drv_id).is_some() {
            Ok(false)
        } else {
            Err(DRV8825Error::NotFound)
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
