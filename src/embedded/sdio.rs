#![no_std]
#![no_main]

/// OOP-based SDIO for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2026
/// Implements SDIO interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SDIOID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SDIOError { Success = 0, NotFound = 1 }

pub trait SDIODevice {
    fn id(&self) -> SDIOID;
    fn is_connected(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSDIODevice {
    pub id: SDIOID,
    pub connected: AtomicUsize,
}

impl SimpleSDIODevice {
    pub fn new(id: SDIOID) -> Self {
        SimpleSDIODevice {
            id,
            connected: AtomicUsize::new(0),
        }
    }
}

impl SDIODevice for SimpleSDIODevice {
    fn id(&self) -> SDIOID { self.id }
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
}

pub trait SDIOController {
    fn init(&mut self, sdio_id: SDIOID) -> Result<(), SDIOError>;
    fn read_block(&self, sdio_id: SDIOID, block: u32, buffer: &mut [u8]) -> Result<(), SDIOError>;
    def write_block(&self, sdio_id: SDIOID, block: u32, buffer: &[u8]) -> Result<(), SDIOError>;
}

#[repr(C)]
pub struct SimpleSDIOController {
    pub devices: Vec<Option<Box<dyn SDIODevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSDIOController {
    pub fn new() -> Self {
        SimpleSDIOController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SDIOController for SimpleSDIOController {
    fn init(&mut self, sdio_id: SDIOID) -> Result<(), SDIOError> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == sdio_id {
                    device.connected.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SDIOError::NotFound)
    }
    
    fn read_block(&self, sdio_id: SDIOID, _block: u32, buffer: &mut [u8]) -> Result<(), SDIOError> {
        if self.get_device(sdio_id).is_some() {
            for byte in buffer.iter_mut() {
                *byte = 0;
            }
            Ok(())
        } else {
            Err(SDIOError::NotFound)
        }
    }
    
    fn write_block(&self, sdio_id: SDIOID, _block: u32, _buffer: &[u8]) -> Result<(), SDIOError> {
        if self.get_device(sdio_id).is_some() {
            Ok(())
        } else {
            Err(SDIOError::NotFound)
        }
    }
    
    fn get_device(&self, id: SDIOID) -> Option<&dyn SDIODevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait SDIOClock {
    def set_clock(&mut self, sdio_id: SDIOID, frequency: u32) -> Result<(), SDIOError>;
    def get_clock(&self, sdio_id: SDIOID) -> Result<u32, SDIOError>;
}

#[repr(C)]
pub struct SimpleSDIOClock {
    pub controller: SimpleSDIOController,
    pub clocks: Vec<(SDIOID, AtomicUsize)>,
}

impl SimpleSDIOClock {
    pub fn new(controller: SimpleSDIOController) -> Self {
        SimpleSDIOClock {
            controller,
            clocks: Vec::new(),
        }
    }
}

impl SDIOClock for SimpleSDIOClock {
    fn set_clock(&mut self, sdio_id: SDIOID, frequency: u32) -> Result<(), SDIOError> {
        self.clocks.push((sdio_id, AtomicUsize::new(frequency as usize)));
        Ok(())
    }
    
    fn get_clock(&self, sdio_id: SDIOID) -> Result<u32, SDIOError> {
        for &(id, ref clock) in &self.clocks {
            if id == sdio_id {
                return Ok(clock.load(Ordering::SeqCst) as u32);
            }
        }
        Err(SDIOError::NotFound)
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
