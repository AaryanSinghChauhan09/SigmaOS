#![no_std]
#![no_main]

/// OOP-based PCF8574 IO Expander for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3616
/// Implements PCF8574 I2C I/O expander

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCF8574ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCF8574Error { Success = 0, NotFound = 1 }

pub trait PCF8574Device {
    fn id(&self) -> PCF8574ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCF8574Device {
    pub id: PCF8574ID,
    pub initialized: AtomicUsize,
}

impl SimplePCF8574Device {
    pub fn new(id: PCF8574ID) -> Self {
        SimplePCF8574Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCF8574Device for SimplePCF8574Device {
    fn id(&self) -> PCF8574ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCF8574Controller {
    fn init(&mut self, exp_id: PCF8574ID) -> Result<(), PCF8574Error>;
    fn write(&self, exp_id: PCF8574ID, data: u8) -> Result<(), PCF8574Error>;
    def read(&self, exp_id: PCF8574ID) -> Result<u8, PCF8574Error>;
}

#[repr(C)]
pub struct SimplePCF8574Controller {
    pub expanders: Vec<Option<Box<dyn PCF8574Device>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCF8574Controller {
    pub fn new() -> Self {
        SimplePCF8574Controller {
            expanders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCF8574Controller for SimplePCF8574Controller {
    fn init(&mut self, exp_id: PCF8574ID) -> Result<(), PCF8574Error> {
        for exp_option in &mut self.expanders {
            if let Some(ref mut exp) = *exp_option {
                if exp.id() == exp_id {
                    exp.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCF8574Error::NotFound)
    }
    
    fn write(&self, exp_id: PCF8574ID, _data: u8) -> Result<(), PCF8574Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(())
        } else {
            Err(PCF8574Error::NotFound)
        }
    }
    
    fn read(&self, exp_id: PCF8574ID) -> Result<u8, PCF8574Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(0)
        } else {
            Err(PCF8574Error::NotFound)
        }
    }
    
    fn get_expander(&self, id: PCF8574ID) -> Option<&dyn PCF8574Device> {
        for exp_option in &self.expanders {
            if let Some(ref exp) = *exp_option {
                if exp.id() == id { return Some(exp.as_ref()); }
            }
        }
        None
    }
}

pub trait PCF8574Pin {
    def write_pin(&self, exp_id: PCF8574ID, pin: u8, value: bool) -> Result<(), PCF8574Error>;
    def read_pin(&self, exp_id: PCF8574ID, pin: u8) -> Result<bool, PCF8574Error>;
}

#[repr(C)]
pub struct SimplePCF8574Pin {
    pub controller: SimplePCF8574Controller,
}

impl SimplePCF8574Pin {
    pub fn new(controller: SimplePCF8574Controller) -> Self {
        SimplePCF8574Pin { controller }
    }
}

impl PCF8574Pin for SimplePCF8574Pin {
    fn write_pin(&self, exp_id: PCF8574ID, _pin: u8, _value: bool) -> Result<(), PCF8574Error> {
        if self.controller.get_expander(exp_id).is_some() {
            Ok(())
        } else {
            Err(PCF8574Error::NotFound)
        }
    }
    
    fn read_pin(&self, exp_id: PCF8574ID, _pin: u8) -> Result<bool, PCF8574Error> {
        if self.controller.get_expander(exp_id).is_some() {
            Ok(false)
        } else {
            Err(PCF8574Error::NotFound)
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
