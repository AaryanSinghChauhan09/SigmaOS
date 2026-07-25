#![no_std]
#![no_main]

/// OOP-based SH1106 OLED for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1576
/// Implements SH1106 OLED controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SH1106Error { Success = 0, NotFound = 1 }

pub trait SH1106Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u8;
    fn height(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSH1106Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleSH1106Display {
    pub fn new(id: DisplayID, width: u8, height: u8) -> Self {
        SimpleSH1106Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl SH1106Display for SimpleSH1106Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u8 { self.width.load(Ordering::SeqCst) as u8 }
    fn height(&self) -> u8 { self.height.load(Ordering::SeqCst) as u8 }
}

pub trait SH1106Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), SH1106Error>;
    fn set_contrast(&mut self, display_id: DisplayID, contrast: u8) -> Result<(), SH1106Error>;
    def invert(&mut self, display_id: DisplayID, invert: bool) -> Result<(), SH1106Error>;
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
    fn init(&mut self, display_id: DisplayID) -> Result<(), SH1106Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn set_contrast(&mut self, display_id: DisplayID, _contrast: u8) -> Result<(), SH1106Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn invert(&mut self, display_id: DisplayID, _invert: bool) -> Result<(), SH1106Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn SH1106Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait SH1106Scroll {
    def scroll_right(&mut self, display_id: DisplayID, start: u8, end: u8) -> Result<(), SH1106Error>;
    def stop_scroll(&mut self, display_id: DisplayID) -> Result<(), SH1106Error>;
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
    fn scroll_right(&mut self, display_id: DisplayID, _start: u8, _end: u8) -> Result<(), SH1106Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
        }
    }
    
    fn stop_scroll(&mut self, display_id: DisplayID) -> Result<(), SH1106Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(SH1106Error::NotFound)
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
