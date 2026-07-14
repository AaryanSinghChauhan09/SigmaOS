#![no_std]
#![no_main]

/// OOP-based HX8357 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2566
/// Implements HX8357 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HX8357ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HX8357Error { Success = 0, NotFound = 1 }

pub trait HX8357Display {
    fn id(&self) -> HX8357ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHX8357Display {
    pub id: HX8357ID,
    pub initialized: AtomicUsize,
}

impl SimpleHX8357Display {
    pub fn new(id: HX8357ID) -> Self {
        SimpleHX8357Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HX8357Display for SimpleHX8357Display {
    fn id(&self) -> HX8357ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HX8357Controller {
    fn init(&mut self, hx_id: HX8357ID) -> Result<(), HX8357Error>;
    fn set_window(&self, hx_id: HX8357ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), HX8357Error>;
    def push_colors(&self, hx_id: HX8357ID, colors: &[u16]) -> Result<(), HX8357Error>;
}

#[repr(C)]
pub struct SimpleHX8357Controller {
    pub displays: Vec<Option<Box<dyn HX8357Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHX8357Controller {
    pub fn new() -> Self {
        SimpleHX8357Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HX8357Controller for SimpleHX8357Controller {
    fn init(&mut self, hx_id: HX8357ID) -> Result<(), HX8357Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == hx_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HX8357Error::NotFound)
    }
    
    fn set_window(&self, hx_id: HX8357ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), HX8357Error> {
        if self.get_display(hx_id).is_some() {
            Ok(())
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn push_colors(&self, hx_id: HX8357ID, _colors: &[u16]) -> Result<(), HX8357Error> {
        if self.get_display(hx_id).is_some() {
            Ok(())
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn get_display(&self, id: HX8357ID) -> Option<&dyn HX8357Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait HX8357TE {
    def enable_te(&mut self, hx_id: HX8357ID, enable: bool) -> Result<(), HX8357Error>;
    def set_te_line(&mut self, hx_id: HX8357ID, line: u8) -> Result<(), HX8357Error>;
}

#[repr(C)]
pub struct SimpleHX8357TE {
    pub controller: SimpleHX8357Controller,
}

impl SimpleHX8357TE {
    pub fn new(controller: SimpleHX8357Controller) -> Self {
        SimpleHX8357TE { controller }
    }
}

impl HX8357TE for SimpleHX8357TE {
    fn enable_te(&mut self, _hx_id: HX8357ID, _enable: bool) -> Result<(), HX8357Error> {
        Ok(())
    }
    
    fn set_te_line(&mut self, _hx_id: HX8357ID, _line: u8) -> Result<(), HX8357Error> {
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
