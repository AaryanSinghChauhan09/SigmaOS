#![no_std]
#![no_main]

/// OOP-based TB6612 DC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3546
/// Implements TB6612 DC motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TB6612ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TB6612Error { Success = 0, NotFound = 1 }

pub trait TB6612Motor {
    fn id(&self) -> TB6612ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTB6612Motor {
    pub id: TB6612ID,
    pub initialized: AtomicUsize,
}

impl SimpleTB6612Motor {
    pub fn new(id: TB6612ID) -> Self {
        SimpleTB6612Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TB6612Motor for SimpleTB6612Motor {
    fn id(&self) -> TB6612ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TB6612Controller {
    fn init(&mut self, motor_id: TB6612ID) -> Result<(), TB6612Error>;
    fn set_speed(&self, motor_id: TB6612ID, speed: u8) -> Result<(), TB6612Error>;
    def set_direction(&self, motor_id: TB6612ID, forward: bool) -> Result<(), TB6612Error>;
}

#[repr(C)]
pub struct SimpleTB6612Controller {
    pub motors: Vec<Option<Box<dyn TB6612Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTB6612Controller {
    pub fn new() -> Self {
        SimpleTB6612Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TB6612Controller for SimpleTB6612Controller {
    fn init(&mut self, motor_id: TB6612ID) -> Result<(), TB6612Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TB6612Error::NotFound)
    }
    
    fn set_speed(&self, motor_id: TB6612ID, _speed: u8) -> Result<(), TB6612Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(TB6612Error::NotFound)
        }
    }
    
    fn set_direction(&self, motor_id: TB6612ID, _forward: bool) -> Result<(), TB6612Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(TB6612Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: TB6612ID) -> Option<&dyn TB6612Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait TB6612Brake {
    def brake(&self, motor_id: TB6612ID) -> Result<(), TB6612Error>;
}

#[repr(C)]
pub struct SimpleTB6612Brake {
    pub controller: SimpleTB6612Controller,
}

impl SimpleTB6612Brake {
    pub fn new(controller: SimpleTB6612Controller) -> Self {
        SimpleTB6612Brake { controller }
    }
}

impl TB6612Brake for SimpleTB6612Brake {
    fn brake(&self, motor_id: TB6612ID) -> Result<(), TB6612Error> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(TB6612Error::NotFound)
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
