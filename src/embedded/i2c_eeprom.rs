#![no_std]
#![no_main]

/// OOP-based I2C EEPROM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2366
/// Implements I2C EEPROM

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EEPROMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EEPROMError { Success = 0, NotFound = 1 }

pub trait I2CEEPROM {
    fn id(&self) -> EEPROMID;
    fn size(&self) -> u16;
}

#[repr(C)]
pub struct SimpleI2CEEPROM {
    pub id: EEPROMID,
    pub size: AtomicUsize,
}

impl SimpleI2CEEPROM {
    pub fn new(id: EEPROMID, size: u16) -> Self {
        SimpleI2CEEPROM {
            id,
            size: AtomicUsize::new(size as usize),
        }
    }
}

impl I2CEEPROM for SimpleI2CEEPROM {
    fn id(&self) -> EEPROMID { self.id }
    fn size(&self) -> u16 { self.size.load(Ordering::SeqCst) as u16 }
}

pub trait EEPROMController {
    fn read(&self, eeprom_id: EEPROMID, address: u16, buffer: &mut [u8]) -> Result<(), EEPROMError>;
    fn write(&self, eeprom_id: EEPROMID, address: u16, data: &[u8]) -> Result<(), EEPROMError>;
    def erase(&mut self, eeprom_id: EEPROMID, address: u16, len: u16) -> Result<(), EEPROMError>;
}

#[repr(C)]
pub struct SimpleEEPROMController {
    pub eeproms: Vec<Option<Box<dyn I2CEEPROM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEEPROMController {
    pub fn new() -> Self {
        SimpleEEPROMController {
            eeproms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EEPROMController for SimpleEEPROMController {
    fn read(&self, eeprom_id: EEPROMID, _address: u16, buffer: &mut [u8]) -> Result<(), EEPROMError> {
        if self.get_eeprom(eeprom_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(EEPROMError::NotFound)
        }
    }
    
    fn write(&self, eeprom_id: EEPROMID, _address: u16, _data: &[u8]) -> Result<(), EEPROMError> {
        if self.get_eeprom(eeprom_id).is_some() {
            Ok(())
        } else {
            Err(EEPROMError::NotFound)
        }
    }
    
    fn erase(&mut self, _eeprom_id: EEPROMID, _address: u16, _len: u16) -> Result<(), EEPROMError> {
        Ok(())
    }
    
    fn get_eeprom(&self, id: EEPROMID) -> Option<&dyn I2CEEPROM> {
        for eeprom_option in &self.eeproms {
            if let Some(ref eeprom) = *eeprom_option {
                if eeprom.id() == id { return Some(eeprom.as_ref()); }
            }
        }
        None
    }
}

pub trait EEPROMProtection {
    def set_write_protect(&mut self, eeprom_id: EEPROMID, protect: bool) -> Result<(), EEPROMError>;
    def is_protected(&self, eeprom_id: EEPROMID) -> Result<bool, EEPROMError>;
}

#[repr(C)]
pub struct SimpleEEPROMProtection {
    pub controller: SimpleEEPROMController,
    pub protections: Vec<(EEPROMID, AtomicUsize)>,
}

impl SimpleEEPROMProtection {
    pub fn new(controller: SimpleEEPROMController) -> Self {
        SimpleEEPROMProtection {
            controller,
            protections: Vec::new(),
        }
    }
}

impl EEPROMProtection for SimpleEEPROMProtection {
    fn set_write_protect(&mut self, eeprom_id: EEPROMID, protect: bool) -> Result<(), EEPROMError> {
        self.protections.push((eeprom_id, AtomicUsize::new(if protect { 1 } else { 0 })));
        Ok(())
    }
    
    fn is_protected(&self, eeprom_id: EEPROMID) -> Result<bool, EEPROMError> {
        for &(id, ref prot) in &self.protections {
            if id == eeprom_id {
                return Ok(prot.load(Ordering::SeqCst) == 1);
            }
        }
        Err(EEPROMError::NotFound)
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
