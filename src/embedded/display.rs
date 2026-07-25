#![no_std]
#![no_main]

/// OOP-based 7-Segment Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1456
/// Implements 7-segment display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DisplayError { Success = 0, NotFound = 1 }

pub trait SevenSegment {
    fn id(&self) -> DisplayID;
    def set_digit(&mut self, digit: u8);
    def clear(&mut self);
}

#[repr(C)]
pub struct SimpleSevenSegment {
    pub id: DisplayID,
    pub segments: AtomicUsize,
}

impl SimpleSevenSegment {
    pub fn new(id: DisplayID) -> Self {
        SimpleSevenSegment {
            id,
            segments: AtomicUsize::new(0),
        }
    }
}

impl SevenSegment for SimpleSevenSegment {
    fn id(&self) -> DisplayID { self.id }
    
    fn set_digit(&mut self, digit: u8) {
        const PATTERNS: [u8; 16] = [
            0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07,
            0x7F, 0x6F, 0x77, 0x7C, 0x39, 0x5E, 0x79, 0x71,
        ];
        let pattern = if digit < 16 { PATTERNS[digit as usize] } else { 0 };
        self.segments.store(pattern as usize, Ordering::SeqCst);
    }
    
    fn clear(&mut self) {
        self.segments.store(0, Ordering::SeqCst);
    }
}

pub trait DisplayController {
    fn set_number(&mut self, display_id: DisplayID, number: u32) -> Result<(), DisplayError>;
    def set_hex(&mut self, display_id: DisplayID, value: u8) -> Result<(), DisplayError>;
}

#[repr(C)]
pub struct SimpleDisplayController {
    pub displays: Vec<Option<Box<dyn SevenSegment>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDisplayController {
    pub fn new() -> Self {
        SimpleDisplayController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DisplayController for SimpleDisplayController {
    fn set_number(&mut self, display_id: DisplayID, number: u32) -> Result<(), DisplayError> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    let digit = (number % 10) as u8;
                    display.set_digit(digit);
                    return Ok(());
                }
            }
        }
        Err(DisplayError::NotFound)
    }
    
    fn set_hex(&mut self, display_id: DisplayID, value: u8) -> Result<(), DisplayError> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.set_digit(value);
                    return Ok(());
                }
            }
        }
        Err(DisplayError::NotFound)
    }
}

pub trait MultiplexedDisplay {
    def scan(&mut self);
    def set_brightness(&mut self, brightness: u8);
}

#[repr(C)]
pub struct SimpleMultiplexedDisplay {
    pub controller: SimpleDisplayController,
    pub brightness: AtomicUsize,
    pub current_digit: AtomicUsize,
}

impl SimpleMultiplexedDisplay {
    pub fn new(controller: SimpleDisplayController) -> Self {
        SimpleMultiplexedDisplay {
            controller,
            brightness: AtomicUsize::new(255),
            current_digit: AtomicUsize::new(0),
        }
    }
}

impl MultiplexedDisplay for SimpleMultiplexedDisplay {
    fn scan(&mut self) {
        let current = self.current_digit.load(Ordering::SeqCst);
        self.current_digit.store((current + 1) % 4, Ordering::SeqCst);
    }
    
    fn set_brightness(&mut self, brightness: u8) {
        self.brightness.store(brightness as usize, Ordering::SeqCst);
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
