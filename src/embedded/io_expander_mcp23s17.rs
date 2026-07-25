#![no_std]
#![no_main]

/// OOP-based MCP23S17 IO Expander for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3636
/// Implements MCP23S17 SPI I/O expander

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MCP23S17ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MCP23S17Error { Success = 0, NotFound = 1 }

pub trait MCP23S17Device {
    fn id(&self) -> MCP23S17ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMCP23S17Device {
    pub id: MCP23S17ID,
    pub initialized: AtomicUsize,
}

impl SimpleMCP23S17Device {
    pub fn new(id: MCP23S17ID) -> Self {
        SimpleMCP23S17Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MCP23S17Device for SimpleMCP23S17Device {
    fn id(&self) -> MCP23S17ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MCP23S17Controller {
    fn init(&mut self, exp_id: MCP23S17ID) -> Result<(), MCP23S17Error>;
    fn write_port(&self, exp_id: MCP23S17ID, port: u8, data: u8) -> Result<(), MCP23S17Error>;
    def read_port(&self, exp_id: MCP23S17ID, port: u8) -> Result<u8, MCP23S17Error>;
}

#[repr(C)]
pub struct SimpleMCP23S17Controller {
    pub expanders: Vec<Option<Box<dyn MCP23S17Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMCP23S17Controller {
    pub fn new() -> Self {
        SimpleMCP23S17Controller {
            expanders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MCP23S17Controller for SimpleMCP23S17Controller {
    fn init(&mut self, exp_id: MCP23S17ID) -> Result<(), MCP23S17Error> {
        for exp_option in &mut self.expanders {
            if let Some(ref mut exp) = *exp_option {
                if exp.id() == exp_id {
                    exp.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MCP23S17Error::NotFound)
    }
    
    fn write_port(&self, exp_id: MCP23S17ID, _port: u8, _data: u8) -> Result<(), MCP23S17Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(())
        } else {
            Err(MCP23S17Error::NotFound)
        }
    }
    
    fn read_port(&self, exp_id: MCP23S17ID, _port: u8) -> Result<u8, MCP23S17Error> {
        if self.get_expander(exp_id).is_some() {
            Ok(0)
        } else {
            Err(MCP23S17Error::NotFound)
        }
    }
    
    fn get_expander(&self, id: MCP23S17ID) -> Option<&dyn MCP23S17Device> {
        for exp_option in &self.expanders {
            if let Some(ref exp) = *exp_option {
                if exp.id() == id { return Some(exp.as_ref()); }
            }
        }
        None
    }
}

pub trait MCP23S17Interrupt {
    def enable_interrupt(&mut self, exp_id: MCP23S17ID, pin: u8) -> Result<(), MCP23S17Error>;
}

#[repr(C)]
pub struct SimpleMCP23S17Interrupt {
    pub controller: SimpleMCP23S17Controller,
    pub interrupts: Vec<(MCP23S17ID, AtomicUsize)>,
}

impl SimpleMCP23S17Interrupt {
    pub fn new(controller: SimpleMCP23S17Controller) -> Self {
        SimpleMCP23S17Interrupt {
            controller,
            interrupts: Vec::new(),
        }
    }
}

impl MCP23S17Interrupt for SimpleMCP23S17Interrupt {
    fn enable_interrupt(&mut self, exp_id: MCP23S17ID, _pin: u8) -> Result<(), MCP23S17Error> {
        self.interrupts.push((exp_id, AtomicUsize::new(1)));
        Ok(())
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
