#![no_std]
#![no_main]

/// OOP-based SSD1306 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3246
/// Implements SSD1306 OLED display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SSD1306ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSD1306Error { Success = 0, NotFound = 1 }

pub trait SSD1306Display {
    fn id(&self) -> SSD1306ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSSD1306Display {
    pub id: SSD1306ID,
    pub initialized: AtomicUsize,
}

impl SimpleSSD1306Display {
    pub fn new(id: SSD1306ID) -> Self {
        SimpleSSD1306Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SSD1306Display for SimpleSSD1306Display {
    fn id(&self) -> SSD1306ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SSD1306Controller {
    fn init(&mut self, display_id: SSD1306ID) -> Result<(), SSD1306Error>;
    fn clear(&self, display_id: SSD1306ID) -> Result<(), SSD1306Error>;
    def draw_pixel(&self, display_id: SSD1306ID, x: u8, y: u8, color: bool) -> Result<(), SSD1306Error>;
}

#[repr(C)]
pub struct SimpleSSD1306Controller {
    pub displays: Vec<Option<Box<dyn SSD1306Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSSD1306Controller {
    pub fn new() -> Self {
        SimpleSSD1306Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SSD1306Controller for SimpleSSD1306Controller {
    fn init(&mut self, display_id: SSD1306ID) -> Result<(), SSD1306Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSD1306Error::NotFound)
    }
    
    fn clear(&self, display_id: SSD1306ID) -> Result<(), SSD1306Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: SSD1306ID, _x: u8, _y: u8, _color: bool) -> Result<(), SSD1306Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn get_display(&self, id: SSD1306ID) -> Option<&dyn SSD1306Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SSD1306Buffer {
    def draw_buffer(&self, display_id: SSD1306ID, buffer: &[u8]) -> Result<(), SSD1306Error>;
    def display(&self, display_id: SSD1306ID) -> Result<(), SSD1306Error>;
}

#[repr(C)]
pub struct SimpleSSD1306Buffer {
    pub controller: SimpleSSD1306Controller,
}

impl SimpleSSD1306Buffer {
    pub fn new(controller: SimpleSSD1306Controller) -> Self {
        SimpleSSD1306Buffer { controller }
    }
}

impl SSD1306Buffer for SimpleSSD1306Buffer {
    fn draw_buffer(&self, display_id: SSD1306ID, _buffer: &[u8]) -> Result<(), SSD1306Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn display(&self, display_id: SSD1306ID) -> Result<(), SSD1306Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
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
