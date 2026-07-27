#![no_std]
#![no_main]

/// OOP-based Omron Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3326
/// Implements Omron incremental encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OmronID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OmronError { Success = 0, NotFound = 1 }

pub trait OmronEncoder {
    fn id(&self) -> OmronID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOmronEncoder {
    pub id: OmronID,
    pub initialized: AtomicUsize,
}

impl SimpleOmronEncoder {
    pub fn new(id: OmronID) -> Self {
        SimpleOmronEncoder {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl OmronEncoder for SimpleOmronEncoder {
    fn id(&self) -> OmronID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait OmronController {
    fn init(&mut self, encoder_id: OmronID) -> Result<(), OmronError>;
    fn read_count(&self, encoder_id: OmronID) -> Result<i32, OmronError>;
    def reset(&mut self, encoder_id: OmronID) -> Result<(), OmronError>;
}

#[repr(C)]
pub struct SimpleOmronController {
    pub encoders: Vec<Option<Box<dyn OmronEncoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOmronController {
    pub fn new() -> Self {
        SimpleOmronController {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OmronController for SimpleOmronController {
    fn init(&mut self, encoder_id: OmronID) -> Result<(), OmronError> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    encoder.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(OmronError::NotFound)
    }
    
    fn read_count(&self, encoder_id: OmronID) -> Result<i32, OmronError> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(OmronError::NotFound)
        }
    }
    
    fn reset(&mut self, encoder_id: OmronID) -> Result<(), OmronError> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(())
        } else {
            Err(OmronError::NotFound)
        }
    }
    
    fn get_encoder(&self, id: OmronID) -> Option<&dyn OmronEncoder> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == id { return Some(encoder.as_ref()); }
            }
        }
        None
    }
}

pub trait OmronZIndex {
    def read_z(&self, encoder_id: OmronID) -> Result<bool, OmronError>;
}

#[repr(C)]
pub struct SimpleOmronZIndex {
    pub controller: SimpleOmronController,
}

impl SimpleOmronZIndex {
    pub fn new(controller: SimpleOmronController) -> Self {
        SimpleOmronZIndex { controller }
    }
}

impl OmronZIndex for SimpleOmronZIndex {
    fn read_z(&self, encoder_id: OmronID) -> Result<bool, OmronError> {
        if self.controller.get_encoder(encoder_id).is_some() {
            Ok(false)
        } else {
            Err(OmronError::NotFound)
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
