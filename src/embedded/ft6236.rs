#![no_std]
#![no_main]

/// OOP-based FT6236 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2596
/// Implements FT6236 capacitive touch controller

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FT6236ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FT6236Error { Success = 0, NotFound = 1 }

pub trait FT6236Touch {
    fn id(&self) -> FT6236ID;
    fn is_touched(&self) -> bool;
}

#[repr(C)]
pub struct SimpleFT6236Touch {
    pub id: FT6236ID,
    pub touched: AtomicUsize,
}

impl SimpleFT6236Touch {
    pub fn new(id: FT6236ID) -> Self {
        SimpleFT6236Touch {
            id,
            touched: AtomicUsize::new(0),
        }
    }
}

impl FT6236Touch for SimpleFT6236Touch {
    fn id(&self) -> FT6236ID { self.id }
    fn is_touched(&self) -> bool { self.touched.load(Ordering::SeqCst) == 1 }
}

pub trait FT6236Controller {
    fn init(&mut self, ft_id: FT6236ID) -> Result<(), FT6236Error>;
    fn read(&self, ft_id: FT6236ID) -> Result<(u16, u16), FT6236Error>;
    def get_touch_count(&self, ft_id: FT6236ID) -> Result<u8, FT6236Error>;
}

#[repr(C)]
pub struct SimpleFT6236Controller {
    pub touches: Vec<Option<Box<dyn FT6236Touch>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFT6236Controller {
    pub fn new() -> Self {
        SimpleFT6236Controller {
            touches: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FT6236Controller for SimpleFT6236Controller {
    fn init(&mut self, ft_id: FT6236ID) -> Result<(), FT6236Error> {
        for touch_option in &mut self.touches {
            if let Some(ref mut touch) = *touch_option {
                if touch.id() == ft_id {
                    touch.touched.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(FT6236Error::NotFound)
    }
    
    fn read(&self, ft_id: FT6236ID) -> Result<(u16, u16), FT6236Error> {
        if self.get_touch(ft_id).is_some() {
            Ok((0, 0))
        } else {
            Err(FT6236Error::NotFound)
        }
    }
    
    fn get_touch_count(&self, ft_id: FT6236ID) -> Result<u8, FT6236Error> {
        if self.get_touch(ft_id).is_some() {
            Ok(0)
        } else {
            Err(FT6236Error::NotFound)
        }
    }
    
    fn get_touch(&self, id: FT6236ID) -> Option<&dyn FT6236Touch> {
        for touch_option in &self.touches {
            if let Some(ref touch) = *touch_option {
                if touch.id() == id { return Some(touch.as_ref()); }
            }
        }
        None
    }
}

pub trait FT6236Calibration {
    def set_threshold(&mut self, ft_id: FT6236ID, threshold: u8) -> Result<(), FT6236Error>;
    def get_threshold(&self, ft_id: FT6236ID) -> Result<u8, FT6236Error>;
}

#[repr(C)]
pub struct SimpleFT6236Calibration {
    pub controller: SimpleFT6236Controller,
    pub thresholds: Vec<(FT6236ID, AtomicUsize)>,
}

impl SimpleFT6236Calibration {
    pub fn new(controller: SimpleFT6236Controller) -> Self {
        SimpleFT6236Calibration {
            controller,
            thresholds: Vec::new(),
        }
    }
}

impl FT6236Calibration for SimpleFT6236Calibration {
    fn set_threshold(&mut self, ft_id: FT6236ID, threshold: u8) -> Result<(), FT6236Error> {
        self.thresholds.push((ft_id, AtomicUsize::new(threshold as usize)));
        Ok(())
    }
    
    fn get_threshold(&self, ft_id: FT6236ID) -> Result<u8, FT6236Error> {
        for &(id, ref thresh) in &self.thresholds {
            if id == ft_id {
                return Ok(thresh.load(Ordering::SeqCst) as u8);
            }
        }
        Err(FT6236Error::NotFound)
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
