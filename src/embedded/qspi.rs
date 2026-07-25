#![no_std]
#![no_main]

/// OOP-based QSPI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2046
/// Implements QSPI interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type QSPIID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum QSPIError { Success = 0, NotFound = 1 }

pub trait QSPIDevice {
    fn id(&self) -> QSPIID;
    fn is_busy(&self) -> bool;
}

#[repr(C)]
pub struct SimpleQSPIDevice {
    pub id: QSPIID,
    pub busy: AtomicUsize,
}

impl SimpleQSPIDevice {
    pub fn new(id: QSPIID) -> Self {
        SimpleQSPIDevice {
            id,
            busy: AtomicUsize::new(0),
        }
    }
}

impl QSPIDevice for SimpleQSPIDevice {
    fn id(&self) -> QSPIID { self.id }
    fn is_busy(&self) -> bool { self.busy.load(Ordering::SeqCst) == 1 }
}

pub trait QSPIController {
    fn init(&mut self, qspi_id: QSPIID) -> Result<(), QSPIError>;
    fn read(&self, qspi_id: QSPIID, address: u32, buffer: &mut [u8]) -> Result<(), QSPIError>;
    def write(&self, qspi_id: QSPIID, address: u32, buffer: &[u8]) -> Result<(), QSPIError>;
}

#[repr(C)]
pub struct SimpleQSPIController {
    pub devices: Vec<Option<Box<dyn QSPIDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleQSPIController {
    pub fn new() -> Self {
        SimpleQSPIController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl QSPIController for SimpleQSPIController {
    fn init(&mut self, _qspi_id: QSPIID) -> Result<(), QSPIError> {
        Ok(())
    }
    
    fn read(&self, qspi_id: QSPIID, _address: u32, buffer: &mut [u8]) -> Result<(), QSPIError> {
        if self.get_device(qspi_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(QSPIError::NotFound)
        }
    }
    
    fn write(&self, qspi_id: QSPIID, _address: u32, _buffer: &[u8]) -> Result<(), QSPIError> {
        if self.get_device(qspi_id).is_some() {
            Ok(())
        } else {
            Err(QSPIError::NotFound)
        }
    }
    
    fn get_device(&self, id: QSPIID) -> Option<&dyn QSPIDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait QSPIMemoryMapped {
    def enable_memory_mapped(&mut self, qspi_id: QSPIID) -> Result<(), QSPIError>;
    def get_base_address(&self, qspi_id: QSPIID) -> Result<usize, QSPIError>;
}

#[repr(C)]
pub struct SimpleQSPIMemoryMapped {
    pub controller: SimpleQSPIController,
    pub base_addresses: Vec<(QSPIID, AtomicUsize)>,
}

impl SimpleQSPIMemoryMapped {
    pub fn new(controller: SimpleQSPIController) -> Self {
        SimpleQSPIMemoryMapped {
            controller,
            base_addresses: Vec::new(),
        }
    }
}

impl QSPIMemoryMapped for SimpleQSPIMemoryMapped {
    fn enable_memory_mapped(&mut self, qspi_id: QSPIID) -> Result<(), QSPIError> {
        self.base_addresses.push((qspi_id, AtomicUsize::new(0x90000000)));
        Ok(())
    }
    
    fn get_base_address(&self, qspi_id: QSPIID) -> Result<usize, QSPIError> {
        for &(id, ref addr) in &self.base_addresses {
            if id == qspi_id {
                return Ok(addr.load(Ordering::SeqCst));
            }
        }
        Err(QSPIError::NotFound)
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
