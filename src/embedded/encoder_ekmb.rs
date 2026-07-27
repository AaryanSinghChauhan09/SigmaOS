#![no_std]
#![no_main]

/// OOP-based EKMB Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3306
/// Implements EKMB incremental encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EKMBID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EKMBError { Success = 0, NotFound = 1 }

pub trait EKMBEncoder {
    fn id(&self) -> EKMBID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleEKMBEncoder {
    pub id: EKMBID,
    pub initialized: AtomicUsize,
}

impl SimpleEKMBEncoder {
    pub fn new(id: EKMBID) -> Self {
        SimpleEKMBEncoder {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl EKMBEncoder for SimpleEKMBEncoder {
    fn id(&self) -> EKMBID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait EKMBController {
    fn init(&mut self, encoder_id: EKMBID) -> Result<(), EKMBError>;
    fn read_count(&self, encoder_id: EKMBID) -> Result<i32, EKMBError>;
    def reset(&mut self, encoder_id: EKMBID) -> Result<(), EKMBError>;
}

#[repr(C)]
pub struct SimpleEKMBController {
    pub encoders: Vec<Option<Box<dyn EKMBEncoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEKMBController {
    pub fn new() -> Self {
        SimpleEKMBController {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EKMBController for SimpleEKMBController {
    fn init(&mut self, encoder_id: EKMBID) -> Result<(), EKMBError> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    encoder.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(EKMBError::NotFound)
    }
    
    fn read_count(&self, encoder_id: EKMBID) -> Result<i32, EKMBError> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(EKMBError::NotFound)
        }
    }
    
    fn reset(&mut self, encoder_id: EKMBID) -> Result<(), EKMBError> {
        if self.get_encoder(encoder_id).is_some() {
            Ok(())
        } else {
            Err(EKMBError::NotFound)
        }
    }
    
    fn get_encoder(&self, id: EKMBID) -> Option<&dyn EKMBEncoder> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == id { return Some(encoder.as_ref()); }
            }
        }
        None
    }
}

pub trait EKMBDirection {
    def get_direction(&self, encoder_id: EKMBID) -> Result<i8, EKMBError>;
}

#[repr(C)]
pub struct SimpleEKMBDirection {
    pub controller: SimpleEKMBController,
}

impl SimpleEKMBDirection {
    pub fn new(controller: SimpleEKMBController) -> Self {
        SimpleEKMBDirection { controller }
    }
}

impl EKMBDirection for SimpleEKMBDirection {
    fn get_direction(&self, encoder_id: EKMBID) -> Result<i8, EKMBError> {
        if self.controller.get_encoder(encoder_id).is_some() {
            Ok(0)
        } else {
            Err(EKMBError::NotFound)
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
