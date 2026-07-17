#![no_std]
#![no_main]

/// OOP-based GC9A01 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2496
/// Implements GC9A01 TFT LCD controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GC9A01ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GC9A01Error { Success = 0, NotFound = 1 }

pub trait GC9A01Display {
    fn id(&self) -> GC9A01ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleGC9A01Display {
    pub id: GC9A01ID,
    pub initialized: AtomicUsize,
}

impl SimpleGC9A01Display {
    pub fn new(id: GC9A01ID) -> Self {
        SimpleGC9A01Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl GC9A01Display for SimpleGC9A01Display {
    fn id(&self) -> GC9A01ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait GC9A01Controller {
    fn init(&mut self, gc_id: GC9A01ID) -> Result<(), GC9A01Error>;
    fn set_window(&self, gc_id: GC9A01ID, x0: u16, y0: u16, x1: u16, y1: u16) -> Result<(), GC9A01Error>;
    def push_colors(&self, gc_id: GC9A01ID, colors: &[u16]) -> Result<(), GC9A01Error>;
}

#[repr(C)]
pub struct SimpleGC9A01Controller {
    pub displays: Vec<Option<Box<dyn GC9A01Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGC9A01Controller {
    pub fn new() -> Self {
        SimpleGC9A01Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GC9A01Controller for SimpleGC9A01Controller {
    fn init(&mut self, gc_id: GC9A01ID) -> Result<(), GC9A01Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == gc_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(GC9A01Error::NotFound)
    }
    
    fn set_window(&self, gc_id: GC9A01ID, _x0: u16, _y0: u16, _x1: u16, _y1: u16) -> Result<(), GC9A01Error> {
        if self.get_display(gc_id).is_some() {
            Ok(())
        } else {
            Err(GC9A01Error::NotFound)
        }
    }
    
    fn push_colors(&self, gc_id: GC9A01ID, _colors: &[u16]) -> Result<(), GC9A01Error> {
        if self.get_display(gc_id).is_some() {
            Ok(())
        } else {
            Err(GC9A01Error::NotFound)
        }
    }
    
    fn get_display(&self, id: GC9A01ID) -> Option<&dyn GC9A01Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait GC9A01RGB {
    def set_rgb_mode(&mut self, gc_id: GC9A01ID, rgb: bool) -> Result<(), GC9A01Error>;
    def get_rgb_mode(&self, gc_id: GC9A01ID) -> Result<bool, GC9A01Error>;
}

#[repr(C)]
pub struct SimpleGC9A01RGB {
    pub controller: SimpleGC9A01Controller,
    pub rgb_modes: Vec<(GC9A01ID, AtomicUsize)>,
}

impl SimpleGC9A01RGB {
    pub fn new(controller: SimpleGC9A01Controller) -> Self {
        SimpleGC9A01RGB {
            controller,
            rgb_modes: Vec::new(),
        }
    }
}

impl GC9A01RGB for SimpleGC9A01RGB {
    fn set_rgb_mode(&mut self, gc_id: GC9A01ID, rgb: bool) -> Result<(), GC9A01Error> {
        self.rgb_modes.push((gc_id, AtomicUsize::new(if rgb { 1 } else { 0 })));
        Ok(())
    }
    
    fn get_rgb_mode(&self, gc_id: GC9A01ID) -> Result<bool, GC9A01Error> {
        for &(id, ref mode) in &self.rgb_modes {
            if id == gc_id {
                return Ok(mode.load(Ordering::SeqCst) == 1);
            }
        }
        Err(GC9A01Error::NotFound)
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
