#![no_std]
#![no_main]

/// OOP-based BMP180 Pressure for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3786
/// Implements BMP180 barometric pressure sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type BMP180ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BMP180Error { Success = 0, NotFound = 1 }

pub trait BMP180Sensor {
    fn id(&self) -> BMP180ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleBMP180Sensor {
    pub id: BMP180ID,
    pub initialized: AtomicUsize,
}

impl SimpleBMP180Sensor {
    pub fn new(id: BMP180ID) -> Self {
        SimpleBMP180Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl BMP180Sensor for SimpleBMP180Sensor {
    fn id(&self) -> BMP180ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait BMP180Controller {
    fn init(&mut self, sensor_id: BMP180ID) -> Result<(), BMP180Error>;
    fn read_pressure(&self, sensor_id: BMP180ID) -> Result<u32, BMP180Error>;
    def read_temp(&self, sensor_id: BMP180ID) -> Result<i16, BMP180Error>;
}

#[repr(C)]
pub struct SimpleBMP180Controller {
    pub sensors: Vec<Option<Box<dyn BMP180Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBMP180Controller {
    pub fn new() -> Self {
        SimpleBMP180Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BMP180Controller for SimpleBMP180Controller {
    fn init(&mut self, sensor_id: BMP180ID) -> Result<(), BMP180Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(BMP180Error::NotFound)
    }
    
    fn read_pressure(&self, sensor_id: BMP180ID) -> Result<u32, BMP180Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BMP180Error::NotFound)
        }
    }
    
    fn read_temp(&self, sensor_id: BMP180ID) -> Result<i16, BMP180Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(BMP180Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: BMP180ID) -> Option<&dyn BMP180Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait BMP180Altitude {
    def read_altitude(&self, sensor_id: BMP180ID) -> Result<f32, BMP180Error>;
}

#[repr(C)]
pub struct SimpleBMP180Altitude {
    pub controller: SimpleBMP180Controller,
}

impl SimpleBMP180Altitude {
    pub fn new(controller: SimpleBMP180Controller) -> Self {
        SimpleBMP180Altitude { controller }
    }
}

impl BMP180Altitude for SimpleBMP180Altitude {
    fn read_altitude(&self, sensor_id: BMP180ID) -> Result<f32, BMP180Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(BMP180Error::NotFound)
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
