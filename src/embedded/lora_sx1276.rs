#![no_std]
#![no_main]

/// OOP-based SX1276 LoRa for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3036
/// Implements SX1276 LoRa transceiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SX1276ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SX1276Error { Success = 0, NotFound = 1 }

pub trait SX1276Device {
    fn id(&self) -> SX1276ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSX1276Device {
    pub id: SX1276ID,
    pub initialized: AtomicUsize,
}

impl SimpleSX1276Device {
    pub fn new(id: SX1276ID) -> Self {
        SimpleSX1276Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SX1276Device for SimpleSX1276Device {
    fn id(&self) -> SX1276ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SX1276Controller {
    fn init(&mut self, sx_id: SX1276ID) -> Result<(), SX1276Error>;
    fn send(&self, sx_id: SX1276ID, data: &[u8]) -> Result<(), SX1276Error>;
    def receive(&self, sx_id: SX1276ID, buffer: &mut [u8]) -> Result<usize, SX1276Error>;
}

#[repr(C)]
pub struct SimpleSX1276Controller {
    pub devices: Vec<Option<Box<dyn SX1276Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSX1276Controller {
    pub fn new() -> Self {
        SimpleSX1276Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SX1276Controller for SimpleSX1276Controller {
    fn init(&mut self, sx_id: SX1276ID) -> Result<(), SX1276Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == sx_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SX1276Error::NotFound)
    }
    
    fn send(&self, sx_id: SX1276ID, _data: &[u8]) -> Result<(), SX1276Error> {
        if self.get_device(sx_id).is_some() {
            Ok(())
        } else {
            Err(SX1276Error::NotFound)
        }
    }
    
    fn receive(&self, sx_id: SX1276ID, buffer: &mut [u8]) -> Result<usize, SX1276Error> {
        if self.get_device(sx_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(SX1276Error::NotFound)
        }
    }
    
    fn get_device(&self, id: SX1276ID) -> Option<&dyn SX1276Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait SX1276Config {
    def set_frequency(&mut self, sx_id: SX1276ID, freq: u32) -> Result<(), SX1276Error>;
    def set_spreading_factor(&mut self, sx_id: SX1276ID, sf: u8) -> Result<(), SX1276Error>;
}

#[repr(C)]
pub struct SimpleSX1276Config {
    pub controller: SimpleSX1276Controller,
    pub frequencies: Vec<(SX1276ID, AtomicUsize)>,
}

impl SimpleSX1276Config {
    pub fn new(controller: SimpleSX1276Controller) -> Self {
        SimpleSX1276Config {
            controller,
            frequencies: Vec::new(),
        }
    }
}

impl SX1276Config for SimpleSX1276Config {
    fn set_frequency(&mut self, sx_id: SX1276ID, freq: u32) -> Result<(), SX1276Error> {
        self.frequencies.push((sx_id, AtomicUsize::new(freq as usize)));
        Ok(())
    }
    
    fn set_spreading_factor(&mut self, _sx_id: SX1276ID, _sf: u8) -> Result<(), SX1276Error> {
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
