#![no_std]
#![no_main]

/// OOP-based HC-SR501 Motion for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3836
/// Implements HC-SR501 PIR motion sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HCSR501ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HCSR501Error { Success = 0, NotFound = 1 }

pub trait HCSR501Sensor {
    fn id(&self) -> HCSR501ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHCSR501Sensor {
    pub id: HCSR501ID,
    pub initialized: AtomicUsize,
}

impl SimpleHCSR501Sensor {
    pub fn new(id: HCSR501ID) -> Self {
        SimpleHCSR501Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HCSR501Sensor for SimpleHCSR501Sensor {
    fn id(&self) -> HCSR501ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HCSR501Controller {
    fn init(&mut self, sensor_id: HCSR501ID) -> Result<(), HCSR501Error>;
    fn read(&self, sensor_id: HCSR501ID) -> Result<bool, HCSR501Error>;
    def set_delay(&mut self, sensor_id: HCSR501ID, delay_ms: u16) -> Result<(), HCSR501Error>;
}

#[repr(C)]
pub struct SimpleHCSR501Controller {
    pub sensors: Vec<Option<Box<dyn HCSR501Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHCSR501Controller {
    pub fn new() -> Self {
        SimpleHCSR501Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HCSR501Controller for SimpleHCSR501Controller {
    fn init(&mut self, sensor_id: HCSR501ID) -> Result<(), HCSR501Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HCSR501Error::NotFound)
    }
    
    fn read(&self, sensor_id: HCSR501ID) -> Result<bool, HCSR501Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(HCSR501Error::NotFound)
        }
    }
    
    fn set_delay(&mut self, sensor_id: HCSR501ID, _delay_ms: u16) -> Result<(), HCSR501Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(HCSR501Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: HCSR501ID) -> Option<&dyn HCSR501Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait HCSR501Trigger {
    def set_trigger_mode(&mut self, sensor_id: HCSR501ID, repeat: bool) -> Result<(), HCSR501Error>;
}

#[repr(C)]
pub struct SimpleHCSR501Trigger {
    pub controller: SimpleHCSR501Controller,
    pub modes: Vec<(HCSR501ID, AtomicUsize)>,
}

impl SimpleHCSR501Trigger {
    pub fn new(controller: SimpleHCSR501Controller) -> Self {
        SimpleHCSR501Trigger {
            controller,
            modes: Vec::new(),
        }
    }
}

impl HCSR501Trigger for SimpleHCSR501Trigger {
    fn set_trigger_mode(&mut self, sensor_id: HCSR501ID, repeat: bool) -> Result<(), HCSR501Error> {
        self.modes.push((sensor_id, AtomicUsize::new(if repeat { 1 } else { 0 })));
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
