#![no_std]
#![no_main]

/// OOP-based PCA9685 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2826
/// Implements PCA9685 PWM/Servo driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCA9685ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCA9685Error { Success = 0, NotFound = 1 }

pub trait PCA9685Driver {
    fn id(&self) -> PCA9685ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCA9685Driver {
    pub id: PCA9685ID,
    pub initialized: AtomicUsize,
}

impl SimplePCA9685Driver {
    pub fn new(id: PCA9685ID) -> Self {
        SimplePCA9685Driver {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCA9685Driver for SimplePCA9685Driver {
    fn id(&self) -> PCA9685ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCA9685Controller {
    fn init(&mut self, pca_id: PCA9685ID) -> Result<(), PCA9685Error>;
    fn set_pwm(&self, pca_id: PCA9685ID, channel: u8, on: u16, off: u16) -> Result<(), PCA9685Error>;
    def set_duty(&self, pca_id: PCA9685ID, channel: u8, duty: u16) -> Result<(), PCA9685Error>;
}

#[repr(C)]
pub struct SimplePCA9685Controller {
    pub drivers: Vec<Option<Box<dyn PCA9685Driver>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCA9685Controller {
    pub fn new() -> Self {
        SimplePCA9685Controller {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCA9685Controller for SimplePCA9685Controller {
    fn init(&mut self, pca_id: PCA9685ID) -> Result<(), PCA9685Error> {
        for driver_option in &mut self.drivers {
            if let Some(ref mut driver) = *driver_option {
                if driver.id() == pca_id {
                    driver.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCA9685Error::NotFound)
    }
    
    fn set_pwm(&self, pca_id: PCA9685ID, _channel: u8, _on: u16, _off: u16) -> Result<(), PCA9685Error> {
        if self.get_driver(pca_id).is_some() {
            Ok(())
        } else {
            Err(PCA9685Error::NotFound)
        }
    }
    
    fn set_duty(&self, pca_id: PCA9685ID, _channel: u8, _duty: u16) -> Result<(), PCA9685Error> {
        if self.get_driver(pca_id).is_some() {
            Ok(())
        } else {
            Err(PCA9685Error::NotFound)
        }
    }
    
    fn get_driver(&self, id: PCA9685ID) -> Option<&dyn PCA9685Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait PCA9685Frequency {
    def set_pwm_freq(&mut self, pca_id: PCA9685ID, freq: u16) -> Result<(), PCA9685Error>;
    def get_pwm_freq(&self, pca_id: PCA9685ID) -> Result<u16, PCA9685Error>;
}

#[repr(C)]
pub struct SimplePCA9685Frequency {
    pub controller: SimplePCA9685Controller,
    pub freqs: Vec<(PCA9685ID, AtomicUsize)>,
}

impl SimplePCA9685Frequency {
    pub fn new(controller: SimplePCA9685Controller) -> Self {
        SimplePCA9685Frequency {
            controller,
            freqs: Vec::new(),
        }
    }
}

impl PCA9685Frequency for SimplePCA9685Frequency {
    fn set_pwm_freq(&mut self, pca_id: PCA9685ID, freq: u16) -> Result<(), PCA9685Error> {
        self.freqs.push((pca_id, AtomicUsize::new(freq as usize)));
        Ok(())
    }
    
    fn get_pwm_freq(&self, pca_id: PCA9685ID) -> Result<u16, PCA9685Error> {
        for &(id, ref freq) in &self.freqs {
            if id == pca_id {
                return Ok(freq.load(Ordering::SeqCst) as u16);
            }
        }
        Err(PCA9685Error::NotFound)
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
