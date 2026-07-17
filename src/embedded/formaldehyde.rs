#![no_std]
#![no_main]

/// OOP-based Formaldehyde Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1946
/// Implements formaldehyde sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FormalID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FormalError { Success = 0, NotFound = 1 }

pub trait FormaldehydeSensor {
    fn id(&self) -> FormalID;
    fn hcho_ppb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleFormaldehydeSensor {
    pub id: FormalID,
    pub hcho_ppb: AtomicUsize,
}

impl SimpleFormaldehydeSensor {
    pub fn new(id: FormalID) -> Self {
        SimpleFormaldehydeSensor {
            id,
            hcho_ppb: AtomicUsize::new(0),
        }
    }
}

impl FormaldehydeSensor for SimpleFormaldehydeSensor {
    fn id(&self) -> FormalID { self.id }
    fn hcho_ppb(&self) -> u16 { self.hcho_ppb.load(Ordering::SeqCst) as u16 }
}

pub trait FormalController {
    fn read(&self, sensor_id: FormalID) -> Result<u16, FormalError>;
    def calibrate(&mut self, sensor_id: FormalID) -> Result<(), FormalError>;
}

#[repr(C)]
pub struct SimpleFormalController {
    pub sensors: Vec<Option<Box<dyn FormaldehydeSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFormalController {
    pub fn new() -> Self {
        SimpleFormalController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FormalController for SimpleFormalController {
    fn read(&self, sensor_id: FormalID) -> Result<u16, FormalError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.hcho_ppb());
                }
            }
        }
        Err(FormalError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: FormalID) -> Result<(), FormalError> {
        Ok(())
    }
}

pub trait HealthRisk {
    def get_risk(&self, sensor_id: FormalID) -> Result<u8, FormalError>;
    def is_safe(&self, sensor_id: FormalID) -> Result<bool, FormalError>;
}

#[repr(C)]
pub struct SimpleHealthRisk {
    pub controller: SimpleFormalController,
}

impl SimpleHealthRisk {
    pub fn new(controller: SimpleFormalController) -> Self {
        SimpleHealthRisk { controller }
    }
}

impl HealthRisk for SimpleHealthRisk {
    fn get_risk(&self, sensor_id: FormalID) -> Result<u8, FormalError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(FormalError::NotFound)
        }
    }
    
    fn is_safe(&self, sensor_id: FormalID) -> Result<bool, FormalError> {
        if let Ok(ppb) = self.controller.read(sensor_id) {
            Ok(ppb < 100)
        } else {
            Err(FormalError::NotFound)
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
