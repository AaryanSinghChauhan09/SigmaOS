#![no_std]
#![no_main]

/// OOP-based I2C Keyboard for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3906
/// Implements I2C keyboard matrix

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type KeyboardI2CID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum KeyboardI2CError { Success = 0, NotFound = 1 }

pub trait KeyboardI2CDevice {
    fn id(&self) -> KeyboardI2CID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleKeyboardI2CDevice {
    pub id: KeyboardI2CID,
    pub initialized: AtomicUsize,
}

impl SimpleKeyboardI2CDevice {
    pub fn new(id: KeyboardI2CID) -> Self {
        SimpleKeyboardI2CDevice {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl KeyboardI2CDevice for SimpleKeyboardI2CDevice {
    fn id(&self) -> KeyboardI2CID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait KeyboardI2CController {
    fn init(&mut self, kb_id: KeyboardI2CID) -> Result<(), KeyboardI2CError>;
    fn read(&self, kb_id: KeyboardI2CID) -> Result<u8, KeyboardI2CError>;
    def set_led(&self, kb_id: KeyboardI2CID, led: u8, state: bool) -> Result<(), KeyboardI2CError>;
}

#[repr(C)]
pub struct SimpleKeyboardI2CController {
    pub keyboards: Vec<Option<Box<dyn KeyboardI2CDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleKeyboardI2CController {
    pub fn new() -> Self {
        SimpleKeyboardI2CController {
            keyboards: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl KeyboardI2CController for SimpleKeyboardI2CController {
    fn init(&mut self, kb_id: KeyboardI2CID) -> Result<(), KeyboardI2CError> {
        for kb_option in &mut self.keyboards {
            if let Some(ref mut kb) = *kb_option {
                if kb.id() == kb_id {
                    kb.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(KeyboardI2CError::NotFound)
    }
    
    fn read(&self, kb_id: KeyboardI2CID) -> Result<u8, KeyboardI2CError> {
        if self.get_keyboard(kb_id).is_some() {
            Ok(0)
        } else {
            Err(KeyboardI2CError::NotFound)
        }
    }
    
    fn set_led(&self, kb_id: KeyboardI2CID, _led: u8, _state: bool) -> Result<(), KeyboardI2CError> {
        if self.get_keyboard(kb_id).is_some() {
            Ok(())
        } else {
            Err(KeyboardI2CError::NotFound)
        }
    }
    
    fn get_keyboard(&self, id: KeyboardI2CID) -> Option<&dyn KeyboardI2CDevice> {
        for kb_option in &self.keyboards {
            if let Some(ref kb) = *kb_option {
                if kb.id() == id { return Some(kb.as_ref()); }
            }
        }
        None
    }
}

pub trait KeyboardI2CMatrix {
    def scan_matrix(&self, kb_id: KeyboardI2CID, buffer: &mut [u8]) -> Result<(), KeyboardI2CError>;
}

#[repr(C)]
pub struct SimpleKeyboardI2CMatrix {
    pub controller: SimpleKeyboardI2CController,
}

impl SimpleKeyboardI2CMatrix {
    pub fn new(controller: SimpleKeyboardI2CController) -> Self {
        SimpleKeyboardI2CMatrix { controller }
    }
}

impl KeyboardI2CMatrix for SimpleKeyboardI2CMatrix {
    fn scan_matrix(&self, kb_id: KeyboardI2CID, buffer: &mut [u8]) -> Result<(), KeyboardI2CError> {
        if self.controller.get_keyboard(kb_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(())
        } else {
            Err(KeyboardI2CError::NotFound)
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
