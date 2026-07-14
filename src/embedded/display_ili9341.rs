#![no_std]
#![no_main]

/// OOP-based ILI9341 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3266
/// Implements ILI9341 TFT display

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
    fn init(&mut self, display_id: ILI9341ID) -> Result<(), ILI9341Error>;
    fn clear(&self, display_id: ILI9341ID, color: u16) -> Result<(), ILI9341Error>;
    def draw_pixel(&self, display_id: ILI9341ID, x: u16, y: u16, color: u16) -> Result<(), ILI9341Error>;
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
    fn init(&mut self, display_id: ILI9341ID) -> Result<(), ILI9341Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ILI9341Error::NotFound)
    }
    
    fn clear(&self, display_id: ILI9341ID, _color: u16) -> Result<(), ILI9341Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(ILI9341Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: ILI9341ID, _x: u16, _y: u16, _color: u16) -> Result<(), ILI9341Error> {
        if self.get_display(display_id).is_some() {
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

pub trait ILI9341Rotation {
    def set_rotation(&mut self, display_id: ILI9341ID, rotation: u8) -> Result<(), ILI9341Error>;
}

#[repr(C)]
pub struct SimpleILI9341Rotation {
    pub controller: SimpleILI9341Controller,
    pub rotations: Vec<(ILI9341ID, AtomicUsize)>,
}

impl SimpleILI9341Rotation {
    pub fn new(controller: SimpleILI9341Controller) -> Self {
        SimpleILI9341Rotation {
            controller,
            rotations: Vec::new(),
        }
    }
}

impl ILI9341Rotation for SimpleILI9341Rotation {
    fn set_rotation(&mut self, display_id: ILI9341ID, rotation: u8) -> Result<(), ILI9341Error> {
        self.rotations.push((display_id, AtomicUsize::new(rotation as usize)));
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
