#![no_std]
#![no_main]

/// OOP-based Leaf Wetness Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1876
/// Implements leaf wetness sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LeafID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LeafError { Success = 0, NotFound = 1 }

pub trait LeafWetnessSensor {
    fn id(&self) -> LeafID;
    fn wetness(&self) -> u8;
}

#[repr(C)]
pub struct SimpleLeafWetnessSensor {
    pub id: LeafID,
    pub wetness: AtomicUsize,
}

impl SimpleLeafWetnessSensor {
    pub fn new(id: LeafID) -> Self {
        SimpleLeafWetnessSensor {
            id,
            wetness: AtomicUsize::new(0),
        }
    }
}

impl LeafWetnessSensor for SimpleLeafWetnessSensor {
    fn id(&self) -> LeafID { self.id }
    fn wetness(&self) -> u8 { self.wetness.load(Ordering::SeqCst) as u8 }
}

pub trait LeafController {
    fn read(&self, sensor_id: LeafID) -> Result<u8, LeafError>;
    def is_wet(&self, sensor_id: LeafID) -> Result<bool, LeafError>;
}

#[repr(C)]
pub struct SimpleLeafController {
    pub sensors: Vec<Option<Box<dyn LeafWetnessSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLeafController {
    pub fn new() -> Self {
        SimpleLeafController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LeafController for SimpleLeafController {
    fn read(&self, sensor_id: LeafID) -> Result<u8, LeafError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.wetness());
                }
            }
        }
        Err(LeafError::NotFound)
    }
    
    fn is_wet(&self, sensor_id: LeafID) -> Result<bool, LeafError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.wetness() > 50);
                }
            }
        }
        Err(LeafError::NotFound)
    }
}

pub trait DiseaseRisk {
    def get_risk_level(&self, sensor_id: LeafID) -> Result<u8, LeafError>;
    def get_duration(&self, sensor_id: LeafID) -> Result<u32, LeafError>;
}

#[repr(C)]
pub struct SimpleDiseaseRisk {
    pub controller: SimpleLeafController,
    pub durations: Vec<(LeafID, AtomicUsize)>,
}

impl SimpleDiseaseRisk {
    pub fn new(controller: SimpleLeafController) -> Self {
        SimpleDiseaseRisk {
            controller,
            durations: Vec::new(),
        }
    }
}

impl DiseaseRisk for SimpleDiseaseRisk {
    fn get_risk_level(&self, sensor_id: LeafID) -> Result<u8, LeafError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(LeafError::NotFound)
        }
    }
    
    fn get_duration(&self, sensor_id: LeafID) -> Result<u32, LeafError> {
        if self.controller.read(sensor_id).is_ok() {
            for &(id, ref dur) in &self.durations {
                if id == sensor_id {
                    return Ok(dur.load(Ordering::SeqCst) as u32);
                }
            }
            Ok(0)
        } else {
            Err(LeafError::NotFound)
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
