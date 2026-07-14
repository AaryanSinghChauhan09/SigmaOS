#![no_std]
#![no_main]

/// OOP-based LCD TFT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2436
/// Implements LCD TFT display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TFTID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TFTError { Success = 0, NotFound = 1 }

pub trait LCDDisplay {
    fn id(&self) -> TFTID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLCDDisplay {
    pub id: TFTID,
    pub initialized: AtomicUsize,
}

impl SimpleLCDDisplay {
    pub fn new(id: TFTID) -> Self {
        SimpleLCDDisplay {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl LCDDisplay for SimpleLCDDisplay {
    fn id(&self) -> TFTID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait LCDController {
    fn init(&mut self, tft_id: TFTID) -> Result<(), TFTError>;
    fn set_pixel(&self, tft_id: TFTID, x: u16, y: u16, color: u16) -> Result<(), TFTError>;
    def clear(&self, tft_id: TFTID, color: u16) -> Result<(), TFTError>;
}

#[repr(C)]
pub struct SimpleLCDController {
    pub displays: Vec<Option<Box<dyn LCDDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLCDController {
    pub fn new() -> Self {
        SimpleLCDController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LCDController for SimpleLCDController {
    fn init(&mut self, tft_id: TFTID) -> Result<(), TFTError> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == tft_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TFTError::NotFound)
    }
    
    fn set_pixel(&self, tft_id: TFTID, _x: u16, _y: u16, _color: u16) -> Result<(), TFTError> {
        if self.get_display(tft_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
        }
    }
    
    fn clear(&self, tft_id: TFTID, _color: u16) -> Result<(), TFTError> {
        if self.get_display(tft_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
        }
    }
    
    fn get_display(&self, id: TFTID) -> Option<&dyn LCDDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait LCDBacklight {
    def set_backlight(&mut self, tft_id: TFTID, brightness: u8) -> Result<(), TFTError>;
    def get_backlight(&self, tft_id: TFTID) -> Result<u8, TFTError>;
}

#[repr(C)]
pub struct SimpleLCDBacklight {
    pub controller: SimpleLCDController,
    pub backlights: Vec<(TFTID, AtomicUsize)>,
}

impl SimpleLCDBacklight {
    pub fn new(controller: SimpleLCDController) -> Self {
        SimpleLCDBacklight {
            controller,
            backlights: Vec::new(),
        }
    }
}

impl LCDBacklight for SimpleLCDBacklight {
    fn set_backlight(&mut self, tft_id: TFTID, brightness: u8) -> Result<(), TFTError> {
        self.backlights.push((tft_id, AtomicUsize::new(brightness as usize)));
        Ok(())
    }
    
    fn get_backlight(&self, tft_id: TFTID) -> Result<u8, TFTError> {
        for &(id, ref bl) in &self.backlights {
            if id == tft_id {
                return Ok(bl.load(Ordering::SeqCst) as u8);
            }
        }
        Err(TFTError::NotFound)
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
