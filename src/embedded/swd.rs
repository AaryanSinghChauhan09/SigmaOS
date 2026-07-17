#![no_std]
#![no_main]

/// OOP-based SWD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2146
/// Implements SWD (Serial Wire Debug) interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SWDID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SWDError { Success = 0, NotFound = 1 }

pub trait SWDInterface {
    fn id(&self) -> SWDID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSWDInterface {
    pub id: SWDID,
    pub connected: AtomicUsize,
}

impl SimpleSWDInterface {
    pub fn new(id: SWDID) -> Self {
        SimpleSWDInterface {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl SWDInterface for SimpleSWDInterface {
    fn id(&self) -> SWDID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait SWDController {
    fn init(&mut self, swd_id: SWDID) -> Result<(), SWDError>;
    fn read_reg(&self, swd_id: SWDID, addr: u32) -> Result<u32, SWDError>;
    def write_reg(&self, swd_id: SWDID, addr: u32, value: u32) -> Result<(), SWDError>;
}

#[repr(C)]
pub struct SimpleSWDController {
    pub interfaces: Vec<Option<Box<dyn SWDInterface>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSWDController {
    pub fn new() -> Self {
        SimpleSWDController {
            interfaces: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SWDController for SimpleSWDController {
    fn init(&mut self, swd_id: SWDID) -> Result<(), SWDError> {
        for iface_option in &mut self.interfaces {
            if let Some(ref mut iface) = *iface_option {
                if iface.id() == swd_id {
                    iface.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SWDError::NotFound)
    }
    
    fn read_reg(&self, swd_id: SWDID, _addr: u32) -> Result<u32, SWDError> {
        if self.get_interface(swd_id).is_some() {
            Ok(0)
        } else {
            Err(SWDError::NotFound)
        }
    }
    
    fn write_reg(&self, swd_id: SWDID, _addr: u32, _value: u32) -> Result<(), SWDError> {
        if self.get_interface(swd_id).is_some() {
            Ok(())
        } else {
            Err(SWDError::NotFound)
        }
    }
    
    fn get_interface(&self, id: SWDID) -> Option<&dyn SWDInterface> {
        for iface_option in &self.interfaces {
            if let Some(ref iface) = *iface_option {
                if iface.id() == id { return Some(iface.as_ref()); }
            }
        }
        None
    }
}

pub trait SWDMemory {
    def read_mem(&self, swd_id: SWDID, address: u32, data: &mut [u8]) -> Result<(), SWDError>;
    def write_mem(&self, swd_id: SWDID, address: u32, data: &[u8]) -> Result<(), SWDError>;
}

#[repr(C)]
pub struct SimpleSWDMemory {
    pub controller: SimpleSWDController,
}

impl SimpleSWDMemory {
    pub fn new(controller: SimpleSWDController) -> Self {
        SimpleSWDMemory { controller }
    }
}

impl SWDMemory for SimpleSWDMemory {
    fn read_mem(&self, swd_id: SWDID, _address: u32, data: &mut [u8]) -> Result<(), SWDError> {
        if self.controller.get_interface(swd_id).is_some() {
            for byte in data.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(SWDError::NotFound)
        }
    }
    
    fn write_mem(&self, swd_id: SWDID, _address: u32, _data: &[u8]) -> Result<(), SWDError> {
        if self.controller.get_interface(swd_id).is_some() {
            Ok(())
        } else {
            Err(SWDError::NotFound)
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
