#![no_std]
#![no_main]

/// OOP-based Crypto Accelerator for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2076
/// Implements cryptographic acceleration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CryptoID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CryptoError { Success = 0, NotFound = 1 }

pub trait CryptoAccelerator {
    fn id(&self) -> CryptoID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleCryptoAccelerator {
    pub id: CryptoID,
    pub busy: AtomicUsize,
}

impl SimpleCryptoAccelerator {
    pub fn new(id: CryptoID) -> Self {
        SimpleCryptoAccelerator {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl CryptoAccelerator for SimpleCryptoAccelerator {
    fn id(&self) -> CryptoID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait CryptoController {
    fn encrypt(&self, crypto_id: CryptoID, input: &[u8], output: &mut [u8]) -> Result<(), CryptoError>;
    fn decrypt(&self, crypto_id: CryptoID, input: &[u8], output: &mut [u8]) -> Result<(), CryptoError>;
    def hash(&self, crypto_id: CryptoID, input: &[u8], output: &mut [u8]) -> Result<(), CryptoError>;
}

#[repr(C)]
pub struct SimpleCryptoController {
    pub accelerators: Vec<Option<Box<dyn CryptoAccelerator>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCryptoController {
    pub fn new() -> Self {
        SimpleCryptoController {
            accelerators: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CryptoController for SimpleCryptoController {
    fn encrypt(&self, crypto_id: CryptoID, _input: &[u8], output: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_accelerator(crypto_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn decrypt(&self, crypto_id: CryptoID, _input: &[u8], output: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_accelerator(crypto_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn hash(&self, crypto_id: CryptoID, _input: &[u8], output: &mut [u8]) -> Result<(), CryptoError> {
        if self.get_accelerator(crypto_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(CryptoError::NotFound)
        }
    }
    
    fn get_accelerator(&self, id: CryptoID) -> Option<&dyn CryptoAccelerator> {
        for acc_option in &self.accelerators {
            if let Some(ref acc) = *acc_option {
                if acc.id() == id { return Some(acc.as_ref()); }
            }
        }
        None
    }
}

pub trait AES {
    def set_key(&mut self, crypto_id: CryptoID, key: &[u8]) -> Result<(), CryptoError>;
    def set_iv(&mut self, crypto_id: CryptoID, iv: &[u8]) -> Result<(), CryptoError>;
}

#[repr(C)]
pub struct SimpleAES {
    pub controller: SimpleCryptoController,
}

impl SimpleAES {
    pub fn new(controller: SimpleCryptoController) -> Self {
        SimpleAES { controller }
    }
}

impl AES for SimpleAES {
    fn set_key(&mut self, _crypto_id: CryptoID, _key: &[u8]) -> Result<(), CryptoError> {
        Ok(())
    }
    
    fn set_iv(&mut self, _crypto_id: CryptoID, _iv: &[u8]) -> Result<(), CryptoError> {
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
