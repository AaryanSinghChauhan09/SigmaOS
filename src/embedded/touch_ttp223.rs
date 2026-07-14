#![no_std]
#![no_main]

/// OOP-based TTP223 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3866
/// Implements TTP223 capacitive touch sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TTP223ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TTP223Error { Success = 0, NotFound = 1 }

pub trait TTP223Sensor {
    fn id(&self) -> TTP223ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTTP223Sensor {
    pub id: TTP223ID,
    pub initialized: AtomicUsize,
}

impl SimpleTTP223Sensor {
    pub fn new(id: TTP223ID) -> Self {
        SimpleTTP223Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TTP223Sensor for SimpleTTP223Sensor {
    fn id(&self) -> TTP223ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TTP223Controller {
    fn init(&mut self, sensor_id: TTP223ID) -> Result<(), TTP223Error>;
    fn read(&self, sensor_id: TTP223ID) -> Result<bool, TTP223Error>;
    def set_sensitivity(&mut self, sensor_id: TTP223ID, level: u8) -> Result<(), TTP223Error>;
}

#[repr(C)]
pub struct SimpleTTP223Controller {
    pub sensors: Vec<Option<Box<dyn TTP223Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTTP223Controller {
    pub fn new() -> Self {
        SimpleTTP223Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TTP223Controller for SimpleTTP223Controller {
    fn init(&mut self, sensor_id: TTP223ID) -> Result<(), TTP223Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TTP223Error::NotFound)
    }
    
    fn read(&self, sensor_id: TTP223ID) -> Result<bool, TTP223Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(TTP223Error::NotFound)
        }
    }
    
    fn set_sensitivity(&mut self, sensor_id: TTP223ID, _level: u8) -> Result<(), TTP223Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(TTP223Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: TTP223ID) -> Option<&dyn TTP223Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait TTP223Mode {
    def set_mode(&mut self, sensor_id: TTP223ID, toggle: bool) -> Result<(), TTP223Error>;
}

#[repr(C)]
pub struct SimpleTTP223Mode {
    pub controller: SimpleTTP223Controller,
    pub modes: Vec<(TTP223ID, AtomicUsize)>,
}

impl SimpleTTP223Mode {
    pub fn new(controller: SimpleTTP223Controller) -> Self {
        SimpleTTP223Mode {
            controller,
            modes: Vec::new(),
        }
    }
}

impl TTP223Mode for SimpleTTP223Mode {
    fn set_mode(&mut self, sensor_id: TTP223ID, toggle: bool) -> Result<(), TTP223Error> {
        self.modes.push((sensor_id, AtomicUsize::new(if toggle { 1 } else { 0 })));
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
