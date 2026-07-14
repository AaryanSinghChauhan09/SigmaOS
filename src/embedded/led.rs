#![no_std]
#![no_main]

/// OOP-based LED for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1416
/// Implements LED control

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LEDID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LEDState { Off = 0, On = 1, Blink = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LEDError { Success = 0, NotFound = 1 }

pub trait LED {
    fn id(&self) -> LEDID;
    fn state(&self) -> LEDState;
    fn brightness(&self) -> u8;
}

#[repr(C)]
pub struct SimpleLED {
    pub id: LEDID,
    pub state: AtomicUsize,
    pub brightness: AtomicUsize,
}

impl SimpleLED {
    pub fn new(id: LEDID) -> Self {
        SimpleLED {
            id,
            state: AtomicUsize::new(LEDState::Off as usize),
            brightness: AtomicUsize::new(255),
        }
    }
}

impl LED for SimpleLED {
    fn id(&self) -> LEDID { self.id }
    fn state(&self) -> LEDState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn brightness(&self) -> u8 { self.brightness.load(Ordering::SeqCst) as u8 }
}

pub trait LEDController {
    fn turn_on(&mut self, led_id: LEDID) -> Result<(), LEDError>;
    fn turn_off(&mut self, led_id: LEDID) -> Result<(), LEDError>;
    fn set_brightness(&mut self, led_id: LEDID, brightness: u8) -> Result<(), LEDError>;
    def blink(&mut self, led_id: LEDID, period: u32) -> Result<(), LEDError>;
}

#[repr(C)]
pub struct SimpleLEDController {
    pub leds: Vec<Option<Box<dyn LED>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLEDController {
    pub fn new() -> Self {
        SimpleLEDController {
            leds: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LEDController for SimpleLEDController {
    fn turn_on(&mut self, led_id: LEDID) -> Result<(), LEDError> {
        for led_option in &mut self.leds {
            if let Some(ref mut led) = *led_option {
                if led.id() == led_id {
                    led.state.store(LEDState::On as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LEDError::NotFound)
    }
    
    fn turn_off(&mut self, led_id: LEDID) -> Result<(), LEDError> {
        for led_option in &mut self.leds {
            if let Some(ref mut led) = *led_option {
                if led.id() == led_id {
                    led.state.store(LEDState::Off as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LEDError::NotFound)
    }
    
    fn set_brightness(&mut self, led_id: LEDID, brightness: u8) -> Result<(), LEDError> {
        for led_option in &mut self.leds {
            if let Some(ref mut led) = *led_option {
                if led.id() == led_id {
                    led.brightness.store(brightness as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LEDError::NotFound)
    }
    
    fn blink(&mut self, led_id: LEDID, _period: u32) -> Result<(), LEDError> {
        for led_option in &mut self.leds {
            if let Some(ref mut led) = *led_option {
                if led.id() == led_id {
                    led.state.store(LEDState::Blink as usize, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LEDError::NotFound)
    }
}

pub trait RGBLED {
    def set_color(&mut self, led_id: LEDID, r: u8, g: u8, b: u8) -> Result<(), LEDError>;
    def get_color(&self, led_id: LEDID) -> Result<(u8, u8, u8), LEDError>;
}

#[repr(C)]
pub struct SimpleRGBLED {
    pub controller: SimpleLEDController,
    pub colors: Vec<(LEDID, (AtomicUsize, AtomicUsize, AtomicUsize))>,
}

impl SimpleRGBLED {
    pub fn new(controller: SimpleLEDController) -> Self {
        SimpleRGBLED {
            controller,
            colors: Vec::new(),
        }
    }
}

impl RGBLED for SimpleRGBLED {
    fn set_color(&mut self, led_id: LEDID, r: u8, g: u8, b: u8) -> Result<(), LEDError> {
        self.colors.push((led_id, (AtomicUsize::new(r as usize), AtomicUsize::new(g as usize), AtomicUsize::new(b as usize))));
        Ok(())
    }
    
    fn get_color(&self, led_id: LEDID) -> Result<(u8, u8, u8), LEDError> {
        for &(id, (ref r, ref g, ref b)) in &self.colors {
            if id == led_id {
                return Ok((r.load(Ordering::SeqCst) as u8, g.load(Ordering::SeqCst) as u8, b.load(Ordering::SeqCst) as u8));
            }
        }
        Err(LEDError::NotFound)
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
