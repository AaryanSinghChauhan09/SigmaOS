#![no_std]
#![no_main]

/// OOP-based PM Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1916
/// Implements PM (Particulate Matter) sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PMError { Success = 0, NotFound = 1 }

pub trait PMSensor {
    fn id(&self) -> PMID;
    fn pm1_0(&self) -> u16;
    fn pm2_5(&self) -> u16;
    fn pm10(&self) -> u16;
}

#[repr(C)]
pub struct SimplePMSensor {
    pub id: PMID,
    pub pm1_0: AtomicUsize,
    pub pm2_5: AtomicUsize,
    pub pm10: AtomicUsize,
}

impl SimplePMSensor {
    pub fn new(id: PMID) -> Self {
        SimplePMSensor {
            id,
            pm1_0: AtomicUsize::new(0),
            pm2_5: AtomicUsize::new(0),
            pm10: AtomicUsize::new(0),
        }
    }
}

impl PMSensor for SimplePMSensor {
    fn id(&self) -> PMID { self.id }
    fn pm1_0(&self) -> u16 { self.pm1_0.load(Ordering::SeqCst) as u16 }
    fn pm2_5(&self) -> u16 { self.pm2_5.load(Ordering::SeqCst) as u16 }
    fn pm10(&self) -> u16 { self.pm10.load(Ordering::SeqCst) as u16 }
}

pub trait PMController {
    fn read(&self, sensor_id: PMID) -> Result<(u16, u16, u16), PMError>;
    def set_mode(&mut self, sensor_id: PMID, mode: u8) -> Result<(), PMError>;
}

#[repr(C)]
pub struct SimplePMController {
    pub sensors: Vec<Option<Box<dyn PMSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimplePMController {
    pub fn new() -> Self {
        SimplePMController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PMController for SimplePMController {
    fn read(&self, sensor_id: PMID) -> Result<(u16, u16, u16), PMError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok((sensor.pm1_0(), sensor.pm2_5(), sensor.pm10()));
                }
            }
        }
        Err(PMError::NotFound)
    }
    
    fn set_mode(&mut self, _sensor_id: PMID, _mode: u8) -> Result<(), PMError> {
        Ok(())
    }
}

pub trait AQICalc {
    def get_aqi(&self, sensor_id: PMID) -> Result<u16, PMError>;
    def get_category(&self, sensor_id: PMID) -> Result<u8, PMError>;
}

#[repr(C)]
pub struct SimpleAQICalc {
    pub controller: SimplePMController,
}

impl SimpleAQICalc {
    pub fn new(controller: SimplePMController) -> Self {
        SimpleAQICalc { controller }
    }
}

impl AQICalc for SimpleAQICalc {
    fn get_aqi(&self, sensor_id: PMID) -> Result<u16, PMError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(PMError::NotFound)
        }
    }
    
    fn get_category(&self, sensor_id: PMID) -> Result<u8, PMError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(PMError::NotFound)
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
