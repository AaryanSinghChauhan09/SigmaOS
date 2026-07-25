#![no_std]
#![no_main]

/// OOP-based 24C64 EEPROM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3716
/// Implements 24C64 I2C EEPROM

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EEPROM24C64ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EEPROM24C64Error { Success = 0, NotFound = 1 }

pub trait EEPROM24C64Device {
    fn id(&self) -> EEPROM24C64ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEEPROM24C64Device {
    pub id: EEPROM24C64ID,
    pub initialized: AtomicUsize,
}

impl SimpleEEPROM24C64Device {
    pub fn new(id: EEPROM24C64ID) -> Self {
        SimpleEEPROM24C64Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl EEPROM24C64Device for SimpleEEPROM24C64Device {
    fn id(&self) -> EEPROM24C64ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait EEPROM24C64Controller {
    fn init(&mut self, eeprom_id: EEPROM24C64ID) -> Result<(), EEPROM24C64Error>;
    fn read(&self, eeprom_id: EEPROM24C64ID, addr: u16) -> Result<u8, EEPROM24C64Error>;
    def write(&self, eeprom_id: EEPROM24C64ID, addr: u16, data: u8) -> Result<(), EEPROM24C64Error>;
}

#[repr(C)]
pub struct SimpleEEPROM24C64Controller {
    pub eeproms: Vec<Option<Box<dyn EEPROM24C64Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEEPROM24C64Controller {
    pub fn new() -> Self {
        SimpleEEPROM24C64Controller {
            eeproms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EEPROM24C64Controller for SimpleEEPROM24C64Controller {
    fn init(&mut self, eeprom_id: EEPROM24C64ID) -> Result<(), EEPROM24C64Error> {
        for eeprom_option in &mut self.eeproms {
            if let Some(ref mut eeprom) = *eeprom_option {
                if eeprom.id() == eeprom_id {
                    eeprom.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EEPROM24C64Error::NotFound)
    }
    
    fn read(&self, eeprom_id: EEPROM24C64ID, _addr: u16) -> Result<u8, EEPROM24C64Error> {
        if self.get_eeprom(eeprom_id).is_some() {
            Ok(0)
        } else {
            Err(EEPROM24C64Error::NotFound)
        }
    }
    
    fn write(&self, eeprom_id: EEPROM24C64ID, _addr: u16, _data: u8) -> Result<(), EEPROM24C64Error> {
        if self.get_eeprom(eeprom_id).is_some() {
            Ok(())
        } else {
            Err(EEPROM24C64Error::NotFound)
        }
    }
    
    fn get_eeprom(&self, id: EEPROM24C64ID) -> Option<&dyn EEPROM24C64Device> {
        for eeprom_option in &self.eeproms {
            if let Some(ref eeprom) = *eeprom_option {
                if eeprom.id() == id { return Some(eeprom.as_ref()); }
            }
        }
        None
    }
}

pub trait EEPROM24C64Block {
    def read_block(&self, eeprom_id: EEPROM24C64ID, addr: u16, buffer: &mut [u8]) -> Result<(), EEPROM24C64Error>;
}

#[repr(C)]
pub struct SimpleEEPROM24C64Block {
    pub controller: SimpleEEPROM24C64Controller,
}

impl SimpleEEPROM24C64Block {
    pub fn new(controller: SimpleEEPROM24C64Controller) -> Self {
        SimpleEEPROM24C64Block { controller }
    }
}

impl EEPROM24C64Block for SimpleEEPROM24C64Block {
    fn read_block(&self, eeprom_id: EEPROM24C64ID, _addr: u16, buffer: &mut [u8]) -> Result<(), EEPROM24C64Error> {
        if self.controller.get_eeprom(eeprom_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(EEPROM24C64Error::NotFound)
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
