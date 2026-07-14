#![no_std]
#![no_main]

/// OOP-based Barometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1286
/// Implements barometric pressure sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait Barometer {
    fn id(&self) -> SensorID;
    fn read_pressure(&self) -> u32;
    fn read_temperature(&self) -> i16;
}

#[repr(C)]
pub struct SimpleBarometer {
    pub id: SensorID,
    pub pressure: AtomicUsize,
    pub temperature: AtomicUsize,
}

impl SimpleBarometer {
    pub fn new(id: SensorID) -> Self {
        SimpleBarometer {
            id,
            pressure: AtomicUsize::new(101325),
            temperature: AtomicUsize::new(250),
        }
    }
}

impl Barometer for SimpleBarometer {
    fn id(&self) -> SensorID { self.id }
    fn read_pressure(&self) -> u32 { self.pressure.load(Ordering::SeqCst) as u32 }
    fn read_temperature(&self) -> i16 { self.temperature.load(Ordering::SeqCst) as i16 }
}

pub trait Altimeter {
    fn get_altitude(&self, sea_level_pressure: u32) -> f32;
    def set_sea_level_pressure(&mut self, pressure: u32);
}

#[repr(C)]
pub struct SimpleAltimeter {
    pub barometer: SimpleBarometer,
    pub sea_level_pressure: AtomicUsize,
}

impl SimpleAltimeter {
    pub fn new(barometer: SimpleBarometer) -> Self {
        SimpleAltimeter {
            barometer,
            sea_level_pressure: AtomicUsize::new(101325),
        }
    }
}

impl Altimeter for SimpleAltimeter {
    fn get_altitude(&self, sea_level_pressure: u32) -> f32 {
        let pressure = self.barometer.read_pressure() as f32;
        let slp = sea_level_pressure as f32;
        44330.0 * (1.0 - (pressure / slp).powf(0.1903))
    }
    
    fn set_sea_level_pressure(&mut self, pressure: u32) {
        self.sea_level_pressure.store(pressure as usize, Ordering::SeqCst);
    }
}

pub trait WeatherStation {
    def get_pressure_trend(&self) -> &[u8];
    def forecast(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleWeatherStation {
    pub barometer: SimpleBarometer,
    pub history: Vec<u32>,
}

impl SimpleWeatherStation {
    pub fn new(barometer: SimpleBarometer) -> Self {
        SimpleWeatherStation {
            barometer,
            history: Vec::new(),
        }
    }
}

impl WeatherStation for SimpleWeatherStation {
    fn get_pressure_trend(&self) -> &[u8] {
        if self.history.len() > 1 {
            let last = self.history[self.history.len() - 1];
            let prev = self.history[self.history.len() - 2];
            if last > prev { b"rising" } else if last < prev { b"falling" } else { b"stable" }
        } else {
            b"unknown"
        }
    }
    
    fn forecast(&self) -> &[u8] {
        b"clear"
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
