#![no_std]
#![no_main]

/// OOP-based HYDR Soil Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3986
/// Implements HYDR soil moisture sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SoilHYDRID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SoilHYDRError { Success = 0, NotFound = 1 }

pub trait SoilHYDRSensor {
    fn id(&self) -> SoilHYDRID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSoilHYDRSensor {
    pub id: SoilHYDRID,
    pub initialized: AtomicUsize,
}

impl SimpleSoilHYDRSensor {
    pub fn new(id: SoilHYDRID) -> Self {
        SimpleSoilHYDRSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl SoilHYDRSensor for SimpleSoilHYDRSensor {
    fn id(&self) -> SoilHYDRID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait SoilHYDRController {
    fn init(&mut self, sensor_id: SoilHYDRID) -> Result<(), SoilHYDRError>;
    fn read(&self, sensor_id: SoilHYDRID) -> Result<u16, SoilHYDRError>;
    def calibrate(&mut self, sensor_id: SoilHYDRID, dry: u16, wet: u16) -> Result<(), SoilHYDRError>;
}

#[repr(C)]
pub struct SimpleSoilHYDRController {
    pub sensors: Vec<Option<Box<dyn SoilHYDRSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSoilHYDRController {
    pub fn new() -> Self {
        SimpleSoilHYDRController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SoilHYDRController for SimpleSoilHYDRController {
    fn init(&mut self, sensor_id: SoilHYDRID) -> Result<(), SoilHYDRError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SoilHYDRError::NotFound)
    }
    
    fn read(&self, sensor_id: SoilHYDRID) -> Result<u16, SoilHYDRError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(SoilHYDRError::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: SoilHYDRID, _dry: u16, _wet: u16) -> Result<(), SoilHYDRError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(SoilHYDRError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: SoilHYDRID) -> Option<&dyn SoilHYDRSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait SoilHYDRMoisture {
    def read_moisture(&self, sensor_id: SoilHYDRID) -> Result<u8, SoilHYDRError>;
}

#[repr(C)]
pub struct SimpleSoilHYDRMoisture {
    pub controller: SimpleSoilHYDRController,
}

impl SimpleSoilHYDRMoisture {
    pub fn new(controller: SimpleSoilHYDRController) -> Self {
        SimpleSoilHYDRMoisture { controller }
    }
}

impl SoilHYDRMoisture for SimpleSoilHYDRMoisture {
    fn read_moisture(&self, sensor_id: SoilHYDRID) -> Result<u8, SoilHYDRError> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(SoilHYDRError::NotFound)
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
