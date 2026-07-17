#![no_std]
#![no_main]

/// OOP-based GDE0432 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2506
/// Implements GDE0432 e-paper display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GDE0432ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GDE0432Error { Success = 0, NotFound = 1 }

pub trait GDE0432Display {
    fn id(&self) -> GDE0432ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleGDE0432Display {
    pub id: GDE0432ID,
    pub initialized: AtomicUsize,
}

impl SimpleGDE0432Display {
    pub fn new(id: GDE0432ID) -> Self {
        SimpleGDE0432Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl GDE0432Display for SimpleGDE0432Display {
    fn id(&self) -> GDE0432ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait GDE0432Controller {
    fn init(&mut self, gde_id: GDE0432ID) -> Result<(), GDE0432Error>;
    fn update(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error>;
    def clear(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error>;
}

#[repr(C)]
pub struct SimpleGDE0432Controller {
    pub displays: Vec<Option<Box<dyn GDE0432Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGDE0432Controller {
    pub fn new() -> Self {
        SimpleGDE0432Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GDE0432Controller for SimpleGDE0432Controller {
    fn init(&mut self, gde_id: GDE0432ID) -> Result<(), GDE0432Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == gde_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(GDE0432Error::NotFound)
    }
    
    fn update(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error> {
        if self.get_display(gde_id).is_some() {
            Ok(())
        } else {
            Err(GDE0432Error::NotFound)
        }
    }
    
    fn clear(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error> {
        if self.get_display(gde_id).is_some() {
            Ok(())
        } else {
            Err(GDE0432Error::NotFound)
        }
    }
    
    fn get_display(&self, id: GDE0432ID) -> Option<&dyn GDE0432Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait GDE0432Partial {
    def set_window(&self, gde_id: GDE0432ID, x: u16, y: u16, w: u16, h: u16) -> Result<(), GDE0432Error>;
    def update_partial(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error>;
}

#[repr(C)]
pub struct SimpleGDE0432Partial {
    pub controller: SimpleGDE0432Controller,
}

impl SimpleGDE0432Partial {
    pub fn new(controller: SimpleGDE0432Controller) -> Self {
        SimpleGDE0432Partial { controller }
    }
}

impl GDE0432Partial for SimpleGDE0432Partial {
    fn set_window(&self, gde_id: GDE0432ID, _x: u16, _y: u16, _w: u16, _h: u16) -> Result<(), GDE0432Error> {
        if self.controller.get_display(gde_id).is_some() {
            Ok(())
        } else {
            Err(GDE0432Error::NotFound)
        }
    }
    
    fn update_partial(&self, gde_id: GDE0432ID) -> Result<(), GDE0432Error> {
        if self.controller.get_display(gde_id).is_some() {
            Ok(())
        } else {
            Err(GDE0432Error::NotFound)
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
