#![no_std]
#![no_main]

/// OOP-based HX8357 TFT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1546
/// Implements HX8357 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HX8357Error { Success = 0, NotFound = 1 }

pub trait HX8357Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[repr(C)]
pub struct SimpleHX8357Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleHX8357Display {
    pub fn new(id: DisplayID, width: u16, height: u16) -> Self {
        SimpleHX8357Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl HX8357Display for SimpleHX8357Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
}

pub trait HX8357Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), HX8357Error>;
    fn set_scroll(&mut self, display_id: DisplayID, y: u16) -> Result<(), HX8357Error>;
    def invert_display(&mut self, display_id: DisplayID, invert: bool) -> Result<(), HX8357Error>;
}

#[repr(C)]
pub struct SimpleHX8357Controller {
    pub displays: Vec<Option<Box<dyn HX8357Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHX8357Controller {
    pub fn new() -> Self {
        SimpleHX8357Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HX8357Controller for SimpleHX8357Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), HX8357Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn set_scroll(&mut self, display_id: DisplayID, _y: u16) -> Result<(), HX8357Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn invert_display(&mut self, display_id: DisplayID, _invert: bool) -> Result<(), HX8357Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn HX8357Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait HX8357Touch {
    def read_touch(&self, display_id: DisplayID) -> Result<(u16, u16), HX8357Error>;
    def is_touched(&self, display_id: DisplayID) -> Result<bool, HX8357Error>;
}

#[repr(C)]
pub struct SimpleHX8357Touch {
    pub controller: SimpleHX8357Controller,
}

impl SimpleHX8357Touch {
    pub fn new(controller: SimpleHX8357Controller) -> Self {
        SimpleHX8357Touch { controller }
    }
}

impl HX8357Touch for SimpleHX8357Touch {
    fn read_touch(&self, display_id: DisplayID) -> Result<(u16, u16), HX8357Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok((160, 120))
        } else {
            Err(HX8357Error::NotFound)
        }
    }
    
    fn is_touched(&self, display_id: DisplayID) -> Result<bool, HX8357Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(false)
        } else {
            Err(HX8357Error::NotFound)
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
