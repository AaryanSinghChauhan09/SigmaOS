// (no_std only applicable at crate root - removed)
#![no_main]

/// OOP-based EC pH Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4006
/// Implements EC (electrical conductivity) sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ECPHID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ECPHError { Success = 0, NotFound = 1 }

pub trait ECPHSensor {
    fn id(&self) -> ECPHID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleECPHSensor {
    pub id: ECPHID,
    pub initialized: AtomicUsize,
}

impl SimpleECPHSensor {
    pub fn new(id: ECPHID) -> Self {
        SimpleECPHSensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ECPHSensor for SimpleECPHSensor {
    fn id(&self) -> ECPHID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ECPHController {
    fn init(&mut self, sensor_id: ECPHID) -> Result<(), ECPHError>;
    fn read_ec(&self, sensor_id: ECPHID) -> Result<u16, ECPHError>;
    def read_ph(&self, sensor_id: ECPHID) -> Result<f32, ECPHError>;
}

#[repr(C)]
pub struct SimpleECPHController {
    pub sensors: Vec<Option<Box<dyn ECPHSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleECPHController {
    pub fn new() -> Self {
        SimpleECPHController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ECPHController for SimpleECPHController {
    fn init(&mut self, sensor_id: ECPHID) -> Result<(), ECPHError> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ECPHError::NotFound)
    }
    
    fn read_ec(&self, sensor_id: ECPHID) -> Result<u16, ECPHError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(ECPHError::NotFound)
        }
    }
    
    fn read_ph(&self, sensor_id: ECPHID) -> Result<f32, ECPHError> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(ECPHError::NotFound)
        }
    }
    
    fn get_sensor(&self, id: ECPHID) -> Option<&dyn ECPHSensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait ECPHTemperature {
    def read_temp(&self, sensor_id: ECPHID) -> Result<f32, ECPHError>;
}

#[repr(C)]
pub struct SimpleECPHTemperature {
    pub controller: SimpleECPHController,
}

impl SimpleECPHTemperature {
    pub fn new(controller: SimpleECPHController) -> Self {
        SimpleECPHTemperature { controller }
    }
}

impl ECPHTemperature for SimpleECPHTemperature {
    fn read_temp(&self, sensor_id: ECPHID) -> Result<f32, ECPHError> {
        if self.controller.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(ECPHError::NotFound)
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
