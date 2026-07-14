#![no_std]
#![no_main]

/// OOP-based Humidity Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1296
/// Implements humidity sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait HumiditySensor {
    fn id(&self) -> SensorID;
    fn read_humidity(&self) -> u8;
    fn read_temperature(&self) -> i16;
}

#[repr(C)]
pub struct SimpleHumiditySensor {
    pub id: SensorID,
    pub humidity: AtomicUsize,
    pub temperature: AtomicUsize,
}

impl SimpleHumiditySensor {
    pub fn new(id: SensorID) -> Self {
        SimpleHumiditySensor {
            id,
            humidity: AtomicUsize::new(50),
            temperature: AtomicUsize::new(250),
        }
    }
}

impl HumiditySensor for SimpleHumiditySensor {
    fn id(&self) -> SensorID { self.id }
    fn read_humidity(&self) -> u8 { self.humidity.load(Ordering::SeqCst) as u8 }
    fn read_temperature(&self) -> i16 { self.temperature.load(Ordering::SeqCst) as i16 }
}

pub trait EnvironmentalSensor {
    fn get_dew_point(&self) -> f32;
    fn get_heat_index(&self) -> f32;
}

#[repr(C)]
pub struct SimpleEnvironmentalSensor {
    pub humidity_sensor: SimpleHumiditySensor,
}

impl SimpleEnvironmentalSensor {
    pub fn new(humidity_sensor: SimpleHumiditySensor) -> Self {
        SimpleEnvironmentalSensor { humidity_sensor }
    }
}

impl EnvironmentalSensor for SimpleEnvironmentalSensor {
    fn get_dew_point(&self) -> f32 {
        let h = self.humidity_sensor.read_humidity() as f32;
        let t = self.humidity_sensor.read_temperature() as f32 / 10.0;
        let a = (17.27 * t) / (237.7 + t);
        let alpha = ((h as f32) / 100.0).ln() + a;
        (237.7 * alpha) / (17.27 - alpha)
    }
    
    fn get_heat_index(&self) -> f32 {
        let h = self.humidity_sensor.read_humidity() as f32;
        let t = self.humidity_sensor.read_temperature() as f32 / 10.0;
        let c1 = -8.78469475556;
        let c2 = 1.61139411;
        let c3 = 2.33854883889;
        let c4 = -0.14611605;
        let c5 = -0.012308094;
        let c6 = -0.0164248277778;
        let c7 = 0.002211732;
        let c8 = 0.00072546;
        let c9 = -0.000003582;
        t + (c1 + c2 * t + c3 * h + c4 * t * h + c5 * t * t + c6 * h * h + c7 * t * t * h + c8 * t * h * h + c9 * t * t * h * h)
    }
}

pub trait ComfortLevel {
    def get_comfort_level(&self) -> &[u8];
    def get_recommendation(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleComfortLevel {
    pub sensor: SimpleEnvironmentalSensor,
}

impl SimpleComfortLevel {
    pub fn new(sensor: SimpleEnvironmentalSensor) -> Self {
        SimpleComfortLevel { sensor }
    }
}

impl ComfortLevel for SimpleComfortLevel {
    fn get_comfort_level(&self) -> &[u8] {
        let h = self.sensor.humidity_sensor.read_humidity();
        let t = self.sensor.humidity_sensor.read_temperature() as f32 / 10.0;
        if h >= 40 && h <= 60 && t >= 20.0 && t <= 26.0 {
            b"comfortable"
        } else if h > 60 {
            b"humid"
        } else if h < 40 {
            b"dry"
        } else {
            b"uncomfortable"
        }
    }
    
    fn get_recommendation(&self) -> &[u8] {
        b"adjust_humidity"
    }
}
