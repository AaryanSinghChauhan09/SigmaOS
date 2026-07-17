#![no_std]
#![no_main]

/// OOP-based SK6812 LED for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3436
/// Implements SK6812 RGBW addressable LED

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SK6812ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SK6812Error { Success = 0, NotFound = 1 }

pub trait SK6812LED {
    fn id(&self) -> SK6812ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSK6812LED {
    pub id: SK6812ID,
    pub initialized: AtomicUsize,
}

impl SimpleSK6812LED {
    pub fn new(id: SK6812ID) -> Self {
        SimpleSK6812LED {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SK6812LED for SimpleSK6812LED {
    fn id(&self) -> SK6812ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SK6812Controller {
    fn init(&mut self, led_id: SK6812ID) -> Result<(), SK6812Error>;
    fn set_color(&self, led_id: SK6812ID, r: u8, g: u8, b: u8, w: u8) -> Result<(), SK6812Error>;
    def set_brightness(&mut self, led_id: SK6812ID, brightness: u8) -> Result<(), SK6812Error>;
}

#[repr(C)]
pub struct SimpleSK6812Controller {
    pub leds: Vec<Option<Box<dyn SK6812LED>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSK6812Controller {
    pub fn new() -> Self {
        SimpleSK6812Controller {
            leds: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SK6812Controller for SimpleSK6812Controller {
    fn init(&mut self, led_id: SK6812ID) -> Result<(), SK6812Error> {
        for led_option in &mut self.leds {
            if let Some(ref mut led) = *led_option {
                if led.id() == led_id {
                    led.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SK6812Error::NotFound)
    }
    
    fn set_color(&self, led_id: SK6812ID, _r: u8, _g: u8, _b: u8, _w: u8) -> Result<(), SK6812Error> {
        if self.get_led(led_id).is_some() {
            Ok(())
        } else {
            Err(SK6812Error::NotFound)
        }
    }
    
    fn set_brightness(&mut self, led_id: SK6812ID, _brightness: u8) -> Result<(), SK6812Error> {
        if self.get_led(led_id).is_some() {
            Ok(())
        } else {
            Err(SK6812Error::NotFound)
        }
    }
    
    fn get_led(&self, id: SK6812ID) -> Option<&dyn SK6812LED> {
        for led_option in &self.leds {
            if let Some(ref led) = *led_option {
                if led.id() == id { return Some(led.as_ref()); }
            }
        }
        None
    }
}

pub trait SK6812Strip {
    def set_strip(&self, led_id: SK6812ID, colors: &[(u8, u8, u8, u8)]) -> Result<(), SK6812Error>;
}

#[repr(C)]
pub struct SimpleSK6812Strip {
    pub controller: SimpleSK6812Controller,
}

impl SimpleSK6812Strip {
    pub fn new(controller: SimpleSK6812Controller) -> Self {
        SimpleSK6812Strip { controller }
    }
}

impl SK6812Strip for SimpleSK6812Strip {
    fn set_strip(&self, led_id: SK6812ID, _colors: &[(u8, u8, u8, u8)]) -> Result<(), SK6812Error> {
        if self.controller.get_led(led_id).is_some() {
            Ok(())
        } else {
            Err(SK6812Error::NotFound)
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
