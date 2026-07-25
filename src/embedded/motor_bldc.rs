#![no_std]
#![no_main]

/// OOP-based BLDC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2706
/// Implements BLDC motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BLDCMotorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BLDCMotorError { Success = 0, NotFound = 1 }

pub trait BLDCMotor {
    fn id(&self) -> BLDCMotorID;
    fn is_running(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBLDCMotor {
    pub id: BLDCMotorID,
    pub running: AtomicUsize,
}

impl SimpleBLDCMotor {
    pub fn new(id: BLDCMotorID) -> Self {
        SimpleBLDCMotor {
            id,
            running: AtomicUsize::new(0),
        }
    }
}

impl BLDCMotor for SimpleBLDCMotor {
    fn id(&self) -> BLDCMotorID { self.id }
    fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) == 1 }
}

pub trait BLDCController {
    fn set_speed(&self, motor_id: BLDCMotorID, duty: u16) -> Result<(), BLDCMotorError>;
    fn stop(&self, motor_id: BLDCMotorID) -> Result<(), BLDCMotorError>;
    def set_direction(&mut self, motor_id: BLDCMotorID, forward: bool) -> Result<(), BLDCMotorError>;
}

#[repr(C)]
pub struct SimpleBLDCController {
    pub motors: Vec<Option<Box<dyn BLDCMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBLDCController {
    pub fn new() -> Self {
        SimpleBLDCController {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BLDCController for SimpleBLDCController {
    fn set_speed(&self, motor_id: BLDCMotorID, duty: u16) -> Result<(), BLDCMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    if duty > 0 {
                        motor.running.store(1, Ordering::SeqCst);
                    } else {
                        motor.running.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(BLDCMotorError::NotFound)
    }
    
    fn stop(&self, motor_id: BLDCMotorID) -> Result<(), BLDCMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BLDCMotorError::NotFound)
    }
    
    fn set_direction(&mut self, _motor_id: BLDCMotorID, _forward: bool) -> Result<(), BLDCMotorError> {
        Ok(())
    }
    
    fn get_motor(&self, id: BLDCMotorID) -> Option<&dyn BLDCMotor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait BLDCCommutation {
    def set_commutation(&mut self, motor_id: BLDCMotorID, mode: u8) -> Result<(), BLDCMotorError>;
    def get_commutation(&self, motor_id: BLDCMotorID) -> Result<u8, BLDCMotorError>;
}

#[repr(C)]
pub struct SimpleBLDCCommutation {
    pub controller: SimpleBLDCController,
    pub commutations: Vec<(BLDCMotorID, AtomicUsize)>,
}

impl SimpleBLDCCommutation {
    pub fn new(controller: SimpleBLDCController) -> Self {
        SimpleBLDCCommutation {
            controller,
            commutations: Vec::new(),
        }
    }
}

impl BLDCCommutation for SimpleBLDCCommutation {
    fn set_commutation(&mut self, motor_id: BLDCMotorID, mode: u8) -> Result<(), BLDCMotorError> {
        self.commutations.push((motor_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_commutation(&self, motor_id: BLDCMotorID) -> Result<u8, BLDCMotorError> {
        for &(id, ref mode) in &self.commutations {
            if id == motor_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(BLDCMotorError::NotFound)
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
