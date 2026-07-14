#![no_std]
#![no_main]

/// OOP-based DRV8825 DC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3556
/// Implements DRV8825 stepper motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DRV8825ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DRV8825Error { Success = 0, NotFound = 1 }

pub trait DRV8825Motor {
    fn id(&self) -> DRV8825ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDRV8825Motor {
    pub id: DRV8825ID,
    pub initialized: AtomicUsize,
}

impl SimpleDRV8825Motor {
    pub fn new(id: DRV8825ID) -> Self {
        SimpleDRV8825Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DRV8825Motor for SimpleDRV8825Motor {
    fn id(&self) -> DRV8825ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DRV8825Controller {
    fn init(&mut self, motor_id: DRV8825ID) -> Result<(), DRV8825Error>;
    fn step(&self, motor_id: DRV8825ID, steps: i32) -> Result<(), DRV8825Error>;
    def set_speed(&mut self, motor_id: DRV8825ID, rpm: u16) -> Result<(), DRV8825Error>;
}

#[repr(C)]
pub struct SimpleDRV8825Controller {
    pub motors: Vec<Option<Box<dyn DRV8825Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDRV8825Controller {
    pub fn new() -> Self {
        SimpleDRV8825Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DRV8825Controller for SimpleDRV8825Controller {
    fn init(&mut self, motor_id: DRV8825ID) -> Result<(), DRV8825Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DRV8825Error::NotFound)
    }
    
    fn step(&self, motor_id: DRV8825ID, _steps: i32) -> Result<(), DRV8825Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(DRV8825Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: DRV8825ID, _rpm: u16) -> Result<(), DRV8825Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(DRV8825Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: DRV8825ID) -> Option<&dyn DRV8825Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait DRV8825Microstep {
    def set_microstep(&mut self, motor_id: DRV8825ID, mode: u8) -> Result<(), DRV8825Error>;
}

#[repr(C)]
pub struct SimpleDRV8825Microstep {
    pub controller: SimpleDRV8825Controller,
    pub microsteps: Vec<(DRV8825ID, AtomicUsize)>,
}

impl SimpleDRV8825Microstep {
    pub fn new(controller: SimpleDRV8825Controller) -> Self {
        SimpleDRV8825Microstep {
            controller,
            microsteps: Vec::new(),
        }
    }
}

impl DRV8825Microstep for SimpleDRV8825Microstep {
    fn set_microstep(&mut self, motor_id: DRV8825ID, mode: u8) -> Result<(), DRV8825Error> {
        self.microsteps.push((motor_id, AtomicUsize::new(mode as usize)));
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
