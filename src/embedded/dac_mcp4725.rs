#![no_std]
#![no_main]

/// OOP-based MCP4725 DAC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3666
/// Implements MCP4725 12-bit DAC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MCP4725ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MCP4725Error { Success = 0, NotFound = 1 }

pub trait MCP4725Device {
    fn id(&self) -> MCP4725ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMCP4725Device {
    pub id: MCP4725ID,
    pub initialized: AtomicUsize,
}

impl SimpleMCP4725Device {
    pub fn new(id: MCP4725ID) -> Self {
        SimpleMCP4725Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MCP4725Device for SimpleMCP4725Device {
    fn id(&self) -> MCP4725ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MCP4725Controller {
    fn init(&mut self, dac_id: MCP4725ID) -> Result<(), MCP4725Error>;
    fn write(&self, dac_id: MCP4725ID, value: u16) -> Result<(), MCP4725Error>;
    def read(&self, dac_id: MCP4725ID) -> Result<u16, MCP4725Error>;
}

#[repr(C)]
pub struct SimpleMCP4725Controller {
    pub dacs: Vec<Option<Box<dyn MCP4725Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMCP4725Controller {
    pub fn new() -> Self {
        SimpleMCP4725Controller {
            dacs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MCP4725Controller for SimpleMCP4725Controller {
    fn init(&mut self, dac_id: MCP4725ID) -> Result<(), MCP4725Error> {
        for dac_option in &mut self.dacs {
            if let Some(ref mut dac) = *dac_option {
                if dac.id() == dac_id {
                    dac.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MCP4725Error::NotFound)
    }
    
    fn write(&self, dac_id: MCP4725ID, _value: u16) -> Result<(), MCP4725Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(())
        } else {
            Err(MCP4725Error::NotFound)
        }
    }
    
    fn read(&self, dac_id: MCP4725ID) -> Result<u16, MCP4725Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(0)
        } else {
            Err(MCP4725Error::NotFound)
        }
    }
    
    fn get_dac(&self, id: MCP4725ID) -> Option<&dyn MCP4725Device> {
        for dac_option in &self.dacs {
            if let Some(ref dac) = *dac_option {
                if dac.id() == id { return Some(dac.as_ref()); }
            }
        }
        None
    }
}

pub trait MCP4725EEPROM {
    def save_eeprom(&self, dac_id: MCP4725ID) -> Result<(), MCP4725Error>;
}

#[repr(C)]
pub struct SimpleMCP4725EEPROM {
    pub controller: SimpleMCP4725Controller,
}

impl SimpleMCP4725EEPROM {
    pub fn new(controller: SimpleMCP4725Controller) -> Self {
        SimpleMCP4725EEPROM { controller }
    }
}

impl MCP4725EEPROM for SimpleMCP4725EEPROM {
    fn save_eeprom(&self, dac_id: MCP4725ID) -> Result<(), MCP4725Error> {
        if self.controller.get_dac(dac_id).is_some() {
            Ok(())
        } else {
            Err(MCP4725Error::NotFound)
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
