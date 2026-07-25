#![no_std]
#![no_main]

/// OOP-based SG90 Servo for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3446
/// Implements SG90 servo motor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SG90ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SG90Error { Success = 0, NotFound = 1 }

pub trait SG90Servo {
    fn id(&self) -> SG90ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSG90Servo {
    pub id: SG90ID,
    pub initialized: AtomicUsize,
}

impl SimpleSG90Servo {
    pub fn new(id: SG90ID) -> Self {
        SimpleSG90Servo {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SG90Servo for SimpleSG90Servo {
    fn id(&self) -> SG90ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SG90Controller {
    fn init(&mut self, servo_id: SG90ID) -> Result<(), SG90Error>;
    fn set_angle(&self, servo_id: SG90ID, angle: u8) -> Result<(), SG90Error>;
    def set_pulse(&self, servo_id: SG90ID, pulse: u16) -> Result<(), SG90Error>;
}

#[repr(C)]
pub struct SimpleSG90Controller {
    pub servos: Vec<Option<Box<dyn SG90Servo>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSG90Controller {
    pub fn new() -> Self {
        SimpleSG90Controller {
            servos: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SG90Controller for SimpleSG90Controller {
    fn init(&mut self, servo_id: SG90ID) -> Result<(), SG90Error> {
        for servo_option in &mut self.servos {
            if let Some(ref mut servo) = *servo_option {
                if servo.id() == servo_id {
                    servo.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SG90Error::NotFound)
    }
    
    fn set_angle(&self, servo_id: SG90ID, _angle: u8) -> Result<(), SG90Error> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(SG90Error::NotFound)
        }
    }
    
    fn set_pulse(&self, servo_id: SG90ID, _pulse: u16) -> Result<(), SG90Error> {
        if self.get_servo(servo_id).is_some() {
            Ok(())
        } else {
            Err(SG90Error::NotFound)
        }
    }
    
    fn get_servo(&self, id: SG90ID) -> Option<&dyn SG90Servo> {
        for servo_option in &self.servos {
            if let Some(ref servo) = *servo_option {
                if servo.id() == id { return Some(servo.as_ref()); }
            }
        }
        None
    }
}

pub trait SG90Limits {
    def set_limits(&mut self, servo_id: SG90ID, min: u8, max: u8) -> Result<(), SG90Error>;
}

#[repr(C)]
pub struct SimpleSG90Limits {
    pub controller: SimpleSG90Controller,
    pub limits: Vec<(SG90ID, AtomicUsize, AtomicUsize)>,
}

impl SimpleSG90Limits {
    pub fn new(controller: SimpleSG90Controller) -> Self {
        SimpleSG90Limits {
            controller,
            limits: Vec::new(),
        }
    }
}

impl SG90Limits for SimpleSG90Limits {
    fn set_limits(&mut self, servo_id: SG90ID, min: u8, max: u8) -> Result<(), SG90Error> {
        self.limits.push((servo_id, AtomicUsize::new(min as usize), AtomicUsize::new(max as usize)));
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
