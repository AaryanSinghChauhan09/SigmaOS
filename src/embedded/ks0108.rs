#![no_std]
#![no_main]

/// OOP-based KS0108 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1606
/// Implements KS0108 LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KS0108Error { Success = 0, NotFound = 1 }

pub trait KS0108Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

#[repr(C)]
pub struct SimpleKS0108Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleKS0108Display {
    pub fn new(id: DisplayID, width: u8, height: u8) -> Self {
        SimpleKS0108Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl KS0108Display for SimpleKS0108Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u8 { self.width.load(Ordering::SeqCst) as u8 }
    fn height(&self) -> u8 { self.height.load(Ordering::SeqCst) as u8 }
}

pub trait KS0108Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), KS0108Error>;
    fn clear(&self, display_id: DisplayID) -> Result<(), KS0108Error>;
    def set_page(&self, display_id: DisplayID, page: u8) -> Result<(), KS0108Error>;
}

#[repr(C)]
pub struct SimpleKS0108Controller {
    pub displays: Vec<Option<Box<dyn KS0108Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleKS0108Controller {
    pub fn new() -> Self {
        SimpleKS0108Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl KS0108Controller for SimpleKS0108Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), KS0108Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(KS0108Error::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID) -> Result<(), KS0108Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(KS0108Error::NotFound)
        }
    }
    
    fn set_page(&self, display_id: DisplayID, _page: u8) -> Result<(), KS0108Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(KS0108Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn KS0108Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait KS0108Graphics {
    def set_column(&self, display_id: DisplayID, column: u8) -> Result<(), KS0108Error>;
    def write_data(&self, display_id: DisplayID, data: u8) -> Result<(), KS0108Error>;
}

#[repr(C)]
pub struct SimpleKS0108Graphics {
    pub controller: SimpleKS0108Controller,
}

impl SimpleKS0108Graphics {
    pub fn new(controller: SimpleKS0108Controller) -> Self {
        SimpleKS0108Graphics { controller }
    }
}

impl KS0108Graphics for SimpleKS0108Graphics {
    fn set_column(&self, display_id: DisplayID, _column: u8) -> Result<(), KS0108Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(KS0108Error::NotFound)
        }
    }
    
    fn write_data(&self, display_id: DisplayID, _data: u8) -> Result<(), KS0108Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(KS0108Error::NotFound)
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
