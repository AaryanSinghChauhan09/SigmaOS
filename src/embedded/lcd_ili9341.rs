#![no_std]
#![no_main]

/// OOP-based ILI9341 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2556
/// Implements ILI9341 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ILI9341ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ILI9341Error { Success = 0, NotFound = 1 }

pub trait ILI9341Display {
    fn id(&self) -> ILI9341ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleILI9341Display {
    pub id: ILI9341ID,
    pub initialized: AtomicUsize,
}

impl SimpleILI9341Display {
    pub fn new(id: ILI9341ID) -> Self {
        SimpleILI9341Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ILI9341Display for SimpleILI9341Display {
    fn id(&self) -> ILI9341ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ILI9341Controller {
    fn init(&mut self, ili_id: ILI9341ID) -> Result<(), ILI9341Error>;
    fn set_window(&self, ili_id: ILI9341ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), ILI9341Error>;
    def push_colors(&self, ili_id: ILI9341ID, colors: &[u16]) -> Result<(), ILI9341Error>;
}

#[repr(C)]
pub struct SimpleILI9341Controller {
    pub displays: Vec<Option<Box<dyn ILI9341Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleILI9341Controller {
    pub fn new() -> Self {
        SimpleILI9341Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ILI9341Controller for SimpleILI9341Controller {
    fn init(&mut self, ili_id: ILI9341ID) -> Result<(), ILI9341Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == ili_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ILI9341Error::NotFound)
    }
    
    fn set_window(&self, ili_id: ILI9341ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), ILI9341Error> {
        if self.get_display(ili_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn push_colors(&self, ili_id: ILI9341ID, _colors: &[u16]) -> Result<(), ILI9341Error> {
        if self.get_display(ili_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn get_display(&self, id: ILI9341ID) -> Option<&dyn ILI9341Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait ILI9341Scroll {
    def scroll(&mut self, ili_id: ILI9341ID, y: u16) -> Result<(), ILI9341Error>;
    def stop_scroll(&mut self, ili_id: ILI9341ID) -> Result<(), ILI9341Error>;
}

#[repr(C)]
pub struct SimpleILI9341Scroll {
    pub controller: SimpleILI9341Controller,
}

impl SimpleILI9341Scroll {
    pub fn new(controller: SimpleILI9341Controller) -> Self {
        SimpleILI9341Scroll { controller }
    }
}

impl ILI9341Scroll for SimpleILI9341Scroll {
    fn scroll(&mut self, _ili_id: ILI9341ID, _y: u16) -> Result<(), ILI9341Error> {
        Ok(())
    }
    
    fn stop_scroll(&mut self, _ili_id: ILI9341ID) -> Result<(), ILI9341Error> {
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
