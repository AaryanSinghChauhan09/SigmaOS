#![no_std]
#![no_main]

/// OOP-based RA8875 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2576
/// Implements RA8875 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RA8875ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RA8875Error { Success = 0, NotFound = 1 }

pub trait RA8875Display {
    fn id(&self) -> RA8875ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRA8875Display {
    pub id: RA8875ID,
    pub initialized: AtomicUsize,
}

impl SimpleRA8875Display {
    pub fn new(id: RA8875ID) -> Self {
        SimpleRA8875Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl RA8875Display for SimpleRA8875Display {
    fn id(&self) -> RA8875ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait RA8875Controller {
    fn init(&mut self, ra_id: RA8875ID) -> Result<(), RA8875Error>;
    fn set_window(&self, ra_id: RA8875ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), RA8875Error>;
    def push_colors(&self, ra_id: RA8875ID, colors: &[u16]) -> Result<(), RA8875Error>;
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
    fn init(&mut self, ra_id: RA8875ID) -> Result<(), RA8875Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == ra_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RA8875Error::NotFound)
    }
    
    fn set_window(&self, ra_id: RA8875ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), RA8875Error> {
        if self.get_display(ra_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn push_colors(&self, ra_id: RA8875ID, _colors: &[u16]) -> Result<(), RA8875Error> {
        if self.get_display(ra_id).is_some() {
            Ok(())
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn get_display(&self, id: RA8875ID) -> Option<&dyn RA8875Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait RA8875Touch {
    def read_touch(&self, ra_id: RA8875ID) -> Result<(u16, u16), RA8875Error>;
    def set_touch_mode(&mut self, ra_id: RA8875ID, mode: u8) -> Result<(), RA8875Error>;
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
    fn read_touch(&self, ra_id: RA8875ID) -> Result<(u16, u16), RA8875Error> {
        if self.controller.get_display(ra_id).is_some() {
            Ok((0, 0))
        } else {
            Err(RA8875Error::NotFound)
        }
    }
    
    fn set_touch_mode(&mut self, _ra_id: RA8875ID, _mode: u8) -> Result<(), RA8875Error> {
        Ok(())
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
