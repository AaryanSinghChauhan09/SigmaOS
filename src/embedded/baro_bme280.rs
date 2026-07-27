#![no_std]
#![no_main]

/// OOP-based BME280 Barometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3166
/// Implements BME280 pressure/temperature/humidity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BME280ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BME280Error { Success = 0, NotFound = 1 }

pub trait BME280Sensor {
    fn id(&self) -> BME280ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBME280Sensor {
    pub id: BME280ID,
    pub initialized: AtomicUsize,
}

impl SimpleBME280Sensor {
    pub fn new(id: BME280ID) -> Self {
        SimpleBME280Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl BME280Sensor for SimpleBME280Sensor {
    fn id(&self) -> BME280ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait BME280Controller {
    fn init(&mut self, sensor_id: BME280ID) -> Result<(), BME280Error>;
    fn read_pressure(&self, sensor_id: BME280ID) -> Result<u32, BME280Error>;
    def read_temp(&self, sensor_id: BME280ID) -> Result<i32, BME280Error>;
}

#[repr(C)]
pub struct SimpleBME280Controller {
    pub sensors: Vec<Option<Box<dyn BME280Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBME280Controller {
    pub fn new() -> Self {
        SimpleBME280Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BME280Controller for SimpleBME280Controller {
    fn init(&mut self, sensor_id: BME280ID) -> Result<(), BME280Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BME280Error::NotFound)
    }
    
    fn read_pressure(&self, sensor_id: BME280ID) -> Result<u32, BME280Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BME280Error::NotFound)
        }
    }
    
    fn read_temp(&self, sensor_id: BME280ID) -> Result<i32, BME280Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BME280Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: BME280ID) -> Option<&dyn BME280Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait BME280Humidity {
    def read_humidity(&self, sensor_id: BME280ID) -> Result<u32, BME280Error>;
}

#[repr(C)]
pub struct SimpleBME280Humidity {
    pub controller: SimpleBME280Controller,
}

impl SimpleBME280Humidity {
    pub fn new(controller: SimpleBME280Controller) -> Self {
        SimpleBME280Humidity { controller }
    }
}

impl BME280Humidity for SimpleBME280Humidity {
    fn read_humidity(&self, sensor_id: BME280ID) -> Result<u32, BME280Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BME280Error::NotFound)
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
