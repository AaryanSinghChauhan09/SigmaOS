#![no_std]
#![no_main]

/// OOP-based Servo Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1396
/// Implements servo motor control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ServoID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ServoError { Success = 0, NotFound = 1 }

pub trait ServoMotor {
    fn id(&self) -> ServoID;
    fn get_angle(&self) -> f32;
}

#[repr(C)]
pub struct SimpleServoMotor {
    pub id: ServoID,
    pub angle: AtomicUsize,
}

impl SimpleServoMotor {
    pub fn new(id: ServoID) -> Self {
        SimpleServoMotor {
            id,
            angle: AtomicUsize::new(900),
        }
    }
}

impl ServoMotor for SimpleServoMotor {
    fn id(&self) -> ServoID { self.id }
    fn get_angle(&self) -> f32 { (self.angle.load(Ordering::SeqCst) as f32) / 10.0 }
}

pub trait ServoController {
    fn set_angle(&mut self, servo_id: ServoID, angle: f32) -> Result<(), ServoError>;
    fn get_angle(&self, servo_id: ServoID) -> Result<f32, ServoError>;
    def detach(&mut self, servo_id: ServoID) -> Result<(), ServoError>;
}

#[repr(C)]
pub struct SimpleServoController {
    pub servos: Vec<Option<Box<dyn ServoMotor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleServoController {
    pub fn new() -> Self {
        SimpleServoController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ServoController for SimpleServoController {
    fn set_angle(&mut self, servo_id: ServoID, angle: f32) -> Result<(), ServoError> {
        let clamped_angle = angle.max(0.0).min(180.0);
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.angle.store((clamped_angle * 10.0) as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ServoError::NotFound)
    }
    
    fn get_angle(&self, servo_id: ServoID) -> Result<f32, ServoError> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == servo_id {
                    return Ok(servo.get_angle());
                }
            }
        }
        Err(ServoError::NotFound)
    }
    
    fn detach(&mut self, servo_id: ServoID) -> Result<(), ServoError> {
        for servo_option in &mut self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == servo_id {
                    return Ok(());
                }
            }
        }
        Err(ServoError::NotFound)
    }
}

pub trait ContinuousServo {
    def set_speed(&mut self, servo_id: ServoID, speed: f32) -> Result<(), ServoError>;
    def stop(&mut self, servo_id: ServoID) -> Result<(), ServoError>;
}

#[repr(C)]
pub struct SimpleContinuousServo {
    pub controller: SimpleServoController,
    pub speeds: Vec<(ServoID, AtomicUsize)>,
}

impl SimpleContinuousServo {
    pub fn new(controller: SimpleServoController) -> Self {
        SimpleContinuousServo {
            controller,
            speeds: Vec::new(),
        }
    }
}

impl ContinuousServo for SimpleContinuousServo {
    fn set_speed(&mut self, servo_id: ServoID, speed: f32) -> Result<(), ServoError> {
        let clamped_speed = speed.max(-1.0).min(1.0);
        self.speeds.push((servo_id, AtomicUsize::new((clamped_speed * 1000.0) as usize)));
        Ok(())
    }
    
    fn stop(&mut self, servo_id: ServoID) -> Result<(), ServoError> {
        self.controller.set_angle(servo_id, 90.0)
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
