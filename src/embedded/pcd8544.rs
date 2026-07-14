#![no_std]
#![no_main]

/// OOP-based PCD8544 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1586
/// Implements PCD8544 (Nokia 5110) LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCD8544Error { Success = 0, NotFound = 1 }

pub trait PCD8544Display {
    fn id(&self) -> DisplayID;
    fn is_on(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCD8544Display {
    pub id: DisplayID,
    pub on: AtomicUsize,
}

impl SimplePCD8544Display {
    pub fn new(id: DisplayID) -> Self {
        SimplePCD8544Display {
            id,
            on: AtomicUsize::new(0),
        }
    }
}

impl PCD8544Display for SimplePCD8544Display {
    fn id(&self) -> DisplayID { self.id }
    fn is_on(&self) -> bool { self.on.load(Ordering::SeqCst) == 1 }
}

pub trait PCD8544Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error>;
    fn power_on(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error>;
    def power_off(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error>;
}

#[repr(C)]
pub struct SimplePCD8544Controller {
    pub displays: Vec<Option<Box<dyn PCD8544Display>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCD8544Controller {
    pub fn new() -> Self {
        SimplePCD8544Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCD8544Controller for SimplePCD8544Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(PCD8544Error::NotFound)
        }
    }
    
    fn power_on(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.on.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCD8544Error::NotFound)
    }
    
    fn power_off(&mut self, display_id: DisplayID) -> Result<(), PCD8544Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.on.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCD8544Error::NotFound)
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn PCD8544Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait PCD8544Graphics {
    def set_cursor(&mut self, display_id: DisplayID, x: u8, y: u8) -> Result<(), PCD8544Error>;
    def write_char(&self, display_id: DisplayID, c: u8) -> Result<(), PCD8544Error>;
}

#[repr(C)]
pub struct SimplePCD8544Graphics {
    pub controller: SimplePCD8544Controller,
}

impl SimplePCD8544Graphics {
    pub fn new(controller: SimplePCD8544Controller) -> Self {
        SimplePCD8544Graphics { controller }
    }
}

impl PCD8544Graphics for SimplePCD8544Graphics {
    fn set_cursor(&mut self, display_id: DisplayID, _x: u8, _y: u8) -> Result<(), PCD8544Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(PCD8544Error::NotFound)
        }
    }
    
    fn write_char(&self, display_id: DisplayID, _c: u8) -> Result<(), PCD8544Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(PCD8544Error::NotFound)
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
