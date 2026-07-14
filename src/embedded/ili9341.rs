#![no_std]
#![no_main]

/// OOP-based ILI9341 TFT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1536
/// Implements ILI9341 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DisplayID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ILI9341Error { Success = 0, NotFound = 1 }

pub trait ILI9341Display {
    fn id(&self) -> DisplayID;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
}

#[repr(C)]
pub struct SimpleILI9341Display {
    pub id: DisplayID,
    pub width: AtomicUsize,
    pub height: AtomicUsize,
}

impl SimpleILI9341Display {
    pub fn new(id: DisplayID, width: u16, height: u16) -> Self {
        SimpleILI9341Display {
            id,
            width: AtomicUsize::new(width as usize),
            height: AtomicUsize::new(height as usize),
        }
    }
}

impl ILI9341Display for SimpleILI9341Display {
    fn id(&self) -> DisplayID { self.id }
    fn width(&self) -> u16 { self.width.load(Ordering::SeqCst) as u16 }
    fn height(&self) -> u16 { self.height.load(Ordering::SeqCst) as u16 }
}

pub trait ILI9341Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ILI9341Error>;
    fn set_rotation(&mut self, display_id: DisplayID, rotation: u8) -> Result<(), ILI9341Error>;
    def fill_screen(&self, display_id: DisplayID, color: u16) -> Result<(), ILI9341Error>;
}

#[repr(C)]
pub struct SimpleILI9341Controller {
    pub displays: Vec<Option<Box<dyn ILI9341Display>>>,
    pub rotations: Vec<(DisplayID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleILI9341Controller {
    pub fn new() -> Self {
        SimpleILI9341Controller {
            displays: Vec::new(),
            rotations: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ILI9341Controller for SimpleILI9341Controller {
    fn init(&mut self, display_id: DisplayID) -> Result<(), ILI9341Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn set_rotation(&mut self, display_id: DisplayID, rotation: u8) -> Result<(), ILI9341Error> {
        self.rotations.push((display_id, AtomicUsize::new(rotation as usize)));
        Ok(())
    }
    
    fn fill_screen(&self, display_id: DisplayID, _color: u16) -> Result<(), ILI9341Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn get_display(&self, id: DisplayID) -> Option<&dyn ILI9341Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ILI9341Graphics {
    def draw_pixel(&self, display_id: DisplayID, x: u16, y: u16, color: u16) -> Result<(), ILI9341Error>;
    def draw_line(&self, display_id: DisplayID, x1: u16, y1: u16, x2: u16, y2: u16, color: u16) -> Result<(), ILI9341Error>;
}

#[repr(C)]
pub struct SimpleILI9341Graphics {
    pub controller: SimpleILI9341Controller,
}

impl SimpleILI9341Graphics {
    pub fn new(controller: SimpleILI9341Controller) -> Self {
        SimpleILI9341Graphics { controller }
    }
}

impl ILI9341Graphics for SimpleILI9341Graphics {
    fn draw_pixel(&self, display_id: DisplayID, _x: u16, _y: u16, _color: u16) -> Result<(), ILI9341Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn draw_line(&self, display_id: DisplayID, _x1: u16, _y1: u16, _x2: u16, _y2: u16, _color: u16) -> Result<(), ILI9341Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
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
