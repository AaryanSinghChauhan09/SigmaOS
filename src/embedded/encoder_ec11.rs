#![no_std]
#![no_main]

/// OOP-based EC11 Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3316
/// Implements EC11 rotary encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EC11ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EC11Error { Success = 0, NotFound = 1 }

pub trait EC11Encoder {
    fn id(&self) -> EC11ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEC11Encoder {
    pub id: EC11ID,
    pub initialized: AtomicUsize,
}

impl SimpleEC11Encoder {
    pub fn new(id: EC11ID) -> Self {
        SimpleEC11Encoder {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl EC11Encoder for SimpleEC11Encoder {
    fn id(&self) -> EC11ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait EC11Controller {
    fn init(&mut self, encoder_id: EC11ID) -> Result<(), EC11Error>;
    fn read(&self, encoder_id: EC11ID) -> Result<i8, EC11Error>;
    def reset(&mut self, encoder_id: EC11ID) -> Result<(), EC11Error>;
}

#[repr(C)]
pub struct SimpleEC11Controller {
    pub encoders: Vec<Option<Box<dyn EC11Encoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEC11Controller {
    pub fn new() -> Self {
        SimpleEC11Controller {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EC11Controller for SimpleEC11Controller {
    fn init(&mut self, encoder_id: EC11ID) -> Result<(), EC11Error> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    encoder.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EC11Error::NotFound)
    }
    
    fn read(&self, encoder_id: EC11ID) -> Result<i8, EC11Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(EC11Error::NotFound)
        }
    }
    
    fn reset(&mut self, encoder_id: EC11ID) -> Result<(), EC11Error> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(())
        } else {
            Err(EC11Error::NotFound)
        }
    }
    
    fn get_encoder(&self, id: EC11ID) -> Option<&dyn EC11Encoder> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == id { return Some(encoder.as_ref()); }
            }
        }
        None
    }
}

pub trait EC11Button {
    def read_button(&self, encoder_id: EC11ID) -> Result<bool, EC11Error>;
}

#[repr(C)]
pub struct SimpleEC11Button {
    pub controller: SimpleEC11Controller,
}

impl SimpleEC11Button {
    pub fn new(controller: SimpleEC11Controller) -> Self {
        SimpleEC11Button { controller }
    }
}

impl EC11Button for SimpleEC11Button {
    fn read_button(&self, encoder_id: EC11ID) -> Result<bool, EC11Error> {
        if self.controller.get_encoder(encoder_id).is_some() {
            Ok(false)
        } else {
            Err(EC11Error::NotFound)
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
