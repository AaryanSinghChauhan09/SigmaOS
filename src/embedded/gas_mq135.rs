#![no_std]
#![no_main]

/// OOP-based MQ135 Gas for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3816
/// Implements MQ135 air quality sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MQ135ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MQ135Error { Success = 0, NotFound = 1 }

pub trait MQ135Sensor {
    fn id(&self) -> MQ135ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMQ135Sensor {
    pub id: MQ135ID,
    pub initialized: AtomicUsize,
}

impl SimpleMQ135Sensor {
    pub fn new(id: MQ135ID) -> Self {
        SimpleMQ135Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MQ135Sensor for SimpleMQ135Sensor {
    fn id(&self) -> MQ135ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MQ135Controller {
    fn init(&mut self, sensor_id: MQ135ID) -> Result<(), MQ135Error>;
    fn read(&self, sensor_id: MQ135ID) -> Result<u16, MQ135Error>;
    def calibrate(&mut self, sensor_id: MQ135ID) -> Result<(), MQ135Error>;
}

#[repr(C)]
pub struct SimpleMQ135Controller {
    pub sensors: Vec<Option<Box<dyn MQ135Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMQ135Controller {
    pub fn new() -> Self {
        SimpleMQ135Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MQ135Controller for SimpleMQ135Controller {
    fn init(&mut self, sensor_id: MQ135ID) -> Result<(), MQ135Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MQ135Error::NotFound)
    }
    
    fn read(&self, sensor_id: MQ135ID) -> Result<u16, MQ135Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(MQ135Error::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: MQ135ID) -> Result<(), MQ135Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(MQ135Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: MQ135ID) -> Option<&dyn MQ135Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait MQ135PPM {
    def read_co2(&self, sensor_id: MQ135ID) -> Result<f32, MQ135Error>;
}

#[repr(C)]
pub struct SimpleMQ135PPM {
    pub controller: SimpleMQ135Controller,
}

impl SimpleMQ135PPM {
    pub fn new(controller: SimpleMQ135Controller) -> Self {
        SimpleMQ135PPM { controller }
    }
}

impl MQ135PPM for SimpleMQ135PPM {
    fn read_co2(&self, sensor_id: MQ135ID) -> Result<f32, MQ135Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(MQ135Error::NotFound)
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
