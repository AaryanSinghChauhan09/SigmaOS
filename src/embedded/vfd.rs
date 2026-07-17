#![no_std]
#![no_main]

/// OOP-based VFD Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1496
/// Implements VFD (Vacuum Fluorescent Display)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VFDError { Success = 0, NotFound = 1 }

pub trait VFDDisplay {
    fn id(&self) -> DisplayID;
    fn num_digits(&self) -> u8;
}

#[repr(C)]
pub struct SimpleVFDDisplay {
    pub id: DisplayID,
    pub num_digits: AtomicUsize,
}

impl SimpleVFDDisplay {
    pub fn new(id: DisplayID, num_digits: u8) -> Self {
        SimpleVFDDisplay {
            id,
            num_digits: AtomicUsize::new(num_digits as usize),
        }
    }
}

impl VFDDisplay for SimpleVFDDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn num_digits(&self) -> u8 { self.num_digits.load(Ordering::SeqCst) as u8 }
}

pub trait VFDController {
    fn set_brightness(&self, display_id: DisplayID, brightness: u8) -> Result<(), VFDError>;
    fn set_digit(&self, display_id: DisplayID, position: u8, value: u8) -> Result<(), VFDError>;
    def clear(&self, display_id: DisplayID) -> Result<(), VFDError>;
}

#[repr(C)]
pub struct SimpleVFDController {
    pub displays: Vec<Option<Box<dyn VFDDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleVFDController {
    pub fn new() -> Self {
        SimpleVFDController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VFDController for SimpleVFDController {
    fn set_brightness(&self, display_id: DisplayID, _brightness: u8) -> Result<(), VFDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(VFDError::NotFound)
        }
    }
    
    fn set_digit(&self, display_id: DisplayID, _position: u8, _value: u8) -> Result<(), VFDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(VFDError::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID) -> Result<(), VFDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(VFDError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn VFDDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait VFDText {
    def write_string(&self, display_id: DisplayID, text: &[u8]) -> Result<(), VFDError>;
    def scroll_left(&self, display_id: DisplayID) -> Result<(), VFDError>;
}

#[repr(C)]
pub struct SimpleVFDText {
    pub controller: SimpleVFDController,
}

impl SimpleVFDText {
    pub fn new(controller: SimpleVFDController) -> Self {
        SimpleVFDText { controller }
    }
}

impl VFDText for SimpleVFDText {
    fn write_string(&self, display_id: DisplayID, _text: &[u8]) -> Result<(), VFDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(VFDError::NotFound)
        }
    }
    
    fn scroll_left(&self, display_id: DisplayID) -> Result<(), VFDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(VFDError::NotFound)
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
