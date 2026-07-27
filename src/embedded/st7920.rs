#![no_std]
#![no_main]

/// OOP-based ST7920 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1596
/// Implements ST7920 LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7920Error { Success = 0, NotFound = 1 }

pub trait ST7920Display {
    fn id(&self) -> DisplayID;
    fn is_on(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7920Display {
    pub id: DisplayID,
    pub on: AtomicUsize,
}

impl SimpleST7920Display {
    pub fn new(id: DisplayID) -> Self {
        SimpleST7920Display {
            id,
            on: AtomicUsize::new(0),
        }
    }
}

impl ST7920Display for SimpleST7920Display {
    fn id(&self) -> DisplayID { self.id }
    fn is_on(&self) -> bool { self.on.load(Ordering::SeqCst) == 1 }
}

pub trait ST7920Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ST7920Error>;
    fn clear(&self, display_id: DisplayID) -> Result<(), ST7920Error>;
    def home(&self, display_id: DisplayID) -> Result<(), ST7920Error>;
}

#[repr(C)]
pub struct SimpleST7920Controller {
    pub displays: Vec<Option<Box<dyn ST7920Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7920Controller {
    pub fn new() -> Self {
        SimpleST7920Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7920Controller for SimpleST7920Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ST7920Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7920Error::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID) -> Result<(), ST7920Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7920Error::NotFound)
        }
    }
    
    fn home(&self, display_id: DisplayID) -> Result<(), ST7920Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7920Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn ST7920Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7920Graphics {
    def set_pixel(&self, display_id: DisplayID, x: u8, y: u8, on: bool) -> Result<(), ST7920Error>;
    def draw_line(&self, display_id: DisplayID, x1: u8, y1: u8, x2: u8, y2: u8, on: bool) -> Result<(), ST7920Error>;
}

#[repr(C)]
pub struct SimpleST7920Graphics {
    pub controller: SimpleST7920Controller,
}

impl SimpleST7920Graphics {
    pub fn new(controller: SimpleST7920Controller) -> Self {
        SimpleST7920Graphics { controller }
    }
}

impl ST7920Graphics for SimpleST7920Graphics {
    fn set_pixel(&self, display_id: DisplayID, _x: u8, _y: u8, _on: bool) -> Result<(), ST7920Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7920Error::NotFound)
        }
    }
    
    fn draw_line(&self, display_id: DisplayID, _x1: u8, _y1: u8, _x2: u8, _y2: u8, _on: bool) -> Result<(), ST7920Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ST7920Error::NotFound)
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
