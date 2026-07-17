#![no_std]
#![no_main]

/// OOP-based TFT 240x320 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3946
/// Implements 240x320 TFT display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TFT240x320ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TFT240x320Error { Success = 0, NotFound = 1 }

pub trait TFT240x320Display {
    fn id(&self) -> TFT240x320ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTFT240x320Display {
    pub id: TFT240x320ID,
    pub initialized: AtomicUsize,
}

impl SimpleTFT240x320Display {
    pub fn new(id: TFT240x320ID) -> Self {
        SimpleTFT240x320Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TFT240x320Display for SimpleTFT240x320Display {
    fn id(&self) -> TFT240x320ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TFT240x320Controller {
    fn init(&mut self, display_id: TFT240x320ID) -> Result<(), TFT240x320Error>;
    fn clear(&self, display_id: TFT240x320ID) -> Result<(), TFT240x320Error>;
    def draw_pixel(&self, display_id: TFT240x320ID, x: u16, y: u16, color: u16) -> Result<(), TFT240x320Error>;
}

#[repr(C)]
pub struct SimpleTFT240x320Controller {
    pub displays: Vec<Option<Box<dyn TFT240x320Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTFT240x320Controller {
    pub fn new() -> Self {
        SimpleTFT240x320Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TFT240x320Controller for SimpleTFT240x320Controller {
    fn init(&mut self, display_id: TFT240x320ID) -> Result<(), TFT240x320Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TFT240x320Error::NotFound)
    }
    
    fn clear(&self, display_id: TFT240x320ID) -> Result<(), TFT240x320Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFT240x320Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: TFT240x320ID, _x: u16, _y: u16, _color: u16) -> Result<(), TFT240x320Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFT240x320Error::NotFound)
        }
    }
    
    fn get_display(&self, id: TFT240x320ID) -> Option<&dyn TFT240x320Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait TFT240x320Rect {
    def fill_rect(&self, display_id: TFT240x320ID, x: u16, y: u16, w: u16, h: u16, color: u16) -> Result<(), TFT240x320Error>;
}

#[repr(C)]
pub struct SimpleTFT240x320Rect {
    pub controller: SimpleTFT240x320Controller,
}

impl SimpleTFT240x320Rect {
    pub fn new(controller: SimpleTFT240x320Controller) -> Self {
        SimpleTFT240x320Rect { controller }
    }
}

impl TFT240x320Rect for SimpleTFT240x320Rect {
    fn fill_rect(&self, display_id: TFT240x320ID, _x: u16, _y: u16, _w: u16, _h: u16, _color: u16) -> Result<(), TFT240x320Error> {
        if self.controller.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFT240x320Error::NotFound)
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
