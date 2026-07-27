#![no_std]
#![no_main]

/// OOP-based SCD30 CO2 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3186
/// Implements SCD30 CO2 sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SCD30ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SCD30Error { Success = 0, NotFound = 1 }

pub trait SCD30Sensor {
    fn id(&self) -> SCD30ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSCD30Sensor {
    pub id: SCD30ID,
    pub initialized: AtomicUsize,
}

impl SimpleSCD30Sensor {
    pub fn new(id: SCD30ID) -> Self {
        SimpleSCD30Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SCD30Sensor for SimpleSCD30Sensor {
    fn id(&self) -> SCD30ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SCD30Controller {
    fn init(&mut self, sensor_id: SCD30ID) -> Result<(), SCD30Error>;
    fn read_co2(&self, sensor_id: SCD30ID) -> Result<u16, SCD30Error>;
    def read_humidity(&self, sensor_id: SCD30ID) -> Result<u16, SCD30Error>;
}

#[repr(C)]
pub struct SimpleSCD30Controller {
    pub sensors: Vec<Option<Box<dyn SCD30Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSCD30Controller {
    pub fn new() -> Self {
        SimpleSCD30Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SCD30Controller for SimpleSCD30Controller {
    fn init(&mut self, sensor_id: SCD30ID) -> Result<(), SCD30Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SCD30Error::NotFound)
    }
    
    fn read_co2(&self, sensor_id: SCD30ID) -> Result<u16, SCD30Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(SCD30Error::NotFound)
        }
    }
    
    fn read_humidity(&self, sensor_id: SCD30ID) -> Result<u16, SCD30Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(SCD30Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: SCD30ID) -> Option<&dyn SCD30Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait SCD30Temp {
    def read_temp(&self, sensor_id: SCD30ID) -> Result<i16, SCD30Error>;
}

#[repr(C)]
pub struct SimpleSCD30Temp {
    pub controller: SimpleSCD30Controller,
}

impl SimpleSCD30Temp {
    pub fn new(controller: SimpleSCD30Controller) -> Self {
        SimpleSCD30Temp { controller }
    }
}

impl SCD30Temp for SimpleSCD30Temp {
    fn read_temp(&self, sensor_id: SCD30ID) -> Result<i16, SCD30Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(SCD30Error::NotFound)
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
