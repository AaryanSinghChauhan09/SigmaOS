#![no_std]
#![no_main]

/// OOP-based FRAM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2386
/// Implements FRAM (Ferroelectric RAM)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FRAMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FRAMError { Success = 0, NotFound = 1 }

pub trait FRAMDevice {
    fn id(&self) -> FRAMID;
    fn size_kb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleFRAMDevice {
    pub id: FRAMID,
    pub size_kb: AtomicUsize,
}

impl SimpleFRAMDevice {
    pub fn new(id: FRAMID, size_kb: u16) -> Self {
        SimpleFRAMDevice {
            id,
            size_kb: AtomicUsize::new(size_kb as usize),
        }
    }
}

impl FRAMDevice for SimpleFRAMDevice {
    fn id(&self) -> FRAMID { self.id }
    fn size_kb(&self) -> u16 { self.size_kb.load(Ordering::SeqCst) as u16 }
}

pub trait FRAMController {
    fn read(&self, fram_id: FRAMID, address: u16, buffer: &mut [u8]) -> Result<(), FRAMError>;
    fn write(&self, fram_id: FRAMID, address: u16, data: &[u8]) -> Result<(), FRAMError>;
}

#[repr(C)]
pub struct SimpleFRAMController {
    pub frams: Vec<Option<Box<dyn FRAMDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFRAMController {
    pub fn new() -> Self {
        SimpleFRAMController {
            frams: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FRAMController for SimpleFRAMController {
    fn read(&self, fram_id: FRAMID, _address: u16, buffer: &mut [u8]) -> Result<(), FRAMError> {
        if self.get_fram(fram_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(FRAMError::NotFound)
        }
    }
    
    fn write(&self, fram_id: FRAMID, _address: u16, _data: &[u8]) -> Result<(), FRAMError> {
        if self.get_fram(fram_id).is_some() {
            Ok(())
        } else {
            Err(FRAMError::NotFound)
        }
    }
    
    fn get_fram(&self, id: FRAMID) -> Option<&dyn FRAMDevice> {
        for fram_option in &self.frams {
            if let Some(ref fram) = *fram_option {
                if fram.id() == id { return Some(fram.as_ref()); }
            }
        }
        None
    }
}

pub trait FRAMWriteProtect {
    def set_write_protect(&mut self, fram_id: FRAMID, block: u8, protect: bool) -> Result<(), FRAMError>;
    def is_protected(&self, fram_id: FRAMID, block: u8) -> Result<bool, FRAMError>;
}

#[repr(C)]
pub struct SimpleFRAMWriteProtect {
    pub controller: SimpleFRAMController,
    pub protections: Vec<(FRAMID, AtomicUsize)>,
}

impl SimpleFRAMWriteProtect {
    pub fn new(controller: SimpleFRAMController) -> Self {
        SimpleFRAMWriteProtect {
            controller,
            protections: Vec::new(),
        }
    }
}

impl FRAMWriteProtect for SimpleFRAMWriteProtect {
    fn set_write_protect(&mut self, fram_id: FRAMID, _block: u8, protect: bool) -> Result<(), FRAMError> {
        self.protections.push((fram_id, AtomicUsize::new(if protect { 1 } else { 0 })));
        Ok(())
    }
    
    fn is_protected(&self, fram_id: FRAMID, _block: u8) -> Result<bool, FRAMError> {
        for &(id, ref prot) in &self.protections {
            if id == fram_id {
                return Ok(prot.load(Ordering::SeqCst) == 1);
            }
        }
        Err(FRAMError::NotFound)
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
