#![no_std]
#![no_main]

/// OOP-based Rotary Encoder for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1376
/// Implements rotary encoder

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type EncoderID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EncoderError { Success = 0, NotFound = 1 }

pub trait RotaryEncoder {
    fn id(&self) -> EncoderID;
    fn get_position(&self) -> i32;
    def set_position(&mut self, position: i32);
}

#[repr(C)]
pub struct SimpleRotaryEncoder {
    pub id: EncoderID,
    pub position: AtomicUsize,
}

impl SimpleRotaryEncoder {
    pub fn new(id: EncoderID) -> Self {
        SimpleRotaryEncoder {
            id,
            position: AtomicUsize::new(0),
        }
    }
}

impl RotaryEncoder for SimpleRotaryEncoder {
    fn id(&self) -> EncoderID { self.id }
    fn get_position(&self) -> i32 { self.position.load(Ordering::SeqCst) as i32 }
    
    fn set_position(&mut self, position: i32) {
        self.position.store(position as usize, Ordering::SeqCst);
    }
}

pub trait EncoderController {
    def increment(&mut self, encoder_id: EncoderID) -> Result<(), EncoderError>;
    def decrement(&mut self, encoder_id: EncoderID) -> Result<(), EncoderError>;
    def get_delta(&self, encoder_id: EncoderID) -> Result<i32, EncoderError>;
}

#[repr(C)]
pub struct SimpleEncoderController {
    pub encoders: Vec<Option<Box<dyn RotaryEncoder>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEncoderController {
    pub fn new() -> Self {
        SimpleEncoderController {
            encoders: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EncoderController for SimpleEncoderController {
    fn increment(&mut self, encoder_id: EncoderID) -> Result<(), EncoderError> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    let current = encoder.get_position();
                    encoder.set_position(current + 1);
                    return Ok(());
                }
            }
        }
        Err(EncoderError::NotFound)
    }
    
    fn decrement(&mut self, encoder_id: EncoderID) -> Result<(), EncoderError> {
        for encoder_option in &mut self.encoders {
            if let Some(ref mut encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    let current = encoder.get_position();
                    encoder.set_position(current - 1);
                    return Ok(());
                }
            }
        }
        Err(EncoderError::NotFound)
    }
    
    fn get_delta(&self, encoder_id: EncoderID) -> Result<i32, EncoderError> {
        for encoder_option in &self.encoders {
            if let Some(ref encoder) = *encoder_option {
                if encoder.id() == encoder_id {
                    return Ok(encoder.get_position());
                }
            }
        }
        Err(EncoderError::NotFound)
    }
}

pub trait EncoderButton {
    def is_pressed(&self) -> bool;
    def get_press_count(&self) -> u32;
}

#[repr(C)]
pub struct SimpleEncoderButton {
    pub pressed: AtomicUsize,
    pub press_count: AtomicUsize,
}

impl SimpleEncoderButton {
    pub fn new() -> Self {
        SimpleEncoderButton {
            pressed: AtomicUsize::new(0),
            press_count: AtomicUsize::new(0),
        }
    }
}

impl EncoderButton for SimpleEncoderButton {
    fn is_pressed(&self) -> bool {
        self.pressed.load(Ordering::SeqCst) == 1
    }
    
    fn get_press_count(&self) -> u32 {
        self.press_count.load(Ordering::SeqCst) as u32
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
