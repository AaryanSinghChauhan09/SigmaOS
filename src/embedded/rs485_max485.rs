#![no_std]
#![no_main]

/// OOP-based MAX485 RS485 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3086
/// Implements MAX485 RS485 transceiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MAX485ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MAX485Error { Success = 0, NotFound = 1 }

pub trait MAX485Device {
    fn id(&self) -> MAX485ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMAX485Device {
    pub id: MAX485ID,
    pub initialized: AtomicUsize,
}

impl SimpleMAX485Device {
    pub fn new(id: MAX485ID) -> Self {
        SimpleMAX485Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MAX485Device for SimpleMAX485Device {
    fn id(&self) -> MAX485ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MAX485Controller {
    fn init(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error>;
    fn send(&self, max_id: MAX485ID, data: &[u8]) -> Result<usize, MAX485Error>;
    def receive(&self, max_id: MAX485ID, buffer: &mut [u8]) -> Result<usize, MAX485Error>;
}

#[repr(C)]
pub struct SimpleMAX485Controller {
    pub devices: Vec<Option<Box<dyn MAX485Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMAX485Controller {
    pub fn new() -> Self {
        SimpleMAX485Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MAX485Controller for SimpleMAX485Controller {
    fn init(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == max_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MAX485Error::NotFound)
    }
    
    fn send(&self, max_id: MAX485ID, data: &[u8]) -> Result<usize, MAX485Error> {
        if self.get_device(max_id).is_some() {
            Ok(data.len())
        } else {
            Err(MAX485Error::NotFound)
        }
    }
    
    fn receive(&self, max_id: MAX485ID, buffer: &mut [u8]) -> Result<usize, MAX485Error> {
        if self.get_device(max_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(MAX485Error::NotFound)
        }
    }
    
    fn get_device(&self, id: MAX485ID) -> Option<&dyn MAX485Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait MAX485Mode {
    def set_tx_mode(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error>;
    def set_rx_mode(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error>;
}

#[repr(C)]
pub struct SimpleMAX485Mode {
    pub controller: SimpleMAX485Controller,
    pub modes: Vec<(MAX485ID, AtomicUsize)>,
}

impl SimpleMAX485Mode {
    pub fn new(controller: SimpleMAX485Controller) -> Self {
        SimpleMAX485Mode {
            controller,
            modes: Vec::new(),
        }
    }
}

impl MAX485Mode for SimpleMAX485Mode {
    fn set_tx_mode(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error> {
        self.modes.push((max_id, AtomicUsize::new(1)));
        Ok(())
    }
    
    fn set_rx_mode(&mut self, max_id: MAX485ID) -> Result<(), MAX485Error> {
        self.modes.push((max_id, AtomicUsize::new(0)));
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
