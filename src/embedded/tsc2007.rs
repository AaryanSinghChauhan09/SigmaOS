#![no_std]
#![no_main]

/// OOP-based TSC2007 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2616
/// Implements TSC2007 resistive touch controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TSC2007ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TSC2007Error { Success = 0, NotFound = 1 }

pub trait TSC2007Touch {
    fn id(&self) -> TSC2007ID;
    fn is_touched(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTSC2007Touch {
    pub id: TSC2007ID,
    pub touched: AtomicUsize,
}

impl SimpleTSC2007Touch {
    pub fn new(id: TSC2007ID) -> Self {
        SimpleTSC2007Touch {
            id,
            touched: AtomicUsize::new(0),
        }
    }
}

impl TSC2007Touch for SimpleTSC2007Touch {
    fn id(&self) -> TSC2007ID { self.id }
    fn is_touched(&self) -> bool { self.touched.load(Ordering::SeqCst) == 1 }
}

pub trait TSC2007Controller {
    fn init(&mut self, tsc_id: TSC2007ID) -> Result<(), TSC2007Error>;
    fn read(&self, tsc_id: TSC2007ID) -> Result<(u16, u16), TSC2007Error>;
    def read_pressure(&self, tsc_id: TSC2007ID) -> Result<u16, TSC2007Error>;
}

#[repr(C)]
pub struct SimpleTSC2007Controller {
    pub touches: Vec<Option<Box<dyn TSC2007Touch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTSC2007Controller {
    pub fn new() -> Self {
        SimpleTSC2007Controller {
            touches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TSC2007Controller for SimpleTSC2007Controller {
    fn init(&mut self, tsc_id: TSC2007ID) -> Result<(), TSC2007Error> {
        for touch_option in &mut self.touches {
            if let Some(ref mut touch) = *touch_option {
                if touch.id() == tsc_id {
                    touch.touched.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TSC2007Error::NotFound)
    }
    
    fn read(&self, tsc_id: TSC2007ID) -> Result<(u16, u16), TSC2007Error> {
        if self.get_touch(tsc_id).is_some() {
            Ok((0, 0))
        } else {
            Err(TSC2007Error::NotFound)
        }
    }
    
    fn read_pressure(&self, tsc_id: TSC2007ID) -> Result<u16, TSC2007Error> {
        if self.get_touch(tsc_id).is_some() {
            Ok(0)
        } else {
            Err(TSC2007Error::NotFound)
        }
    }
    
    fn get_touch(&self, id: TSC2007ID) -> Option<&dyn TSC2007Touch> {
        for touch_option in &self.touches {
            if let Some(ref touch) = *touch_option {
                if touch.id() == id { return Some(touch.as_ref()); }
            }
        }
        None
    }
}

pub trait TSC2007Temp {
    def read_temp(&self, tsc_id: TSC2007ID) -> Result<i16, TSC2007Error>;
    def read_vbat(&self, tsc_id: TSC2007ID) -> Result<u16, TSC2007Error>;
}

#[repr(C)]
pub struct SimpleTSC2007Temp {
    pub controller: SimpleTSC2007Controller,
}

impl SimpleTSC2007Temp {
    pub fn new(controller: SimpleTSC2007Controller) -> Self {
        SimpleTSC2007Temp { controller }
    }
}

impl TSC2007Temp for SimpleTSC2007Temp {
    fn read_temp(&self, tsc_id: TSC2007ID) -> Result<i16, TSC2007Error> {
        if self.controller.get_touch(tsc_id).is_some() {
            Ok(0)
        } else {
            Err(TSC2007Error::NotFound)
        }
    }
    
    fn read_vbat(&self, tsc_id: TSC2007ID) -> Result<u16, TSC2007Error> {
        if self.controller.get_touch(tsc_id).is_some() {
            Ok(0)
        } else {
            Err(TSC2007Error::NotFound)
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
