#![no_std]
#![no_main]

/// OOP-based ST7735 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3256
/// Implements ST7735 TFT display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ST7735ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7735Error { Success = 0, NotFound = 1 }

pub trait ST7735Display {
    fn id(&self) -> ST7735ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7735Display {
    pub id: ST7735ID,
    pub initialized: AtomicUsize,
}

impl SimpleST7735Display {
    pub fn new(id: ST7735ID) -> Self {
        SimpleST7735Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ST7735Display for SimpleST7735Display {
    fn id(&self) -> ST7735ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ST7735Controller {
    fn init(&mut self, display_id: ST7735ID) -> Result<(), ST7735Error>;
    fn clear(&self, display_id: ST7735ID, color: u16) -> Result<(), ST7735Error>;
    def draw_pixel(&self, display_id: ST7735ID, x: u16, y: u16, color: u16) -> Result<(), ST7735Error>;
}

#[repr(C)]
pub struct SimpleST7735Controller {
    pub displays: Vec<Option<Box<dyn ST7735Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7735Controller {
    pub fn new() -> Self {
        SimpleST7735Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7735Controller for SimpleST7735Controller {
    fn init(&mut self, display_id: ST7735ID) -> Result<(), ST7735Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7735Error::NotFound)
    }
    
    fn clear(&self, display_id: ST7735ID, _color: u16) -> Result<(), ST7735Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7735Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: ST7735ID, _x: u16, _y: u16, _color: u16) -> Result<(), ST7735Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7735Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ST7735ID) -> Option<&dyn ST7735Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7735Rect {
    def fill_rect(&self, display_id: ST7735ID, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), ST7735Error>;
}

#[repr(C)]
pub struct SimpleST7735Rect {
    pub controller: SimpleST7735Controller,
}

impl SimpleST7735Rect {
    pub fn new(controller: SimpleST7735Controller) -> Self {
        SimpleST7735Rect { controller }
    }
}

impl ST7735Rect for SimpleST7735Rect {
    fn fill_rect(&self, display_id: ST7735ID, _x: u16, _y: u16, _w: u16, _h: u16, _color: u16) -> Result<(), ST7735Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7735Error::NotFound)
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
