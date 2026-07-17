#![no_std]
#![no_main]

/// OOP-based NEMA34 Stepper for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3526
/// Implements NEMA34 stepper motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Nema34ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Nema34Error { Success = 0, NotFound = 1 }

pub trait Nema34Motor {
    fn id(&self) -> Nema34ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNema34Motor {
    pub id: Nema34ID,
    pub initialized: AtomicUsize,
}

impl SimpleNema34Motor {
    pub fn new(id: Nema34ID) -> Self {
        SimpleNema34Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Nema34Motor for SimpleNema34Motor {
    fn id(&self) -> Nema34ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Nema34Controller {
    fn init(&mut self, motor_id: Nema34ID) -> Result<(), Nema34Error>;
    fn step(&self, motor_id: Nema34ID, steps: i32) -> Result<(), Nema34Error>;
    def set_speed(&mut self, motor_id: Nema34ID, rpm: u16) -> Result<(), Nema34Error>;
}

#[repr(C)]
pub struct SimpleNema34Controller {
    pub motors: Vec<Option<Box<dyn Nema34Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNema34Controller {
    pub fn new() -> Self {
        SimpleNema34Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Nema34Controller for SimpleNema34Controller {
    fn init(&mut self, motor_id: Nema34ID) -> Result<(), Nema34Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Nema34Error::NotFound)
    }
    
    fn step(&self, motor_id: Nema34ID, _steps: i32) -> Result<(), Nema34Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema34Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: Nema34ID, _rpm: u16) -> Result<(), Nema34Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema34Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: Nema34ID) -> Option<&dyn Nema34Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait Nema34Torque {
    def get_torque(&self, motor_id: Nema34ID) -> Result<u16, Nema34Error>;
}

#[repr(C)]
pub struct SimpleNema34Torque {
    pub controller: SimpleNema34Controller,
}

impl SimpleNema34Torque {
    pub fn new(controller: SimpleNema34Controller) -> Self {
        SimpleNema34Torque { controller }
    }
}

impl Nema34Torque for SimpleNema34Torque {
    fn get_torque(&self, motor_id: Nema34ID) -> Result<u16, Nema34Error> {
        if self.controller.get_motor(motor_id).is_some() {
            Ok(0)
        } else {
            Err(Nema34Error::NotFound)
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
