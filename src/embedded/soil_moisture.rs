#![no_std]
#![no_main]

/// OOP-based Soil Moisture Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1866
/// Implements soil moisture sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SoilID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SoilError { Success = 0, NotFound = 1 }

pub trait SoilMoistureSensor {
    fn id(&self) -> SoilID;
    fn moisture(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSoilMoistureSensor {
    pub id: SoilID,
    pub moisture: AtomicUsize,
}

impl SimpleSoilMoistureSensor {
    pub fn new(id: SoilID) -> Self {
        SimpleSoilMoistureSensor {
            id,
            moisture: AtomicUsize::new(0),
        }
    }
}

impl SoilMoistureSensor for SimpleSoilMoistureSensor {
    fn id(&self) -> SoilID { self.id }
    fn moisture(&self) -> u8 { self.moisture.load(Ordering::SeqCst) as u8 }
}

pub trait SoilController {
    fn read(&self, sensor_id: SoilID) -> Result<u8, SoilError>;
    def calibrate(&mut self, sensor_id: SoilID, dry: u16, wet: u16) -> Result<(), SoilError>;
}

#[repr(C)]
pub struct SimpleSoilController {
    pub sensors: Vec<Option<Box<dyn SoilMoistureSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSoilController {
    pub fn new() -> Self {
        SimpleSoilController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SoilController for SimpleSoilController {
    fn read(&self, sensor_id: SoilID) -> Result<u8, SoilError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.moisture());
                }
            }
        }
        Err(SoilError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: SoilID, _dry: u16, _wet: u16) -> Result<(), SoilError> {
        Ok(())
    }
}

pub trait IrrigationControl {
    def needs_water(&self, sensor_id: SoilID) -> Result<bool, SoilError>;
    def set_threshold(&mut self, sensor_id: SoilID, threshold: u8) -> Result<(), SoilError>;
}

#[repr(C)]
pub struct SimpleIrrigationControl {
    pub controller: SimpleSoilController,
    pub thresholds: Vec<(SoilID, AtomicUsize)>,
}

impl SimpleIrrigationControl {
    pub fn new(controller: SimpleSoilController) -> Self {
        SimpleIrrigationControl {
            controller,
            thresholds: Vec::new(),
        }
    }
}

impl IrrigationControl for SimpleIrrigationControl {
    fn needs_water(&self, sensor_id: SoilID) -> Result<bool, SoilError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(SoilError::NotFound)
        }
    }
    
    fn set_threshold(&mut self, sensor_id: SoilID, threshold: u8) -> Result<(), SoilError> {
        self.thresholds.push((sensor_id, AtomicUsize::new(threshold as usize)));
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
