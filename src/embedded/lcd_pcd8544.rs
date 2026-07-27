#![no_std]
#![no_main]

/// OOP-based PCD8544 LCD for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2536
/// Implements PCD8544 Nokia 5110 LCD

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCD8544ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCD8544Error { Success = 0, NotFound = 1 }

pub trait PCD8544Display {
    fn id(&self) -> PCD8544ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCD8544Display {
    pub id: PCD8544ID,
    pub initialized: AtomicUsize,
}

impl SimplePCD8544Display {
    pub fn new(id: PCD8544ID) -> Self {
        SimplePCD8544Display {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCD8544Display for SimplePCD8544Display {
    fn id(&self) -> PCD8544ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCD8544Controller {
    fn init(&mut self, pcd_id: PCD8544ID) -> Result<(), PCD8544Error>;
    fn set_pixel(&self, pcd_id: PCD8544ID, x: u8, y: u8, on: bool) -> Result<(), PCD8544Error>;
    def display(&self, pcd_id: PCD8544ID) -> Result<(), PCD8544Error>;
}

#[repr(C)]
pub struct SimplePCD8544Controller {
    pub displays: Vec<Option<Box<dyn PCD8544Display>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCD8544Controller {
    pub fn new() -> Self {
        SimplePCD8544Controller {
            displays: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCD8544Controller for SimplePCD8544Controller {
    fn init(&mut self, pcd_id: PCD8544ID) -> Result<(), PCD8544Error> {
        for display_option in &mut self.displays {
            if let Some(ref mut display) = *display_option {
                if display.id() == pcd_id {
                    display.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCD8544Error::NotFound)
    }
    
    fn set_pixel(&self, pcd_id: PCD8544ID, _x: u8, _y: u8, _on: bool) -> Result<(), PCD8544Error> {
        if self.get_display(pcd_id).is_some() {
            Ok(())
        } else {
            Err(PCD8544Error::NotFound)
        }
    }
    
    fn display(&self, pcd_id: PCD8544ID) -> Result<(), PCD8544Error> {
        if self.get_display(pcd_id).is_some() {
            Ok(())
        } else {
            Err(PCD8544Error::NotFound)
        }
    }
    
    fn get_display(&self, id: PCD8544ID) -> Option<&dyn PCD8544Display> {
        for display_option in &self.displays {
            if let Some(ref display) = *display_option {
                if display.id() == id { return Some(display.as_ref()); }
            }
        }
        None
    }
}

pub trait PCD8544Contrast {
    def set_contrast(&mut self, pcd_id: PCD8544ID, contrast: u8) -> Result<(), PCD8544Error>;
    def get_contrast(&self, pcd_id: PCD8544ID) -> Result<u8, PCD8544Error>;
}

#[repr(C)]
pub struct SimplePCD8544Contrast {
    pub controller: SimplePCD8544Controller,
    pub contrasts: Vec<(PCD8544ID, AtomicUsize)>,
}

impl SimplePCD8544Contrast {
    pub fn new(controller: SimplePCD8544Controller) -> Self {
        SimplePCD8544Contrast {
            controller,
            contrasts: Vec::new(),
        }
    }
}

impl PCD8544Contrast for SimplePCD8544Contrast {
    fn set_contrast(&mut self, pcd_id: PCD8544ID, contrast: u8) -> Result<(), PCD8544Error> {
        self.contrasts.push((pcd_id, AtomicUsize::new(contrast as usize)));
        Ok(())
    }
    
    fn get_contrast(&self, pcd_id: PCD8544ID) -> Result<u8, PCD8544Error> {
        for &(id, ref contrast) in &self.contrasts {
            if id == pcd_id {
                return Ok(contrast.load(Ordering::SeqCst) as u8);
            }
        }
        Err(PCD8544Error::NotFound)
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
