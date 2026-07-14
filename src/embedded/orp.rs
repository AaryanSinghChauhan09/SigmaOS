#![no_std]
#![no_main]

/// OOP-based ORP Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1846
/// Implements ORP (Oxidation-Reduction Potential) sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ORPID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ORPError { Success = 0, NotFound = 1 }

pub trait ORPSensor {
    fn id(&self) -> ORPID;
    fn orp_value(&self) -> i16;
}

#[repr(C)]
pub struct SimpleORPSensor {
    pub id: ORPID,
    pub orp_value: AtomicUsize,
}

impl SimpleORPSensor {
    pub fn new(id: ORPID) -> Self {
        SimpleORPSensor {
            id,
            orp_value: AtomicUsize::new(0),
        }
    }
}

impl ORPSensor for SimpleORPSensor {
    fn id(&self) -> ORPID { self.id }
    fn orp_value(&self) -> i16 { self.orp_value.load(Ordering::SeqCst) as i16 }
}

pub trait ORPController {
    fn read(&self, sensor_id: ORPID) -> Result<i16, ORPError>;
    def calibrate(&mut self, sensor_id: ORPID, value: i16) -> Result<(), ORPError>;
}

#[repr(C)]
pub struct SimpleORPController {
    pub sensors: Vec<Option<Box<dyn ORPSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleORPController {
    pub fn new() -> Self {
        SimpleORPController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ORPController for SimpleORPController {
    fn read(&self, sensor_id: ORPID) -> Result<i16, ORPError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.orp_value());
                }
            }
        }
        Err(ORPError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: ORPID, _value: i16) -> Result<(), ORPError> {
        Ok(())
    }
}

pub trait RedoxStatus {
    def get_status(&self, sensor_id: ORPID) -> Result<u8, ORPError>;
    def is_oxidizing(&self, sensor_id: ORPID) -> Result<bool, ORPError>;
}

#[repr(C)]
pub struct SimpleRedoxStatus {
    pub controller: SimpleORPController,
}

impl SimpleRedoxStatus {
    pub fn new(controller: SimpleORPController) -> Self {
        SimpleRedoxStatus { controller }
    }
}

impl RedoxStatus for SimpleRedoxStatus {
    fn get_status(&self, sensor_id: ORPID) -> Result<u8, ORPError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(ORPError::NotFound)
        }
    }
    
    fn is_oxidizing(&self, sensor_id: ORPID) -> Result<bool, ORPError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(ORPError::NotFound)
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
