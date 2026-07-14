#![no_std]
#![no_main]

/// OOP-based Brushless Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2726
/// Implements generic brushless motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BrushlessMotorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BrushlessMotorError { Success = 0, NotFound = 1 }

pub trait BrushlessMotor {
    fn id(&self) -> BrushlessMotorID;
    fn is_running(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBrushlessMotor {
    pub id: BrushlessMotorID,
    pub running: AtomicUsize,
}

impl SimpleBrushlessMotor {
    pub fn new(id: BrushlessMotorID) -> Self {
        SimpleBrushlessMotor {
            id,
            running: AtomicUsize::new(0),
        }
    }
}

impl BrushlessMotor for SimpleBrushlessMotor {
    fn id(&self) -> BrushlessMotorID { self.id }
    fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) == 1 }
}

pub trait BrushlessController {
    fn set_speed(&self, motor_id: BrushlessMotorID, throttle: u8) -> Result<(), BrushlessMotorError>;
    fn stop(&self, motor_id: BrushlessMotorID) -> Result<(), BrushlessMotorError>;
    def arm(&self, motor_id: BrushlessMotorID) -> Result<(), BrushlessMotorError>;
}

#[repr(C)]
pub struct SimpleBrushlessController {
    pub motors: Vec<Option<Box<dyn BrushlessMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBrushlessController {
    pub fn new() -> Self {
        SimpleBrushlessController {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BrushlessController for SimpleBrushlessController {
    fn set_speed(&self, motor_id: BrushlessMotorID, throttle: u8) -> Result<(), BrushlessMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    if throttle > 0 {
                        motor.running.store(1, Ordering::SeqCst);
                    } else {
                        motor.running.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(BrushlessMotorError::NotFound)
    }
    
    fn stop(&self, motor_id: BrushlessMotorID) -> Result<(), BrushlessMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BrushlessMotorError::NotFound)
    }
    
    fn arm(&self, motor_id: BrushlessMotorID) -> Result<(), BrushlessMotorError> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(BrushlessMotorError::NotFound)
        }
    }
    
    fn get_motor(&self, id: BrushlessMotorID) -> Option<&dyn BrushlessMotor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait BrushlessTelemetry {
    def read_rpm(&self, motor_id: BrushlessMotorID) -> Result<u16, BrushlessMotorError>;
    def read_voltage(&self, motor_id: BrushlessMotorID) -> Result<u16, BrushlessMotorError>;
}

#[repr(C)]
pub struct SimpleBrushlessTelemetry {
    pub controller: SimpleBrushlessController,
}

impl SimpleBrushlessTelemetry {
    pub fn new(controller: SimpleBrushlessController) -> Self {
        SimpleBrushlessTelemetry { controller }
    }
}

impl BrushlessTelemetry for SimpleBrushlessTelemetry {
    fn read_rpm(&self, motor_id: BrushlessMotorID) -> Result<u16, BrushlessMotorError> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(0)
        } else {
            Err(BrushlessMotorError::NotFound)
        }
    }
    
    fn read_voltage(&self, motor_id: BrushlessMotorID) -> Result<u16, BrushlessMotorError> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(0)
        } else {
            Err(BrushlessMotorError::NotFound)
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
