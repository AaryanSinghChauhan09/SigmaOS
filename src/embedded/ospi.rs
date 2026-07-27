#![no_std]
#![no_main]

/// OOP-based OSPI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2056
/// Implements OSPI (Octal SPI) interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OSPIID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OSPIError { Success = 0, NotFound = 1 }

pub trait OSPIDevice {
    fn id(&self) -> OSPIID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleOSPIDevice {
    pub id: OSPIID,
    pub busy: AtomicUsize,
}

impl SimpleOSPIDevice {
    pub fn new(id: OSPIID) -> Self {
        SimpleOSPIDevice {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl OSPIDevice for SimpleOSPIDevice {
    fn id(&self) -> OSPIID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait OSPIController {
    fn init(&mut self, ospi_id: OSPIID) -> Result<(), OSPIError>;
    fn read(&self, ospi_id: OSPIID, address: u32, buffer: &mut [u8]) -> Result<(), OSPIError>;
    def write(&self, ospi_id: OSPIID, address: u32, buffer: &[u8]) -> Result<(), OSPIError>;
}

#[repr(C)]
pub struct SimpleOSPIController {
    pub devices: Vec<Option<Box<dyn OSPIDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOSPIController {
    pub fn new() -> Self {
        SimpleOSPIController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OSPIController for SimpleOSPIController {
    fn init(&mut self, _ospi_id: OSPIID) -> Result<(), OSPIError> {
        Ok(())
    }
    
    fn read(&self, ospi_id: OSPIID, _address: u32, buffer: &mut [u8]) -> Result<(), OSPIError> {
        if self.get_device(ospi_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(OSPIError::NotFound)
        }
    }
    
    fn write(&self, ospi_id: OSPIID, _address: u32, _buffer: &[u8]) -> Result<(), OSPIError> {
        if self.get_device(ospi_id).is_some() {
            Ok(())
        } else {
            Err(OSPIError::NotFound)
        }
    }
    
    fn get_device(&self, id: OSPIID) -> Option<&dyn OSPIDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait OSPIHyperBus {
    def set_hyper_mode(&mut self, ospi_id: OSPIID) -> Result<(), OSPIError>;
    def get_latency(&self, ospi_id: OSPIID) -> Result<u8, OSPIError>;
}

#[repr(C)]
pub struct SimpleOSPIHyperBus {
    pub controller: SimpleOSPIController,
    pub latencies: Vec<(OSPIID, AtomicUsize)>,
}

impl SimpleOSPIHyperBus {
    pub fn new(controller: SimpleOSPIController) -> Self {
        SimpleOSPIHyperBus {
            controller,
            latencies: Vec::new(),
        }
    }
}

impl OSPIHyperBus for SimpleOSPIHyperBus {
    fn set_hyper_mode(&mut self, _ospi_id: OSPIID) -> Result<(), OSPIError> {
        Ok(())
    }
    
    fn get_latency(&self, ospi_id: OSPIID) -> Result<u8, OSPIError> {
        for &(id, ref lat) in &self.latencies {
            if id == ospi_id {
                return Ok(lat.load(Ordering::SeqCst) as u8);
            }
        }
        Err(OSPIError::NotFound)
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
