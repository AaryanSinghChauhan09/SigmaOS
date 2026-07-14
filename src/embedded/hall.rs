#![no_std]
#![no_main]

/// OOP-based Hall Effect Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1786
/// Implements Hall effect sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HallID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HallError { Success = 0, NotFound = 1 }

pub trait HallSensor {
    fn id(&self) -> HallID;
    fn field_strength(&self) -> i16;
}

#[repr(C)]
pub struct SimpleHallSensor {
    pub id: HallID,
    pub field_strength: AtomicUsize,
}

impl SimpleHallSensor {
    pub fn new(id: HallID) -> Self {
        SimpleHallSensor {
            id,
            field_strength: AtomicUsize::new(0),
        }
    }
}

impl HallSensor for SimpleHallSensor {
    fn id(&self) -> HallID { self.id }
    fn field_strength(&self) -> i16 { self.field_strength.load(Ordering::SeqCst) as i16 }
}

pub trait HallController {
    fn read(&self, sensor_id: HallID) -> Result<i16, HallError>;
    def calibrate(&mut self, sensor_id: HallID) -> Result<(), HallError>;
}

#[repr(C)]
pub struct SimpleHallController {
    pub sensors: Vec<Option<Box<dyn HallSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHallController {
    pub fn new() -> Self {
        SimpleHallController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HallController for SimpleHallController {
    fn read(&self, sensor_id: HallID) -> Result<i16, HallError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.field_strength());
                }
            }
        }
        Err(HallError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: HallID) -> Result<(), HallError> {
        Ok(())
    }
}

pub trait MagneticDetection {
    def is_magnet_present(&self, sensor_id: HallID) -> Result<bool, HallError>;
    def get_polarity(&self, sensor_id: HallID) -> Result<i8, HallError>;
}

#[repr(C)]
pub struct SimpleMagneticDetection {
    pub controller: SimpleHallController,
}

impl SimpleMagneticDetection {
    pub fn new(controller: SimpleHallController) -> Self {
        SimpleMagneticDetection { controller }
    }
}

impl MagneticDetection for SimpleMagneticDetection {
    fn is_magnet_present(&self, sensor_id: HallID) -> Result<bool, HallError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(HallError::NotFound)
        }
    }
    
    fn get_polarity(&self, sensor_id: HallID) -> Result<i8, HallError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(HallError::NotFound)
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
