#![no_std]
#![no_main]

/// OOP-based ACS712 Current Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3336
/// Implements ACS712 Hall effect current sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ACS712ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ACS712Error { Success = 0, NotFound = 1 }

pub trait ACS712Sensor {
    fn id(&self) -> ACS712ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleACS712Sensor {
    pub id: ACS712ID,
    pub initialized: AtomicUsize,
}

impl SimpleACS712Sensor {
    pub fn new(id: ACS712ID) -> Self {
        SimpleACS712Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ACS712Sensor for SimpleACS712Sensor {
    fn id(&self) -> ACS712ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ACS712Controller {
    fn init(&mut self, sensor_id: ACS712ID) -> Result<(), ACS712Error>;
    fn read_current(&self, sensor_id: ACS712ID) -> Result<f32, ACS712Error>;
    def calibrate(&mut self, sensor_id: ACS712ID) -> Result<(), ACS712Error>;
}

#[repr(C)]
pub struct SimpleACS712Controller {
    pub sensors: Vec<Option<Box<dyn ACS712Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleACS712Controller {
    pub fn new() -> Self {
        SimpleACS712Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ACS712Controller for SimpleACS712Controller {
    fn init(&mut self, sensor_id: ACS712ID) -> Result<(), ACS712Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ACS712Error::NotFound)
    }
    
    fn read_current(&self, sensor_id: ACS712ID) -> Result<f32, ACS712Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(ACS712Error::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: ACS712ID) -> Result<(), ACS712Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(ACS712Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: ACS712ID) -> Option<&dyn ACS712Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait ACS712Sensitivity {
    def set_sensitivity(&mut self, sensor_id: ACS712ID, sens: f32) -> Result<(), ACS712Error>;
}

#[repr(C)]
pub struct SimpleACS712Sensitivity {
    pub controller: SimpleACS712Controller,
    pub sensitivities: Vec<(ACS712ID, AtomicUsize)>,
}

impl SimpleACS712Sensitivity {
    pub fn new(controller: SimpleACS712Controller) -> Self {
        SimpleACS712Sensitivity {
            controller,
            sensitivities: Vec::new(),
        }
    }
}

impl ACS712Sensitivity for SimpleACS712Sensitivity {
    fn set_sensitivity(&mut self, sensor_id: ACS712ID, sens: f32) -> Result<(), ACS712Error> {
        self.sensitivities.push((sensor_id, AtomicUsize::new(sens.to_bits() as usize)));
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
