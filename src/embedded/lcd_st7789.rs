#![no_std]
#![no_main]

/// OOP-based ST7789 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2456
/// Implements ST7789 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ST7789ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7789Error { Success = 0, NotFound = 1 }

pub trait ST7789Display {
    fn id(&self) -> ST7789ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7789Display {
    pub id: ST7789ID,
    pub initialized: AtomicUsize,
}

impl SimpleST7789Display {
    pub fn new(id: ST7789ID) -> Self {
        SimpleST7789Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ST7789Display for SimpleST7789Display {
    fn id(&self) -> ST7789ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ST7789Controller {
    fn init(&mut self, st_id: ST7789ID) -> Result<(), ST7789Error>;
    fn set_window(&self, st_id: ST7789ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), ST7789Error>;
    def push_colors(&self, st_id: ST7789ID, colors: &[u16]) -> Result<(), ST7789Error>;
}

#[repr(C)]
pub struct SimpleST7789Controller {
    pub displays: Vec<Option<Box<dyn ST7789Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7789Controller {
    pub fn new() -> Self {
        SimpleST7789Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7789Controller for SimpleST7789Controller {
    fn init(&mut self, st_id: ST7789ID) -> Result<(), ST7789Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == st_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7789Error::NotFound)
    }
    
    fn set_window(&self, st_id: ST7789ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), ST7789Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7789Error::NotFound)
        }
    }
    
    fn push_colors(&self, st_id: ST7789ID, _colors: &[u16]) -> Result<(), ST7789Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7789Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ST7789ID) -> Option<&dyn ST7789Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7789Inversion {
    def set_inversion(&mut self, st_id: ST7789ID, invert: bool) -> Result<(), ST7789Error>;
    def get_inversion(&self, st_id: ST7789ID) -> Result<bool, ST7789Error>;
}

#[repr(C)]
pub struct SimpleST7789Inversion {
    pub controller: SimpleST7789Controller,
    pub inversions: Vec<(ST7789ID, AtomicUsize)>,
}

impl SimpleST7789Inversion {
    pub fn new(controller: SimpleST7789Controller) -> Self {
        SimpleST7789Inversion {
            controller,
            inversions: Vec::new(),
        }
    }
}

impl ST7789Inversion for SimpleST7789Inversion {
    fn set_inversion(&mut self, st_id: ST7789ID, invert: bool) -> Result<(), ST7789Error> {
        self.inversions.push((st_id, AtomicUsize::new(if invert { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_inversion(&self, st_id: ST7789ID) -> Result<bool, ST7789Error> {
        for &(id, ref inv) in &self.inversions {
            if id == st_id {
                return Ok(inv.load(Ordering::SeqCst) == 1);
            }
        }
        Err(ST7789Error::NotFound)
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
