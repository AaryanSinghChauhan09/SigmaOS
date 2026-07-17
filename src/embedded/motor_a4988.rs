#![no_std]
#![no_main]

/// OOP-based A4988 Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2776
/// Implements A4988 stepper motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type A4988ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum A4988Error { Success = 0, NotFound = 1 }

pub trait A4988Driver {
    fn id(&self) -> A4988ID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleA4988Driver {
    pub id: A4988ID,
    pub enabled: AtomicUsize,
}

impl SimpleA4988Driver {
    pub fn new(id: A4988ID) -> Self {
        SimpleA4988Driver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl A4988Driver for SimpleA4988Driver {
    fn id(&self) -> A4988ID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait A4988Controller {
    fn step(&self, a4988_id: A4988ID, steps: i32) -> Result<(), A4988Error>;
    def set_microstep(&mut self, a4988_id: A4988ID, mode: u8) -> Result<(), A4988Error>;
    def set_direction(&mut self, a4988_id: A4988ID, forward: bool) -> Result<(), A4988Error>;
}

#[repr(C)]
pub struct SimpleA4988Controller {
    pub drivers: Vec<Option<Box<dyn A4988Driver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleA4988Controller {
    pub fn new() -> Self {
        SimpleA4988Controller {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl A4988Controller for SimpleA4988Controller {
    fn step(&self, a4988_id: A4988ID, steps: i32) -> Result<(), A4988Error> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == a4988_id {
                    if steps != 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(A4988Error::NotFound)
    }
    
    fn set_microstep(&mut self, _a4988_id: A4988ID, _mode: u8) -> Result<(), A4988Error> {
        Ok(())
    }
    
    fn set_direction(&mut self, _a4988_id: A4988ID, _forward: bool) -> Result<(), A4988Error> {
        Ok(())
    }
    
    fn get_driver(&self, id: A4988ID) -> Option<&dyn A4988Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait A4988Enable {
    def set_enable(&mut self, a4988_id: A4988ID, enable: bool) -> Result<(), A4988Error>;
    def get_enable(&self, a4988_id: A4988ID) -> Result<bool, A4988Error>;
}

#[repr(C)]
pub struct SimpleA4988Enable {
    pub controller: SimpleA4988Controller,
}

impl SimpleA4988Enable {
    pub fn new(controller: SimpleA4988Controller) -> Self {
        SimpleA4988Enable { controller }
    }
}

impl A4988Enable for SimpleA4988Enable {
    fn set_enable(&mut self, _a4988_id: A4988ID, _enable: bool) -> Result<(), A4988Error> {
        Ok(())
    }
    
    fn get_enable(&self, a4988_id: A4988ID) -> Result<bool, A4988Error> {
        if self.controller.get_driver(a4988_id).is_some() {
            Ok(false)
        } else {
            Err(A4988Error::NotFound)
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
