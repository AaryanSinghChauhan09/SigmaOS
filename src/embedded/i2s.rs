#![no_std]
#![no_main]

/// OOP-based I2S for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2316
/// Implements I2S audio interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type I2SID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum I2SError { Success = 0, NotFound = 1 }

pub trait I2SDevice {
    fn id(&self) -> I2SID;
    fn is_active(&self) -> bool;
}

#[repr(C)]
pub struct SimpleI2SDevice {
    pub id: I2SID,
    pub active: AtomicUsize,
}

impl SimpleI2SDevice {
    pub fn new(id: I2SID) -> Self {
        SimpleI2SDevice {
            id,
            active: AtomicUsize::new(0),
        }
    }
}

impl I2SDevice for SimpleI2SDevice {
    fn id(&self) -> I2SID { self.id }
    fn is_active(&self) -> bool { self.active.load(Ordering::SeqCst) == 1 }
}

pub trait I2SController {
    fn init(&mut self, i2s_id: I2SID) -> Result<(), I2SError>;
    fn send(&self, i2s_id: I2SID, data: &[i16]) -> Result<usize, I2SError>;
    def receive(&self, i2s_id: I2SID, buffer: &mut [i16]) -> Result<usize, I2SError>;
}

#[repr(C)]
pub struct SimpleI2SController {
    pub devices: Vec<Option<Box<dyn I2SDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleI2SController {
    pub fn new() -> Self {
        SimpleI2SController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl I2SController for SimpleI2SController {
    fn init(&mut self, i2s_id: I2SID) -> Result<(), I2SError> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == i2s_id {
                    device.active.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(I2SError::NotFound)
    }
    
    fn send(&self, i2s_id: I2SID, _data: &[i16]) -> Result<usize, I2SError> {
        if self.get_device(i2s_id).is_some() {
            Ok(0)
        } else {
            Err(I2SError::NotFound)
        }
    }
    
    fn receive(&self, i2s_id: I2SID, buffer: &mut [i16]) -> Result<usize, I2SError> {
        if self.get_device(i2s_id).is_some() {
            for sample in buffer.iter_mut() { *sample = 0; }
            Ok(buffer.len())
        } else {
            Err(I2SError::NotFound)
        }
    }
    
    fn get_device(&self, id: I2SID) -> Option<&dyn I2SDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait I2SClock {
    def set_sample_rate(&mut self, i2s_id: I2SID, rate: u32) -> Result<(), I2SError>;
    def get_sample_rate(&self, i2s_id: I2SID) -> Result<u32, I2SError>;
}

#[repr(C)]
pub struct SimpleI2SClock {
    pub controller: SimpleI2SController,
    pub sample_rates: Vec<(I2SID, AtomicUsize)>,
}

impl SimpleI2SClock {
    pub fn new(controller: SimpleI2SController) -> Self {
        SimpleI2SClock {
            controller,
            sample_rates: Vec::new(),
        }
    }
}

impl I2SClock for SimpleI2SClock {
    fn set_sample_rate(&mut self, i2s_id: I2SID, rate: u32) -> Result<(), I2SError> {
        self.sample_rates.push((i2s_id, AtomicUsize::new(rate as usize)));
        Ok(())
    }
    
    fn get_sample_rate(&self, i2s_id: I2SID) -> Result<u32, I2SError> {
        for &(id, ref rate) in &self.sample_rates {
            if id == i2s_id {
                return Ok(rate.load(Ordering::SeqCst) as u32);
            }
        }
        Err(I2SError::NotFound)
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
