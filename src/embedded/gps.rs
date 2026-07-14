#![no_std]
#![no_main]

/// OOP-based GPS for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1706
/// Implements GPS module

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GPSID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPSFix { NoFix = 0, Fix2D = 1, Fix3D = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GPSError { Success = 0, NotFound = 1 }

pub trait GPS {
    fn id(&self) -> GPSID;
    fn fix(&self) -> GPSFix;
}

#[repr(C)]
pub struct SimpleGPS {
    pub id: GPSID,
    pub fix: AtomicUsize,
}

impl SimpleGPS {
    pub fn new(id: GPSID) -> Self {
        SimpleGPS {
            id,
            fix: AtomicUsize::new(GPSFix::NoFix as usize),
        }
    }
}

impl GPS for SimpleGPS {
    fn id(&self) -> GPSID { self.id }
    fn fix(&self) -> GPSFix { unsafe { core::mem::transmute(self.fix.load(Ordering::SeqCst)) } }
}

pub trait GPSController {
    fn read_position(&self, gps_id: GPSID) -> Result<(f32, f32), GPSError>;
    def read_altitude(&self, gps_id: GPSID) -> Result<f32, GPSError>;
}

#[repr(C)]
pub struct SimpleGPSController {
    pub gps_modules: Vec<Option<Box<dyn GPS>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGPSController {
    pub fn new() -> Self {
        SimpleGPSController {
            gps_modules: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GPSController for SimpleGPSController {
    fn read_position(&self, gps_id: GPSID) -> Result<(f32, f32), GPSError> {
        if self.get_gps(gps_id).is_some() {
            Ok((0.0, 0.0))
        } else {
            Err(GPSError::NotFound)
        }
    }
    
    fn read_altitude(&self, gps_id: GPSID) -> Result<f32, GPSError> {
        if self.get_gps(gps_id).is_some() {
            Ok(0.0)
        } else {
            Err(GPSError::NotFound)
        }
    }
    
    fn get_gps(&self, id: GPSID) -> Option<&dyn GPS> {
        for gps_option in &self.gps_modules {
            if let Some(ref gps) = *gps_option {
                if gps.id() == id { return Some(gps.as_ref()); }
            }
        }
        None
    }
}

pub trait NMEA {
    def parse_nmea(&self, nmea: &[u8]) -> Result<(f32, f32, f32), GPSError>;
    def get_time(&self, gps_id: GPSID) -> Result<(u8, u8, u8), GPSError>;
}

#[repr(C)]
pub struct SimpleNMEA {
    pub controller: SimpleGPSController,
}

impl SimpleNMEA {
    pub fn new(controller: SimpleGPSController) -> Self {
        SimpleNMEA { controller }
    }
}

impl NMEA for SimpleNMEA {
    fn parse_nmea(&self, _nmea: &[u8]) -> Result<(f32, f32, f32), GPSError> {
        Ok((0.0, 0.0, 0.0))
    }
    
    fn get_time(&self, gps_id: GPSID) -> Result<(u8, u8, u8), GPSError> {
        if self.controller.get_gps(gps_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(GPSError::NotFound)
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
