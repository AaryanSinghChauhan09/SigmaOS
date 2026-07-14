#![no_std]
#![no_main]

/// OOP-based Vibration Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1336
/// Implements vibration sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait VibrationSensor {
    fn id(&self) -> SensorID;
    fn read_acceleration(&self) -> (i16, i16, i16);
    fn is_vibrating(&self) -> bool;
}

#[repr(C)]
pub struct SimpleVibrationSensor {
    pub id: SensorID,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub z: AtomicUsize,
    pub threshold: AtomicUsize,
}

impl SimpleVibrationSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleVibrationSensor {
            id,
            x: AtomicUsize::new(0),
            y: AtomicUsize::new(0),
            z: AtomicUsize::new(0),
            threshold: AtomicUsize::new(100),
        }
    }
}

impl VibrationSensor for SimpleVibrationSensor {
    fn id(&self) -> SensorID { self.id }
    fn read_acceleration(&self) -> (i16, i16, i16) {
        (
            self.x.load(Ordering::SeqCst) as i16,
            self.y.load(Ordering::SeqCst) as i16,
            self.z.load(Ordering::SeqCst) as i16,
        )
    }
    fn is_vibrating(&self) -> bool {
        let (x, y, z) = self.read_acceleration();
        let threshold = self.threshold.load(Ordering::SeqCst) as i16;
        x.abs() > threshold || y.abs() > threshold || z.abs() > threshold
    }
}

pub trait ShockDetector {
    def detect_shock(&self) -> bool;
    def get_shock_magnitude(&self) -> u16;
}

#[repr(C)]
pub struct SimpleShockDetector {
    pub sensor: SimpleVibrationSensor,
    pub shock_detected: AtomicUsize,
}

impl SimpleShockDetector {
    pub fn new(sensor: SimpleVibrationSensor) -> Self {
        SimpleShockDetector {
            sensor,
            shock_detected: AtomicUsize::new(0),
        }
    }
}

impl ShockDetector for SimpleShockDetector {
    fn detect_shock(&self) -> bool {
        self.shock_detected.load(Ordering::SeqCst) == 1
    }
    
    fn get_shock_magnitude(&self) -> u16 {
        let (x, y, z) = self.sensor.read_acceleration();
        ((x.abs() + y.abs() + z.abs()) as u16) / 3
    }
}

pub trait SeismicMonitor {
    def record_event(&mut self, magnitude: u16);
    def get_events(&self) -> Vec<u16>;
}

#[repr(C)]
pub struct SimpleSeismicMonitor {
    pub events: Vec<u16>,
}

impl SimpleSeismicMonitor {
    pub fn new() -> Self {
        SimpleSeismicMonitor {
            events: Vec::new(),
        }
    }
}

impl SeismicMonitor for SimpleSeismicMonitor {
    fn record_event(&mut self, magnitude: u16) {
        self.events.push(magnitude);
    }
    
    fn get_events(&self) -> Vec<u16> {
        let mut result = Vec::new();
        for &event in &self.events {
            result.push(event);
        }
        result
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
