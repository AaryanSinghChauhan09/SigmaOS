#![no_std]
#![no_main]

/// OOP-based BMP280 Barometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3156
/// Implements BMP280 pressure/temperature sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BMP280ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BMP280Error { Success = 0, NotFound = 1 }

pub trait BMP280Sensor {
    fn id(&self) -> BMP280ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBMP280Sensor {
    pub id: BMP280ID,
    pub initialized: AtomicUsize,
}

impl SimpleBMP280Sensor {
    pub fn new(id: BMP280ID) -> Self {
        SimpleBMP280Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl BMP280Sensor for SimpleBMP280Sensor {
    fn id(&self) -> BMP280ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait BMP280Controller {
    fn init(&mut self, sensor_id: BMP280ID) -> Result<(), BMP280Error>;
    fn read_pressure(&self, sensor_id: BMP280ID) -> Result<u32, BMP280Error>;
    def read_temp(&self, sensor_id: BMP280ID) -> Result<i32, BMP280Error>;
}

#[repr(C)]
pub struct SimpleBMP280Controller {
    pub sensors: Vec<Option<Box<dyn BMP280Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBMP280Controller {
    pub fn new() -> Self {
        SimpleBMP280Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BMP280Controller for SimpleBMP280Controller {
    fn init(&mut self, sensor_id: BMP280ID) -> Result<(), BMP280Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BMP280Error::NotFound)
    }
    
    fn read_pressure(&self, sensor_id: BMP280ID) -> Result<u32, BMP280Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BMP280Error::NotFound)
        }
    }
    
    fn read_temp(&self, sensor_id: BMP280ID) -> Result<i32, BMP280Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BMP280Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: BMP280ID) -> Option<&dyn BMP280Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait BMP280Config {
    def set_oversampling(&mut self, sensor_id: BMP280ID, pressure: u8, temp: u8) -> Result<(), BMP280Error>;
    def set_filter(&mut self, sensor_id: BMP280ID, filter: u8) -> Result<(), BMP280Error>;
}

#[repr(C)]
pub struct SimpleBMP280Config {
    pub controller: SimpleBMP280Controller,
    pub oversampling: Vec<(BMP280ID, AtomicUsize)>,
}

impl SimpleBMP280Config {
    pub fn new(controller: SimpleBMP280Controller) -> Self {
        SimpleBMP280Config {
            controller,
            oversampling: Vec::new(),
        }
    }
}

impl BMP280Config for SimpleBMP280Config {
    fn set_oversampling(&mut self, sensor_id: BMP280ID, _pressure: u8, _temp: u8) -> Result<(), BMP280Error> {
        self.oversampling.push((sensor_id, AtomicUsize::new(0)));
        Ok(())
    }
    
    fn set_filter(&mut self, _sensor_id: BMP280ID, _filter: u8) -> Result<(), BMP280Error> {
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
