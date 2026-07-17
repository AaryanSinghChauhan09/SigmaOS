#![no_std]
#![no_main]

/// OOP-based ST7567 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2546
/// Implements ST7567 monochrome LCD

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ST7567ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ST7567Error { Success = 0, NotFound = 1 }

pub trait ST7567Display {
    fn id(&self) -> ST7567ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleST7567Display {
    pub id: ST7567ID,
    pub initialized: AtomicUsize,
}

impl SimpleST7567Display {
    pub fn new(id: ST7567ID) -> Self {
        SimpleST7567Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ST7567Display for SimpleST7567Display {
    fn id(&self) -> ST7567ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ST7567Controller {
    fn init(&mut self, st_id: ST7567ID) -> Result<(), ST7567Error>;
    fn set_pixel(&self, st_id: ST7567ID, x: u8, y: u8, on: bool) -> Result<(), ST7567Error>;
    def display(&self, st_id: ST7567ID) -> Result<(), ST7567Error>;
}

#[repr(C)]
pub struct SimpleST7567Controller {
    pub displays: Vec<Option<Box<dyn ST7567Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleST7567Controller {
    pub fn new() -> Self {
        SimpleST7567Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ST7567Controller for SimpleST7567Controller {
    fn init(&mut self, st_id: ST7567ID) -> Result<(), ST7567Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == st_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ST7567Error::NotFound)
    }
    
    fn set_pixel(&self, st_id: ST7567ID, _x: u8, _y: u8, _on: bool) -> Result<(), ST7567Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7567Error::NotFound)
        }
    }
    
    fn display(&self, st_id: ST7567ID) -> Result<(), ST7567Error> {
        if self.get_display(st_id).is_some() {
            Ok(())
        } else {
            Err(ST7567Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ST7567ID) -> Option<&dyn ST7567Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ST7567Bias {
    def set_bias(&mut self, st_id: ST7567ID, bias: u8) -> Result<(), ST7567Error>;
    def get_bias(&self, st_id: ST7567ID) -> Result<u8, ST7567Error>;
}

#[repr(C)]
pub struct SimpleST7567Bias {
    pub controller: SimpleST7567Controller,
    pub biases: Vec<(ST7567ID, AtomicUsize)>,
}

impl SimpleST7567Bias {
    pub fn new(controller: SimpleST7567Controller) -> Self {
        SimpleST7567Bias {
            controller,
            biases: Vec::new(),
        }
    }
}

impl ST7567Bias for SimpleST7567Bias {
    fn set_bias(&mut self, st_id: ST7567ID, bias: u8) -> Result<(), ST7567Error> {
        self.biases.push((st_id, AtomicUsize::new(bias as usize)));
        Ok(())
    }
    
    fn get_bias(&self, st_id: ST7567ID) -> Result<u8, ST7567Error> {
        for &(id, ref bias) in &self.biases {
            if id == st_id {
                return Ok(bias.load(Ordering::SeqCst) as u8);
            }
        }
        Err(ST7567Error::NotFound)
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
