#![no_std]
#![no_main]

/// OOP-based 28BYJ-48 Stepper for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3496
/// Implements 28BYJ-48 stepper motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Stepper28ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Stepper28Error { Success = 0, NotFound = 1 }

pub trait Stepper28Motor {
    fn id(&self) -> Stepper28ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleStepper28Motor {
    pub id: Stepper28ID,
    pub initialized: AtomicUsize,
}

impl SimpleStepper28Motor {
    pub fn new(id: Stepper28ID) -> Self {
        SimpleStepper28Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Stepper28Motor for SimpleStepper28Motor {
    fn id(&self) -> Stepper28ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Stepper28Controller {
    fn init(&mut self, motor_id: Stepper28ID) -> Result<(), Stepper28Error>;
    fn step(&self, motor_id: Stepper28ID, steps: i32) -> Result<(), Stepper28Error>;
    def set_speed(&mut self, motor_id: Stepper28ID, rpm: u16) -> Result<(), Stepper28Error>;
}

#[repr(C)]
pub struct SimpleStepper28Controller {
    pub motors: Vec<Option<Box<dyn Stepper28Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleStepper28Controller {
    pub fn new() -> Self {
        SimpleStepper28Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Stepper28Controller for SimpleStepper28Controller {
    fn init(&mut self, motor_id: Stepper28ID) -> Result<(), Stepper28Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Stepper28Error::NotFound)
    }
    
    fn step(&self, motor_id: Stepper28ID, _steps: i32) -> Result<(), Stepper28Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Stepper28Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: Stepper28ID, _rpm: u16) -> Result<(), Stepper28Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Stepper28Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: Stepper28ID) -> Option<&dyn Stepper28Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait Stepper28Home {
    def home(&self, motor_id: Stepper28ID) -> Result<(), Stepper28Error>;
}

#[repr(C)]
pub struct SimpleStepper28Home {
    pub controller: SimpleStepper28Controller,
}

impl SimpleStepper28Home {
    pub fn new(controller: SimpleStepper28Controller) -> Self {
        SimpleStepper28Home { controller }
    }
}

impl Stepper28Home for SimpleStepper28Home {
    fn home(&self, motor_id: Stepper28ID) -> Result<(), Stepper28Error> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Stepper28Error::NotFound)
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
