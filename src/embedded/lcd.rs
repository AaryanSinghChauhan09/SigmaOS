#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based LCD Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1246
/// Implements LCD display controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorFormat { RGB565 = 0, RGB888 = 1, ARGB8888 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LCDError { Success = 0, NotFound = 1 }

pub trait LCDDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn color_format(&self) -> ColorFormat;
}

#[repr(C)]
pub struct SimpleLCDDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub color_format: AtomicUsize,
}

impl SimpleLCDDisplay {
    pub fn new(id: DisplayID, width: u16, height: u16, color_format: ColorFormat) -> Self {
        SimpleLCDDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            color_format: AtomicUsize::new(color_format as usize),
        }
    }
}

impl LCDDisplay for SimpleLCDDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
    fn color_format(&self) -> ColorFormat { unsafe { core::mem::transmute(self.color_format.load(Ordering::SeqCst)) } }
}

pub trait LCDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), LCDError>;
    fn clear(&self, display_id: DisplayID, color: u32) -> Result<(), LCDError>;
    def draw_pixel(&self, display_id: DisplayID, x: u16, y: u16, color: u32) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleLCDController {
    pub displays: Vec<Option<Box<dyn LCDDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLCDController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleLCDController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LCDController for SimpleLCDController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn clear(&self, display_id: DisplayID, _color: u32) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: DisplayID, _x: u16, _y: u16, _color: u32) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn LCDDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait Framebuffer {
    def get_buffer(&self, display_id: DisplayID) -> Option<&[u8]>;
    def flush(&self, display_id: DisplayID) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleFramebuffer {
    pub controller: SimpleLCDController,
    pub buffers: Vec<(DisplayID, Vec<u8>)>,
}

impl SimpleFramebuffer {
    pub fn new(controller: SimpleLCDController) -> Self {
        SimpleFramebuffer {
            controller,
            buffers: Vec::new(),
        }
    }
}

impl Framebuffer for SimpleFramebuffer {
    fn get_buffer(&self, display_id: DisplayID) -> Option<&[u8]> {
        for &(id, ref buffer) in &self.buffers {
            if id == display_id {
                return Some(buffer);
            }
        }
        None
    }
    
    fn flush(&self, display_id: DisplayID) -> Result<(), LCDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
