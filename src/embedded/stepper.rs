#![no_std]
#![no_main]

/// OOP-based Stepper Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1386
/// Implements stepper motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MotorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum StepMode { Full = 0, Half = 1, Quarter = 2, Eighth = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MotorError { Success = 0, NotFound = 1 }

pub trait StepperMotor {
    fn id(&self) -> MotorID;
    fn get_position(&self) -> i32;
    fn is_moving(&self) -> bool;
}

#[repr(C)]
pub struct SimpleStepperMotor {
    pub id: MotorID,
    pub position: AtomicUsize,
    pub moving: AtomicUsize,
}

impl SimpleStepperMotor {
    pub fn new(id: MotorID) -> Self {
        SimpleStepperMotor {
            id,
            position: AtomicUsize::new(0),
            moving: AtomicUsize::new(0),
        }
    }
}

impl StepperMotor for SimpleStepperMotor {
    fn id(&self) -> MotorID { self.id }
    fn get_position(&self) -> i32 { self.position.load(Ordering::SeqCst) as i32 }
    fn is_moving(&self) -> bool { self.moving.load(Ordering::SeqCst) == 1 }
}

pub trait StepperController {
    fn step(&mut self, motor_id: MotorID, steps: i32, mode: StepMode) -> Result<(), MotorError>;
    fn set_speed(&mut self, motor_id: MotorID, speed: u32) -> Result<(), MotorError>;
    fn stop(&mut self, motor_id: MotorID) -> Result<(), MotorError>;
}

#[repr(C)]
pub struct SimpleStepperController {
    pub motors: Vec<Option<Box<dyn StepperMotor>>>,
    pub speeds: Vec<(MotorID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleStepperController {
    pub fn new() -> Self {
        SimpleStepperController {
            motors: Vec::new(),
            speeds: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl StepperController for SimpleStepperController {
    fn step(&mut self, motor_id: MotorID, steps: i32, _mode: StepMode) -> Result<(), MotorError> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    let current = motor.get_position();
                    motor.position.store((current + steps) as usize, Ordering::SeqCst);
                    motor.moving.store(1, Ordering::SeqCst);
                    motor.moving.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MotorError::NotFound)
    }
    
    fn set_speed(&mut self, motor_id: MotorID, speed: u32) -> Result<(), MotorError> {
        for &mut (id, ref speed_atomic) in &mut self.speeds {
            if id == motor_id {
                speed_atomic.store(speed as usize, Ordering::SeqCst);
                return Ok(());
            }
        }
        self.speeds.push((motor_id, AtomicUsize::new(speed as usize)));
        Ok(())
    }
    
    fn stop(&mut self, motor_id: MotorID) -> Result<(), MotorError> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.moving.store(0, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MotorError::NotFound)
    }
}

pub trait Homing {
    def home(&mut self, motor_id: MotorID) -> Result<(), MotorError>;
    def is_homed(&self, motor_id: MotorID) -> bool;
}

#[repr(C)]
pub struct SimpleHoming {
    pub controller: SimpleStepperController,
    pub homed: Vec<(MotorID, AtomicUsize)>,
}

impl SimpleHoming {
    pub fn new(controller: SimpleStepperController) -> Self {
        SimpleHoming {
            controller,
            homed: Vec::new(),
        }
    }
}

impl Homing for SimpleHoming {
    fn home(&mut self, motor_id: MotorID) -> Result<(), MotorError> {
        self.homed.push((motor_id, AtomicUsize::new(1)));
        Ok(())
    }
    
    fn is_homed(&self, motor_id: MotorID) -> bool {
        for &(id, ref homed) in &self.homed {
            if id == motor_id {
                return homed.load(Ordering::SeqCst) == 1;
            }
        }
        false
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
