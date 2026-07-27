#![no_std]
#![no_main]

/// OOP-based Ozone Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1936
/// Implements ozone sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type OzoneID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum OzoneError { Success = 0, NotFound = 1 }

pub trait OzoneSensor {
    fn id(&self) -> OzoneID;
    fn ozone_ppb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleOzoneSensor {
    pub id: OzoneID,
    pub ozone_ppb: AtomicUsize,
}

impl SimpleOzoneSensor {
    pub fn new(id: OzoneID) -> Self {
        SimpleOzoneSensor {
            id,
            ozone_ppb: AtomicUsize::new(0),
        }
    }
}

impl OzoneSensor for SimpleOzoneSensor {
    fn id(&self) -> OzoneID { self.id }
    fn ozone_ppb(&self) -> u16 { self.ozone_ppb.load(Ordering::SeqCst) as u16 }
}

pub trait OzoneController {
    fn read(&self, sensor_id: OzoneID) -> Result<u16, OzoneError>;
    def calibrate(&mut self, sensor_id: OzoneID) -> Result<(), OzoneError>;
}

#[repr(C)]
pub struct SimpleOzoneController {
    pub sensors: Vec<Option<Box<dyn OzoneSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleOzoneController {
    pub fn new() -> Self {
        SimpleOzoneController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl OzoneController for SimpleOzoneController {
    fn read(&self, sensor_id: OzoneID) -> Result<u16, OzoneError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.ozone_ppb());
                }
            }
        }
        Err(OzoneError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: OzoneID) -> Result<(), OzoneError> {
        Ok(())
    }
}

pub trait OzoneAlert {
    def set_limit(&mut self, sensor_id: OzoneID, limit: u16) -> Result<(), OzoneError>;
    def is_exceeded(&self, sensor_id: OzoneID) -> Result<bool, OzoneError>;
}

#[repr(C)]
pub struct SimpleOzoneAlert {
    pub controller: SimpleOzoneController,
    pub limits: Vec<(AtomicUsize, AtomicUsize)>,
}

impl SimpleOzoneAlert {
    pub fn new(controller: SimpleOzoneController) -> Self {
        SimpleOzoneAlert {
            controller,
            limits: Vec::new(),
        }
    }
}

impl OzoneAlert for SimpleOzoneAlert {
    fn set_limit(&mut self, sensor_id: OzoneID, limit: u16) -> Result<(), OzoneError> {
        self.limits.push((AtomicUsize::new(sensor_id), AtomicUsize::new(limit as usize)));
        Ok(())
    }
    
    fn is_exceeded(&self, sensor_id: OzoneID) -> Result<bool, OzoneError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(OzoneError::NotFound)
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
