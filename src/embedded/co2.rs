#![no_std]
#![no_main]

/// OOP-based CO2 Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1896
/// Implements CO2 sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CO2ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CO2Error { Success = 0, NotFound = 1 }

pub trait CO2Sensor {
    fn id(&self) -> CO2ID;
    fn co2_ppm(&self) -> u16;
}

#[repr(C)]
pub struct SimpleCO2Sensor {
    pub id: CO2ID,
    pub co2_ppm: AtomicUsize,
}

impl SimpleCO2Sensor {
    pub fn new(id: CO2ID) -> Self {
        SimpleCO2Sensor {
            id,
            co2_ppm: AtomicUsize::new(400),
        }
    }
}

impl CO2Sensor for SimpleCO2Sensor {
    fn id(&self) -> CO2ID { self.id }
    fn co2_ppm(&self) -> u16 { self.co2_ppm.load(Ordering::SeqCst) as u16 }
}

pub trait CO2Controller {
    fn read(&self, sensor_id: CO2ID) -> Result<u16, CO2Error>;
    def calibrate(&mut self, sensor_id: CO2ID) -> Result<(), CO2Error>;
}

#[repr(C)]
pub struct SimpleCO2Controller {
    pub sensors: Vec<Option<Box<dyn CO2Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCO2Controller {
    pub fn new() -> Self {
        SimpleCO2Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CO2Controller for SimpleCO2Controller {
    fn read(&self, sensor_id: CO2ID) -> Result<u16, CO2Error> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.co2_ppm());
                }
            }
        }
        Err(CO2Error::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: CO2ID) -> Result<(), CO2Error> {
        Ok(())
    }
}

pub trait AirQuality {
    def get_quality(&self, sensor_id: CO2ID) -> Result<u8, CO2Error>;
    def is_high(&self, sensor_id: CO2ID) -> Result<bool, CO2Error>;
}

#[repr(C)]
pub struct SimpleAirQuality {
    pub controller: SimpleCO2Controller,
}

impl SimpleAirQuality {
    pub fn new(controller: SimpleCO2Controller) -> Self {
        SimpleAirQuality { controller }
    }
}

impl AirQuality for SimpleAirQuality {
    fn get_quality(&self, sensor_id: CO2ID) -> Result<u8, CO2Error> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(CO2Error::NotFound)
        }
    }
    
    fn is_high(&self, sensor_id: CO2ID) -> Result<bool, CO2Error> {
        if let Ok(ppm) = self.controller.read(sensor_id) {
            Ok(ppm > 1000)
        } else {
            Err(CO2Error::NotFound)
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
