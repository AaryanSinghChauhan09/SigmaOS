#![no_std]
#![no_main]

/// OOP-based MLX90614 Temperature for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3746
/// Implements MLX90614 infrared temperature sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type MLX90614ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum MLX90614Error { Success = 0, NotFound = 1 }

pub trait MLX90614Sensor {
    fn id(&self) -> MLX90614ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleMLX90614Sensor {
    pub id: MLX90614ID,
    pub initialized: AtomicUsize,
}

impl SimpleMLX90614Sensor {
    pub fn new(id: MLX90614ID) -> Self {
        SimpleMLX90614Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl MLX90614Sensor for SimpleMLX90614Sensor {
    fn id(&self) -> MLX90614ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait MLX90614Controller {
    fn init(&mut self, sensor_id: MLX90614ID) -> Result<(), MLX90614Error>;
    fn read_object(&self, sensor_id: MLX90614ID) -> Result<f32, MLX90614Error>;
    def read_ambient(&self, sensor_id: MLX90614ID) -> Result<f32, MLX90614Error>;
}

#[repr(C)]
pub struct SimpleMLX90614Controller {
    pub sensors: Vec<Option<Box<dyn MLX90614Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleMLX90614Controller {
    pub fn new() -> Self {
        SimpleMLX90614Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl MLX90614Controller for SimpleMLX90614Controller {
    fn init(&mut self, sensor_id: MLX90614ID) -> Result<(), MLX90614Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(MLX90614Error::NotFound)
    }
    
    fn read_object(&self, sensor_id: MLX90614ID) -> Result<f32, MLX90614Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(MLX90614Error::NotFound)
        }
    }
    
    fn read_ambient(&self, sensor_id: MLX90614ID) -> Result<f32, MLX90614Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(MLX90614Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: MLX90614ID) -> Option<&dyn MLX90614Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait MLX90614Emissivity {
    def set_emissivity(&mut self, sensor_id: MLX90614ID, emissivity: f32) -> Result<(), MLX90614Error>;
}

#[repr(C)]
pub struct SimpleMLX90614Emissivity {
    pub controller: SimpleMLX90614Controller,
    pub emissivities: Vec<(MLX90614ID, AtomicUsize)>,
}

impl SimpleMLX90614Emissivity {
    pub fn new(controller: SimpleMLX90614Controller) -> Self {
        SimpleMLX90614Emissivity {
            controller,
            emissivities: Vec::new(),
        }
    }
}

impl MLX90614Emissivity for SimpleMLX90614Emissivity {
    fn set_emissivity(&mut self, sensor_id: MLX90614ID, emissivity: f32) -> Result<(), MLX90614Error> {
        self.emissivities.push((sensor_id, AtomicUsize::new(emissivity.to_bits() as usize)));
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
