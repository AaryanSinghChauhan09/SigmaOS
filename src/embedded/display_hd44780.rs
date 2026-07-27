#![no_std]
#![no_main]

/// OOP-based HD44780 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3276
/// Implements HD44780 LCD display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HD44780ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HD44780Error { Success = 0, NotFound = 1 }

pub trait HD44780Display {
    fn id(&self) -> HD44780ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHD44780Display {
    pub id: HD44780ID,
    pub initialized: AtomicUsize,
}

impl SimpleHD44780Display {
    pub fn new(id: HD44780ID) -> Self {
        SimpleHD44780Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HD44780Display for SimpleHD44780Display {
    fn id(&self) -> HD44780ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HD44780Controller {
    fn init(&mut self, display_id: HD44780ID) -> Result<(), HD44780Error>;
    fn clear(&self, display_id: HD44780ID) -> Result<(), HD44780Error>;
    def write(&self, display_id: HD44780ID, text: &[u8]) -> Result<(), HD44780Error>;
}

#[repr(C)]
pub struct SimpleHD44780Controller {
    pub displays: Vec<Option<Box<dyn HD44780Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHD44780Controller {
    pub fn new() -> Self {
        SimpleHD44780Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HD44780Controller for SimpleHD44780Controller {
    fn init(&mut self, display_id: HD44780ID) -> Result<(), HD44780Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HD44780Error::NotFound)
    }
    
    fn clear(&self, display_id: HD44780ID) -> Result<(), HD44780Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HD44780Error::NotFound)
        }
    }
    
    fn write(&self, display_id: HD44780ID, _text: &[u8]) -> Result<(), HD44780Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HD44780Error::NotFound)
        }
    }
    
    fn get_display(&self, id: HD44780ID) -> Option<&dyn HD44780Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait HD44780Cursor {
    def set_cursor(&mut self, display_id: HD44780ID, col: u8, row: u8) -> Result<(), HD44780Error>;
}

#[repr(C)]
pub struct SimpleHD44780Cursor {
    pub controller: SimpleHD44780Controller,
}

impl SimpleHD44780Cursor {
    pub fn new(controller: SimpleHD44780Controller) -> Self {
        SimpleHD44780Cursor { controller }
    }
}

impl HD44780Cursor for SimpleHD44780Cursor {
    fn set_cursor(&mut self, display_id: HD44780ID, _col: u8, _row: u8) -> Result<(), HD44780Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(HD44780Error::NotFound)
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
