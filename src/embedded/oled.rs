#![no_std]
#![no_main]

/// OOP-based OLED Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1486
/// Implements OLED display controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OLEDError { Success = 0, NotFound = 1 }

pub trait OLEDDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

#[repr(C)]
pub struct SimpleOLEDDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleOLEDDisplay {
    pub fn new(id: DisplayID, width: u8, height: u8) -> Self {
        SimpleOLEDDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl OLEDDisplay for SimpleOLEDDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u8 { self.width.load(Ordering::SeqCst) as u8 }
    fn height(&self) -> u8 { self.height.load(Ordering::SeqCst) as u8 }
}

pub trait OLEDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), OLEDError>;
    fn clear(&self, display_id: DisplayID) -> Result<(), OLEDError>;
    def draw_pixel(&self, display_id: DisplayID, x: u8, y: u8, color: bool) -> Result<(), OLEDError>;
}

#[repr(C)]
pub struct SimpleOLEDController {
    pub displays: Vec<Option<Box<dyn OLEDDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOLEDController {
    pub fn new() -> Self {
        SimpleOLEDController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OLEDController for SimpleOLEDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), OLEDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLEDError::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID) -> Result<(), OLEDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLEDError::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: DisplayID, _x: u8, _y: u8, _color: bool) -> Result<(), OLEDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLEDError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn OLEDDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SSD1306 {
    def send_command(&self, display_id: DisplayID, command: u8) -> Result<(), OLEDError>;
    def send_data(&self, display_id: DisplayID, data: &[u8]) -> Result<(), OLEDError>;
}

#[repr(C)]
pub struct SimpleSSD1306 {
    pub controller: SimpleOLEDController,
}

impl SimpleSSD1306 {
    pub fn new(controller: SimpleOLEDController) -> Self {
        SimpleSSD1306 { controller }
    }
}

impl SSD1306 for SimpleSSD1306 {
    fn send_command(&self, display_id: DisplayID, _command: u8) -> Result<(), OLEDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLEDError::NotFound)
        }
    }
    
    fn send_data(&self, display_id: DisplayID, _data: &[u8]) -> Result<(), OLEDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(OLEDError::NotFound)
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
