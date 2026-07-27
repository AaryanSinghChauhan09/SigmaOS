#![no_std]
#![no_main]

/// OOP-based Bluetooth LE for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1976
/// Implements Bluetooth LE module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BLEID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BLEError { Success = 0, NotFound = 1 }

pub trait BLEModule {
    fn id(&self) -> BLEID;
    fn is_advertising(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBLEModule {
    pub id: BLEID,
    pub advertising: AtomicUsize,
}

impl SimpleBLEModule {
    pub fn new(id: BLEID) -> Self {
        SimpleBLEModule {
            id,
            advertising: AtomicUsize::new(0),
        }
    }
}

impl BLEModule for SimpleBLEModule {
    fn id(&self) -> BLEID { self.id }
    fn is_advertising(&self) -> bool { self.advertising.load(Ordering::SeqCst) == 1 }
}

pub trait BLEController {
    fn start_advertising(&mut self, ble_id: BLEID) -> Result<(), BLEError>;
    fn stop_advertising(&mut self, ble_id: BLEID) -> Result<(), BLEError>;
    def scan(&mut self, ble_id: BLEID) -> Result<(), BLEError>;
}

#[repr(C)]
pub struct SimpleBLEController {
    pub modules: Vec<Option<Box<dyn BLEModule>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBLEController {
    pub fn new() -> Self {
        SimpleBLEController {
            modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BLEController for SimpleBLEController {
    fn start_advertising(&mut self, ble_id: BLEID) -> Result<(), BLEError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == ble_id {
                    module.advertising.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BLEError::NotFound)
    }
    
    fn stop_advertising(&mut self, ble_id: BLEID) -> Result<(), BLEError> {
        for module_option in &mut self.modules {
            if let Some(ref mut module) = *module_option {
                if module.id() == ble_id {
                    module.advertising.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BLEError::NotFound)
    }
    
    fn scan(&mut self, _ble_id: BLEID) -> Result<(), BLEError> {
        Ok(())
    }
}

pub trait GATT {
    def add_service(&mut self, ble_id: BLEID, uuid: &[u8]) -> Result<(), BLEError>;
    def add_characteristic(&mut self, ble_id: BLEID, uuid: &[u8]) -> Result<(), BLEError>;
}

#[repr(C)]
pub struct SimpleGATT {
    pub controller: SimpleBLEController,
}

impl SimpleGATT {
    pub fn new(controller: SimpleBLEController) -> Self {
        SimpleGATT { controller }
    }
}

impl GATT for SimpleGATT {
    fn add_service(&mut self, _ble_id: BLEID, _uuid: &[u8]) -> Result<(), BLEError> {
        Ok(())
    }
    
    fn add_characteristic(&mut self, _ble_id: BLEID, _uuid: &[u8]) -> Result<(), BLEError> {
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
