#![no_std]
#![no_main]

/// OOP-based ST7735 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2446
/// Implements ST7735 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ST7735ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7735Error { Success = 0, NotFound = 1 }

pub trait ST7735Display {
    fn id(&self) -> ST7735ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7735Display {
    pub id: ST7735ID,
    pub initialized: AtomicUsize,
}

impl SimpleST7735Display {
    pub fn new(id: ST7735ID) -> Self {
        SimpleST7735Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ST7735Display for SimpleST7735Display {
    fn id(&self) -> ST7735ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ST7735Controller {
    fn init(&mut self, st_id: ST7735ID) -> Result<(), ST7735Error>;
    fn set_window(&self, st_id: ST7735ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), ST7735Error>;
    def push_colors(&self, st_id: ST7735ID, colors: &[u16]) -> Result<(), ST7735Error>;
}

#[repr(C)]
pub struct SimpleST7735Controller {
    pub displays: Vec<Option<Box<dyn ST7735Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7735Controller {
    pub fn new() -> Self {
        SimpleST7735Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7735Controller for SimpleST7735Controller {
    fn init(&mut self, st_id: ST7735ID) -> Result<(), ST7735Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == st_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7735Error::NotFound)
    }
    
    fn set_window(&self, st_id: ST7735ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), ST7735Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7735Error::NotFound)
        }
    }
    
    fn push_colors(&self, st_id: ST7735ID, _colors: &[u16]) -> Result<(), ST7735Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7735Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ST7735ID) -> Option<&dyn ST7735Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7735Rotation {
    def set_rotation(&mut self, st_id: ST7735ID, rotation: u8) -> Result<(), ST7735Error>;
    def get_rotation(&self, st_id: ST7735ID) -> Result<u8, ST7735Error>;
}

#[repr(C)]
pub struct SimpleST7735Rotation {
    pub controller: SimpleST7735Controller,
    pub rotations: Vec<(ST7735ID, AtomicUsize)>,
}

impl SimpleST7735Rotation {
    pub fn new(controller: SimpleST7735Controller) -> Self {
        SimpleST7735Rotation {
            controller,
            rotations: Vec::new(),
        }
    }
}

impl ST7735Rotation for SimpleST7735Rotation {
    fn set_rotation(&mut self, st_id: ST7735ID, rotation: u8) -> Result<(), ST7735Error> {
        self.rotations.push((st_id, AtomicUsize::new(rotation as usize)));
        Ok(())
    }
    
    fn get_rotation(&self, st_id: ST7735ID) -> Result<u8, ST7735Error> {
        for &(id, ref rot) in &self.rotations {
            if id == st_id {
                return Ok(rot.load(Ordering::SeqCst) as u8);
            }
        }
        Err(ST7735Error::NotFound)
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
