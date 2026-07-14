#![no_std]
#![no_main]

/// OOP-based HC-SR04 Ultrasonic for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3846
/// Implements HC-SR04 ultrasonic distance sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HCSR04ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HCSR04Error { Success = 0, NotFound = 1 }

pub trait HCSR04Sensor {
    fn id(&self) -> HCSR04ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHCSR04Sensor {
    pub id: HCSR04ID,
    pub initialized: AtomicUsize,
}

impl SimpleHCSR04Sensor {
    pub fn new(id: HCSR04ID) -> Self {
        SimpleHCSR04Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HCSR04Sensor for SimpleHCSR04Sensor {
    fn id(&self) -> HCSR04ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HCSR04Controller {
    fn init(&mut self, sensor_id: HCSR04ID) -> Result<(), HCSR04Error>;
    fn read_distance(&self, sensor_id: HCSR04ID) -> Result<u16, HCSR04Error>;
    def trigger(&self, sensor_id: HCSR04ID) -> Result<(), HCSR04Error>;
}

#[repr(C)]
pub struct SimpleHCSR04Controller {
    pub sensors: Vec<Option<Box<dyn HCSR04Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHCSR04Controller {
    pub fn new() -> Self {
        SimpleHCSR04Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HCSR04Controller for SimpleHCSR04Controller {
    fn init(&mut self, sensor_id: HCSR04ID) -> Result<(), HCSR04Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HCSR04Error::NotFound)
    }
    
    fn read_distance(&self, sensor_id: HCSR04ID) -> Result<u16, HCSR04Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(HCSR04Error::NotFound)
        }
    }
    
    fn trigger(&self, sensor_id: HCSR04ID) -> Result<(), HCSR04Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(HCSR04Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: HCSR04ID) -> Option<&dyn HCSR04Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait HCSR04Timeout {
    def set_timeout(&mut self, sensor_id: HCSR04ID, timeout_us: u32) -> Result<(), HCSR04Error>;
}

#[repr(C)]
pub struct SimpleHCSR04Timeout {
    pub controller: SimpleHCSR04Controller,
    pub timeouts: Vec<(HCSR04ID, AtomicUsize)>,
}

impl SimpleHCSR04Timeout {
    pub fn new(controller: SimpleHCSR04Controller) -> Self {
        SimpleHCSR04Timeout {
            controller,
            timeouts: Vec::new(),
        }
    }
}

impl HCSR04Timeout for SimpleHCSR04Timeout {
    fn set_timeout(&mut self, sensor_id: HCSR04ID, timeout_us: u32) -> Result<(), HCSR04Error> {
        self.timeouts.push((sensor_id, AtomicUsize::new(timeout_us as usize)));
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
