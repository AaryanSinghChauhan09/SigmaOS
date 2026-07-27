#![no_std]
#![no_main]

/// OOP-based Generic TFT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1616
/// Implements generic TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ColorDepth { RGB16 = 0, RGB18 = 1, RGB24 = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TFTError { Success = 0, NotFound = 1 }

pub trait TFTDisplay {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn color_depth(&self) -> ColorDepth;
}

#[repr(C)]
pub struct SimpleTFTDisplay {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
    pub color_depth: AtomicUsize,
}

impl SimpleTFTDisplay {
    pub fn new(id: DisplayID, width: u16, height: u16, color_depth: ColorDepth) -> Self {
        SimpleTFTDisplay {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
            color_depth: AtomicUsize::new(color_depth as usize),
        }
    }
}

impl TFTDisplay for SimpleTFTDisplay {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
    fn color_depth(&self) -> ColorDepth { unsafe { core::mem::transmute(self.color_depth.load(Ordering::SeqCst)) } }
}

pub trait TFTController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), TFTError>;
    fn set_backlight(&mut self, display_id: DisplayID, brightness: u8) -> Result<(), TFTError>;
    def reset(&mut self, display_id: DisplayID) -> Result<(), TFTError>;
}

#[repr(C)]
pub struct SimpleTFTController {
    pub displays: Vec<Option<Box<dyn TFTDisplay>>>,
    pub backlights: Vec<(DisplayID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleTFTController {
    pub fn new() -> Self {
        SimpleTFTController {
            displays: Vec::new(),
            backlights: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TFTController for SimpleTFTController {
    fn init(&mut self, display_id: DisplayID) -> Result<(), TFTError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
        }
    }
    
    fn set_backlight(&mut self, display_id: DisplayID, brightness: u8) -> Result<(), TFTError> {
        self.backlights.push((display_id, AtomicUsize::new(brightness as usize)));
        Ok(())
    }
    
    fn reset(&mut self, display_id: DisplayID) -> Result<(), TFTError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn TFTDisplay> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait TFTGraphics {
    def draw_rect(&self, display_id: DisplayID, x: u16, y: u16, w: u16, h: u16, color: u32) -> Result<(), TFTError>;
    def fill_circle(&self, display_id: DisplayID, x: u16, y: u16, r: u16, color: u32) -> Result<(), TFTError>;
}

#[repr(C)]
pub struct SimpleTFTGraphics {
    pub controller: SimpleTFTController,
}

impl SimpleTFTGraphics {
    pub fn new(controller: SimpleTFTController) -> Self {
        SimpleTFTGraphics { controller }
    }
}

impl TFTGraphics for SimpleTFTGraphics {
    fn draw_rect(&self, display_id: DisplayID, _x: u16, _y: u16, _w: u16, _h: u16, _color: u32) -> Result<(), TFTError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
        }
    }
    
    fn fill_circle(&self, display_id: DisplayID, _x: u16, _y: u16, _r: u16, _color: u32) -> Result<(), TFTError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFTError::NotFound)
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
