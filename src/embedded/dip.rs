#![no_std]
#![no_main]

/// OOP-based DIP Switch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1446
/// Implements DIP switch

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DIPID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DIPError { Success = 0, NotFound = 1 }

pub trait DIPSwitch {
    fn id(&self) -> DIPID;
    fn read(&self) -> u8;
    fn set_bit(&mut self, bit: u8, value: bool);
}

#[repr(C)]
pub struct SimpleDIPSwitch {
    pub id: DIPID,
    pub value: AtomicUsize,
}

impl SimpleDIPSwitch {
    pub fn new(id: DIPID) -> Self {
        SimpleDIPSwitch {
            id,
            value: AtomicUsize::new(0),
        }
    }
}

impl DIPSwitch for SimpleDIPSwitch {
    fn id(&self) -> DIPID { self.id }
    fn read(&self) -> u8 { self.value.load(Ordering::SeqCst) as u8 }
    
    fn set_bit(&mut self, bit: u8, value: bool) {
        let current = self.value.load(Ordering::SeqCst);
        let new_value = if value {
            current | (1 << bit)
        } else {
            current & !(1 << bit)
        };
        self.value.store(new_value, Ordering::SeqCst);
    }
}

pub trait DIPController {
    fn read_all(&self, dip_id: DIPID) -> Result<u8, DIPError>;
    fn set_all(&mut self, dip_id: DIPID, value: u8) -> Result<(), DIPError>;
}

#[repr(C)]
pub struct SimpleDIPController {
    pub dips: Vec<Option<Box<dyn DIPSwitch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDIPController {
    pub fn new() -> Self {
        SimpleDIPController {
            dips: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DIPController for SimpleDIPController {
    fn read_all(&self, dip_id: DIPID) -> Result<u8, DIPError> {
        for dip_option in &self.dips {
            if let Some(ref dip) = *dip_option {
                if dip.id() == dip_id {
                    return Ok(dip.read());
                }
            }
        }
        Err(DIPError::NotFound)
    }
    
    fn set_all(&mut self, dip_id: DIPID, value: u8) -> Result<(), DIPError> {
        for dip_option in &mut self.dips {
            if let Some(ref mut dip) = *dip_option {
                if dip.id() == dip_id {
                    dip.value.store(value as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DIPError::NotFound)
    }
}

pub trait ConfigStorage {
    def save_config(&mut self, dip_id: DIPID) -> Result<(), DIPError>;
    def load_config(&self, dip_id: DIPID) -> Result<u8, DIPError>;
}

#[repr(C)]
pub struct SimpleConfigStorage {
    pub controller: SimpleDIPController,
    pub stored: Vec<(DIPID, AtomicUsize)>,
}

impl SimpleConfigStorage {
    pub fn new(controller: SimpleDIPController) -> Self {
        SimpleConfigStorage {
            controller,
            stored: Vec::new(),
        }
    }
}

impl ConfigStorage for SimpleConfigStorage {
    fn save_config(&mut self, dip_id: DIPID) -> Result<(), DIPError> {
        let value = self.controller.read_all(dip_id)?;
        self.stored.push((dip_id, AtomicUsize::new(value as usize)));
        Ok(())
    }
    
    fn load_config(&self, dip_id: DIPID) -> Result<u8, DIPError> {
        for &(id, ref value) in &self.stored {
            if id == dip_id {
                return Ok(value.load(Ordering::SeqCst) as u8);
            }
        }
        Err(DIPError::NotFound)
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
