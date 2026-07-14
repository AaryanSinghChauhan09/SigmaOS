#![no_std]
#![no_main]

/// OOP-based TFT 320x480 Display for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3956
/// Implements 320x480 TFT display

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TFT320x480ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TFT320x480Error { Success = 0, NotFound = 1 }

pub trait TFT320x480Display {
    fn id(&self) -> TFT320x480ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTFT320x480Display {
    pub id: TFT320x480ID,
    pub initialized: AtomicUsize,
}

impl SimpleTFT320x480Display {
    pub fn new(id: TFT320x480ID) -> Self {
        SimpleTFT320x480Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TFT320x480Display for SimpleTFT320x480Display {
    fn id(&self) -> TFT320x480ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TFT320x480Controller {
    fn init(&mut self, display_id: TFT320x480ID) -> Result<(), TFT320x480Error>;
    fn clear(&self, display_id: TFT320x480ID) -> Result<(), TFT320x480Error>;
    def draw_pixel(&self, display_id: TFT320x480ID, x: u16, y: u16, color: u16) -> Result<(), TFT320x480Error>;
}

#[repr(C)]
pub struct SimpleTFT320x480Controller {
    pub displays: Vec<Option<Box<dyn TFT320x480Display>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTFT320x480Controller {
    pub fn new() -> Self {
        SimpleTFT320x480Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TFT320x480Controller for SimpleTFT320x480Controller {
    fn init(&mut self, display_id: TFT320x480ID) -> Result<(), TFT320x480Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == display_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TFT320x480Error::NotFound)
    }
    
    fn clear(&self, display_id: TFT320x480ID) -> Result<(), TFT320x480Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFT320x480Error::NotFound)
        }
    }
    
    fn draw_pixel(&self, display_id: TFT320x480ID, _x: u16, _y: u16, _color: u16) -> Result<(), TFT320x480Error> {
        if self.get_display(display_id).is_some() {
            Ok(())
        } else {
            Err(TFT320x480Error::NotFound)
        }
    }
    
    fn get_display(&self, id: TFT320x480ID) -> Option<&dyn TFT320x480Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait TFT320x480Rotation {
    def set_rotation(&mut self, display_id: TFT320x480ID, rotation: u8) -> Result<(), TFT320x480Error>;
}

#[repr(C)]
pub struct SimpleTFT320x480Rotation {
    pub controller: SimpleTFT320x480Controller,
    pub rotations: Vec<(TFT320x480ID, AtomicUsize)>,
}

impl SimpleTFT320x480Rotation {
    pub fn new(controller: SimpleTFT320x480Controller) -> Self {
        SimpleTFT320x480Rotation {
            controller,
            rotations: Vec::new(),
        }
    }
}

impl TFT320x480Rotation for SimpleTFT320x480Rotation {
    fn set_rotation(&mut self, display_id: TFT320x480ID, rotation: u8) -> Result<(), TFT320x480Error> {
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
