#![no_std]
#![no_main]

/// OOP-based E-Ink 200x200 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3976
/// Implements 200x200 e-paper display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EInk200x200ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EInk200x200Error { Success = 0, NotFound = 1 }

pub trait EInk200x200Display {
    fn id(&self) -> EInk200x200ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEInk200x200Display {
    pub id: EInk200x200ID,
    pub initialized: AtomicUsize,
}

impl SimpleEInk200x200Display {
    pub fn new(id: EInk200x200ID) -> Self {
        SimpleEInk200x200Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl EInk200x200Display for SimpleEInk200x200Display {
    fn id(&self) -> EInk200x200ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait EInk200x200Controller {
    fn init(&mut self, display_id: EInk200x200ID) -> Result<(), EInk200x200Error>;
    fn clear(&self, display_id: EInk200x200ID) -> Result<(), EInk200x200Error>;
    def draw_pixel(&self, display_id: EInk200x200ID, x: u16, y: u16, color: bool) -> Result<(), EInk200x200Error>;
}

#[repr(C)]
pub struct SimpleEInk200x200Controller {
    pub displays: Vec<Option<Box<dyn EInk200x200Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEInk200x200Controller {
    pub fn new() -> Self {
        SimpleEInk200x200Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EInk200x200Controller for SimpleEInk200x200Controller {
    fn init(&mut self, display_id: EInk200x200ID) -> Result<(), EInk200x200Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EInk200x200Error::NotFound)
    }
    
    fn clear(&self, display_id: EInk200x200ID) -> Result<(), EInk200x200Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk200x200Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: EInk200x200ID, _x: u16, _y: u16, _color: bool) -> Result<(), EInk200x200Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk200x200Error::NotFound)
        }
    }
    
    fn get_display(&self, id: EInk200x200ID) -> Option<&dyn EInk200x200Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait EInk200x200Partial {
    def partial_refresh(&self, display_id: EInk200x200ID, x: u16, y: u16, w: u16, h: u16) -> Result<(), EInk200x200Error>;
}

#[repr(C)]
pub struct SimpleEInk200x200Partial {
    pub controller: SimpleEInk200x200Controller,
}

impl SimpleEInk200x200Partial {
    pub fn new(controller: SimpleEInk200x200Controller) -> Self {
        SimpleEInk200x200Partial { controller }
    }
}

impl EInk200x200Partial for SimpleEInk200x200Partial {
    fn partial_refresh(&self, display_id: EInk200x200ID, _x: u16, _y: u16, _w: u16, _h: u16) -> Result<(), EInk200x200Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInk200x200Error::NotFound)
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
