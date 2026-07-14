#![no_std]
#![no_main]

/// OOP-based TB6612FNG Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2756
/// Implements TB6612FNG dual motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TB6612FNGID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TB6612FNGError { Success = 0, NotFound = 1 }

pub trait TB6612FNGDriver {
    fn id(&self) -> TB6612FNGID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTB6612FNGDriver {
    pub id: TB6612FNGID,
    pub enabled: AtomicUsize,
}

impl SimpleTB6612FNGDriver {
    pub fn new(id: TB6612FNGID) -> Self {
        SimpleTB6612FNGDriver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl TB6612FNGDriver for SimpleTB6612FNGDriver {
    fn id(&self) -> TB6612FNGID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait TB6612FNGController {
    fn set_speed(&self, tb_id: TB6612FNGID, motor: u8, speed: u8) -> Result<(), TB6612FNGError>;
    fn set_direction(&self, tb_id: TB6612FNGID, motor: u8, forward: bool) -> Result<(), TB6612FNGError>;
    def stop(&self, tb_id: TB6612FNGID, motor: u8) -> Result<(), TB6612FNGError>;
}

#[repr(C)]
pub struct SimpleTB6612FNGController {
    pub drivers: Vec<Option<Box<dyn TB6612FNGDriver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTB6612FNGController {
    pub fn new() -> Self {
        SimpleTB6612FNGController {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TB6612FNGController for SimpleTB6612FNGController {
    fn set_speed(&self, tb_id: TB6612FNGID, _motor: u8, speed: u8) -> Result<(), TB6612FNGError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == tb_id {
                    if speed > 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    } else {
                        driver.enabled.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(TB6612FNGError::NotFound)
    }
    
    fn set_direction(&self, tb_id: TB6612FNGID, _motor: u8, _forward: bool) -> Result<(), TB6612FNGError> {
        if self.get_driver(tb_id).is_some() {
            Ok(())
        } else {
            Err(TB6612FNGError::NotFound)
        }
    }
    
    fn stop(&self, tb_id: TB6612FNGID, _motor: u8) -> Result<(), TB6612FNGError> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == tb_id {
                    driver.enabled.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TB6612FNGError::NotFound)
    }
    
    fn get_driver(&self, id: TB6612FNGID) -> Option<&dyn TB6612FNGDriver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait TB6612FNGStandby {
    def set_standby(&mut self, tb_id: TB6612FNGID, standby: bool) -> Result<(), TB6612FNGError>;
    def get_standby(&self, tb_id: TB6612FNGID) -> Result<bool, TB6612FNGError>;
}

#[repr(C)]
pub struct SimpleTB6612FNGStandby {
    pub controller: SimpleTB6612FNGController,
    pub standby_states: Vec<(TB6612FNGID, AtomicUsize)>,
}

impl SimpleTB6612FNGStandby {
    pub fn new(controller: SimpleTB6612FNGController) -> Self {
        SimpleTB6612FNGStandby {
            controller,
            standby_states: Vec::new(),
        }
    }
}

impl TB6612FNGStandby for SimpleTB6612FNGStandby {
    fn set_standby(&mut self, tb_id: TB6612FNGID, standby: bool) -> Result<(), TB6612FNGError> {
        self.standby_states.push((tb_id, AtomicUsize::new(if standby { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_standby(&self, tb_id: TB6612FNGID) -> Result<bool, TB6612FNGError> {
        for &(id, ref state) in &self.standby_states {
            if id == tb_id {
                return Ok(state.load(Ordering::SeqCst) == 1);
            }
        }
        Err(TB6612FNGError::NotFound)
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
