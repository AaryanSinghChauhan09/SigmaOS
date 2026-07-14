#![no_std]
#![no_main]

/// OOP-based Hash Accelerator for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2086
/// Implements hash acceleration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HashID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HashError { Success = 0, NotFound = 1 }

pub trait HashAccelerator {
    fn id(&self) -> HashID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHashAccelerator {
    pub id: HashID,
    pub busy: AtomicUsize,
}

impl SimpleHashAccelerator {
    pub fn new(id: HashID) -> Self {
        SimpleHashAccelerator {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl HashAccelerator for SimpleHashAccelerator {
    fn id(&self) -> HashID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait HashController {
    fn compute(&self, hash_id: HashID, input: &[u8], output: &mut [u8]) -> Result<(), HashError>;
    def update(&self, hash_id: HashID, input: &[u8]) -> Result<(), HashError>;
}

#[repr(C)]
pub struct SimpleHashController {
    pub accelerators: Vec<Option<Box<dyn HashAccelerator>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHashController {
    pub fn new() -> Self {
        SimpleHashController {
            accelerators: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HashController for SimpleHashController {
    fn compute(&self, hash_id: HashID, _input: &[u8], output: &mut [u8]) -> Result<(), HashError> {
        if self.get_accelerator(hash_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(HashError::NotFound)
        }
    }
    
    fn update(&self, hash_id: HashID, _input: &[u8]) -> Result<(), HashError> {
        if self.get_accelerator(hash_id).is_some() {
            Ok(())
        } else {
            Err(HashError::NotFound)
        }
    }
    
    fn get_accelerator(&self, id: HashID) -> Option<&dyn HashAccelerator> {
        for acc_option in &self.accelerators {
            if let Some(ref acc) = *acc_option {
                if acc.id() == id { return Some(acc.as_ref()); }
            }
        }
        None
    }
}

pub trait SHA256 {
    def init(&mut self, hash_id: HashID) -> Result<(), HashError>;
    def finalize(&self, hash_id: HashID, output: &mut [u8]) -> Result<(), HashError>;
}

#[repr(C)]
pub struct SimpleSHA256 {
    pub controller: SimpleHashController,
}

impl SimpleSHA256 {
    pub fn new(controller: SimpleHashController) -> Self {
        SimpleSHA256 { controller }
    }
}

impl SHA256 for SimpleSHA256 {
    fn init(&mut self, _hash_id: HashID) -> Result<(), HashError> {
        Ok(())
    }
    
    fn finalize(&self, hash_id: HashID, output: &mut [u8]) -> Result<(), HashError> {
        if self.controller.get_accelerator(hash_id).is_some() {
            for byte in output.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(HashError::NotFound)
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
