#![no_std]
#![no_main]

/// OOP-based XPT2046 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2626
/// Implements XPT2046 resistive touch controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type XPT2046ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum XPT2046Error { Success = 0, NotFound = 1 }

pub trait XPT2046Touch {
    fn id(&self) -> XPT2046ID;
    fn is_touched(&self) -> bool;
}

#[repr(C)]
pub struct SimpleXPT2046Touch {
    pub id: XPT2046ID,
    pub touched: AtomicUsize,
}

impl SimpleXPT2046Touch {
    pub fn new(id: XPT2046ID) -> Self {
        SimpleXPT2046Touch {
            id,
            touched: AtomicUsize::new(0),
        }
    }
}

impl XPT2046Touch for SimpleXPT2046Touch {
    fn id(&self) -> XPT2046ID { self.id }
    fn is_touched(&self) -> bool { self.touched.load(Ordering::SeqCst) == 1 }
}

pub trait XPT2046Controller {
    fn init(&mut self, xpt_id: XPT2046ID) -> Result<(), XPT2046Error>;
    fn read(&self, xpt_id: XPT2046ID) -> Result<(u16, u16), XPT2046Error>;
    def read_pressure(&self, xpt_id: XPT2046ID) -> Result<u16, XPT2046Error>;
}

#[repr(C)]
pub struct SimpleXPT2046Controller {
    pub touches: Vec<Option<Box<dyn XPT2046Touch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleXPT2046Controller {
    pub fn new() -> Self {
        SimpleXPT2046Controller {
            touches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl XPT2046Controller for SimpleXPT2046Controller {
    fn init(&mut self, xpt_id: XPT2046ID) -> Result<(), XPT2046Error> {
        for touch_option in &mut self.touches {
            if let Some(ref mut touch) = *touch_option {
                if touch.id() == xpt_id {
                    touch.touched.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(XPT2046Error::NotFound)
    }
    
    fn read(&self, xpt_id: XPT2046ID) -> Result<(u16, u16), XPT2046Error> {
        if self.get_touch(xpt_id).is_some() {
            Ok((0, 0))
        } else {
            Err(XPT2046Error::NotFound)
        }
    }
    
    fn read_pressure(&self, xpt_id: XPT2046ID) -> Result<u16, XPT2046Error> {
        if self.get_touch(xpt_id).is_some() {
            Ok(0)
        } else {
            Err(XPT2046Error::NotFound)
        }
    }
    
    fn get_touch(&self, id: XPT2046ID) -> Option<&dyn XPT2046Touch> {
        for touch_option in &self.touches {
            if let Some(ref touch) = *touch_option {
                if touch.id() == id { return Some(touch.as_ref()); }
            }
        }
        None
    }
}

pub trait XPT2046Temp {
    def read_temp(&self, xpt_id: XPT2046ID) -> Result<i16, XPT2046Error>;
    def read_vbat(&self, xpt_id: XPT2046ID) -> Result<u16, XPT2046Error>;
}

#[repr(C)]
pub struct SimpleXPT2046Temp {
    pub controller: SimpleXPT2046Controller,
}

impl SimpleXPT2046Temp {
    pub fn new(controller: SimpleXPT2046Controller) -> Self {
        SimpleXPT2046Temp { controller }
    }
}

impl XPT2046Temp for SimpleXPT2046Temp {
    fn read_temp(&self, xpt_id: XPT2046ID) -> Result<i16, XPT2046Error> {
        if self.controller.get_touch(xpt_id).is_some() {
            Ok(0)
        } else {
            Err(XPT2046Error::NotFound)
        }
    }
    
    fn read_vbat(&self, xpt_id: XPT2046ID) -> Result<u16, XPT2046Error> {
        if self.controller.get_touch(xpt_id).is_some() {
            Ok(0)
        } else {
            Err(XPT2046Error::NotFound)
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
