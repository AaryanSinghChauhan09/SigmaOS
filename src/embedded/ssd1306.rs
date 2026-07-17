#![no_std]
#![no_main]

/// OOP-based SSD1306 OLED for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1566
/// Implements SSD1306 OLED controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSD1306Error { Success = 0, NotFound = 1 }

pub trait SSD1306Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSSD1306Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleSSD1306Display {
    pub fn new(id: DisplayID, width: u8, height: u8) -> Self {
        SimpleSSD1306Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl SSD1306Display for SimpleSSD1306Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u8 { self.width.load(Ordering::SeqCst) as u8 }
    fn height(&self) -> u8 { self.height.load(Ordering::SeqCst) as u8 }
}

pub trait SSD1306Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), SSD1306Error>;
    fn clear(&self, display_id: DisplayID) -> Result<(), SSD1306Error>;
    def display(&self, display_id: DisplayID) -> Result<(), SSD1306Error>;
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
    fn init(&mut self, display_id: DisplayID) -> Result<(), SSD1306Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID) -> Result<(), SSD1306Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn display(&self, display_id: DisplayID) -> Result<(), SSD1306Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn SSD1306Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SSD1306Buffer {
    def set_pixel(&self, display_id: DisplayID, x: u8, y: u8, on: bool) -> Result<(), SSD1306Error>;
    def draw_bitmap(&self, display_id: DisplayID, x: u8, y: u8, bitmap: &[u8], w: u8, h: u8) -> Result<(), SSD1306Error>;
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
    fn set_pixel(&self, display_id: DisplayID, _x: u8, _y: u8, _on: bool) -> Result<(), SSD1306Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SSD1306Error::NotFound)
        }
    }
    
    fn draw_bitmap(&self, display_id: DisplayID, _x: u8, _y: u8, _bitmap: &[u8], _w: u8, _h: u8) -> Result<(), SSD1306Error> {
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
