#![no_std]
#![no_main]

/// OOP-based LCD Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1246
/// Implements LCD display controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorFormat { RGB565 = 0, RGB888 = 1, ARGB8888 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LCDError { Success = 0, NotFound = 1 }

pub trait LCDDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn color_format(&self) -> ColorFormat;
}

#[repr(C)]
pub struct SimpleLCDDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub color_format: AtomicUsize,
}

impl SimpleLCDDisplay {
    pub fn new(id: DisplayID, width: u16, height: u16, color_format: ColorFormat) -> Self {
        SimpleLCDDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            color_format: AtomicUsize::new(color_format as usize),
        }
    }
}

impl LCDDisplay for SimpleLCDDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
    fn color_format(&self) -> ColorFormat { unsafe { core::mem::transmute(self.color_format.load(Ordering::SeqCst)) } }
}

pub trait LCDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), LCDError>;
    fn clear(&self, display_id: DisplayID, color: u32) -> Result<(), LCDError>;
    def draw_pixel(&self, display_id: DisplayID, x: u16, y: u16, color: u32) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleLCDController {
    pub displays: Vec<Option<Box<dyn LCDDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLCDController {
    pub fn new() -> Self {
        SimpleLCDController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LCDController for SimpleLCDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID, _color: u32) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: DisplayID, _x: u16, _y: u16, _color: u32) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn LCDDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait Framebuffer {
    def get_buffer(&self, display_id: DisplayID) -> Option<&[u8]>;
    def flush(&self, display_id: DisplayID) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleFramebuffer {
    pub controller: SimpleLCDController,
    pub buffers: Vec<(DisplayID, Vec<u8>)>,
}

impl SimpleFramebuffer {
    pub fn new(controller: SimpleLCDController) -> Self {
        SimpleFramebuffer {
            controller,
            buffers: Vec::new(),
        }
    }
}

impl Framebuffer for SimpleFramebuffer {
    fn get_buffer(&self, display_id: DisplayID) -> Option<&[u8]> {
        for &(id, ref buffer) in &self.buffers {
            if id == display_id {
                return Some(buffer);
            }
        }
        None
    }
    
    fn flush(&self, display_id: DisplayID) -> Result<(), LCDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
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
