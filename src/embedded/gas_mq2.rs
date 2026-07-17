#![no_std]
#![no_main]

/// OOP-based MQ2 Gas for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3806
/// Implements MQ2 gas sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MQ2ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MQ2Error { Success = 0, NotFound = 1 }

pub trait MQ2Sensor {
    fn id(&self) -> MQ2ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMQ2Sensor {
    pub id: MQ2ID,
    pub initialized: AtomicUsize,
}

impl SimpleMQ2Sensor {
    pub fn new(id: MQ2ID) -> Self {
        SimpleMQ2Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MQ2Sensor for SimpleMQ2Sensor {
    fn id(&self) -> MQ2ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MQ2Controller {
    fn init(&mut self, sensor_id: MQ2ID) -> Result<(), MQ2Error>;
    fn read(&self, sensor_id: MQ2ID) -> Result<u16, MQ2Error>;
    def calibrate(&mut self, sensor_id: MQ2ID) -> Result<(), MQ2Error>;
}

#[repr(C)]
pub struct SimpleMQ2Controller {
    pub sensors: Vec<Option<Box<dyn MQ2Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMQ2Controller {
    pub fn new() -> Self {
        SimpleMQ2Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MQ2Controller for SimpleMQ2Controller {
    fn init(&mut self, sensor_id: MQ2ID) -> Result<(), MQ2Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MQ2Error::NotFound)
    }
    
    fn read(&self, sensor_id: MQ2ID) -> Result<u16, MQ2Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(MQ2Error::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: MQ2ID) -> Result<(), MQ2Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(MQ2Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: MQ2ID) -> Option<&dyn MQ2Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait MQ2GasType {
    def detect_lpg(&self, sensor_id: MQ2ID) -> Result<bool, MQ2Error>;
    def detect_smoke(&self, sensor_id: MQ2ID) -> Result<bool, MQ2Error>;
}

#[repr(C)]
pub struct SimpleMQ2GasType {
    pub controller: SimpleMQ2Controller,
}

impl SimpleMQ2GasType {
    pub fn new(controller: SimpleMQ2Controller) -> Self {
        SimpleMQ2GasType { controller }
    }
}

impl MQ2GasType for SimpleMQ2GasType {
    fn detect_lpg(&self, sensor_id: MQ2ID) -> Result<bool, MQ2Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(MQ2Error::NotFound)
        }
    }
    
    fn detect_smoke(&self, sensor_id: MQ2ID) -> Result<bool, MQ2Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(MQ2Error::NotFound)
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
