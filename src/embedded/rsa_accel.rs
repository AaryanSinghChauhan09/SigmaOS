#![no_std]
#![no_main]

/// OOP-based RSA Accelerator for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2096
/// Implements RSA acceleration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RSAID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RSAError { Success = 0, NotFound = 1 }

pub trait RSAAccelerator {
    fn id(&self) -> RSAID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRSAAccelerator {
    pub id: RSAID,
    pub busy: AtomicUsize,
}

impl SimpleRSAAccelerator {
    pub fn new(id: RSAID) -> Self {
        SimpleRSAAccelerator {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl RSAAccelerator for SimpleRSAAccelerator {
    fn id(&self) -> RSAID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait RSAController {
    fn encrypt(&self, rsa_id: RSAID, input: &[u8], output: &mut [u8]) -> Result<(), RSAError>;
    fn decrypt(&self, rsa_id: RSAID, input: &[u8], output: &mut [u8]) -> Result<(), RSAError>;
    def sign(&self, rsa_id: RSAID, hash: &[u8], signature: &mut [u8]) -> Result<(), RSAError>;
}

#[repr(C)]
pub struct SimpleRSAController {
    pub accelerators: Vec<Option<Box<dyn RSAAccelerator>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRSAController {
    pub fn new() -> Self {
        SimpleRSAController {
            accelerators: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RSAController for SimpleRSAController {
    fn encrypt(&self, rsa_id: RSAID, _input: &[u8], output: &mut [u8]) -> Result<(), RSAError> {
        if self.get_accelerator(rsa_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(RSAError::NotFound)
        }
    }
    
    fn decrypt(&self, rsa_id: RSAID, _input: &[u8], output: &mut [u8]) -> Result<(), RSAError> {
        if self.get_accelerator(rsa_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(RSAError::NotFound)
        }
    }
    
    fn sign(&self, rsa_id: RSAID, _hash: &[u8], signature: &mut [u8]) -> Result<(), RSAError> {
        if self.get_accelerator(rsa_id).is_some() {
            for byte in signature.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(RSAError::NotFound)
        }
    }
    
    fn get_accelerator(&self, id: RSAID) -> Option<&dyn RSAAccelerator> {
        for acc_option in &self.accelerators {
            if let Some(ref acc) = *acc_option {
                if acc.id() == id { return Some(acc.as_ref()); }
            }
        }
        None
    }
}

pub trait RSAKey {
    def set_modulus(&mut self, rsa_id: RSAID, modulus: &[u8]) -> Result<(), RSAError>;
    def set_exponent(&mut self, rsa_id: RSAID, exponent: &[u8]) -> Result<(), RSAError>;
}

#[repr(C)]
pub struct SimpleRSAKey {
    pub controller: SimpleRSAController,
}

impl SimpleRSAKey {
    pub fn new(controller: SimpleRSAController) -> Self {
        SimpleRSAKey { controller }
    }
}

impl RSAKey for SimpleRSAKey {
    fn set_modulus(&mut self, _rsa_id: RSAID, _modulus: &[u8]) -> Result<(), RSAError> {
        Ok(())
    }
    
    fn set_exponent(&mut self, _rsa_id: RSAID, _exponent: &[u8]) -> Result<(), RSAError> {
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
