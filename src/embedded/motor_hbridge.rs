#![no_std]
#![no_main]

/// OOP-based H-Bridge Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2736
/// Implements H-bridge motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HBridgeID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HBridgeError { Success = 0, NotFound = 1 }

pub trait HBridgeDriver {
    fn id(&self) -> HBridgeID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHBridgeDriver {
    pub id: HBridgeID,
    pub enabled: AtomicUsize,
}

impl SimpleHBridgeDriver {
    pub fn new(id: HBridgeID) -> Self {
        SimpleHBridgeDriver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl HBridgeDriver for SimpleHBridgeDriver {
    fn id(&self) -> HBridgeID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait HBridgeController {
    fn set_duty(&self, hb_id: HBridgeID, duty: u8) -> Result<(), HBridgeError>;
    fn set_direction(&self, hb_id: HBridgeID, forward: bool) -> Result<(), HBridgeError>;
    def brake(&self, hb_id: HBridgeID) -> Result<(), HBridgeError>;
}

#[repr(C)]
pub struct SimpleHBridgeController {
    pub drivers: Vec<Option<Box<dyn HBridgeDriver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHBridgeController {
    pub fn new() -> Self {
        SimpleHBridgeController {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HBridgeController for SimpleHBridgeController {
    fn set_duty(&self, hb_id: HBridgeID, duty: u8) -> Result<(), HBridgeError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == hb_id {
                    if duty > 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    } else {
                        driver.enabled.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(HBridgeError::NotFound)
    }
    
    fn set_direction(&self, hb_id: HBridgeID, _forward: bool) -> Result<(), HBridgeError> {
        if self.get_driver(hb_id).is_some() {
            Ok(())
        } else {
            Err(HBridgeError::NotFound)
        }
    }
    
    fn brake(&self, hb_id: HBridgeID) -> Result<(), HBridgeError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == hb_id {
                    driver.enabled.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HBridgeError::NotFound)
    }
    
    fn get_driver(&self, id: HBridgeID) -> Option<&dyn HBridgeDriver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait HBridgePWM {
    def set_pwm_freq(&mut self, hb_id: HBridgeID, freq: u32) -> Result<(), HBridgeError>;
    def get_pwm_freq(&self, hb_id: HBridgeID) -> Result<u32, HBridgeError>;
}

#[repr(C)]
pub struct SimpleHBridgePWM {
    pub controller: SimpleHBridgeController,
    pub freqs: Vec<(HBridgeID, AtomicUsize)>,
}

impl SimpleHBridgePWM {
    pub fn new(controller: SimpleHBridgeController) -> Self {
        SimpleHBridgePWM {
            controller,
            freqs: Vec::new(),
        }
    }
}

impl HBridgePWM for SimpleHBridgePWM {
    fn set_pwm_freq(&mut self, hb_id: HBridgeID, freq: u32) -> Result<(), HBridgeError> {
        self.freqs.push((hb_id, AtomicUsize::new(freq as usize)));
        Ok(())
    }
    
    fn get_pwm_freq(&self, hb_id: HBridgeID) -> Result<u32, HBridgeError> {
        for &(id, ref freq) in &self.freqs {
            if id == hb_id {
                return Ok(freq.load(Ordering::SeqCst) as u32);
            }
        }
        Err(HBridgeError::NotFound)
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
