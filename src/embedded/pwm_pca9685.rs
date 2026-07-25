#![no_std]
#![no_main]

/// OOP-based PCA9685 PWM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3406
/// Implements PCA9685 16-channel PWM driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCA9685ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCA9685Error { Success = 0, NotFound = 1 }

pub trait PCA9685Device {
    fn id(&self) -> PCA9685ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCA9685Device {
    pub id: PCA9685ID,
    pub initialized: AtomicUsize,
}

impl SimplePCA9685Device {
    pub fn new(id: PCA9685ID) -> Self {
        SimplePCA9685Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCA9685Device for SimplePCA9685Device {
    fn id(&self) -> PCA9685ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCA9685Controller {
    fn init(&mut self, pca_id: PCA9685ID) -> Result<(), PCA9685Error>;
    fn set_pwm(&self, pca_id: PCA9685ID, channel: u8, on: u16, off: u16) -> Result<(), PCA9685Error>;
    def set_duty(&self, pca_id: PCA9685ID, channel: u8, duty: f32) -> Result<(), PCA9685Error>;
}

#[repr(C)]
pub struct SimplePCA9685Controller {
    pub devices: Vec<Option<Box<dyn PCA9685Device>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCA9685Controller {
    pub fn new() -> Self {
        SimplePCA9685Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCA9685Controller for SimplePCA9685Controller {
    fn init(&mut self, pca_id: PCA9685ID) -> Result<(), PCA9685Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == pca_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCA9685Error::NotFound)
    }
    
    fn set_pwm(&self, pca_id: PCA9685ID, _channel: u8, _on: u16, _off: u16) -> Result<(), PCA9685Error> {
        if self.get_device(pca_id).is_some() {
            Ok(())
        } else {
            Err(PCA9685Error::NotFound)
        }
    }
    
    fn set_duty(&self, pca_id: PCA9685ID, _channel: u8, _duty: f32) -> Result<(), PCA9685Error> {
        if self.get_device(pca_id).is_some() {
            Ok(())
        } else {
            Err(PCA9685Error::NotFound)
        }
    }
    
    fn get_device(&self, id: PCA9685ID) -> Option<&dyn PCA9685Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait PCA9685Freq {
    def set_frequency(&mut self, pca_id: PCA9685ID, freq: f32) -> Result<(), PCA9685Error>;
}

#[repr(C)]
pub struct SimplePCA9685Freq {
    pub controller: SimplePCA9685Controller,
    pub frequencies: Vec<(PCA9685ID, AtomicUsize)>,
}

impl SimplePCA9685Freq {
    pub fn new(controller: SimplePCA9685Controller) -> Self {
        SimplePCA9685Freq {
            controller,
            frequencies: Vec::new(),
        }
    }
}

impl PCA9685Freq for SimplePCA9685Freq {
    fn set_frequency(&mut self, pca_id: PCA9685ID, freq: f32) -> Result<(), PCA9685Error> {
        self.frequencies.push((pca_id, AtomicUsize::new(freq.to_bits() as usize)));
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
