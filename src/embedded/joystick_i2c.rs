#![no_std]
#![no_main]

/// OOP-based I2C Joystick for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3896
/// Implements I2C joystick

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type JoystickI2CID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JoystickI2CError { Success = 0, NotFound = 1 }

pub trait JoystickI2CDevice {
    fn id(&self) -> JoystickI2CID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleJoystickI2CDevice {
    pub id: JoystickI2CID,
    pub initialized: AtomicUsize,
}

impl SimpleJoystickI2CDevice {
    pub fn new(id: JoystickI2CID) -> Self {
        SimpleJoystickI2CDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl JoystickI2CDevice for SimpleJoystickI2CDevice {
    fn id(&self) -> JoystickI2CID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait JoystickI2CController {
    fn init(&mut self, joy_id: JoystickI2CID) -> Result<(), JoystickI2CError>;
    fn read_x(&self, joy_id: JoystickI2CID) -> Result<i8, JoystickI2CError>;
    def read_y(&self, joy_id: JoystickI2CID) -> Result<i8, JoystickI2CError>;
}

#[repr(C)]
pub struct SimpleJoystickI2CController {
    pub joysticks: Vec<Option<Box<dyn JoystickI2CDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleJoystickI2CController {
    pub fn new() -> Self {
        SimpleJoystickI2CController {
            joysticks: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl JoystickI2CController for SimpleJoystickI2CController {
    fn init(&mut self, joy_id: JoystickI2CID) -> Result<(), JoystickI2CError> {
        for joy_option in &mut self.joysticks {
            if let Some(ref mut joy) = *joy_option {
                if joy.id() == joy_id {
                    joy.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(JoystickI2CError::NotFound)
    }
    
    fn read_x(&self, joy_id: JoystickI2CID) -> Result<i8, JoystickI2CError> {
        if self.get_joystick(joy_id).is_some() {
            Ok(0)
        } else {
            Err(JoystickI2CError::NotFound)
        }
    }
    
    fn read_y(&self, joy_id: JoystickI2CID) -> Result<i8, JoystickI2CError> {
        if self.get_joystick(joy_id).is_some() {
            Ok(0)
        } else {
            Err(JoystickI2CError::NotFound)
        }
    }
    
    fn get_joystick(&self, id: JoystickI2CID) -> Option<&dyn JoystickI2CDevice> {
        for joy_option in &self.joysticks {
            if let Some(ref joy) = *joy_option {
                if joy.id() == id { return Some(joy.as_ref()); }
            }
        }
        None
    }
}

pub trait JoystickI2CButtons {
    def read_buttons(&self, joy_id: JoystickI2CID) -> Result<u8, JoystickI2CError>;
}

#[repr(C)]
pub struct SimpleJoystickI2CButtons {
    pub controller: SimpleJoystickI2CController,
}

impl SimpleJoystickI2CButtons {
    pub fn new(controller: SimpleJoystickI2CController) -> Self {
        SimpleJoystickI2CButtons { controller }
    }
}

impl JoystickI2CButtons for SimpleJoystickI2CButtons {
    fn read_buttons(&self, joy_id: JoystickI2CID) -> Result<u8, JoystickI2CError> {
        if self.controller.get_joystick(joy_id).is_some() {
            Ok(0)
        } else {
            Err(JoystickI2CError::NotFound)
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
