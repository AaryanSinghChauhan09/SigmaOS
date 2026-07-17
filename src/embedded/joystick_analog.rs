#![no_std]
#![no_main]

/// OOP-based Analog Joystick for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3886
/// Implements analog joystick

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type JoystickAnalogID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum JoystickAnalogError { Success = 0, NotFound = 1 }

pub trait JoystickAnalogDevice {
    fn id(&self) -> JoystickAnalogID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleJoystickAnalogDevice {
    pub id: JoystickAnalogID,
    pub initialized: AtomicUsize,
}

impl SimpleJoystickAnalogDevice {
    pub fn new(id: JoystickAnalogID) -> Self {
        SimpleJoystickAnalogDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl JoystickAnalogDevice for SimpleJoystickAnalogDevice {
    fn id(&self) -> JoystickAnalogID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait JoystickAnalogController {
    fn init(&mut self, joy_id: JoystickAnalogID) -> Result<(), JoystickAnalogError>;
    fn read_x(&self, joy_id: JoystickAnalogID) -> Result<u16, JoystickAnalogError>;
    def read_y(&self, joy_id: JoystickAnalogID) -> Result<u16, JoystickAnalogError>;
}

#[repr(C)]
pub struct SimpleJoystickAnalogController {
    pub joysticks: Vec<Option<Box<dyn JoystickAnalogDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleJoystickAnalogController {
    pub fn new() -> Self {
        SimpleJoystickAnalogController {
            joysticks: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl JoystickAnalogController for SimpleJoystickAnalogController {
    fn init(&mut self, joy_id: JoystickAnalogID) -> Result<(), JoystickAnalogError> {
        for joy_option in &mut self.joysticks {
            if let Some(ref mut joy) = *joy_option {
                if joy.id() == joy_id {
                    joy.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(JoystickAnalogError::NotFound)
    }
    
    fn read_x(&self, joy_id: JoystickAnalogID) -> Result<u16, JoystickAnalogError> {
        if self.get_joystick(joy_id).is_some() {
            Ok(0)
        } else {
            Err(JoystickAnalogError::NotFound)
        }
    }
    
    fn read_y(&self, joy_id: JoystickAnalogID) -> Result<u16, JoystickAnalogError> {
        if self.get_joystick(joy_id).is_some() {
            Ok(0)
        } else {
            Err(JoystickAnalogError::NotFound)
        }
    }
    
    fn get_joystick(&self, id: JoystickAnalogID) -> Option<&dyn JoystickAnalogDevice> {
        for joy_option in &self.joysticks {
            if let Some(ref joy) = *joy_option {
                if joy.id() == id { return Some(joy.as_ref()); }
            }
        }
        None
    }
}

pub trait JoystickAnalogButton {
    def read_button(&self, joy_id: JoystickAnalogID) -> Result<bool, JoystickAnalogError>;
}

#[repr(C)]
pub struct SimpleJoystickAnalogButton {
    pub controller: SimpleJoystickAnalogController,
}

impl SimpleJoystickAnalogButton {
    pub fn new(controller: SimpleJoystickAnalogController) -> Self {
        SimpleJoystickAnalogButton { controller }
    }
}

impl JoystickAnalogButton for SimpleJoystickAnalogButton {
    fn read_button(&self, joy_id: JoystickAnalogID) -> Result<bool, JoystickAnalogError> {
        if self.controller.get_joystick(joy_id).is_some() {
            Ok(false)
        } else {
            Err(JoystickAnalogError::NotFound)
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
