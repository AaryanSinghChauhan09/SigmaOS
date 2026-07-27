#![no_std]
#![no_main]

/// OOP-based L298N DC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3536
/// Implements L298N DC motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type L298NID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum L298NError { Success = 0, NotFound = 1 }

pub trait L298NMotor {
    fn id(&self) -> L298NID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleL298NMotor {
    pub id: L298NID,
    pub initialized: AtomicUsize,
}

impl SimpleL298NMotor {
    pub fn new(id: L298NID) -> Self {
        SimpleL298NMotor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl L298NMotor for SimpleL298NMotor {
    fn id(&self) -> L298NID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait L298NController {
    fn init(&mut self, motor_id: L298NID) -> Result<(), L298NError>;
    fn set_speed(&self, motor_id: L298NID, speed: u8) -> Result<(), L298NError>;
    def set_direction(&self, motor_id: L298NID, forward: bool) -> Result<(), L298NError>;
}

#[repr(C)]
pub struct SimpleL298NController {
    pub motors: Vec<Option<Box<dyn L298NMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleL298NController {
    pub fn new() -> Self {
        SimpleL298NController {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl L298NController for SimpleL298NController {
    fn init(&mut self, motor_id: L298NID) -> Result<(), L298NError> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(L298NError::NotFound)
    }
    
    fn set_speed(&self, motor_id: L298NID, _speed: u8) -> Result<(), L298NError> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(L298NError::NotFound)
        }
    }
    
    fn set_direction(&self, motor_id: L298NID, _forward: bool) -> Result<(), L298NError> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(L298NError::NotFound)
        }
    }
    
    fn get_motor(&self, id: L298NID) -> Option<&dyn L298NMotor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait L298NStop {
    def stop(&self, motor_id: L298NID) -> Result<(), L298NError>;
}

#[repr(C)]
pub struct SimpleL298NStop {
    pub controller: SimpleL298NController,
}

impl SimpleL298NStop {
    pub fn new(controller: SimpleL298NController) -> Self {
        SimpleL298NStop { controller }
    }
}

impl L298NStop for SimpleL298NStop {
    fn stop(&self, motor_id: L298NID) -> Result<(), L298NError> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(L298NError::NotFound)
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
