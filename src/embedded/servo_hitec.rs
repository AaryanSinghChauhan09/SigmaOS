#![no_std]
#![no_main]

/// OOP-based Hitec Servo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3476
/// Implements Hitec servo motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HitecID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HitecError { Success = 0, NotFound = 1 }

pub trait HitecServo {
    fn id(&self) -> HitecID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHitecServo {
    pub id: HitecID,
    pub initialized: AtomicUsize,
}

impl SimpleHitecServo {
    pub fn new(id: HitecID) -> Self {
        SimpleHitecServo {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HitecServo for SimpleHitecServo {
    fn id(&self) -> HitecID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HitecController {
    fn init(&mut self, servo_id: HitecID) -> Result<(), HitecError>;
    fn set_angle(&self, servo_id: HitecID, angle: u8) -> Result<(), HitecError>;
    def set_pulse(&self, servo_id: HitecID, pulse: u16) -> Result<(), HitecError>;
}

#[repr(C)]
pub struct SimpleHitecController {
    pub servos: Vec<Option<Box<dyn HitecServo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHitecController {
    pub fn new() -> Self {
        SimpleHitecController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HitecController for SimpleHitecController {
    fn init(&mut self, servo_id: HitecID) -> Result<(), HitecError> {
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HitecError::NotFound)
    }
    
    fn set_angle(&self, servo_id: HitecID, _angle: u8) -> Result<(), HitecError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(HitecError::NotFound)
        }
    }
    
    fn set_pulse(&self, servo_id: HitecID, _pulse: u16) -> Result<(), HitecError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(HitecError::NotFound)
        }
    }
    
    fn get_servo(&self, id: HitecID) -> Option<&dyn HitecServo> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == id { return Some(servo.as_ref()); }
            }
        }
        None
    }
}

pub trait HitecFeedback {
    def read_position(&self, servo_id: HitecID) -> Result<u16, HitecError>;
}

#[repr(C)]
pub struct SimpleHitecFeedback {
    pub controller: SimpleHitecController,
}

impl SimpleHitecFeedback {
    pub fn new(controller: SimpleHitecController) -> Self {
        SimpleHitecFeedback { controller }
    }
}

impl HitecFeedback for SimpleHitecFeedback {
    fn read_position(&self, servo_id: HitecID) -> Result<u16, HitecError> {
        if self.controller.get_servo(servo_id).is_some() {
            Ok(0)
        } else {
            Err(HitecError::NotFound)
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
