#![no_std]
#![no_main]

/// OOP-based SSD1351 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2586
/// Implements SSD1351 OLED display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SSD1351ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SSD1351Error { Success = 0, NotFound = 1 }

pub trait SSD1351Display {
    fn id(&self) -> SSD1351ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSSD1351Display {
    pub id: SSD1351ID,
    pub initialized: AtomicUsize,
}

impl SimpleSSD1351Display {
    pub fn new(id: SSD1351ID) -> Self {
        SimpleSSD1351Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SSD1351Display for SimpleSSD1351Display {
    fn id(&self) -> SSD1351ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SSD1351Controller {
    fn init(&mut self, ssd_id: SSD1351ID) -> Result<(), SSD1351Error>;
    fn set_window(&self, ssd_id: SSD1351ID, x0: u8, y0: u8, x1: u8, y1: u8) -> Result<(), SSD1351Error>;
    def push_colors(&self, ssd_id: SSD1351ID, colors: &[u16]) -> Result<(), SSD1351Error>;
}

#[repr(C)]
pub struct SimpleSSD1351Controller {
    pub displays: Vec<Option<Box<dyn SSD1351Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSSD1351Controller {
    pub fn new() -> Self {
        SimpleSSD1351Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SSD1351Controller for SimpleSSD1351Controller {
    fn init(&mut self, ssd_id: SSD1351ID) -> Result<(), SSD1351Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == ssd_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SSD1351Error::NotFound)
    }
    
    fn set_window(&self, ssd_id: SSD1351ID, _x0: u8, _y0: u8, _x1: u8, _y1: u8) -> Result<(), SSD1351Error> {
        if self.get_display(ssd_id).is_some() {
            Ok(())
        } else {
            Err(SSD1351Error::NotFound)
        }
    }
    
    fn push_colors(&self, ssd_id: SSD1351ID, _colors: &[u16]) -> Result<(), SSD1351Error> {
        if self.get_display(ssd_id).is_some() {
            Ok(())
        } else {
            Err(SSD1351Error::NotFound)
        }
    }
    
    fn get_display(&self, id: SSD1351ID) -> Option<&dyn SSD1351Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SSD1351Scroll {
    def scroll(&mut self, ssd_id: SSD1351ID, direction: u8, lines: u8) -> Result<(), SSD1351Error>;
    def stop_scroll(&mut self, ssd_id: SSD1351ID) -> Result<(), SSD1351Error>;
}

#[repr(C)]
pub struct SimpleSSD1351Scroll {
    pub controller: SimpleSSD1351Controller,
}

impl SimpleSSD1351Scroll {
    pub fn new(controller: SimpleSSD1351Controller) -> Self {
        SimpleSSD1351Scroll { controller }
    }
}

impl SSD1351Scroll for SimpleSSD1351Scroll {
    fn scroll(&mut self, _ssd_id: SSD1351ID, _direction: u8, _lines: u8) -> Result<(), SSD1351Error> {
        Ok(())
    }
    
    fn stop_scroll(&mut self, _ssd_id: SSD1351ID) -> Result<(), SSD1351Error> {
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
