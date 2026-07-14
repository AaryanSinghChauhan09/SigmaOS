#![no_std]
#![no_main]

/// OOP-based ST7565 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1526
/// Implements ST7565 LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7565Error { Success = 0, NotFound = 1 }

pub trait ST7565Display {
    fn id(&self) -> DisplayID;
    fn is_on(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7565Display {
    pub id: DisplayID,
    pub on: AtomicUsize,
}

impl SimpleST7565Display {
    pub fn new(id: DisplayID) -> Self {
        SimpleST7565Display {
            id,
            on: AtomicUsize::new(0),
        }
    }
}

impl ST7565Display for SimpleST7565Display {
    fn id(&self) -> DisplayID { self.id }
    fn is_on(&self) -> bool { self.on.load(Ordering::SeqCst) == 1 }
}

pub trait ST7565Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ST7565Error>;
    fn turn_on(&mut self, display_id: DisplayID) -> Result<(), ST7565Error>;
    fn turn_off(&mut self, display_id: DisplayID) -> Result<(), ST7565Error>;
}

#[repr(C)]
pub struct SimpleST7565Controller {
    pub displays: Vec<Option<Box<dyn ST7565Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7565Controller {
    pub fn new() -> Self {
        SimpleST7565Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7565Controller for SimpleST7565Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ST7565Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7565Error::NotFound)
        }
    }
    
    fn turn_on(&mut self, display_id: DisplayID) -> Result<(), ST7565Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.on.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7565Error::NotFound)
    }
    
    fn turn_off(&mut self, display_id: DisplayID) -> Result<(), ST7565Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.on.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7565Error::NotFound)
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn ST7565Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7565Buffer {
    def set_contrast(&mut self, contrast: u8);
    def update_buffer(&self, display_id: DisplayID, buffer: &[u8]) -> Result<(), ST7565Error>;
}

#[repr(C)]
pub struct SimpleST7565Buffer {
    pub controller: SimpleST7565Controller,
    pub contrast: AtomicUsize,
}

impl SimpleST7565Buffer {
    pub fn new(controller: SimpleST7565Controller) -> Self {
        SimpleST7565Buffer {
            controller,
            contrast: AtomicUsize::new(32),
        }
    }
}

impl ST7565Buffer for SimpleST7565Buffer {
    fn set_contrast(&mut self, contrast: u8) {
        self.contrast.store(contrast as usize, Ordering::SeqCst);
    }
    
    fn update_buffer(&self, display_id: DisplayID, _buffer: &[u8]) -> Result<(), ST7565Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7565Error::NotFound)
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
