#![no_std]
#![no_main]

/// OOP-based ENC28J60 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2996
/// Implements ENC28J60 Ethernet controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ENC28J60ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ENC28J60Error { Success = 0, NotFound = 1 }

pub trait ENC28J60Device {
    fn id(&self) -> ENC28J60ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleENC28J60Device {
    pub id: ENC28J60ID,
    pub initialized: AtomicUsize,
}

impl SimpleENC28J60Device {
    pub fn new(id: ENC28J60ID) -> Self {
        SimpleENC28J60Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ENC28J60Device for SimpleENC28J60Device {
    fn id(&self) -> ENC28J60ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ENC28J60Controller {
    fn init(&mut self, enc_id: ENC28J60ID) -> Result<(), ENC28J60Error>;
    fn read_packet(&self, enc_id: ENC28J60ID, buffer: &mut [u8]) -> Result<usize, ENC28J60Error>;
    def write_packet(&self, enc_id: ENC28J60ID, data: &[u8]) -> Result<usize, ENC28J60Error>;
}

#[repr(C)]
pub struct SimpleENC28J60Controller {
    pub devices: Vec<Option<Box<dyn ENC28J60Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleENC28J60Controller {
    pub fn new() -> Self {
        SimpleENC28J60Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ENC28J60Controller for SimpleENC28J60Controller {
    fn init(&mut self, enc_id: ENC28J60ID) -> Result<(), ENC28J60Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == enc_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ENC28J60Error::NotFound)
    }
    
    fn read_packet(&self, enc_id: ENC28J60ID, buffer: &mut [u8]) -> Result<usize, ENC28J60Error> {
        if self.get_device(enc_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(ENC28J60Error::NotFound)
        }
    }
    
    fn write_packet(&self, enc_id: ENC28J60ID, data: &[u8]) -> Result<usize, ENC28J60Error> {
        if self.get_device(enc_id).is_some() {
            Ok(data.len())
        } else {
            Err(ENC28J60Error::NotFound)
        }
    }
    
    fn get_device(&self, id: ENC28J60ID) -> Option<&dyn ENC28J60Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait ENC28J60MAC {
    def set_mac(&mut self, enc_id: ENC28J60ID, mac: [u8; 6]) -> Result<(), ENC28J60Error>;
    def get_mac(&self, enc_id: ENC28J60ID) -> Result<[u8; 6], ENC28J60Error>;
}

#[repr(C)]
pub struct SimpleENC28J60MAC {
    pub controller: SimpleENC28J60Controller,
    pub macs: Vec<(ENC28J60ID, [u8; 6])>,
}

impl SimpleENC28J60MAC {
    pub fn new(controller: SimpleENC28J60Controller) -> Self {
        SimpleENC28J60MAC {
            controller,
            macs: Vec::new(),
        }
    }
}

impl ENC28J60MAC for SimpleENC28J60MAC {
    fn set_mac(&mut self, enc_id: ENC28J60ID, mac: [u8; 6]) -> Result<(), ENC28J60Error> {
        self.macs.push((enc_id, mac));
        Ok(())
    }
    
    fn get_mac(&self, enc_id: ENC28J60ID) -> Result<[u8; 6], ENC28J60Error> {
        for &(id, mac) in &self.macs {
            if id == enc_id {
                return Ok(mac);
            }
        }
        Err(ENC28J60Error::NotFound)
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
