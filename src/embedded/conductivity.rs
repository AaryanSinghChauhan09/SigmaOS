#![no_std]
#![no_main]

/// OOP-based Conductivity Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1826
/// Implements conductivity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CondID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CondError { Success = 0, NotFound = 1 }

pub trait ConductivitySensor {
    fn id(&self) -> CondID;
    fn conductivity(&self) -> f32;
}

#[repr(C)]
pub struct SimpleConductivitySensor {
    pub id: CondID,
    pub conductivity: AtomicUsize,
}

impl SimpleConductivitySensor {
    pub fn new(id: CondID) -> Self {
        SimpleConductivitySensor {
            id,
            conductivity: AtomicUsize::new(0),
        }
    }
}

impl ConductivitySensor for SimpleConductivitySensor {
    fn id(&self) -> CondID { self.id }
    fn conductivity(&self) -> f32 { self.conductivity.load(Ordering::SeqCst) as f32 }
}

pub trait CondController {
    fn read(&self, sensor_id: CondID) -> Result<f32, CondError>;
    def calibrate(&mut self, sensor_id: CondID, value: f32) -> Result<(), CondError>;
}

#[repr(C)]
pub struct SimpleCondController {
    pub sensors: Vec<Option<Box<dyn ConductivitySensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCondController {
    pub fn new() -> Self {
        SimpleCondController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CondController for SimpleCondController {
    fn read(&self, sensor_id: CondID) -> Result<f32, CondError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.conductivity());
                }
            }
        }
        Err(CondError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: CondID, _value: f32) -> Result<(), CondError> {
        Ok(())
    }
}

pub trait TDS {
    def get_tds(&self, sensor_id: CondID) -> Result<f32, CondError>;
    def get_salinity(&self, sensor_id: CondID) -> Result<f32, CondError>;
}

#[repr(C)]
pub struct SimpleTDS {
    pub controller: SimpleCondController,
}

impl SimpleTDS {
    pub fn new(controller: SimpleCondController) -> Self {
        SimpleTDS { controller }
    }
}

impl TDS for SimpleTDS {
    fn get_tds(&self, sensor_id: CondID) -> Result<f32, CondError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(CondError::NotFound)
        }
    }
    
    fn get_salinity(&self, sensor_id: CondID) -> Result<f32, CondError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(CondError::NotFound)
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
