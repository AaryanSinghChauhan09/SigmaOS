#![no_std]
#![no_main]

/// OOP-based E-Ink Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1476
/// Implements E-Ink display controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorMode { BlackWhite = 0, Grayscale = 1, Color = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EInkError { Success = 0, NotFound = 1 }

pub trait EInkDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn color_mode(&self) -> ColorMode;
}

#[repr(C)]
pub struct SimpleEInkDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub color_mode: AtomicUsize,
}

impl SimpleEInkDisplay {
    pub fn new(id: DisplayID, width: u16, height: u16, color_mode: ColorMode) -> Self {
        SimpleEInkDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            color_mode: AtomicUsize::new(color_mode as usize),
        }
    }
}

impl EInkDisplay for SimpleEInkDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
    fn color_mode(&self) -> ColorMode { unsafe { core::mem::transmute(self.color_mode.load(Ordering::SeqCst)) } }
}

pub trait EInkController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), EInkError>;
    fn update(&self, display_id: DisplayID, buffer: &[u8]) -> Result<(), EInkError>;
    def partial_update(&self, display_id: DisplayID, x: u16, y: u16, width: u16, height: u16, buffer: &[u8]) -> Result<(), EInkError>;
}

#[repr(C)]
pub struct SimpleEInkController {
    pub displays: Vec<Option<Box<dyn EInkDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEInkController {
    pub fn new() -> Self {
        SimpleEInkController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EInkController for SimpleEInkController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn update(&self, display_id: DisplayID, _buffer: &[u8]) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn partial_update(&self, display_id: DisplayID, _x: u16, _y: u16, _width: u16, _height: u16, _buffer: &[u8]) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn EInkDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait RefreshController {
    def set_refresh_mode(&mut self, mode: u8);
    def trigger_refresh(&self, display_id: DisplayID) -> Result<(), EInkError>;
}

#[repr(C)]
pub struct SimpleRefreshController {
    pub controller: SimpleEInkController,
    pub refresh_mode: AtomicUsize,
}

impl SimpleRefreshController {
    pub fn new(controller: SimpleEInkController) -> Self {
        SimpleRefreshController {
            controller,
            refresh_mode: AtomicUsize::new(0),
        }
    }
}

impl RefreshController for SimpleRefreshController {
    fn set_refresh_mode(&mut self, mode: u8) {
        self.refresh_mode.store(mode as usize, Ordering::SeqCst);
    }
    
    fn trigger_refresh(&self, display_id: DisplayID) -> Result<(), EInkError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
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
