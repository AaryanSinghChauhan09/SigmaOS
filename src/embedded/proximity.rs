#![no_std]
#![no_main]

/// OOP-based Proximity Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1756
/// Implements proximity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ProxID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ProxError { Success = 0, NotFound = 1 }

pub trait ProximitySensor {
    fn id(&self) -> ProxID;
    fn distance(&self) -> u16;
}

#[repr(C)]
pub struct SimpleProximitySensor {
    pub id: ProxID,
    pub distance: AtomicUsize,
}

impl SimpleProximitySensor {
    pub fn new(id: ProxID) -> Self {
        SimpleProximitySensor {
            id,
            distance: AtomicUsize::new(0),
        }
    }
}

impl ProximitySensor for SimpleProximitySensor {
    fn id(&self) -> ProxID { self.id }
    fn distance(&self) -> u16 { self.distance.load(Ordering::SeqCst) as u16 }
}

pub trait ProxController {
    fn read(&self, sensor_id: ProxID) -> Result<u16, ProxError>;
    def set_threshold(&mut self, sensor_id: ProxID, threshold: u16) -> Result<(), ProxError>;
}

#[repr(C)]
pub struct SimpleProxController {
    pub sensors: Vec<Option<Box<dyn ProximitySensor>>>,
    pub thresholds: Vec<(ProxID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleProxController {
    pub fn new() -> Self {
        SimpleProxController {
            sensors: Vec::new(),
            thresholds: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ProxController for SimpleProxController {
    fn read(&self, sensor_id: ProxID) -> Result<u16, ProxError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.distance());
                }
            }
        }
        Err(ProxError::NotFound)
    }
    
    fn set_threshold(&mut self, sensor_id: ProxID, threshold: u16) -> Result<(), ProxError> {
        self.thresholds.push((sensor_id, AtomicUsize::new(threshold as usize)));
        Ok(())
    }
}

pub trait ObjectDetection {
    def is_detected(&self, sensor_id: ProxID) -> Result<bool, ProxError>;
    def get_count(&self, sensor_id: ProxID) -> Result<u8, ProxError>;
}

#[repr(C)]
pub struct SimpleObjectDetection {
    pub controller: SimpleProxController,
}

impl SimpleObjectDetection {
    pub fn new(controller: SimpleProxController) -> Self {
        SimpleObjectDetection { controller }
    }
}

impl ObjectDetection for SimpleObjectDetection {
    fn is_detected(&self, sensor_id: ProxID) -> Result<bool, ProxError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(false)
        } else {
            Err(ProxError::NotFound)
        }
    }
    
    fn get_count(&self, sensor_id: ProxID) -> Result<u8, ProxError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(ProxError::NotFound)
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
