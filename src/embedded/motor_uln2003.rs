#![no_std]
#![no_main]

/// OOP-based ULN2003 Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2786
/// Implements ULN2003 stepper motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ULN2003ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ULN2003Error { Success = 0, NotFound = 1 }

pub trait ULN2003Driver {
    fn id(&self) -> ULN2003ID;
    fn is_enabled(&self) -> bool;
}

#[repr(C)]
pub struct SimpleULN2003Driver {
    pub id: ULN2003ID,
    pub enabled: AtomicUsize,
}

impl SimpleULN2003Driver {
    pub fn new(id: ULN2003ID) -> Self {
        SimpleULN2003Driver {
            id,
            enabled: AtomicUsize::new(0),
        }
    }
}

impl ULN2003Driver for SimpleULN2003Driver {
    fn id(&self) -> ULN2003ID { self.id }
    fn is_enabled(&self) -> bool { self.enabled.load(Ordering::SeqCst) == 1 }
}

pub trait ULN2003Controller {
    fn step(&self, uln_id: ULN2003ID, steps: i32) -> Result<(), ULN2003Error>;
    def set_direction(&mut self, uln_id: ULN2003ID, forward: bool) -> Result<(), ULN2003Error>;
    def set_speed(&mut self, uln_id: ULN2003ID, delay_ms: u16) -> Result<(), ULN2003Error>;
}

#[repr(C)]
pub struct SimpleULN2003Controller {
    pub drivers: Vec<Option<Box<dyn ULN2003Driver>>>,
    pub next_id: AtomicUsize,
}

impl SimpleULN2003Controller {
    pub fn new() -> Self {
        SimpleULN2003Controller {
            drivers: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ULN2003Controller for SimpleULN2003Controller {
    fn step(&self, uln_id: ULN2003ID, steps: i32) -> Result<(), ULN2003Error> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == uln_id {
                    if steps != 0 {
                        driver.enabled.store(1, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(ULN2003Error::NotFound)
    }
    
    fn set_direction(&mut self, _uln_id: ULN2003ID, _forward: bool) -> Result<(), ULN2003Error> {
        Ok(())
    }
    
    fn set_speed(&mut self, _uln_id: ULN2003ID, _delay_ms: u16) -> Result<(), ULN2003Error> {
        Ok(())
    }
    
    fn get_driver(&self, id: ULN2003ID) -> Option<&dyn ULN2003Driver> {
        for driver_option in &self.drivers {
            if let Some(ref driver) = *driver_option {
                if driver.id() == id { return Some(driver.as_ref()); }
            }
        }
        None
    }
}

pub trait ULN2003StepMode {
    def set_step_mode(&mut self, uln_id: ULN2003ID, mode: u8) -> Result<(), ULN2003Error>;
    def get_step_mode(&self, uln_id: ULN2003ID) -> Result<u8, ULN2003Error>;
}

#[repr(C)]
pub struct SimpleULN2003StepMode {
    pub controller: SimpleULN2003Controller,
    pub step_modes: Vec<(ULN2003ID, AtomicUsize)>,
}

impl SimpleULN2003StepMode {
    pub fn new(controller: SimpleULN2003Controller) -> Self {
        SimpleULN2003StepMode {
            controller,
            step_modes: Vec::new(),
        }
    }
}

impl ULN2003StepMode for SimpleULN2003StepMode {
    fn set_step_mode(&mut self, uln_id: ULN2003ID, mode: u8) -> Result<(), ULN2003Error> {
        self.step_modes.push((uln_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_step_mode(&self, uln_id: ULN2003ID) -> Result<u8, ULN2003Error> {
        for &(id, ref mode) in &self.step_modes {
            if id == uln_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(ULN2003Error::NotFound)
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
