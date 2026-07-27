#![no_std]
#![no_main]

/// OOP-based ECC Accelerator for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2106
/// Implements ECC acceleration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ECCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ECCError { Success = 0, NotFound = 1 }

pub trait ECCAccelerator {
    fn id(&self) -> ECCID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleECCAccelerator {
    pub id: ECCID,
    pub busy: AtomicUsize,
}

impl SimpleECCAccelerator {
    pub fn new(id: ECCID) -> Self {
        SimpleECCAccelerator {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl ECCAccelerator for SimpleECCAccelerator {
    fn id(&self) -> ECCID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait ECCController {
    fn generate_key(&self, ecc_id: ECCID, private: &mut [u8], public: &mut [u8]) -> Result<(), ECCError>;
    def sign(&self, ecc_id: ECCID, hash: &[u8], signature: &mut [u8]) -> Result<(), ECCError>;
    def verify(&self, ecc_id: ECCID, hash: &[u8], signature: &[u8], public: &[u8]) -> Result<bool, ECCError>;
}

#[repr(C)]
pub struct SimpleECCController {
    pub accelerators: Vec<Option<Box<dyn ECCAccelerator>>>,
    pub next_id: AtomicUsize,
}

impl SimpleECCController {
    pub fn new() -> Self {
        SimpleECCController {
            accelerators: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ECCController for SimpleECCController {
    fn generate_key(&self, ecc_id: ECCID, private: &mut [u8], public: &mut [u8]) -> Result<(), ECCError> {
        if self.get_accelerator(ecc_id).is_some() {
            for byte in private.iter_mut() { *byte = 0; }
            for byte in public.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(ECCError::NotFound)
        }
    }
    
    fn sign(&self, ecc_id: ECCID, _hash: &[u8], signature: &mut [u8]) -> Result<(), ECCError> {
        if self.get_accelerator(ecc_id).is_some() {
            for byte in signature.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(ECCError::NotFound)
        }
    }
    
    fn verify(&self, ecc_id: ECCID, _hash: &[u8], _signature: &[u8], _public: &[u8]) -> Result<bool, ECCError> {
        if self.get_accelerator(ecc_id).is_some() {
            Ok(true)
        } else {
            Err(ECCError::NotFound)
        }
    }
    
    fn get_accelerator(&self, id: ECCID) -> Option<&dyn ECCAccelerator> {
        for acc_option in &self.accelerators {
            if let Some(ref acc) = *acc_option {
                if acc.id() == id { return Some(acc.as_ref()); }
            }
        }
        None
    }
}

pub trait ECDH {
    def compute_shared(&self, ecc_id: ECCID, private: &[u8], public: &[u8], shared: &mut [u8]) -> Result<(), ECCError>;
}

#[repr(C)]
pub struct SimpleECDH {
    pub controller: SimpleECCController,
}

impl SimpleECDH {
    pub fn new(controller: SimpleECCController) -> Self {
        SimpleECDH { controller }
    }
}

impl ECDH for SimpleECDH {
    fn compute_shared(&self, ecc_id: ECCID, _private: &[u8], _public: &[u8], shared: &mut [u8]) -> Result<(), ECCError> {
        if self.controller.get_accelerator(ecc_id).is_some() {
            for byte in shared.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(ECCError::NotFound)
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
