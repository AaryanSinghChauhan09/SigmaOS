#![no_std]
#![no_main]

/// OOP-based Dissolved Oxygen Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1836
/// Implements dissolved oxygen sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DOID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DOError { Success = 0, NotFound = 1 }

pub trait DOSensor {
    fn id(&self) -> DOID;
    fn do_value(&self) -> f32;
}

#[repr(C)]
pub struct SimpleDOSensor {
    pub id: DOID,
    pub do_value: AtomicUsize,
}

impl SimpleDOSensor {
    pub fn new(id: DOID) -> Self {
        SimpleDOSensor {
            id,
            do_value: AtomicUsize::new(0),
        }
    }
}

impl DOSensor for SimpleDOSensor {
    fn id(&self) -> DOID { self.id }
    fn do_value(&self) -> f32 { self.do_value.load(Ordering::SeqCst) as f32 }
}

pub trait DOController {
    fn read(&self, sensor_id: DOID) -> Result<f32, DOError>;
    def calibrate(&mut self, sensor_id: DOID, point: u8, value: f32) -> Result<(), DOError>;
}

#[repr(C)]
pub struct SimpleDOController {
    pub sensors: Vec<Option<Box<dyn DOSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDOController {
    pub fn new() -> Self {
        SimpleDOController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DOController for SimpleDOController {
    fn read(&self, sensor_id: DOID) -> Result<f32, DOError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.do_value());
                }
            }
        }
        Err(DOError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: DOID, _point: u8, _value: f32) -> Result<(), DOError> {
        Ok(())
    }
}

pub trait SaturationCalc {
    def get_saturation(&self, sensor_id: DOID) -> Result<f32, DOError>;
    def set_temperature(&mut self, sensor_id: DOID, temp: f32) -> Result<(), DOError>;
}

#[repr(C)]
pub struct SimpleSaturationCalc {
    pub controller: SimpleDOController,
    pub temperatures: Vec<(DOID, AtomicUsize)>,
}

impl SimpleSaturationCalc {
    pub fn new(controller: SimpleDOController) -> Self {
        SimpleSaturationCalc {
            controller,
            temperatures: Vec::new(),
        }
    }
}

impl SaturationCalc for SimpleSaturationCalc {
    fn get_saturation(&self, sensor_id: DOID) -> Result<f32, DOError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(DOError::NotFound)
        }
    }
    
    fn set_temperature(&mut self, sensor_id: DOID, temp: f32) -> Result<(), DOError> {
        self.temperatures.push((sensor_id, AtomicUsize::new((temp * 100.0) as usize)));
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
