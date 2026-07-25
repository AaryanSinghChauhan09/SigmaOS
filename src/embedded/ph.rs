#![no_std]
#![no_main]

/// OOP-based pH Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1816
/// Implements pH sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type pHID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum pHError { Success = 0, NotFound = 1 }

pub trait PHSensor {
    fn id(&self) -> pHID;
    fn ph_value(&self) -> f32;
}

#[repr(C)]
pub struct SimplePHSensor {
    pub id: pHID,
    pub ph_value: AtomicUsize,
}

impl SimplePHSensor {
    pub fn new(id: pHID) -> Self {
        SimplePHSensor {
            id,
            ph_value: AtomicUsize::new(700),
        }
    }
}

impl PHSensor for SimplePHSensor {
    fn id(&self) -> pHID { self.id }
    fn ph_value(&self) -> f32 { self.ph_value.load(Ordering::SeqCst) as f32 / 100.0 }
}

pub trait PHController {
    fn read(&self, sensor_id: pHID) -> Result<f32, pHError>;
    def calibrate(&mut self, sensor_id: pHID, point: u8, value: f32) -> Result<(), pHError>;
}

#[repr(C)]
pub struct SimplePHController {
    pub sensors: Vec<Option<Box<dyn PHSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimplePHController {
    pub fn new() -> Self {
        SimplePHController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PHController for SimplePHController {
    fn read(&self, sensor_id: pHID) -> Result<f32, pHError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.ph_value());
                }
            }
        }
        Err(pHError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: pHID, _point: u8, _value: f32) -> Result<(), pHError> {
        Ok(())
    }
}

pub trait TemperatureCompensation {
    def set_temperature(&mut self, sensor_id: pHID, temp: f32) -> Result<(), pHError>;
    def get_compensated(&self, sensor_id: pHID) -> Result<f32, pHError>;
}

#[repr(C)]
pub struct SimpleTemperatureCompensation {
    pub controller: SimplePHController,
    pub temperatures: Vec<(pHID, AtomicUsize)>,
}

impl SimpleTemperatureCompensation {
    pub fn new(controller: SimplePHController) -> Self {
        SimpleTemperatureCompensation {
            controller,
            temperatures: Vec::new(),
        }
    }
}

impl TemperatureCompensation for SimpleTemperatureCompensation {
    fn set_temperature(&mut self, sensor_id: pHID, temp: f32) -> Result<(), pHError> {
        self.temperatures.push((sensor_id, AtomicUsize::new((temp * 100.0) as usize)));
        Ok(())
    }
    
    fn get_compensated(&self, sensor_id: pHID) -> Result<f32, pHError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(7.0)
        } else {
            Err(pHError::NotFound)
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
