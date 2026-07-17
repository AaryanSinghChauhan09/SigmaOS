#![no_std]
#![no_main]

/// OOP-based RM68120 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2486
/// Implements RM68120 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RM68120ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RM68120Error { Success = 0, NotFound = 1 }

pub trait RM68120Display {
    fn id(&self) -> RM68120ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRM68120Display {
    pub id: RM68120ID,
    pub initialized: AtomicUsize,
}

impl SimpleRM68120Display {
    pub fn new(id: RM68120ID) -> Self {
        SimpleRM68120Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl RM68120Display for SimpleRM68120Display {
    fn id(&self) -> RM68120ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait RM68120Controller {
    fn init(&mut self, rm_id: RM68120ID) -> Result<(), RM68120Error>;
    fn set_window(&self, rm_id: RM68120ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), RM68120Error>;
    def push_colors(&self, rm_id: RM68120ID, colors: &[u16]) -> Result<(), RM68120Error>;
}

#[repr(C)]
pub struct SimpleRM68120Controller {
    pub displays: Vec<Option<Box<dyn RM68120Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRM68120Controller {
    pub fn new() -> Self {
        SimpleRM68120Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RM68120Controller for SimpleRM68120Controller {
    fn init(&mut self, rm_id: RM68120ID) -> Result<(), RM68120Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == rm_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RM68120Error::NotFound)
    }
    
    fn set_window(&self, rm_id: RM68120ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), RM68120Error> {
        if self.get_display(rm_id).is_some() {
            Ok(())
        } else {
            Err(RM68120Error::NotFound)
        }
    }
    
    fn push_colors(&self, rm_id: RM68120ID, _colors: &[u16]) -> Result<(), RM68120Error> {
        if self.get_display(rm_id).is_some() {
            Ok(())
        } else {
            Err(RM68120Error::NotFound)
        }
    }
    
    fn get_display(&self, id: RM68120ID) -> Option<&dyn RM68120Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait RM68120LVGL {
    def set_lvgl_mode(&mut self, rm_id: RM68120ID, mode: u8) -> Result<(), RM68120Error>;
    def flush(&self, rm_id: RM68120ID) -> Result<(), RM68120Error>;
}

#[repr(C)]
pub struct SimpleRM68120LVGL {
    pub controller: SimpleRM68120Controller,
}

impl SimpleRM68120LVGL {
    pub fn new(controller: SimpleRM68120Controller) -> Self {
        SimpleRM68120LVGL { controller }
    }
}

impl RM68120LVGL for SimpleRM68120LVGL {
    fn set_lvgl_mode(&mut self, _rm_id: RM68120ID, _mode: u8) -> Result<(), RM68120Error> {
        Ok(())
    }
    
    fn flush(&self, rm_id: RM68120ID) -> Result<(), RM68120Error> {
        if self.controller.get_display(rm_id).is_some() {
            Ok(())
        } else {
            Err(RM68120Error::NotFound)
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
