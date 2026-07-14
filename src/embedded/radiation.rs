#![no_std]
#![no_main]

/// OOP-based Radiation Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1926
/// Implements radiation sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RadID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RadError { Success = 0, NotFound = 1 }

pub trait RadiationSensor {
    fn id(&self) -> RadID;
    fn cpm(&self) -> u16;
}

#[repr(C)]
pub struct SimpleRadiationSensor {
    pub id: RadID,
    pub cpm: AtomicUsize,
}

impl SimpleRadiationSensor {
    pub fn new(id: RadID) -> Self {
        SimpleRadiationSensor {
            id,
            cpm: AtomicUsize::new(0),
        }
    }
}

impl RadiationSensor for SimpleRadiationSensor {
    fn id(&self) -> RadID { self.id }
    fn cpm(&self) -> u16 { self.cpm.load(Ordering::SeqCst) as u16 }
}

pub trait RadController {
    fn read(&self, sensor_id: RadID) -> Result<u16, RadError>;
    def get_usv(&self, sensor_id: RadID) -> Result<f32, RadError>;
}

#[repr(C)]
pub struct SimpleRadController {
    pub sensors: Vec<Option<Box<dyn RadiationSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRadController {
    pub fn new() -> Self {
        SimpleRadController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RadController for SimpleRadController {
    fn read(&self, sensor_id: RadID) -> Result<u16, RadError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.cpm());
                }
            }
        }
        Err(RadError::NotFound)
    }
    
    fn get_usv(&self, sensor_id: RadID) -> Result<f32, RadError> {
        if self.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(RadError::NotFound)
        }
    }
}

pub trait AlarmThreshold {
    def set_threshold(&mut self, sensor_id: RadID, threshold: f32) -> Result<(), RadError>;
    def is_alarm(&self, sensor_id: RadID) -> Result<bool, RadError>;
}

#[repr(C)]
pub struct SimpleAlarmThreshold {
    pub controller: SimpleRadController,
    pub thresholds: Vec<(RadID, AtomicUsize)>,
}

impl SimpleAlarmThreshold {
    pub fn new(controller: SimpleRadController) -> Self {
        SimpleAlarmThreshold {
            controller,
            thresholds: Vec::new(),
        }
    }
}

impl AlarmThreshold for SimpleAlarmThreshold {
    fn set_threshold(&mut self, sensor_id: RadID, threshold: f32) -> Result<(), RadError> {
        self.thresholds.push((sensor_id, AtomicUsize::new((threshold * 100.0) as usize)));
        Ok(())
    }
    
    fn is_alarm(&self, sensor_id: RadID) -> Result<bool, RadError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(RadError::NotFound)
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
