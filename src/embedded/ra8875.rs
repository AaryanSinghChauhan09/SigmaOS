#![no_std]
#![no_main]

/// OOP-based RA8875 TFT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1556
/// Implements RA8875 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RA8875Error { Success = 0, NotFound = 1 }

pub trait RA8875Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[repr(C)]
pub struct SimpleRA8875Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleRA8875Display {
    pub fn new(id: DisplayID, width: u16, height: u16) -> Self {
        SimpleRA8875Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl RA8875Display for SimpleRA8875Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
}

pub trait RA8875Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), RA8875Error>;
    fn set_window(&mut self, display_id: DisplayID, x: u16, y: u16, w: u16, h: u16) -> Result<(), RA8875Error>;
    def set_cursor(&mut self, display_id: DisplayID, x: u16, y: u16) -> Result<(), RA8875Error>;
}

#[repr(C)]
pub struct SimpleRA8875Controller {
    pub displays: Vec<Option<Box<dyn RA8875Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRA8875Controller {
    pub fn new() -> Self {
        SimpleRA8875Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RA8875Controller for SimpleRA8875Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), RA8875Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn set_window(&mut self, display_id: DisplayID, _x: u16, _y: u16, _w: u16, _h: u16) -> Result<(), RA8875Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn set_cursor(&mut self, display_id: DisplayID, _x: u16, _y: u16) -> Result<(), RA8875Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn RA8875Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait RA8875Touch {
    def enable_touch(&mut self, display_id: DisplayID) -> Result<(), RA8875Error>;
    def read_touch(&self, display_id: DisplayID) -> Result<(u16, u16), RA8875Error>;
}

#[repr(C)]
pub struct SimpleRA8875Touch {
    pub controller: SimpleRA8875Controller,
}

impl SimpleRA8875Touch {
    pub fn new(controller: SimpleRA8875Controller) -> Self {
        SimpleRA8875Touch { controller }
    }
}

impl RA8875Touch for SimpleRA8875Touch {
    fn enable_touch(&mut self, display_id: DisplayID) -> Result<(), RA8875Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn read_touch(&self, display_id: DisplayID) -> Result<(u16, u16), RA8875Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok((320, 240))
        } else {
            Err(RA8875Error::NotFound)
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
