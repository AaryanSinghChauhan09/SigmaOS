#![no_std]
#![no_main]

/// OOP-based DHT22 Humidity for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3176
/// Implements DHT22 temperature/humidity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DHT22ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DHT22Error { Success = 0, NotFound = 1 }

pub trait DHT22Sensor {
    fn id(&self) -> DHT22ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDHT22Sensor {
    pub id: DHT22ID,
    pub initialized: AtomicUsize,
}

impl SimpleDHT22Sensor {
    pub fn new(id: DHT22ID) -> Self {
        SimpleDHT22Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DHT22Sensor for SimpleDHT22Sensor {
    fn id(&self) -> DHT22ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DHT22Controller {
    fn init(&mut self, sensor_id: DHT22ID) -> Result<(), DHT22Error>;
    fn read(&self, sensor_id: DHT22ID) -> Result<(i16, u16), DHT22Error>;
}

#[repr(C)]
pub struct SimpleDHT22Controller {
    pub sensors: Vec<Option<Box<dyn DHT22Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDHT22Controller {
    pub fn new() -> Self {
        SimpleDHT22Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DHT22Controller for SimpleDHT22Controller {
    fn init(&mut self, sensor_id: DHT22ID) -> Result<(), DHT22Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DHT22Error::NotFound)
    }
    
    fn read(&self, sensor_id: DHT22ID) -> Result<(i16, u16), DHT22Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok((0, 0))
        } else {
            Err(DHT22Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: DHT22ID) -> Option<&dyn DHT22Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
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
