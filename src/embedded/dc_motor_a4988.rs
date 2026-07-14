#![no_std]
#![no_main]

/// OOP-based A4988 DC Motor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3566
/// Implements A4988 stepper motor driver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type A4988ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum A4988Error { Success = 0, NotFound = 1 }

pub trait A4988Motor {
    fn id(&self) -> A4988ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleA4988Motor {
    pub id: A4988ID,
    pub initialized: AtomicUsize,
}

impl SimpleA4988Motor {
    pub fn new(id: A4988ID) -> Self {
        SimpleA4988Motor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl A4988Motor for SimpleA4988Motor {
    fn id(&self) -> A4988ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait A4988Controller {
    fn init(&mut self, motor_id: A4988ID) -> Result<(), A4988Error>;
    fn step(&self, motor_id: A4988ID, steps: i32) -> Result<(), A4988Error>;
    def set_speed(&mut self, motor_id: A4988ID, rpm: u16) -> Result<(), A4988Error>;
}

#[repr(C)]
pub struct SimpleA4988Controller {
    pub motors: Vec<Option<Box<dyn A4988Motor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleA4988Controller {
    pub fn new() -> Self {
        SimpleA4988Controller {
            motors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl A4988Controller for SimpleA4988Controller {
    fn init(&mut self, motor_id: A4988ID) -> Result<(), A4988Error> {
        for motor_option in &mut self.motors {
            if let Some(ref mut motor) = *motor_option {
                if motor.id() == motor_id {
                    motor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(A4988Error::NotFound)
    }
    
    fn step(&self, motor_id: A4988ID, _steps: i32) -> Result<(), A4988Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(A4988Error::NotFound)
        }
    }
    
    fn set_speed(&mut self, motor_id: A4988ID, _rpm: u16) -> Result<(), A4988Error> {
        if self.get_motor(motor_id).is_some() {
            Ok(())
        } else {
            Err(A4988Error::NotFound)
        }
    }
    
    fn get_motor(&self, id: A4988ID) -> Option<&dyn A4988Motor> {
        for motor_option in &self.motors {
            if let Some(ref motor) = *motor_option {
                if motor.id() == id { return Some(motor.as_ref()); }
            }
        }
        None
    }
}

pub trait A4988Enable {
    def enable(&mut self, motor_id: A4988ID) -> Result<(), A4988Error>;
    def disable(&mut self, motor_id: A4988ID) -> Result<(), A4988Error>;
}

#[repr(C)]
pub struct SimpleA4988Enable {
    pub controller: SimpleA4988Controller,
    pub enabled: Vec<(A4988ID, AtomicUsize)>,
}

impl SimpleA4988Enable {
    pub fn new(controller: SimpleA4988Controller) -> Self {
        SimpleA4988Enable {
            controller,
            enabled: Vec::new(),
        }
    }
}

impl A4988Enable for SimpleA4988Enable {
    fn enable(&mut self, motor_id: A4988ID) -> Result<(), A4988Error> {
        self.enabled.push((motor_id, AtomicUsize::new(1)));
        Ok(())
    }
    
    fn disable(&mut self, motor_id: A4988ID) -> Result<(), A4988Error> {
        self.enabled.push((motor_id, AtomicUsize::new(0)));
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
