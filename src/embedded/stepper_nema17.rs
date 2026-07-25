#![no_std]
#![no_main]

/// OOP-based NEMA17 Stepper for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3506
/// Implements NEMA17 stepper motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type Nema17ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Nema17Error { Success = 0, NotFound = 1 }

pub trait Nema17Motor {
    fn id(&self) -> Nema17ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleNema17Motor {
    pub id: Nema17ID,
    pub initialized: AtomicUsize,
}

impl SimpleNema17Motor {
    pub fn new(id: Nema17ID) -> Self {
        SimpleNema17Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl Nema17Motor for SimpleNema17Motor {
    fn id(&self) -> Nema17ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait Nema17Controller {
    fn init(&mut self, motor_id: Nema17ID) -> Result<(), Nema17Error>;
    fn step(&self, motor_id: Nema17ID, steps: i32) -> Result<(), Nema17Error>;
    def set_speed(&mut self, motor_id: Nema17ID, rpm: u16) -> Result<(), Nema17Error>;
}

#[repr(C)]
pub struct SimpleNema17Controller {
    pub motors: Vec<Option<Box<dyn Nema17Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleNema17Controller {
    pub fn new() -> Self {
        SimpleNema17Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl Nema17Controller for SimpleNema17Controller {
    fn init(&mut self, motor_id: Nema17ID) -> Result<(), Nema17Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(Nema17Error::NotFound)
    }
    
    fn step(&self, motor_id: Nema17ID, _steps: i32) -> Result<(), Nema17Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema17Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: Nema17ID, _rpm: u16) -> Result<(), Nema17Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(Nema17Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: Nema17ID) -> Option<&dyn Nema17Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait Nema17Microstep {
    def set_microstep(&mut self, motor_id: Nema17ID, mode: u8) -> Result<(), Nema17Error>;
}

#[repr(C)]
pub struct SimpleNema17Microstep {
    pub controller: SimpleNema17Controller,
    pub microsteps: Vec<(Nema17ID, AtomicUsize)>,
}

impl SimpleNema17Microstep {
    pub fn new(controller: SimpleNema17Controller) -> Self {
        SimpleNema17Microstep {
            controller,
            microsteps: Vec::new(),
        }
    }
}

impl Nema17Microstep for SimpleNema17Microstep {
    fn set_microstep(&mut self, motor_id: Nema17ID, mode: u8) -> Result<(), Nema17Error> {
        self.microsteps.push((motor_id, AtomicUsize::new(mode as usize)));
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
