#![no_std]
#![no_main]

/// OOP-based OLED 128x64 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3926
/// Implements 128x64 OLED display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OLED128x64ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OLED128x64Error { Success = 0, NotFound = 1 }

pub trait OLED128x64Display {
    fn id(&self) -> OLED128x64ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOLED128x64Display {
    pub id: OLED128x64ID,
    pub initialized: AtomicUsize,
}

impl SimpleOLED128x64Display {
    pub fn new(id: OLED128x64ID) -> Self {
        SimpleOLED128x64Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl OLED128x64Display for SimpleOLED128x64Display {
    fn id(&self) -> OLED128x64ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait OLED128x64Controller {
    fn init(&mut self, display_id: OLED128x64ID) -> Result<(), OLED128x64Error>;
    fn clear(&self, display_id: OLED128x64ID) -> Result<(), OLED128x64Error>;
    def draw_pixel(&self, display_id: OLED128x64ID, x: u8, y: u8, color: bool) -> Result<(), OLED128x64Error>;
}

#[repr(C)]
pub struct SimpleOLED128x64Controller {
    pub displays: Vec<Option<Box<dyn OLED128x64Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOLED128x64Controller {
    pub fn new() -> Self {
        SimpleOLED128x64Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OLED128x64Controller for SimpleOLED128x64Controller {
    fn init(&mut self, display_id: OLED128x64ID) -> Result<(), OLED128x64Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(OLED128x64Error::NotFound)
    }
    
    fn clear(&self, display_id: OLED128x64ID) -> Result<(), OLED128x64Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLED128x64Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: OLED128x64ID, _x: u8, _y: u8, _color: bool) -> Result<(), OLED128x64Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLED128x64Error::NotFound)
        }
    }
    
    fn get_display(&self, id: OLED128x64ID) -> Option<&dyn OLED128x64Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait OLED128x64Buffer {
    def draw_buffer(&self, display_id: OLED128x64ID, buffer: &[u8]) -> Result<(), OLED128x64Error>;
}

#[repr(C)]
pub struct SimpleOLED128x64Buffer {
    pub controller: SimpleOLED128x64Controller,
}

impl SimpleOLED128x64Buffer {
    pub fn new(controller: SimpleOLED128x64Controller) -> Self {
        SimpleOLED128x64Buffer { controller }
    }
}

impl OLED128x64Buffer for SimpleOLED128x64Buffer {
    fn draw_buffer(&self, display_id: OLED128x64ID, _buffer: &[u8]) -> Result<(), OLED128x64Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        }
        else {
            Err(OLED128x64Error::NotFound)
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
