#![no_std]
#![no_main]

/// OOP-based uBlox GPS for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3106
/// Implements uBlox GPS receiver

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type UBloxID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum UBloxError { Success = 0, NotFound = 1 }

pub trait UBloxGPS {
    fn id(&self) -> UBloxID;
    fn has_fix(&self) -> bool;
}

#[repr(C)]
pub struct SimpleUBloxGPS {
    pub id: UBloxID,
    pub fix: AtomicUsize,
}

impl SimpleUBloxGPS {
    pub fn new(id: UBloxID) -> Self {
        SimpleUBloxGPS {
            id,
            fix: AtomicUsize::new(0),
        }
    }
}

impl UBloxGPS for SimpleUBloxGPS {
    fn id(&self) -> UBloxID { self.id }
    fn has_fix(&self) -> bool { self.fix.load(Ordering::SeqCst) == 1 }
}

pub trait UBloxController {
    fn init(&mut self, gps_id: UBloxID) -> Result<(), UBloxError>;
    fn read_ubx(&self, gps_id: UBloxID, buffer: &mut [u8]) -> Result<usize, UBloxError>;
    def get_position(&self, gps_id: UBloxID) -> Result<(f32, f32), UBloxError>;
}

#[repr(C)]
pub struct SimpleUBloxController {
    pub gps_units: Vec<Option<Box<dyn UBloxGPS>>>,
    pub next_id: AtomicUsize,
}

impl SimpleUBloxController {
    pub fn new() -> Self {
        SimpleUBloxController {
            gps_units: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl UBloxController for SimpleUBloxController {
    fn init(&mut self, gps_id: UBloxID) -> Result<(), UBloxError> {
        for gps_option in &mut self.gps_units {
            if let Some(ref mut gps) = *gps_option {
                if gps.id() == gps_id {
                    gps.fix.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(UBloxError::NotFound)
    }
    
    fn read_ubx(&self, gps_id: UBloxID, buffer: &mut [u8]) -> Result<usize, UBloxError> {
        if self.get_gps(gps_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(UBloxError::NotFound)
        }
    }
    
    fn get_position(&self, gps_id: UBloxID) -> Result<(f32, f32), UBloxError> {
        if self.get_gps(gps_id).is_some() {
            Ok((0.0, 0.0))
        } else {
            Err(UBloxError::NotFound)
        }
    }
    
    fn get_gps(&self, id: UBloxID) -> Option<&dyn UBloxGPS> {
        for gps_option in &self.gps_units {
            if let Some(ref gps) = *gps_option {
                if gps.id() == id { return Some(gps.as_ref()); }
            }
        }
        None
    }
}

pub trait UBloxConfig {
    def set_baudrate(&mut self, gps_id: UBloxID, baud: u32) -> Result<(), UBloxError>;
    def set_nav_mode(&mut self, gps_id: UBloxID, mode: u8) -> Result<(), UBloxError>;
}

#[repr(C)]
pub struct SimpleUBloxConfig {
    pub controller: SimpleUBloxController,
    pub baudrates: Vec<(UBloxID, AtomicUsize)>,
}

impl SimpleUBloxConfig {
    pub fn new(controller: SimpleUBloxController) -> Self {
        SimpleUBloxConfig {
            controller,
            baudrates: Vec::new(),
        }
    }
}

impl UBloxConfig for SimpleUBloxConfig {
    fn set_baudrate(&mut self, gps_id: UBloxID, baud: u32) -> Result<(), UBloxError> {
        self.baudrates.push((gps_id, AtomicUsize::new(baud as usize)));
        Ok(())
    }
    
    fn set_nav_mode(&mut self, _gps_id: UBloxID, _mode: u8) -> Result<(), UBloxError> {
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
