#![no_std]
#![no_main]

/// OOP-based Futaba Servo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3486
/// Implements Futaba servo motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FutabaID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FutabaError { Success = 0, NotFound = 1 }

pub trait FutabaServo {
    fn id(&self) -> FutabaID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFutabaServo {
    pub id: FutabaID,
    pub initialized: AtomicUsize,
}

impl SimpleFutabaServo {
    pub fn new(id: FutabaID) -> Self {
        SimpleFutabaServo {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl FutabaServo for SimpleFutabaServo {
    fn id(&self) -> FutabaID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait FutabaController {
    fn init(&mut self, servo_id: FutabaID) -> Result<(), FutabaError>;
    fn set_angle(&self, servo_id: FutabaID, angle: u8) -> Result<(), FutabaError>;
    def set_pulse(&self, servo_id: FutabaID, pulse: u16) -> Result<(), FutabaError>;
}

#[repr(C)]
pub struct SimpleFutabaController {
    pub servos: Vec<Option<Box<dyn FutabaServo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFutabaController {
    pub fn new() -> Self {
        SimpleFutabaController {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FutabaController for SimpleFutabaController {
    fn init(&mut self, servo_id: FutabaID) -> Result<(), FutabaError> {
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FutabaError::NotFound)
    }
    
    fn set_angle(&self, servo_id: FutabaID, _angle: u8) -> Result<(), FutabaError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(FutabaError::NotFound)
        }
    }
    
    fn set_pulse(&self, servo_id: FutabaID, _pulse: u16) -> Result<(), FutabaError> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(FutabaError::NotFound)
        }
    }
    
    fn get_servo(&self, id: FutabaID) -> Option<&dyn FutabaServo> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == id { return Some(servo.as_ref()); }
            }
        }
        None
    }
}

pub trait FutabaSBUS {
    def set_sbus(&mut self, servo_id: FutabaID, enable: bool) -> Result<(), FutabaError>;
}

#[repr(C)]
pub struct SimpleFutabaSBUS {
    pub controller: SimpleFutabaController,
    pub sbus: Vec<(FutabaID, AtomicUsize)>,
}

impl SimpleFutabaSBUS {
    pub fn new(controller: SimpleFutabaController) -> Self {
        SimpleFutabaSBUS {
            controller,
            sbus: Vec::new(),
        }
    }
}

impl FutabaSBUS for SimpleFutabaSBUS {
    fn set_sbus(&mut self, servo_id: FutabaID, enable: bool) -> Result<(), FutabaError> {
        self.sbus.push((servo_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
