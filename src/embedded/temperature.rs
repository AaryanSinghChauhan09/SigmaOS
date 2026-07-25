#![no_std]
#![no_main]

/// OOP-based Temperature Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1316
/// Implements temperature sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait TemperatureSensor {
    fn id(&self) -> SensorID;
    fn read_celsius(&self) -> f32;
    fn read_fahrenheit(&self) -> f32;
}

#[repr(C)]
pub struct SimpleTemperatureSensor {
    pub id: SensorID,
    pub temperature: AtomicUsize,
}

impl SimpleTemperatureSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleTemperatureSensor {
            id,
            temperature: AtomicUsize::new(250),
        }
    }
}

impl TemperatureSensor for SimpleTemperatureSensor {
    fn id(&self) -> SensorID { self.id }
    fn read_celsius(&self) -> f32 { (self.temperature.load(Ordering::SeqCst) as f32) / 10.0 }
    fn read_fahrenheit(&self) -> f32 { self.read_celsius() * 9.0 / 5.0 + 32.0 }
}

pub trait Thermostat {
    def set_target(&mut self, target: f32);
    def get_target(&self) -> f32;
    def is_heating(&self) -> bool;
    def is_cooling(&self) -> bool;
}

#[repr(C)]
pub struct SimpleThermostat {
    pub sensor: SimpleTemperatureSensor,
    pub target: AtomicUsize,
    pub mode: AtomicUsize,
}

impl SimpleThermostat {
    pub fn new(sensor: SimpleTemperatureSensor) -> Self {
        SimpleThermostat {
            sensor,
            target: AtomicUsize::new(220),
            mode: AtomicUsize::new(0),
        }
    }
}

impl Thermostat for SimpleThermostat {
    fn set_target(&mut self, target: f32) {
        self.target.store((target * 10.0) as usize, Ordering::SeqCst);
    }
    
    fn get_target(&self) -> f32 { (self.target.load(Ordering::SeqCst) as f32) / 10.0 }
    
    fn is_heating(&self) -> bool {
        self.mode.load(Ordering::SeqCst) == 1 && self.sensor.read_celsius() < self.get_target()
    }
    
    fn is_cooling(&self) -> bool {
        self.mode.load(Ordering::SeqCst) == 2 && self.sensor.read_celsius() > self.get_target()
    }
}

pub trait ThermalProtection {
    def set_threshold(&mut self, max_temp: f32);
    def is_overheated(&self) -> bool;
    def trigger_shutdown(&mut self) -> bool;
}

#[repr(C)]
pub struct SimpleThermalProtection {
    pub sensor: SimpleTemperatureSensor,
    pub threshold: AtomicUsize,
    pub shutdown_triggered: AtomicUsize,
}

impl SimpleThermalProtection {
    pub fn new(sensor: SimpleTemperatureSensor) -> Self {
        SimpleThermalProtection {
            sensor,
            threshold: AtomicUsize::new(800),
            shutdown_triggered: AtomicUsize::new(0),
        }
    }
}

impl ThermalProtection for SimpleThermalProtection {
    fn set_threshold(&mut self, max_temp: f32) {
        self.threshold.store((max_temp * 10.0) as usize, Ordering::SeqCst);
    }
    
    fn is_overheated(&self) -> bool {
        self.sensor.read_celsius() > (self.threshold.load(Ordering::SeqCst) as f32) / 10.0
    }
    
    fn trigger_shutdown(&mut self) -> bool {
        if self.is_overheated() {
            self.shutdown_triggered.store(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}
