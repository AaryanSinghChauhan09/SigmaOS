#![no_std]
#![no_main]

/// OOP-based Light Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1306
/// Implements light and proximity sensors

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait LightSensor {
    fn id(&self) -> SensorID;
    fn read_lux(&self) -> u32;
}

#[repr(C)]
pub struct SimpleLightSensor {
    pub id: SensorID,
    pub lux: AtomicUsize,
}

impl SimpleLightSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleLightSensor {
            id,
            lux: AtomicUsize::new(500),
        }
    }
}

impl LightSensor for SimpleLightSensor {
    fn id(&self) -> SensorID { self.id }
    fn read_lux(&self) -> u32 { self.lux.load(Ordering::SeqCst) as u32 }
}

pub trait ProximitySensor {
    fn is_near(&self) -> bool;
    fn get_distance(&self) -> u8;
}

#[repr(C)]
pub struct SimpleProximitySensor {
    pub near: AtomicUsize,
    pub distance: AtomicUsize,
}

impl SimpleProximitySensor {
    pub fn new() -> Self {
        SimpleProximitySensor {
            near: AtomicUsize::new(0),
            distance: AtomicUsize::new(255),
        }
    }
}

impl ProximitySensor for SimpleProximitySensor {
    fn is_near(&self) -> bool { self.near.load(Ordering::SeqCst) == 1 }
    fn get_distance(&self) -> u8 { self.distance.load(Ordering::SeqCst) as u8 }
}

pub trait AmbientLight {
    def get_brightness_level(&self) -> u8;
    def auto_adjust_brightness(&mut self, target_lux: u32);
}

#[repr(C)]
pub struct SimpleAmbientLight {
    pub light_sensor: SimpleLightSensor,
    pub brightness: AtomicUsize,
}

impl SimpleAmbientLight {
    pub fn new(light_sensor: SimpleLightSensor) -> Self {
        SimpleAmbientLight {
            light_sensor,
            brightness: AtomicUsize::new(128),
        }
    }
}

impl AmbientLight for SimpleAmbientLight {
    fn get_brightness_level(&self) -> u8 {
        let lux = self.light_sensor.read_lux();
        if lux < 100 { 50 } else if lux < 500 { 128 } else { 255 }
    }
    
    fn auto_adjust_brightness(&mut self, target_lux: u32) {
        let current = self.light_sensor.read_lux();
        if current < target_lux {
            self.brightness.store(255, Ordering::SeqCst);
        } else {
            self.brightness.store(50, Ordering::SeqCst);
        }
    }
}
