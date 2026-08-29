#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based E-Ink Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1476
/// Implements E-Ink display controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorMode { BlackWhite = 0, Grayscale = 1, Color = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum EInkError { Success = 0, NotFound = 1 }

pub trait EInkDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn color_mode(&self) -> ColorMode;
}

#[repr(C)]
pub struct SimpleEInkDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub color_mode: AtomicUsize,
}

impl SimpleEInkDisplay {
    pub fn new(id: DisplayID, width: u16, height: u16, color_mode: ColorMode) -> Self {
        SimpleEInkDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            color_mode: AtomicUsize::new(color_mode as usize),
        }
    }
}

impl EInkDisplay for SimpleEInkDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
    fn color_mode(&self) -> ColorMode { unsafe { core::mem::transmute(self.color_mode.load(Ordering::SeqCst)) } }
}

pub trait EInkController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), EInkError>;
    fn update(&self, display_id: DisplayID, buffer: &[u8]) -> Result<(), EInkError>;
    def partial_update(&self, display_id: DisplayID, x: u16, y: u16, width: u16, height: u16, buffer: &[u8]) -> Result<(), EInkError>;
}

#[repr(C)]
pub struct SimpleEInkController {
    pub displays: Vec<Option<Box<dyn EInkDisplay>>>,
    pub next_id: AtomicUsize,
}

impl SimpleEInkController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleEInkController {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl EInkController for SimpleEInkController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn update(&self, display_id: DisplayID, _buffer: &[u8]) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn partial_update(&self, display_id: DisplayID, _x: u16, _y: u16, _width: u16, _height: u16, _buffer: &[u8]) -> Result<(), EInkError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn EInkDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait RefreshController {
    def set_refresh_mode(&mut self, mode: u8);
    def trigger_refresh(&self, display_id: DisplayID) -> Result<(), EInkError>;
}

#[repr(C)]
pub struct SimpleRefreshController {
    pub controller: SimpleEInkController,
    pub refresh_mode: AtomicUsize,
}

impl SimpleRefreshController {
    pub fn new(controller: SimpleEInkController) -> Self {
        SimpleRefreshController {
            controller,
            refresh_mode: AtomicUsize::new(0),
        }
    }
}

impl RefreshController for SimpleRefreshController {
    fn set_refresh_mode(&mut self, mode: u8) {
        self.refresh_mode.store(mode as usize, Ordering::SeqCst);
    }
    
    fn trigger_refresh(&self, display_id: DisplayID) -> Result<(), EInkError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(EInkError::NotFound)
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
