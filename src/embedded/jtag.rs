#![no_std]
#![no_main]

/// OOP-based JTAG for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2136
/// Implements JTAG interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type JTAGID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JTAGError { Success = 0, NotFound = 1 }

pub trait JTAGInterface {
    fn id(&self) -> JTAGID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleJTAGInterface {
    pub id: JTAGID,
    pub connected: AtomicUsize,
}

impl SimpleJTAGInterface {
    pub fn new(id: JTAGID) -> Self {
        SimpleJTAGInterface {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl JTAGInterface for SimpleJTAGInterface {
    fn id(&self) -> JTAGID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait JTAGController {
    fn reset(&mut self, jtag_id: JTAGID) -> Result<(), JTAGError>;
    fn shift_ir(&self, jtag_id: JTAGID, data: &[u8]) -> Result<(), JTAGError>;
    def shift_dr(&self, jtag_id: JTAGID, data: &[u8]) -> Result<(), JTAGError>;
}

#[repr(C)]
pub struct SimpleJTAGController {
    pub interfaces: Vec<Option<Box<dyn JTAGInterface>>>,
    pub next_id: AtomicUsize,
}

impl SimpleJTAGController {
    pub fn new() -> Self {
        SimpleJTAGController {
            interfaces: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl JTAGController for SimpleJTAGController {
    fn reset(&mut self, _jtag_id: JTAGID) -> Result<(), JTAGError> {
        Ok(())
    }
    
    fn shift_ir(&self, jtag_id: JTAGID, _data: &[u8]) -> Result<(), JTAGError> {
        if self.get_interface(jtag_id).is_some() {
            Ok(())
        } else {
            Err(JTAGError::NotFound)
        }
    }
    
    fn shift_dr(&self, jtag_id: JTAGID, _data: &[u8]) -> Result<(), JTAGError> {
        if self.get_interface(jtag_id).is_some() {
            Ok(())
        } else {
            Err(JTAGError::NotFound)
        }
    }
    
    fn get_interface(&self, id: JTAGID) -> Option<&dyn JTAGInterface> {
        for iface_option in &self.interfaces {
            if let Some(ref iface) = *iface_option {
                if iface.id() == id { return Some(iface.as_ref()); }
            }
        }
        None
    }
}

pub trait JTAGDebug {
    def set_breakpoint(&mut self, jtag_id: JTAGID, address: u32) -> Result<(), JTAGError>;
    def read_memory(&self, jtag_id: JTAGID, address: u32, data: &mut [u8]) -> Result<(), JTAGError>;
}

#[repr(C)]
pub struct SimpleJTAGDebug {
    pub controller: SimpleJTAGController,
}

impl SimpleJTAGDebug {
    pub fn new(controller: SimpleJTAGController) -> Self {
        SimpleJTAGDebug { controller }
    }
}

impl JTAGDebug for SimpleJTAGDebug {
    fn set_breakpoint(&mut self, _jtag_id: JTAGID, _address: u32) -> Result<(), JTAGError> {
        Ok(())
    }
    
    fn read_memory(&self, jtag_id: JTAGID, _address: u32, data: &mut [u8]) -> Result<(), JTAGError> {
        if self.controller.get_interface(jtag_id).is_some() {
            for byte in data.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(JTAGError::NotFound)
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
