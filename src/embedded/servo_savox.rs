#![no_std]
#![no_main]

/// OOP-based Savox Servo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3466
/// Implements Savox servo motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SavoxID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SavoxError { Success = 0, NotFound = 1 }

pub trait SavoxServo {
    fn id(&self) -> SavoxID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSavoxServo {
    pub id: SavoxID,
    pub initialized: AtomicUsize,
}

impl SimpleSavoxServo {
    pub fn new(id: SavoxID) -> Self {
        SimpleSavoxServo {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SavoxServo for SimpleSavoxServo {
    fn id(&self) -> SavoxID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SavoxController {
    fn init(&mut self, servo_id: SavoxID) -> Result<(), SavoxError>;
    fn set_angle(&self, servo_id: SavoxID, angle: u8) -> Result<(), SavoxError>;
    def set_pulse(&self, servo_id: SavoxID, pulse: u16) -> Result<(), SavoxError>;
}

#[repr(C)]
pub struct SimpleSavoxController {
    pub servos: Vec<Option<Box<dyn SavoxServo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSavoxController {
    pub fn new() -> Self {
        SimpleSavoxController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SavoxController for SimpleSavoxController {
    fn init(&mut self, servo_id: SavoxID) -> Result<(), SavoxError> {
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SavoxError::NotFound)
    }
    
    fn set_angle(&self, servo_id: SavoxID, _angle: u8) -> Result<(), SavoxError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(SavoxError::NotFound)
        }
    }
    
    fn set_pulse(&self, servo_id: SavoxID, _pulse: u16) -> Result<(), SavoxError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(SavoxError::NotFound)
        }
    }
    
    fn get_servo(&self, id: SavoxID) -> Option<&dyn SavoxServo> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == id { return Some(servo.as_ref()); }
            }
        }
        None
    }
}

pub trait SavoxSpeed {
    def set_speed(&mut self, servo_id: SavoxID, speed: u8) -> Result<(), SavoxError>;
}

#[repr(C)]
pub struct SimpleSavoxSpeed {
    pub controller: SimpleSavoxController,
    pub speeds: Vec<(SavoxID, AtomicUsize)>,
}

impl SimpleSavoxSpeed {
    pub fn new(controller: SimpleSavoxController) -> Self {
        SimpleSavoxSpeed {
            controller,
            speeds: Vec::new(),
        }
    }
}

impl SavoxSpeed for SimpleSavoxSpeed {
    fn set_speed(&mut self, servo_id: SavoxID, speed: u8) -> Result<(), SavoxError> {
        self.speeds.push((servo_id, AtomicUsize::new(speed as usize)));
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
