#![no_std]
#![no_main]

/// OOP-based SH1106 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2526
/// Implements SH1106 OLED display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SH1106ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SH1106Error { Success = 0, NotFound = 1 }

pub trait SH1106Display {
    fn id(&self) -> SH1106ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSH1106Display {
    pub id: SH1106ID,
    pub initialized: AtomicUsize,
}

impl SimpleSH1106Display {
    pub fn new(id: SH1106ID) -> Self {
        SimpleSH1106Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SH1106Display for SimpleSH1106Display {
    fn id(&self) -> SH1106ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SH1106Controller {
    fn init(&mut self, sh_id: SH1106ID) -> Result<(), SH1106Error>;
    fn set_pixel(&self, sh_id: SH1106ID, x: u8, y: u8, on: bool) -> Result<(), SH1106Error>;
    def display(&self, sh_id: SH1106ID) -> Result<(), SH1106Error>;
}

#[repr(C)]
pub struct SimpleSH1106Controller {
    pub displays: Vec<Option<Box<dyn SH1106Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSH1106Controller {
    pub fn new() -> Self {
        SimpleSH1106Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SH1106Controller for SimpleSH1106Controller {
    fn init(&mut self, sh_id: SH1106ID) -> Result<(), SH1106Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == sh_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SH1106Error::NotFound)
    }
    
    fn set_pixel(&self, sh_id: SH1106ID, _x: u8, _y: u8, _on: bool) -> Result<(), SH1106Error> {
        if self.get_display(sh_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn display(&self, sh_id: SH1106ID) -> Result<(), SH1106Error> {
        if self.get_display(sh_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn get_display(&self, id: SH1106ID) -> Option<&dyn SH1106Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SH1106Scroll {
    def scroll_left(&mut self, sh_id: SH1106ID, start: u8, end: u8) -> Result<(), SH1106Error>;
    def stop_scroll(&mut self, sh_id: SH1106ID) -> Result<(), SH1106Error>;
}

#[repr(C)]
pub struct SimpleSH1106Scroll {
    pub controller: SimpleSH1106Controller,
}

impl SimpleSH1106Scroll {
    pub fn new(controller: SimpleSH1106Controller) -> Self {
        SimpleSH1106Scroll { controller }
    }
}

impl SH1106Scroll for SimpleSH1106Scroll {
    fn scroll_left(&mut self, _sh_id: SH1106ID, _start: u8, _end: u8) -> Result<(), SH1106Error> {
        Ok(())
    }
    
    fn stop_scroll(&mut self, _sh_id: SH1106ID) -> Result<(), SH1106Error> {
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
