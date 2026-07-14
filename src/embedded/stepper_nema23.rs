#![no_std]
#![no_main]

/// OOP-based NEMA23 Stepper for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3516
/// Implements NEMA23 stepper motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Nema23ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Nema23Error { Success = 0, NotFound = 1 }

pub trait Nema23Motor {
    fn id(&self) -> Nema23ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNema23Motor {
    pub id: Nema23ID,
    pub initialized: AtomicUsize,
}

impl SimpleNema23Motor {
    pub fn new(id: Nema23ID) -> Self {
        SimpleNema23Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Nema23Motor for SimpleNema23Motor {
    fn id(&self) -> Nema23ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Nema23Controller {
    fn init(&mut self, motor_id: Nema23ID) -> Result<(), Nema23Error>;
    fn step(&self, motor_id: Nema23ID, steps: i32) -> Result<(), Nema23Error>;
    def set_speed(&mut self, motor_id: Nema23ID, rpm: u16) -> Result<(), Nema23Error>;
}

#[repr(C)]
pub struct SimpleNema23Controller {
    pub motors: Vec<Option<Box<dyn Nema23Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNema23Controller {
    pub fn new() -> Self {
        SimpleNema23Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Nema23Controller for SimpleNema23Controller {
    fn init(&mut self, motor_id: Nema23ID) -> Result<(), Nema23Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Nema23Error::NotFound)
    }
    
    fn step(&self, motor_id: Nema23ID, _steps: i32) -> Result<(), Nema23Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema23Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: Nema23ID, _rpm: u16) -> Result<(), Nema23Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema23Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: Nema23ID) -> Option<&dyn Nema23Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait Nema23Current {
    def set_current(&mut self, motor_id: Nema23ID, current: u16) -> Result<(), Nema23Error>;
}

#[repr(C)]
pub struct SimpleNema23Current {
    pub controller: SimpleNema23Controller,
    pub currents: Vec<(Nema23ID, AtomicUsize)>,
}

impl SimpleNema23Current {
    pub fn new(controller: SimpleNema23Controller) -> Self {
        SimpleNema23Current {
            controller,
            currents: Vec::new(),
        }
    }
}

impl Nema23Current for SimpleNema23Current {
    fn set_current(&mut self, motor_id: Nema23ID, current: u16) -> Result<(), Nema23Error> {
        self.currents.push((motor_id, AtomicUsize::new(current as usize)));
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
