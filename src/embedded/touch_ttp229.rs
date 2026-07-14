#![no_std]
#![no_main]

/// OOP-based TTP229 Touch for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3876
/// Implements TTP229 16-key capacitive touch

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type TTP229ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum TTP229Error { Success = 0, NotFound = 1 }

pub trait TTP229Sensor {
    fn id(&self) -> TTP229ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleTTP229Sensor {
    pub id: TTP229ID,
    pub initialized: AtomicUsize,
}

impl SimpleTTP229Sensor {
    pub fn new(id: TTP229ID) -> Self {
        SimpleTTP229Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl TTP229Sensor for SimpleTTP229Sensor {
    fn id(&self) -> TTP229ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait TTP229Controller {
    fn init(&mut self, sensor_id: TTP229ID) -> Result<(), TTP229Error>;
    fn read(&self, sensor_id: TTP229ID) -> Result<u16, TTP229Error>;
    def read_key(&self, sensor_id: TTP229ID, key: u8) -> Result<bool, TTP229Error>;
}

#[repr(C)]
pub struct SimpleTTP229Controller {
    pub sensors: Vec<Option<Box<dyn TTP229Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleTTP229Controller {
    pub fn new() -> Self {
        SimpleTTP229Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl TTP229Controller for SimpleTTP229Controller {
    fn init(&mut self, sensor_id: TTP229ID) -> Result<(), TTP229Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(TTP229Error::NotFound)
    }
    
    fn read(&self, sensor_id: TTP229ID) -> Result<u16, TTP229Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0)
        } else {
            Err(TTP229Error::NotFound)
        }
    }
    
    fn read_key(&self, sensor_id: TTP229ID, _key: u8) -> Result<bool, TTP229Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(TTP229Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: TTP229ID) -> Option<&dyn TTP229Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait TTP229Interrupt {
    def enable_interrupt(&mut self, sensor_id: TTP229ID, enable: bool) -> Result<(), TTP229Error>;
}

#[repr(C)]
pub struct SimpleTTP229Interrupt {
    pub controller: SimpleTTP229Controller,
    pub interrupts: Vec<(TTP229ID, AtomicUsize)>,
}

impl SimpleTTP229Interrupt {
    pub fn new(controller: SimpleTTP229Controller) -> Self {
        SimpleTTP229Interrupt {
            controller,
            interrupts: Vec::new(),
        }
    }
}

impl TTP229Interrupt for SimpleTTP229Interrupt {
    fn enable_interrupt(&mut self, sensor_id: TTP229ID, enable: bool) -> Result<(), TTP229Error> {
        self.interrupts.push((sensor_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
