#![no_std]
#![no_main]

/// OOP-based AT25 EEPROM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2956
/// Implements AT25 SPI EEPROM

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AT25ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AT25Error { Success = 0, NotFound = 1 }

pub trait AT25EEPROM {
    fn id(&self) -> AT25ID;
    fn size(&self) -> u16;
}

#[repr(C)]
pub struct SimpleAT25EEPROM {
    pub id: AT25ID,
    pub size: AtomicUsize,
}

impl SimpleAT25EEPROM {
    pub fn new(id: AT25ID, size: u16) -> Self {
        SimpleAT25EEPROM {
            id,
            size: AtomicUsize::new(size as usize),
        }
    }
}

impl AT25EEPROM for SimpleAT25EEPROM {
    fn id(&self) -> AT25ID { self.id }
    fn size(&self) -> u16 { self.size.load(Ordering::SeqCst) as u16 }
}

pub trait AT25Controller {
    fn read(&self, at_id: AT25ID, address: u16) -> Result<u8, AT25Error>;
    fn write(&self, at_id: AT25ID, address: u16, data: u8) -> Result<(), AT25Error>;
    def read_page(&self, at_id: AT25ID, address: u16, buffer: &mut [u8]) -> Result<(), AT25Error>;
}

#[repr(C)]
pub struct SimpleAT25Controller {
    pub eeproms: Vec<Option<Box<dyn AT25EEPROM>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAT25Controller {
    pub fn new() -> Self {
        SimpleAT25Controller {
            eeproms: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AT25Controller for SimpleAT25Controller {
    fn read(&self, at_id: AT25ID, _address: u16) -> Result<u8, AT25Error> {
        if self.get_eeprom(at_id).is_some() {
            Ok(0)
        } else {
            Err(AT25Error::NotFound)
        }
    }
    
    fn write(&self, at_id: AT25ID, _address: u16, _data: u8) -> Result<(), AT25Error> {
        if self.get_eeprom(at_id).is_some() {
            Ok(())
        } else {
            Err(AT25Error::NotFound)
        }
    }
    
    fn read_page(&self, at_id: AT25ID, _address: u16, buffer: &mut [u8]) -> Result<(), AT25Error> {
        if self.get_eeprom(at_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(AT25Error::NotFound)
        }
    }
    
    fn get_eeprom(&self, id: AT25ID) -> Option<&dyn AT25EEPROM> {
        for eeprom_option in &self.eeproms {
            if let Some(ref eeprom) = *eeprom_option {
                if eeprom.id() == id { return Some(eeprom.as_ref()); }
            }
        }
        None
    }
}

pub trait AT25Status {
    def read_status(&self, at_id: AT25ID) -> Result<u8, AT25Error>;
    def write_status(&self, at_id: AT25ID, status: u8) -> Result<(), AT25Error>;
}

#[repr(C)]
pub struct SimpleAT25Status {
    pub controller: SimpleAT25Controller,
}

impl SimpleAT25Status {
    pub fn new(controller: SimpleAT25Controller) -> Self {
        SimpleAT25Status { controller }
    }
}

impl AT25Status for SimpleAT25Status {
    fn read_status(&self, at_id: AT25ID) -> Result<u8, AT25Error> {
        if self.controller.get_eeprom(at_id).is_some() {
            Ok(0)
        } else {
            Err(AT25Error::NotFound)
        }
    }
    
    fn write_status(&self, at_id: AT25ID, _status: u8) -> Result<(), AT25Error> {
        if self.controller.get_eeprom(at_id).is_some() {
            Ok(())
        } else {
            Err(AT25Error::NotFound)
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
