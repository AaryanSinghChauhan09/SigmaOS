#![no_std]
#![no_main]

/// OOP-based DHT11 Humidity for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3776
/// Implements DHT11 temperature/humidity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DHT11ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DHT11Error { Success = 0, NotFound = 1 }

pub trait DHT11Sensor {
    fn id(&self) -> DHT11ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDHT11Sensor {
    pub id: DHT11ID,
    pub initialized: AtomicUsize,
}

impl SimpleDHT11Sensor {
    pub fn new(id: DHT11ID) -> Self {
        SimpleDHT11Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DHT11Sensor for SimpleDHT11Sensor {
    fn id(&self) -> DHT11ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DHT11Controller {
    fn init(&mut self, sensor_id: DHT11ID) -> Result<(), DHT11Error>;
    fn read(&self, sensor_id: DHT11ID) -> Result<(u8, u8), DHT11Error>;
    def read_temp(&self, sensor_id: DHT11ID) -> Result<u8, DHT11Error>;
}

#[repr(C)]
pub struct SimpleDHT11Controller {
    pub sensors: Vec<Option<Box<dyn DHT11Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDHT11Controller {
    pub fn new() -> Self {
        SimpleDHT11Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DHT11Controller for SimpleDHT11Controller {
    fn init(&mut self, sensor_id: DHT11ID) -> Result<(), DHT11Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DHT11Error::NotFound)
    }
    
    fn read(&self, sensor_id: DHT11ID) -> Result<(u8, u8), DHT11Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok((0, 0))
        } else {
            Err(DHT11Error::NotFound)
        }
    }
    
    fn read_temp(&self, sensor_id: DHT11ID) -> Result<u8, DHT11Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(DHT11Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: DHT11ID) -> Option<&dyn DHT11Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait DHT11Humidity {
    def read_humidity(&self, sensor_id: DHT11ID) -> Result<u8, DHT11Error>;
}

#[repr(C)]
pub struct SimpleDHT11Humidity {
    pub controller: SimpleDHT11Controller,
}

impl SimpleDHT11Humidity {
    pub fn new(controller: SimpleDHT11Controller) -> Self {
        SimpleDHT11Humidity { controller }
    }
}

impl DHT11Humidity for SimpleDHT11Humidity {
    fn read_humidity(&self, sensor_id: DHT11ID) -> Result<u8, DHT11Error> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(DHT11Error::NotFound)
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
