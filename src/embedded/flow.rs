#![no_std]
#![no_main]

/// OOP-based Flow Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1796
/// Implements flow sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FlowID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FlowError { Success = 0, NotFound = 1 }

pub trait FlowSensor {
    fn id(&self) -> FlowID;
    fn flow_rate(&self) -> f32;
}

#[repr(C)]
pub struct SimpleFlowSensor {
    pub id: FlowID,
    pub flow_rate: AtomicUsize,
}

impl SimpleFlowSensor {
    pub fn new(id: FlowID) -> Self {
        SimpleFlowSensor {
            id,
            flow_rate: AtomicUsize::new(0),
        }
    }
}

impl FlowSensor for SimpleFlowSensor {
    fn id(&self) -> FlowID { self.id }
    fn flow_rate(&self) -> f32 { self.flow_rate.load(Ordering::SeqCst) as f32 }
}

pub trait FlowController {
    fn read(&self, sensor_id: FlowID) -> Result<f32, FlowError>;
    def calibrate(&mut self, sensor_id: FlowID) -> Result<(), FlowError>;
}

#[repr(C)]
pub struct SimpleFlowController {
    pub sensors: Vec<Option<Box<dyn FlowSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFlowController {
    pub fn new() -> Self {
        SimpleFlowController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FlowController for SimpleFlowController {
    fn read(&self, sensor_id: FlowID) -> Result<f32, FlowError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.flow_rate());
                }
            }
        }
        Err(FlowError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: FlowID) -> Result<(), FlowError> {
        Ok(())
    }
}

pub trait VolumeCalc {
    def get_volume(&self, sensor_id: FlowID) -> Result<f32, FlowError>;
    def reset_volume(&mut self, sensor_id: FlowID) -> Result<(), FlowError>;
}

#[repr(C)]
pub struct SimpleVolumeCalc {
    pub controller: SimpleFlowController,
    pub volumes: Vec<(FlowID, AtomicUsize)>,
}

impl SimpleVolumeCalc {
    pub fn new(controller: SimpleFlowController) -> Self {
        SimpleVolumeCalc {
            controller,
            volumes: Vec::new(),
        }
    }
}

impl VolumeCalc for SimpleVolumeCalc {
    fn get_volume(&self, sensor_id: FlowID) -> Result<f32, FlowError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(FlowError::NotFound)
        }
    }
    
    fn reset_volume(&mut self, sensor_id: FlowID) -> Result<(), FlowError> {
        self.volumes.push((sensor_id, AtomicUsize::new(0)));
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
