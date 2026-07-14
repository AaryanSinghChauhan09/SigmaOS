#![no_std]
#![no_main]

/// OOP-based MCP4922 DAC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3676
/// Implements MCP4922 dual 12-bit DAC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MCP4922ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MCP4922Error { Success = 0, NotFound = 1 }

pub trait MCP4922Device {
    fn id(&self) -> MCP4922ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMCP4922Device {
    pub id: MCP4922ID,
    pub initialized: AtomicUsize,
}

impl SimpleMCP4922Device {
    pub fn new(id: MCP4922ID) -> Self {
        SimpleMCP4922Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MCP4922Device for SimpleMCP4922Device {
    fn id(&self) -> MCP4922ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MCP4922Controller {
    fn init(&mut self, dac_id: MCP4922ID) -> Result<(), MCP4922Error>;
    fn write(&self, dac_id: MCP4922ID, channel: u8, value: u16) -> Result<(), MCP4922Error>;
    def read(&self, dac_id: MCP4922ID, channel: u8) -> Result<u16, MCP4922Error>;
}

#[repr(C)]
pub struct SimpleMCP4922Controller {
    pub dacs: Vec<Option<Box<dyn MCP4922Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMCP4922Controller {
    pub fn new() -> Self {
        SimpleMCP4922Controller {
            dacs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MCP4922Controller for SimpleMCP4922Controller {
    fn init(&mut self, dac_id: MCP4922ID) -> Result<(), MCP4922Error> {
        for dac_option in &mut self.dacs {
            if let Some(ref mut dac) = *dac_option {
                if dac.id() == dac_id {
                    dac.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MCP4922Error::NotFound)
    }
    
    fn write(&self, dac_id: MCP4922ID, _channel: u8, _value: u16) -> Result<(), MCP4922Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(())
        } else {
            Err(MCP4922Error::NotFound)
        }
    }
    
    fn read(&self, dac_id: MCP4922ID, _channel: u8) -> Result<u16, MCP4922Error> {
        if self.get_dac(dac_id).is_some() {
            Ok(0)
        } else {
            Err(MCP4922Error::NotFound)
        }
    }
    
    fn get_dac(&self, id: MCP4922ID) -> Option<&dyn MCP4922Device> {
        for dac_option in &self.dacs {
            if let Some(ref dac) = *dac_option {
                if dac.id() == id { return Some(dac.as_ref()); }
            }
        }
        None
    }
}

pub trait MCP4922Buffer {
    def set_buffered(&mut self, dac_id: MCP4922ID, buffered: bool) -> Result<(), MCP4922Error>;
}

#[repr(C)]
pub struct SimpleMCP4922Buffer {
    pub controller: SimpleMCP4922Controller,
    pub buffered: Vec<(MCP4922ID, AtomicUsize)>,
}

impl SimpleMCP4922Buffer {
    pub fn new(controller: SimpleMCP4922Controller) -> Self {
        SimpleMCP4922Buffer {
            controller,
            buffered: Vec::new(),
        }
    }
}

impl MCP4922Buffer for SimpleMCP4922Buffer {
    fn set_buffered(&mut self, dac_id: MCP4922ID, buffered: bool) -> Result<(), MCP4922Error> {
        self.buffered.push((dac_id, AtomicUsize::new(if buffered { 1 } else { 0 })));
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
