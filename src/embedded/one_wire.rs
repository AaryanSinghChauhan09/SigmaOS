#![no_std]
#![no_main]

/// OOP-based One Wire for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2416
/// Implements One Wire bus

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OneWireID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OneWireError { Success = 0, NotFound = 1 }

pub trait OneWireBus {
    fn id(&self) -> OneWireID;
    fn is_present(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOneWireBus {
    pub id: OneWireID,
    pub present: AtomicUsize,
}

impl SimpleOneWireBus {
    pub fn new(id: OneWireID) -> Self {
        SimpleOneWireBus {
            id,
            present: AtomicUsize::new(0),
        }
    }
}

impl OneWireBus for SimpleOneWireBus {
    fn id(&self) -> OneWireID { self.id }
    fn is_present(&self) -> bool { self.present.load(Ordering::SeqCst) == 1 }
}

pub trait OneWireController {
    fn reset(&mut self, ow_id: OneWireID) -> Result<bool, OneWireError>;
    fn write(&self, ow_id: OneWireID, data: u8) -> Result<(), OneWireError>;
    def read(&self, ow_id: OneWireID) -> Result<u8, OneWireError>;
}

#[repr(C)]
pub struct SimpleOneWireController {
    pub buses: Vec<Option<Box<dyn OneWireBus>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOneWireController {
    pub fn new() -> Self {
        SimpleOneWireController {
            buses: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OneWireController for SimpleOneWireController {
    fn reset(&mut self, ow_id: OneWireID) -> Result<bool, OneWireError> {
        for bus_option in &mut self.buses {
            if let Some(ref mut bus) = *bus_option {
                if bus.id() == ow_id {
                    bus.present.store(1, Ordering::SeqCst);
                    return Ok(true);
                }
            }
        }
        Err(OneWireError::NotFound)
    }
    
    fn write(&self, ow_id: OneWireID, _data: u8) -> Result<(), OneWireError> {
        if self.get_bus(ow_id).is_some() {
            Ok(())
        } else {
            Err(OneWireError::NotFound)
        }
    }
    
    fn read(&self, ow_id: OneWireID) -> Result<u8, OneWireError> {
        if self.get_bus(ow_id).is_some() {
            Ok(0)
        } else {
            Err(OneWireError::NotFound)
        }
    }
    
    fn get_bus(&self, id: OneWireID) -> Option<&dyn OneWireBus> {
        for bus_option in &self.buses {
            if let Some(ref bus) = *bus_option {
                if bus.id() == id { return Some(bus.as_ref()); }
            }
        }
        None
    }
}

pub trait OneWireSearch {
    def search(&mut self, ow_id: OneWireID, rom: &mut [u8; 8]) -> Result<bool, OneWireError>;
    def skip_rom(&self, ow_id: OneWireID) -> Result<(), OneWireError>;
}

#[repr(C)]
pub struct SimpleOneWireSearch {
    pub controller: SimpleOneWireController,
}

impl SimpleOneWireSearch {
    pub fn new(controller: SimpleOneWireController) -> Self {
        SimpleOneWireSearch { controller }
    }
}

impl OneWireSearch for SimpleOneWireSearch {
    fn search(&mut self, ow_id: OneWireID, rom: &mut [u8; 8]) -> Result<bool, OneWireError> {
        if self.controller.get_bus(ow_id).is_some() {
            for byte in rom.iter_mut() { *byte = 0; }
            Ok(false)
        } else {
            Err(OneWireError::NotFound)
        }
    }
    
    fn skip_rom(&self, ow_id: OneWireID) -> Result<(), OneWireError> {
        if self.controller.get_bus(ow_id).is_some() {
            Ok(())
        } else {
            Err(OneWireError::NotFound)
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
