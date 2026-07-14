#![no_std]
#![no_main]

/// OOP-based LM35 Temperature for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3766
/// Implements LM35 analog temperature sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type LM35ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LM35Error { Success = 0, NotFound = 1 }

pub trait LM35Sensor {
    fn id(&self) -> LM35ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleLM35Sensor {
    pub id: LM35ID,
    pub initialized: AtomicUsize,
}

impl SimpleLM35Sensor {
    pub fn new(id: LM35ID) -> Self {
        SimpleLM35Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl LM35Sensor for SimpleLM35Sensor {
    fn id(&self) -> LM35ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait LM35Controller {
    fn init(&mut self, sensor_id: LM35ID) -> Result<(), LM35Error>;
    fn read(&self, sensor_id: LM35ID) -> Result<f32, LM35Error>;
    def calibrate(&mut self, sensor_id: LM35ID, offset: f32) -> Result<(), LM35Error>;
}

#[repr(C)]
pub struct SimpleLM35Controller {
    pub sensors: Vec<Option<Box<dyn LM35Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleLM35Controller {
    pub fn new() -> Self {
        SimpleLM35Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl LM35Controller for SimpleLM35Controller {
    fn init(&mut self, sensor_id: LM35ID) -> Result<(), LM35Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(LM35Error::NotFound)
    }
    
    fn read(&self, sensor_id: LM35ID) -> Result<f32, LM35Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(LM35Error::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: LM35ID, _offset: f32) -> Result<(), LM35Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(LM35Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: LM35ID) -> Option<&dyn LM35Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait LM35ADC {
    def set_adc_channel(&mut self, sensor_id: LM35ID, channel: u8) -> Result<(), LM35Error>;
}

#[repr(C)]
pub struct SimpleLM35ADC {
    pub controller: SimpleLM35Controller,
    pub channels: Vec<(LM35ID, AtomicUsize)>,
}

impl SimpleLM35ADC {
    pub fn new(controller: SimpleLM35Controller) -> Self {
        SimpleLM35ADC {
            controller,
            channels: Vec::new(),
        }
    }
}

impl LM35ADC for SimpleLM35ADC {
    fn set_adc_channel(&mut self, sensor_id: LM35ID, channel: u8) -> Result<(), LM35Error> {
        self.channels.push((sensor_id, AtomicUsize::new(channel as usize)));
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
