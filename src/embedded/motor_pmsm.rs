#![no_std]
#![no_main]

/// OOP-based PMSM Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2716
/// Implements PMSM (Permanent Magnet Synchronous Motor) control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PMSMMotorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PMSMMotorError { Success = 0, NotFound = 1 }

pub trait PMSMMotor {
    fn id(&self) -> PMSMMotorID;
    fn is_running(&self) -> bool;
}

#[repr(C)]
pub struct SimplePMSMMotor {
    pub id: PMSMMotorID,
    pub running: AtomicUsize,
}

impl SimplePMSMMotor {
    pub fn new(id: PMSMMotorID) -> Self {
        SimplePMSMMotor {
            id,
            running: AtomicUsize::new(0),
        }
    }
}

impl PMSMMotor for SimplePMSMMotor {
    fn id(&self) -> PMSMMotorID { self.id }
    fn is_running(&self) -> bool { self.running.load(Ordering::SeqCst) == 1 }
}

pub trait PMSMController {
    fn set_speed(&self, motor_id: PMSMMotorID, rpm: u16) -> Result<(), PMSMMotorError>;
    fn stop(&self, motor_id: PMSMMotorID) -> Result<(), PMSMMotorError>;
    def set_torque(&mut self, motor_id: PMSMMotorID, torque: i16) -> Result<(), PMSMMotorError>;
}

#[repr(C)]
pub struct SimplePMSMController {
    pub motors: Vec<Option<Box<dyn PMSMMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimplePMSMController {
    pub fn new() -> Self {
        SimplePMSMController {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PMSMController for SimplePMSMController {
    fn set_speed(&self, motor_id: PMSMMotorID, rpm: u16) -> Result<(), PMSMMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    if rpm > 0 {
                        motor.running.store(1, Ordering::SeqCst);
                    } else {
                        motor.running.store(0, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(PMSMMotorError::NotFound)
    }
    
    fn stop(&self, motor_id: PMSMMotorID) -> Result<(), PMSMMotorError> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.running.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PMSMMotorError::NotFound)
    }
    
    fn set_torque(&mut self, _motor_id: PMSMMotorID, _torque: i16) -> Result<(), PMSMMotorError> {
        Ok(())
    }
    
    fn get_motor(&self, id: PMSMMotorID) -> Option<&dyn PMSMMotor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait PMSMFOC {
    def set_foc_mode(&mut self, motor_id: PMSMMotorID, enable: bool) -> Result<(), PMSMMotorError>;
    def get_foc_mode(&self, motor_id: PMSMMotorID) -> Result<bool, PMSMMotorError>;
}

#[repr(C)]
pub struct SimplePMSMFOC {
    pub controller: SimplePMSMController,
    pub foc_modes: Vec<(PMSMMotorID, AtomicUsize)>,
}

impl SimplePMSMFOC {
    pub fn new(controller: SimplePMSMController) -> Self {
        SimplePMSMFOC {
            controller,
            foc_modes: Vec::new(),
        }
    }
}

impl PMSMFOC for SimplePMSMFOC {
    fn set_foc_mode(&mut self, motor_id: PMSMMotorID, enable: bool) -> Result<(), PMSMMotorError> {
        self.foc_modes.push((motor_id, AtomicUsize::new(if enable { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_foc_mode(&self, motor_id: PMSMMotorID) -> Result<bool, PMSMMotorError> {
        for &(id, ref mode) in &self.foc_modes {
            if id == motor_id {
                return Ok(mode.load(Ordering::SeqCst) == 1);
            }
        }
        Err(PMSMMotorError::NotFound)
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
