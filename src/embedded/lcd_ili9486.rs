#![no_std]
#![no_main]

/// OOP-based ILI9486 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2466
/// Implements ILI9486 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ILI9486ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ILI9486Error { Success = 0, NotFound = 1 }

pub trait ILI9486Display {
    fn id(&self) -> ILI9486ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleILI9486Display {
    pub id: ILI9486ID,
    pub initialized: AtomicUsize,
}

impl SimpleILI9486Display {
    pub fn new(id: ILI9486ID) -> Self {
        SimpleILI9486Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ILI9486Display for SimpleILI9486Display {
    fn id(&self) -> ILI9486ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ILI9486Controller {
    fn init(&mut self, ili_id: ILI9486ID) -> Result<(), ILI9486Error>;
    fn set_window(&self, ili_id: ILI9486ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), ILI9486Error>;
    def push_colors(&self, ili_id: ILI9486ID, colors: &[u16]) -> Result<(), ILI9486Error>;
}

#[repr(C)]
pub struct SimpleILI9486Controller {
    pub displays: Vec<Option<Box<dyn ILI9486Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleILI9486Controller {
    pub fn new() -> Self {
        SimpleILI9486Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ILI9486Controller for SimpleILI9486Controller {
    fn init(&mut self, ili_id: ILI9486ID) -> Result<(), ILI9486Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == ili_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ILI9486Error::NotFound)
    }
    
    fn set_window(&self, ili_id: ILI9486ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), ILI9486Error> {
        if self.get_display(ili_id).is_some() {
            Ok(())
        } else {
            Err(ILI9486Error::NotFound)
        }
    }
    
    fn push_colors(&self, ili_id: ILI9486ID, _colors: &[u16]) -> Result<(), ILI9486Error> {
        if self.get_display(ili_id).is_some() {
            Ok(())
        } else {
            Err(ILI9486Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ILI9486ID) -> Option<&dyn ILI9486Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ILI9486Touch {
    def read_touch(&self, ili_id: ILI9486ID) -> Result<(u16, u16), ILI9486Error>;
    def set_touch_mode(&mut self, ili_id: ILI9486ID, mode: u8) -> Result<(), ILI9486Error>;
}

#[repr(C)]
pub struct SimpleILI9486Touch {
    pub controller: SimpleILI9486Controller,
}

impl SimpleILI9486Touch {
    pub fn new(controller: SimpleILI9486Controller) -> Self {
        SimpleILI9486Touch { controller }
    }
}

impl ILI9486Touch for SimpleILI9486Touch {
    fn read_touch(&self, ili_id: ILI9486ID) -> Result<(u16, u16), ILI9486Error> {
        if self.controller.get_display(ili_id).is_some() {
            Ok((0, 0))
        } else {
            Err(ILI9486Error::NotFound)
        }
    }
    
    fn set_touch_mode(&mut self, _ili_id: ILI9486ID, _mode: u8) -> Result<(), ILI9486Error> {
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
