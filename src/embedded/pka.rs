#![no_std]
#![no_main]

/// OOP-based PKA for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2116
/// Implements PKA (Public Key Accelerator)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PKAID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PKAError { Success = 0, NotFound = 1 }

pub trait PKAAccelerator {
    fn id(&self) -> PKAID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimplePKAAccelerator {
    pub id: PKAID,
    pub busy: AtomicUsize,
}

impl SimplePKAAccelerator {
    pub fn new(id: PKAID) -> Self {
        SimplePKAAccelerator {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl PKAAccelerator for SimplePKAAccelerator {
    fn id(&self) -> PKAID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait PKAController {
    fn mod_exp(&self, pka_id: PKAID, base: &[u8], exp: &[u8], mod_val: &[u8], result: &mut [u8]) -> Result<(), PKAError>;
    def mod_inv(&self, pka_id: PKAID, input: &[u8], mod_val: &[u8], result: &mut [u8]) -> Result<(), PKAError>;
}

#[repr(C)]
pub struct SimplePKAController {
    pub accelerators: Vec<Option<Box<dyn PKAAccelerator>>>,
    pub next_id: AtomicUsize,
}

impl SimplePKAController {
    pub fn new() -> Self {
        SimplePKAController {
            accelerators: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PKAController for SimplePKAController {
    fn mod_exp(&self, pka_id: PKAID, _base: &[u8], _exp: &[u8], _mod_val: &[u8], result: &mut [u8]) -> Result<(), PKAError> {
        if self.get_accelerator(pka_id).is_some() {
            for byte in result.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(PKAError::NotFound)
        }
    }
    
    fn mod_inv(&self, pka_id: PKAID, _input: &[u8], _mod_val: &[u8], result: &mut [u8]) -> Result<(), PKAError> {
        if self.get_accelerator(pka_id).is_some() {
            for byte in result.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(PKAError::NotFound)
        }
    }
    
    fn get_accelerator(&self, id: PKAID) -> Option<&dyn PKAAccelerator> {
        for acc_option in &self.accelerators {
            if let Some(ref acc) = *acc_option {
                if acc.id() == id { return Some(acc.as_ref()); }
            }
        }
        None
    }
}

pub trait PKAMath {
    def add(&self, pka_id: PKAID, a: &[u8], b: &[u8], result: &mut [u8]) -> Result<(), PKAError>;
    def mul(&self, pka_id: PKAID, a: &[u8], b: &[u8], result: &mut [u8]) -> Result<(), PKAError>;
}

#[repr(C)]
pub struct SimplePKAMath {
    pub controller: SimplePKAController,
}

impl SimplePKAMath {
    pub fn new(controller: SimplePKAController) -> Self {
        SimplePKAMath { controller }
    }
}

impl PKAMath for SimplePKAMath {
    fn add(&self, pka_id: PKAID, _a: &[u8], _b: &[u8], result: &mut [u8]) -> Result<(), PKAError> {
        if self.controller.get_accelerator(pka_id).is_some() {
            for byte in result.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(PKAError::NotFound)
        }
    }
    
    fn mul(&self, pka_id: PKAID, _a: &[u8], _b: &[u8], result: &mut [u8]) -> Result<(), PKAError> {
        if self.controller.get_accelerator(pka_id).is_some() {
            for byte in result.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(PKAError::NotFound)
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
