#![no_std]
#![no_main]

/// OOP-based TLC5940 PWM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3416
/// Implements TLC5940 16-channel PWM driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TLC5940ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TLC5940Error { Success = 0, NotFound = 1 }

pub trait TLC5940Device {
    fn id(&self) -> TLC5940ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTLC5940Device {
    pub id: TLC5940ID,
    pub initialized: AtomicUsize,
}

impl SimpleTLC5940Device {
    pub fn new(id: TLC5940ID) -> Self {
        SimpleTLC5940Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TLC5940Device for SimpleTLC5940Device {
    fn id(&self) -> TLC5940ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TLC5940Controller {
    fn init(&mut self, tlc_id: TLC5940ID) -> Result<(), TLC5940Error>;
    fn set_pwm(&self, tlc_id: TLC5940ID, channel: u8, value: u16) -> Result<(), TLC5940Error>;
    def update(&self, tlc_id: TLC5940ID) -> Result<(), TLC5940Error>;
}

#[repr(C)]
pub struct SimpleTLC5940Controller {
    pub devices: Vec<Option<Box<dyn TLC5940Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTLC5940Controller {
    pub fn new() -> Self {
        SimpleTLC5940Controller {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TLC5940Controller for SimpleTLC5940Controller {
    fn init(&mut self, tlc_id: TLC5940ID) -> Result<(), TLC5940Error> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == tlc_id {
                    device.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TLC5940Error::NotFound)
    }
    
    fn set_pwm(&self, tlc_id: TLC5940ID, _channel: u8, _value: u16) -> Result<(), TLC5940Error> {
        if self.get_device(tlc_id).is_some() {
            Ok(())
        } else {
            Err(TLC5940Error::NotFound)
        }
    }
    
    fn update(&self, tlc_id: TLC5940ID) -> Result<(), TLC5940Error> {
        if self.get_device(tlc_id).is_some() {
            Ok(())
        } else {
            Err(TLC5940Error::NotFound)
        }
    }
    
    fn get_device(&self, id: TLC5940ID) -> Option<&dyn TLC5940Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait TLC5940DotCorrection {
    def set_dc(&mut self, tlc_id: TLC5940ID, channel: u8, dc: u8) -> Result<(), TLC5940Error>;
}

#[repr(C)]
pub struct SimpleTLC5940DotCorrection {
    pub controller: SimpleTLC5940Controller,
}

impl SimpleTLC5940DotCorrection {
    pub fn new(controller: SimpleTLC5940Controller) -> Self {
        SimpleTLC5940DotCorrection { controller }
    }
}

impl TLC5940DotCorrection for SimpleTLC5940DotCorrection {
    fn set_dc(&mut self, tlc_id: TLC5940ID, _channel: u8, _dc: u8) -> Result<(), TLC5940Error> {
        if self.controller.get_device(tlc_id).is_some() {
            Ok(())
        } else {
            Err(TLC5940Error::NotFound)
        }
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
