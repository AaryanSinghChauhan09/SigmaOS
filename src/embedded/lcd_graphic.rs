#![no_std]
#![no_main]

/// OOP-based Graphic LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1516
/// Implements graphic LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum Orientation { Portrait = 0, Landscape = 1 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LCDError { Success = 0, NotFound = 1 }

pub trait GraphicLCD {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[repr(C)]
pub struct SimpleGraphicLCD {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleGraphicLCD {
    pub fn new(id: DisplayID, width: u16, height: u16) -> Self {
        SimpleGraphicLCD {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl GraphicLCD for SimpleGraphicLCD {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
}

pub trait GraphicController {
    fn set_orientation(&mut self, display_id: DisplayID, orientation: Orientation) -> Result<(), LCDError>;
    fn draw_rect(&self, display_id: DisplayID, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), LCDError>;
    def fill_screen(&self, display_id: DisplayID, color: u16) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleGraphicController {
    pub displays: Vec<Option<Box<dyn GraphicLCD>>>,
    pub orientations: Vec<(DisplayID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleGraphicController {
    pub fn new() -> Self {
        SimpleGraphicController {
            displays: Vec::new(),
            orientations: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GraphicController for SimpleGraphicController {
    fn set_orientation(&mut self, display_id: DisplayID, orientation: Orientation) -> Result<(), LCDError> {
        self.orientations.push((display_id, AtomicUsize::new(orientation as usize)));
        Ok(())
    }
    
    fn draw_rect(&self, display_id: DisplayID, _x: u16, _y: u16, _w: u16, _h: u16, _color: u16) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn fill_screen(&self, display_id: DisplayID, _color: u16) -> Result<(), LCDError> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn GraphicLCD> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait FontRenderer {
    def draw_char(&self, display_id: DisplayID, x: u16, y: u16, c: u8, color: u16, bg: u16) -> Result<(), LCDError>;
    def draw_string(&self, display_id: DisplayID, x: u16, y: u16, text: &[u8], color: u16, bg: u16) -> Result<(), LCDError>;
}

#[repr(C)]
pub struct SimpleFontRenderer {
    pub controller: SimpleGraphicController,
}

impl SimpleFontRenderer {
    pub fn new(controller: SimpleGraphicController) -> Self {
        SimpleFontRenderer { controller }
    }
}

impl FontRenderer for SimpleFontRenderer {
    fn draw_char(&self, display_id: DisplayID, _x: u16, _y: u16, _c: u8, _color: u16, _bg: u16) -> Result<(), LCDError> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(LCDError::NotFound)
        }
    }
    
    fn draw_string(&self, display_id: DisplayID, _x: u16, _y: u16, _text: &[u8], _color: u16, _bg: u16) -> Result<(), LCDError> {
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
