#![no_std]
#![no_main]

/// OOP-based USB DFU for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2226
/// Implements USB DFU (Device Firmware Upgrade)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DFUID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DFUState { Idle = 0, Busy = 1, Error = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DFUError { Success = 0, NotFound = 1 }

pub trait USBDFU {
    fn id(&self) -> DFUID;
    fn state(&self) -> DFUState;
}

#[repr(C)]
pub struct SimpleUSBDFU {
    pub id: DFUID,
    pub state: AtomicUsize,
}

impl SimpleUSBDFU {
    pub fn new(id: DFUID) -> Self {
        SimpleUSBDFU {
            id,
            state: AtomicUsize::new(DFUState::Idle as usize),
        }
    }
}

impl USBDFU for SimpleUSBDFU {
    fn id(&self) -> DFUID { self.id }
    fn state(&self) -> DFUState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
}

pub trait DFUController {
    fn init(&mut self, dfu_id: DFUID) -> Result<(), DFUError>;
    fn download(&self, dfu_id: DFUID, address: u32, data: &[u8]) -> Result<(), DFUError>;
    def upload(&self, dfu_id: DFUID, address: u32, buffer: &mut [u8]) -> Result<(), DFUError>;
}

#[repr(C)]
pub struct SimpleDFUController {
    pub dfu_devices: Vec<Option<Box<dyn USBDFU>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDFUController {
    pub fn new() -> Self {
        SimpleDFUController {
            dfu_devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DFUController for SimpleDFUController {
    fn init(&mut self, dfu_id: DFUID) -> Result<(), DFUError> {
        for dfu_option in &mut self.dfu_devices {
            if let Some(ref mut dfu) = *dfu_option {
                if dfu.id() == dfu_id {
                    dfu.state.store(DFUState::Idle as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DFUError::NotFound)
    }
    
    fn download(&self, dfu_id: DFUID, _address: u32, _data: &[u8]) -> Result<(), DFUError> {
        if self.get_dfu(dfu_id).is_some() {
            Ok(())
        } else {
            Err(DFUError::NotFound)
        }
    }
    
    fn upload(&self, dfu_id: DFUID, _address: u32, buffer: &mut [u8]) -> Result<(), DFUError> {
        if self.get_dfu(dfu_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(DFUError::NotFound)
        }
    }
    
    fn get_dfu(&self, id: DFUID) -> Option<&dyn USBDFU> {
        for dfu_option in &self.dfu_devices {
            if let Some(ref dfu) = *dfu_option {
                if dfu.id() == id { return Some(dfu.as_ref()); }
            }
        }
        None
    }
}

pub trait DFUStatus {
    def get_status(&self, dfu_id: DFUID) -> Result<(u8, u8, u8), DFUError>;
    def detach(&mut self, dfu_id: DFUID) -> Result<(), DFUError>;
}

#[repr(C)]
pub struct SimpleDFUStatus {
    pub controller: SimpleDFUController,
}

impl SimpleDFUStatus {
    pub fn new(controller: SimpleDFUController) -> Self {
        SimpleDFUStatus { controller }
    }
}

impl DFUStatus for SimpleDFUStatus {
    fn get_status(&self, dfu_id: DFUID) -> Result<(u8, u8, u8), DFUError> {
        if self.controller.get_dfu(dfu_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(DFUError::NotFound)
        }
    }
    
    fn detach(&mut self, dfu_id: DFUID) -> Result<(), DFUError> {
        for dfu_option in &mut self.controller.dfu_devices {
            if let Some(ref mut dfu) = *dfu_option {
                if dfu.id() == dfu_id {
                    dfu.state.store(DFUState::Idle as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DFUError::NotFound)
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
