#![no_std]
#![no_main]

/// OOP-based PAR Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1886
/// Implements PAR (Photosynthetically Active Radiation) sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PARID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PARError { Success = 0, NotFound = 1 }

pub trait PARSensor {
    fn id(&self) -> PARID;
    fn par(&self) -> f32;
}

#[repr(C)]
pub struct SimplePARSensor {
    pub id: PARID,
    pub par: AtomicUsize,
}

impl SimplePARSensor {
    pub fn new(id: PARID) -> Self {
        SimplePARSensor {
            id,
            par: AtomicUsize::new(0),
        }
    }
}

impl PARSensor for SimplePARSensor {
    fn id(&self) -> PARID { self.id }
    fn par(&self) -> f32 { self.par.load(Ordering::SeqCst) as f32 }
}

pub trait PARController {
    fn read(&self, sensor_id: PARID) -> Result<f32, PARError>;
    def calibrate(&mut self, sensor_id: PARID, factor: f32) -> Result<(), PARError>;
}

#[repr(C)]
pub struct SimplePARController {
    pub sensors: Vec<Option<Box<dyn PARSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimplePARController {
    pub fn new() -> Self {
        SimplePARController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PARController for SimplePARController {
    fn read(&self, sensor_id: PARID) -> Result<f32, PARError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.par());
                }
            }
        }
        Err(PARError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: PARID, _factor: f32) -> Result<(), PARError> {
        Ok(())
    }
}

pub trait DLICalc {
    def get_dli(&self, sensor_id: PARID) -> Result<f32, PARError>;
    def reset_daily(&mut self, sensor_id: PARID) -> Result<(), PARError>;
}

#[repr(C)]
pub struct SimpleDLICalc {
    pub controller: SimplePARController,
    pub daily_totals: Vec<(PARID, AtomicUsize)>,
}

impl SimpleDLICalc {
    pub fn new(controller: SimplePARController) -> Self {
        SimpleDLICalc {
            controller,
            daily_totals: Vec::new(),
        }
    }
}

impl DLICalc for SimpleDLICalc {
    fn get_dli(&self, sensor_id: PARID) -> Result<f32, PARError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(PARError::NotFound)
        }
    }
    
    fn reset_daily(&mut self, sensor_id: PARID) -> Result<(), PARError> {
        self.daily_totals.push((sensor_id, AtomicUsize::new(0)));
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
