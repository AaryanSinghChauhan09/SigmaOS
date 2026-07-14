#![no_std]
#![no_main]

/// OOP-based Magnetometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1276
/// Implements magnetometer sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait Magnetometer {
    fn id(&self) -> SensorID;
    fn read_x(&self) -> i16;
    fn read_y(&self) -> i16;
    fn read_z(&self) -> i16;
}

#[repr(C)]
pub struct SimpleMagnetometer {
    pub id: SensorID,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub z: AtomicUsize,
}

impl SimpleMagnetometer {
    pub fn new(id: SensorID) -> Self {
        SimpleMagnetometer {
            id,
            x: AtomicUsize::new(0),
            y: AtomicUsize::new(0),
            z: AtomicUsize::new(0),
        }
    }
}

impl Magnetometer for SimpleMagnetometer {
    fn id(&self) -> SensorID { self.id }
    fn read_x(&self) -> i16 { self.x.load(Ordering::SeqCst) as i16 }
    fn read_y(&self) -> i16 { self.y.load(Ordering::SeqCst) as i16 }
    fn read_z(&self) -> i16 { self.z.load(Ordering::SeqCst) as i16 }
}

pub trait Compass {
    fn get_heading(&self) -> f32;
    def calibrate(&mut self) -> Result<(), SensorError>;
}

#[repr(C)]
pub struct SimpleCompass {
    pub magnetometer: SimpleMagnetometer,
}

impl SimpleCompass {
    pub fn new(magnetometer: SimpleMagnetometer) -> Self {
        SimpleCompass { magnetometer }
    }
}

impl Compass for SimpleCompass {
    fn get_heading(&self) -> f32 {
        let x = self.magnetometer.read_x() as f32;
        let y = self.magnetometer.read_y() as f32;
        ((y.atan2(x)) * 180.0 / 3.14159 + 360.0) % 360.0
    }
    
    fn calibrate(&mut self) -> Result<(), SensorError> {
        Ok(())
    }
}

pub trait OrientationSensor {
    def get_orientation(&self) -> (f32, f32, f32);
    def set_reference(&mut self, reference: (f32, f32, f32));
}

#[repr(C)]
pub struct SimpleOrientationSensor {
    pub reference: (AtomicUsize, AtomicUsize, AtomicUsize),
}

impl SimpleOrientationSensor {
    pub fn new() -> Self {
        SimpleOrientationSensor {
            reference: (AtomicUsize::new(0), AtomicUsize::new(0), AtomicUsize::new(0)),
        }
    }
}

impl OrientationSensor for SimpleOrientationSensor {
    fn get_orientation(&self) -> (f32, f32, f32) {
        (
            self.reference.0.load(Ordering::SeqCst) as f32,
            self.reference.1.load(Ordering::SeqCst) as f32,
            self.reference.2.load(Ordering::SeqCst) as f32,
        )
    }
    
    fn set_reference(&mut self, reference: (f32, f32, f32)) {
        self.reference.0.store(reference.0 as usize, Ordering::SeqCst);
        self.reference.1.store(reference.1 as usize, Ordering::SeqCst);
        self.reference.2.store(reference.2 as usize, Ordering::SeqCst);
    }
}
