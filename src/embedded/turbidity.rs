#![no_std]
#![no_main]

/// OOP-based Turbidity Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1856
/// Implements turbidity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TurbID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TurbError { Success = 0, NotFound = 1 }

pub trait TurbiditySensor {
    fn id(&self) -> TurbID;
    fn ntu(&self) -> f32;
}

#[repr(C)]
pub struct SimpleTurbiditySensor {
    pub id: TurbID,
    pub ntu: AtomicUsize,
}

impl SimpleTurbiditySensor {
    pub fn new(id: TurbID) -> Self {
        SimpleTurbiditySensor {
            id,
            ntu: AtomicUsize::new(0),
        }
    }
}

impl TurbiditySensor for SimpleTurbiditySensor {
    fn id(&self) -> TurbID { self.id }
    fn ntu(&self) -> f32 { self.ntu.load(Ordering::SeqCst) as f32 }
}

pub trait TurbController {
    fn read(&self, sensor_id: TurbID) -> Result<f32, TurbError>;
    def calibrate(&mut self, sensor_id: TurbID, clear: f32, cloudy: f32) -> Result<(), TurbError>;
}

#[repr(C)]
pub struct SimpleTurbController {
    pub sensors: Vec<Option<Box<dyn TurbiditySensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTurbController {
    pub fn new() -> Self {
        SimpleTurbController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TurbController for SimpleTurbController {
    fn read(&self, sensor_id: TurbID) -> Result<f32, TurbError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.ntu());
                }
            }
        }
        Err(TurbError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: TurbID, _clear: f32, _cloudy: f32) -> Result<(), TurbError> {
        Ok(())
    }
}

pub trait WaterQuality {
    def get_quality(&self, sensor_id: TurbID) -> Result<u8, TurbError>;
    def is_clear(&self, sensor_id: TurbID) -> Result<bool, TurbError>;
}

#[repr(C)]
pub struct SimpleWaterQuality {
    pub controller: SimpleTurbController,
}

impl SimpleWaterQuality {
    pub fn new(controller: SimpleTurbController) -> Self {
        SimpleWaterQuality { controller }
    }
}

impl WaterQuality for SimpleWaterQuality {
    fn get_quality(&self, sensor_id: TurbID) -> Result<u8, TurbError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(TurbError::NotFound)
        }
    }
    
    fn is_clear(&self, sensor_id: TurbID) -> Result<bool, TurbError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(true)
        } else {
            Err(TurbError::NotFound)
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
