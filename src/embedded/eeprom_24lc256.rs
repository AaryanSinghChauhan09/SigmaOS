#![no_std]
#![no_main]

/// OOP-based 24LC256 EEPROM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2946
/// Implements 24LC256 I2C EEPROM

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EEPROM24LC256ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EEPROM24LC256Error { Success = 0, NotFound = 1 }

pub trait EEPROM24LC256 {
    fn id(&self) -> EEPROM24LC256ID;
    fn size(&self) -> u16;
}

#[repr(C)]
pub struct SimpleEEPROM24LC256 {
    pub id: EEPROM24LC256ID,
    pub size: AtomicUsize,
}

impl SimpleEEPROM24LC256 {
    pub fn new(id: EEPROM24LC256ID, size: u16) -> Self {
        SimpleEEPROM24LC256 {
            id,
            size: AtomicUsize::new(size as usize),
        }
    }
}

impl EEPROM24LC256 for SimpleEEPROM24LC256 {
    fn id(&self) -> EEPROM24LC256ID { self.id }
    fn size(&self) -> u16 { self.size.load(Ordering::SeqCst) as u16 }
}

pub trait EEPROM24LC256Controller {
    fn read(&self, eeprom_id: EEPROM24LC256ID, address: u16) -> Result<u8, EEPROM24LC256Error>;
    fn write(&self, eeprom_id: EEPROM24LC256ID, address: u16, data: u8) -> Result<(), EEPROM24LC256Error>;
    def read_page(&self, eeprom_id: EEPROM24LC256ID, address: u16, buffer: &mut [u8]) -> Result<(), EEPROM24LC256Error>;
}

#[repr(C)]
pub struct SimpleEEPROM24LC256Controller {
    pub eeproms: Vec<Option<Box<dyn EEPROM24LC256>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEEPROM24LC256Controller {
    pub fn new() -> Self {
        SimpleEEPROM24LC256Controller {
            eeproms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EEPROM24LC256Controller for SimpleEEPROM24LC256Controller {
    fn read(&self, eeprom_id: EEPROM24LC256ID, _address: u16) -> Result<u8, EEPROM24LC256Error> {
        if self.get_eeprom(eeprom_id).is_some() {
            Ok(0)
        } else {
            Err(EEPROM24LC256Error::NotFound)
        }
    }
    
    fn write(&self, eeprom_id: EEPROM24LC256ID, _address: u16, _data: u8) -> Result<(), EEPROM24LC256Error> {
        if self.get_eeprom(eeprom_id).is_some() {
            Ok(())
        } else {
            Err(EEPROM24LC256Error::NotFound)
        }
    }
    
    fn read_page(&self, eeprom_id: EEPROM24LC256ID, _address: u16, buffer: &mut [u8]) -> Result<(), EEPROM24LC256Error> {
        if self.get_eeprom(eeprom_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(EEPROM24LC256Error::NotFound)
        }
    }
    
    fn get_eeprom(&self, id: EEPROM24LC256ID) -> Option<&dyn EEPROM24LC256> {
        for eeprom_option in &self.eeproms {
            if let Some(ref eeprom) = *eeprom_option {
                if eeprom.id() == id { return Some(eeprom.as_ref()); }
            }
        }
        None
    }
}

pub trait EEPROM24LC256WriteProtect {
    def set_write_protect(&mut self, eeprom_id: EEPROM24LC256ID, protect: bool) -> Result<(), EEPROM24LC256Error>;
    def get_write_protect(&self, eeprom_id: EEPROM24LC256ID) -> Result<bool, EEPROM24LC256Error>;
}

#[repr(C)]
pub struct SimpleEEPROM24LC256WriteProtect {
    pub controller: SimpleEEPROM24LC256Controller,
    pub write_protects: Vec<(EEPROM24LC256ID, AtomicUsize)>,
}

impl SimpleEEPROM24LC256WriteProtect {
    pub fn new(controller: SimpleEEPROM24LC256Controller) -> Self {
        SimpleEEPROM24LC256WriteProtect {
            controller,
            write_protects: Vec::new(),
        }
    }
}

impl EEPROM24LC256WriteProtect for SimpleEEPROM24LC256WriteProtect {
    fn set_write_protect(&mut self, eeprom_id: EEPROM24LC256ID, protect: bool) -> Result<(), EEPROM24LC256Error> {
        self.write_protects.push((eeprom_id, AtomicUsize::new(if protect { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_write_protect(&self, eeprom_id: EEPROM24LC256ID) -> Result<bool, EEPROM24LC256Error> {
        for &(id, ref wp) in &self.write_protects {
            if id == eeprom_id {
                return Ok(wp.load(Ordering::SeqCst) == 1);
            }
        }
        Err(EEPROM24LC256Error::NotFound)
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
