#![no_std]
#![no_main]

/// OOP-based PDM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2336
/// Implements PDM (Pulse Density Modulation) microphone interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PDMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PDMError { Success = 0, NotFound = 1 }

pub trait PDMDevice {
    fn id(&self) -> PDMID;
    fn is_active(&self) -> bool;
}

#[repr(C)]
pub struct SimplePDMDevice {
    pub id: PDMID,
    pub active: AtomicUsize,
}

impl SimplePDMDevice {
    pub fn new(id: PDMID) -> Self {
        SimplePDMDevice {
            id,
            active: AtomicUsize::new(0),
        }
    }
}

impl PDMDevice for SimplePDMDevice {
    fn id(&self) -> PDMID { self.id }
    fn is_active(&self) -> bool { self.active.load(Ordering::SeqCst) == 1 }
}

pub trait PDMController {
    fn init(&mut self, pdm_id: PDMID) -> Result<(), PDMError>;
    fn receive(&self, pdm_id: PDMID, buffer: &mut [u8]) -> Result<usize, PDMError>;
    def set_gain(&mut self, pdm_id: PDMID, gain: i8) -> Result<(), PDMError>;
}

#[repr(C)]
pub struct SimplePDMController {
    pub devices: Vec<Option<Box<dyn PDMDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimplePDMController {
    pub fn new() -> Self {
        SimplePDMController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PDMController for SimplePDMController {
    fn init(&mut self, pdm_id: PDMID) -> Result<(), PDMError> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == pdm_id {
                    device.active.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PDMError::NotFound)
    }
    
    fn receive(&self, pdm_id: PDMID, buffer: &mut [u8]) -> Result<usize, PDMError> {
        if self.get_device(pdm_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(PDMError::NotFound)
        }
    }
    
    fn set_gain(&mut self, _pdm_id: PDMID, _gain: i8) -> Result<(), PDMError> {
        Ok(())
    }
    
    fn get_device(&self, id: PDMID) -> Option<&dyn PDMDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait PDMDecimation {
    def set_decimation(&mut self, pdm_id: PDMID, ratio: u8) -> Result<(), PDMError>;
    def get_decimation(&self, pdm_id: PDMID) -> Result<u8, PDMError>;
}

#[repr(C)]
pub struct SimplePDMDecimation {
    pub controller: SimplePDMController,
    pub decimations: Vec<(PDMID, AtomicUsize)>,
}

impl SimplePDMDecimation {
    pub fn new(controller: SimplePDMController) -> Self {
        SimplePDMDecimation {
            controller,
            decimations: Vec::new(),
        }
    }
}

impl PDMDecimation for SimplePDMDecimation {
    fn set_decimation(&mut self, pdm_id: PDMID, ratio: u8) -> Result<(), PDMError> {
        self.decimations.push((pdm_id, AtomicUsize::new(ratio as usize)));
        Ok(())
    }
    
    fn get_decimation(&self, pdm_id: PDMID) -> Result<u8, PDMError> {
        for &(id, ref dec) in &self.decimations {
            if id == pdm_id {
                return Ok(dec.load(Ordering::SeqCst) as u8);
            }
        }
        Err(PDMError::NotFound)
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
