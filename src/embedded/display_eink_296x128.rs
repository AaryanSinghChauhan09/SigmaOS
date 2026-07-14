#![no_std]
#![no_main]

/// OOP-based E-Ink 296x128 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3966
/// Implements 296x128 e-paper display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EInk296x128ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EInk296x128Error { Success = 0, NotFound = 1 }

pub trait EInk296x128Display {
    fn id(&self) -> EInk296x128ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEInk296x128Display {
    pub id: EInk296x128ID,
    pub initialized: AtomicUsize,
}

impl SimpleEInk296x128Display {
    pub fn new(id: EInk296x128ID) -> Self {
        SimpleEInk296x128Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl EInk296x128Display for SimpleEInk296x128Display {
    fn id(&self) -> EInk296x128ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait EInk296x128Controller {
    fn init(&mut self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error>;
    fn clear(&self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error>;
    def draw_pixel(&self, display_id: EInk296x128ID, x: u16, y: u16, color: bool) -> Result<(), EInk296x128Error>;
}

#[repr(C)]
pub struct SimpleEInk296x128Controller {
    pub displays: Vec<Option<Box<dyn EInk296x128Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEInk296x128Controller {
    pub fn new() -> Self {
        SimpleEInk296x128Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EInk296x128Controller for SimpleEInk296x128Controller {
    fn init(&mut self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EInk296x128Error::NotFound)
    }
    
    fn clear(&self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk296x128Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: EInk296x128ID, _x: u16, _y: u16, _color: bool) -> Result<(), EInk296x128Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk296x128Error::NotFound)
        }
    }
    
    fn get_display(&self, id: EInk296x128ID) -> Option<&dyn EInk296x128Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait EInk296x128Refresh {
    def refresh(&self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error>;
}

#[repr(C)]
pub struct SimpleEInk296x128Refresh {
    pub controller: SimpleEInk296x128Controller,
}

impl SimpleEInk296x128Refresh {
    pub fn new(controller: SimpleEInk296x128Controller) -> Self {
        SimpleEInk296x128Refresh { controller }
    }
}

impl EInk296x128Refresh for SimpleEInk296x128Refresh {
    fn refresh(&self, display_id: EInk296x128ID) -> Result<(), EInk296x128Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk296x128Error::NotFound)
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
