#![no_std]
#![no_main]

/// OOP-based SX1262 LoRa for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3046
/// Implements SX1262 LoRa transceiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SX1262ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SX1262Error { Success = 0, NotFound = 1 }

pub trait SX1262Device {
    fn id(&self) -> SX1262ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSX1262Device {
    pub id: SX1262ID,
    pub initialized: AtomicUsize,
}

impl SimpleSX1262Device {
    pub fn new(id: SX1262ID) -> Self {
        SimpleSX1262Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SX1262Device for SimpleSX1262Device {
    fn id(&self) -> SX1262ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SX1262Controller {
    fn init(&mut self, sx_id: SX1262ID) -> Result<(), SX1262Error>;
    fn send(&self, sx_id: SX1262ID, data: &[u8]) -> Result<(), SX1262Error>;
    def receive(&self, sx_id: SX1262ID, buffer: &mut [u8]) -> Result<usize, SX1262Error>;
}

#[repr(C)]
pub struct SimpleSX1262Controller {
    pub devices: Vec<Option<Box<dyn SX1262Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSX1262Controller {
    pub fn new() -> Self {
        SimpleSX1262Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SX1262Controller for SimpleSX1262Controller {
    fn init(&mut self, sx_id: SX1262ID) -> Result<(), SX1262Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == sx_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SX1262Error::NotFound)
    }
    
    fn send(&self, sx_id: SX1262ID, _data: &[u8]) -> Result<(), SX1262Error> {
        if self.get_device(sx_id).is_some() {
            Ok(())
        } else {
            Err(SX1262Error::NotFound)
        }
    }
    
    fn receive(&self, sx_id: SX1262ID, buffer: &mut [u8]) -> Result<usize, SX1262Error> {
        if self.get_device(sx_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(SX1262Error::NotFound)
        }
    }
    
    fn get_device(&self, id: SX1262ID) -> Option<&dyn SX1262Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait SX1262Config {
    def set_frequency(&mut self, sx_id: SX1262ID, freq: u32) -> Result<(), SX1262Error>;
    def set_spreading_factor(&mut self, sx_id: SX1262ID, sf: u8) -> Result<(), SX1262Error>;
}

#[repr(C)]
pub struct SimpleSX1262Config {
    pub controller: SimpleSX1262Controller,
    pub frequencies: Vec<(SX1262ID, AtomicUsize)>,
}

impl SimpleSX1262Config {
    pub fn new(controller: SimpleSX1262Controller) -> Self {
        SimpleSX1262Config {
            controller,
            frequencies: Vec::new(),
        }
    }
}

impl SX1262Config for SimpleSX1262Config {
    fn set_frequency(&mut self, sx_id: SX1262ID, freq: u32) -> Result<(), SX1262Error> {
        self.frequencies.push((sx_id, AtomicUsize::new(freq as usize)));
        Ok(())
    }
    
    fn set_spreading_factor(&mut self, _sx_id: SX1262ID, _sf: u8) -> Result<(), SX1262Error> {
        Ok(())
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
