#![no_std]
#![no_main]

/// OOP-based MG996R Servo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3456
/// Implements MG996R servo motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MG996RID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MG996RError { Success = 0, NotFound = 1 }

pub trait MG996RServo {
    fn id(&self) -> MG996RID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMG996RServo {
    pub id: MG996RID,
    pub initialized: AtomicUsize,
}

impl SimpleMG996RServo {
    pub fn new(id: MG996RID) -> Self {
        SimpleMG996RServo {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MG996RServo for SimpleMG996RServo {
    fn id(&self) -> MG996RID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MG996RController {
    fn init(&mut self, servo_id: MG996RID) -> Result<(), MG996RError>;
    fn set_angle(&self, servo_id: MG996RID, angle: u8) -> Result<(), MG996RError>;
    def set_pulse(&self, servo_id: MG996RID, pulse: u16) -> Result<(), MG996RError>;
}

#[repr(C)]
pub struct SimpleMG996RController {
    pub servos: Vec<Option<Box<dyn MG996RServo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMG996RController {
    pub fn new() -> Self {
        SimpleMG996RController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MG996RController for SimpleMG996RController {
    fn init(&mut self, servo_id: MG996RID) -> Result<(), MG996RError> {
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MG996RError::NotFound)
    }
    
    fn set_angle(&self, servo_id: MG996RID, _angle: u8) -> Result<(), MG996RError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(MG996RError::NotFound)
        }
    }
    
    fn set_pulse(&self, servo_id: MG996RID, _pulse: u16) -> Result<(), MG996RError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(MG996RError::NotFound)
        }
    }
    
    fn get_servo(&self, id: MG996RID) -> Option<&dyn MG996RServo> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == id { return Some(servo.as_ref()); }
            }
        }
        None
    }
}

pub trait MG996RTorque {
    def set_torque(&mut self, servo_id: MG996RID, torque: u8) -> Result<(), MG996RError>;
}

#[repr(C)]
pub struct SimpleMG996RTorque {
    pub controller: SimpleMG996RController,
    pub torques: Vec<(MG996RID, AtomicUsize)>,
}

impl SimpleMG996RTorque {
    pub fn new(controller: SimpleMG996RController) -> Self {
        SimpleMG996RTorque {
            controller,
            torques: Vec::new(),
        }
    }
}

impl MG996RTorque for SimpleMG996RTorque {
    fn set_torque(&mut self, servo_id: MG996RID, torque: u8) -> Result<(), MG996RError> {
        self.torques.push((servo_id, AtomicUsize::new(torque as usize)));
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
