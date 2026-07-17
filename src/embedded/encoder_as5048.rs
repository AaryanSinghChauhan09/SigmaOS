#![no_std]
#![no_main]

/// OOP-based AS5048 Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3286
/// Implements AS5048 magnetic encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type AS5048ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum AS5048Error { Success = 0, NotFound = 1 }

pub trait AS5048Encoder {
    fn id(&self) -> AS5048ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleAS5048Encoder {
    pub id: AS5048ID,
    pub initialized: AtomicUsize,
}

impl SimpleAS5048Encoder {
    pub fn new(id: AS5048ID) -> Self {
        SimpleAS5048Encoder {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl AS5048Encoder for SimpleAS5048Encoder {
    fn id(&self) -> AS5048ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait AS5048Controller {
    fn init(&mut self, encoder_id: AS5048ID) -> Result<(), AS5048Error>;
    fn read_angle(&self, encoder_id: AS5048ID) -> Result<u16, AS5048Error>;
    def zero(&mut self, encoder_id: AS5048ID) -> Result<(), AS5048Error>;
}

#[repr(C)]
pub struct SimpleAS5048Controller {
    pub encoders: Vec<Option<Box<dyn AS5048Encoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleAS5048Controller {
    pub fn new() -> Self {
        SimpleAS5048Controller {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl AS5048Controller for SimpleAS5048Controller {
    fn init(&mut self, encoder_id: AS5048ID) -> Result<(), AS5048Error> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    encoder.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(AS5048Error::NotFound)
    }
    
    fn read_angle(&self, encoder_id: AS5048ID) -> Result<u16, AS5048Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(AS5048Error::NotFound)
        }
    }
    
    fn zero(&mut self, encoder_id: AS5048ID) -> Result<(), AS5048Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(())
        } else {
            Err(AS5048Error::NotFound)
        }
    }
    
    fn get_encoder(&self, id: AS5048ID) -> Option<&dyn AS5048Encoder> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == id { return Some(encoder.as_ref()); }
            }
        }
        None
    }
}

pub trait AS5048Diagnostics {
    def read_diag(&self, encoder_id: AS5048ID) -> Result<u8, AS5048Error>;
}

#[repr(C)]
pub struct SimpleAS5048Diagnostics {
    pub controller: SimpleAS5048Controller,
}

impl SimpleAS5048Diagnostics {
    pub fn new(controller: SimpleAS5048Controller) -> Self {
        SimpleAS5048Diagnostics { controller }
    }
}

impl AS5048Diagnostics for SimpleAS5048Diagnostics {
    fn read_diag(&self, encoder_id: AS5048ID) -> Result<u8, AS5048Error> {
        if self.controller.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(AS5048Error::NotFound)
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
