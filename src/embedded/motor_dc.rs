#![no_std]
#![no_main]

/// OOP-based DC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2686
/// Implements DC motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DCMotorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DCMotorError { Success = 0, NotFound = 1 }

pub trait DCMotor {
    fn id(&self) -> DCMotorID;
    fn is_running(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDCMotor {
    pub id: DCMotorID,
    pub running: AtomicUsize,
}

impl SimpleDCMotor {
    pub fn new(id: DCMotorID) -> Self {
        SimpleDCMotor {
            id,
            running: AtomicUsize::new(0),
        }
    }
}

impl DCMotor for SimpleDCMotor {
    fn id(&self) -> DCMotorID { self.id }
    fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) == 1 }
}

pub trait DCMotorController {
    fn set_speed(&self, motor_id: DCMotorID, speed: i16) -> Result<(), DCMotorError>;
    fn stop(&self, motor_id: DCMotorID) -> Result<(), DCMotorError>;
    def brake(&self, motor_id: DCMotorID) -> Result<(), DCMotorError>;
}

#[repr(C)]
pub struct SimpleDCMotorController {
    pub motors: Vec<Option<Box<dyn DCMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDCMotorController {
    pub fn new() -> Self {
        SimpleDCMotorController {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DCMotorController for SimpleDCMotorController {
    fn set_speed(&self, motor_id: DCMotorID, speed: i16) -> Result<(), DCMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    if speed != 0 {
                        motor.running.store(1, Ordering::SeqCst);
                    } else {
                        motor.running.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(DCMotorError::NotFound)
    }
    
    fn stop(&self, motor_id: DCMotorID) -> Result<(), DCMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DCMotorError::NotFound)
    }
    
    fn brake(&self, motor_id: DCMotorID) -> Result<(), DCMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DCMotorError::NotFound)
    }
    
    fn get_motor(&self, id: DCMotorID) -> Option<&dyn DCMotor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait DCMotorDirection {
    def set_direction(&mut self, motor_id: DCMotorID, forward: bool) -> Result<(), DCMotorError>;
    def get_direction(&self, motor_id: DCMotorID) -> Result<bool, DCMotorError>;
}

#[repr(C)]
pub struct SimpleDCMotorDirection {
    pub controller: SimpleDCMotorController,
    pub directions: Vec<(DCMotorID, AtomicUsize)>,
}

impl SimpleDCMotorDirection {
    pub fn new(controller: SimpleDCMotorController) -> Self {
        SimpleDCMotorDirection {
            controller,
            directions: Vec::new(),
        }
    }
}

impl DCMotorDirection for SimpleDCMotorDirection {
    fn set_direction(&mut self, motor_id: DCMotorID, forward: bool) -> Result<(), DCMotorError> {
        self.directions.push((motor_id, AtomicUsize::new(if forward { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_direction(&self, motor_id: DCMotorID) -> Result<bool, DCMotorError> {
        for &(id, ref dir) in &self.directions {
            if id == motor_id {
                return Ok(dir.load(Ordering::SeqCst) == 1);
            }
        }
        Err(DCMotorError::NotFound)
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
