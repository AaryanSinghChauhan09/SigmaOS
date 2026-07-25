#![no_std]
#![no_main]

/// OOP-based MCP23017 IO Expander for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3626
/// Implements MCP23017 I2C I/O expander

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MCP23017ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MCP23017Error { Success = 0, NotFound = 1 }

pub trait MCP23017Device {
    fn id(&self) -> MCP23017ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMCP23017Device {
    pub id: MCP23017ID,
    pub initialized: AtomicUsize,
}

impl SimpleMCP23017Device {
    pub fn new(id: MCP23017ID) -> Self {
        SimpleMCP23017Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MCP23017Device for SimpleMCP23017Device {
    fn id(&self) -> MCP23017ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MCP23017Controller {
    fn init(&mut self, exp_id: MCP23017ID) -> Result<(), MCP23017Error>;
    fn write_port(&self, exp_id: MCP23017ID, port: u8, data: u8) -> Result<(), MCP23017Error>;
    def read_port(&self, exp_id: MCP23017ID, port: u8) -> Result<u8, MCP23017Error>;
}

#[repr(C)]
pub struct SimpleMCP23017Controller {
    pub expanders: Vec<Option<Box<dyn MCP23017Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMCP23017Controller {
    pub fn new() -> Self {
        SimpleMCP23017Controller {
            expanders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MCP23017Controller for SimpleMCP23017Controller {
    fn init(&mut self, exp_id: MCP23017ID) -> Result<(), MCP23017Error> {
        for exp_option in &mut self.expanders {
            if let Some(ref mut exp) = *exp_option {
                if exp.id() == exp_id {
                    exp.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MCP23017Error::NotFound)
    }
    
    fn write_port(&self, exp_id: MCP23017ID, _port: u8, _data: u8) -> Result<(), MCP23017Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(())
        } else {
            Err(MCP23017Error::NotFound)
        }
    }
    
    fn read_port(&self, exp_id: MCP23017ID, _port: u8) -> Result<u8, MCP23017Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(0)
        } else {
            Err(MCP23017Error::NotFound)
        }
    }
    
    fn get_expander(&self, id: MCP23017ID) -> Option<&dyn MCP23017Device> {
        for exp_option in &self.expanders {
            if let Some(ref exp) = *exp_option {
                if exp.id() == id { return Some(exp.as_ref()); }
            }
        }
        None
    }
}

pub trait MCP23017Pin {
    def set_direction(&mut self, exp_id: MCP23017ID, pin: u8, output: bool) -> Result<(), MCP23017Error>;
    def write_pin(&self, exp_id: MCP23017ID, pin: u8, value: bool) -> Result<(), MCP23017Error>;
}

#[repr(C)]
pub struct SimpleMCP23017Pin {
    pub controller: SimpleMCP23017Controller,
    pub directions: Vec<(MCP23017ID, AtomicUsize)>,
}

impl SimpleMCP23017Pin {
    pub fn new(controller: SimpleMCP23017Controller) -> Self {
        SimpleMCP23017Pin {
            controller,
            directions: Vec::new(),
        }
    }
}

impl MCP23017Pin for SimpleMCP23017Pin {
    fn set_direction(&mut self, exp_id: MCP23017ID, output: bool) -> Result<(), MCP23017Error> {
        self.directions.push((exp_id, AtomicUsize::new(if output { 1 } else { 0 })));
        Ok(())
    }
    
    fn write_pin(&self, exp_id: MCP23017ID, _pin: u8, _value: bool) -> Result<(), MCP23017Error> {
        if self.controller.get_expander(exp_id).is_some() {
            Ok(())
        } else {
            Err(MCP23017Error::NotFound)
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
