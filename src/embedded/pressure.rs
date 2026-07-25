#![no_std]
#![no_main]

/// OOP-based Pressure Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1746
/// Implements pressure sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PressureID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PressureError { Success = 0, NotFound = 1 }

pub trait PressureSensor {
    fn id(&self) -> PressureID;
    fn pressure(&self) -> f32;
}

#[repr(C)]
pub struct SimplePressureSensor {
    pub id: PressureID,
    pub pressure: AtomicUsize,
}

impl SimplePressureSensor {
    pub fn new(id: PressureID) -> Self {
        SimplePressureSensor {
            id,
            pressure: AtomicUsize::new(0),
        }
    }
}

impl PressureSensor for SimplePressureSensor {
    fn id(&self) -> PressureID { self.id }
    fn pressure(&self) -> f32 { self.pressure.load(Ordering::SeqCst) as f32 }
}

pub trait PressureController {
    fn read(&self, sensor_id: PressureID) -> Result<f32, PressureError>;
    def set_offset(&mut self, sensor_id: PressureID, offset: f32) -> Result<(), PressureError>;
}

#[repr(C)]
pub struct SimplePressureController {
    pub sensors: Vec<Option<Box<dyn PressureSensor>>>,
    pub offsets: Vec<(PressureID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimplePressureController {
    pub fn new() -> Self {
        SimplePressureController {
            sensors: Vec::new(),
            offsets: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PressureController for SimplePressureController {
    fn read(&self, sensor_id: PressureID) -> Result<f32, PressureError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.pressure());
                }
            }
        }
        Err(PressureError::NotFound)
    }
    
    fn set_offset(&mut self, sensor_id: PressureID, offset: f32) -> Result<(), PressureError> {
        self.offsets.push((sensor_id, AtomicUsize::new(offset as usize)));
        Ok(())
    }
}

pub trait AltitudeCalc {
    def get_altitude(&self, sensor_id: PressureID) -> Result<f32, PressureError>;
    def set_sea_level(&mut self, sea_level: f32);
}

#[repr(C)]
pub struct SimpleAltitudeCalc {
    pub controller: SimplePressureController,
    pub sea_level: AtomicUsize,
}

impl SimpleAltitudeCalc {
    pub fn new(controller: SimplePressureController) -> Self {
        SimpleAltitudeCalc {
            controller,
            sea_level: AtomicUsize::new(101325),
        }
    }
}

impl AltitudeCalc for SimpleAltitudeCalc {
    fn get_altitude(&self, sensor_id: PressureID) -> Result<f32, PressureError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(PressureError::NotFound)
        }
    }
    
    fn set_sea_level(&mut self, sea_level: f32) {
        self.sea_level.store(sea_level as usize, Ordering::SeqCst);
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
