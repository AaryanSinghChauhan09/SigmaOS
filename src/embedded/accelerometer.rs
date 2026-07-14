#![no_std]
#![no_main]

/// OOP-based Accelerometer for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1266
/// Implements accelerometer sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait Accelerometer {
    fn id(&self) -> SensorID;
    fn read_x(&self) -> i16;
    fn read_y(&self) -> i16;
    fn read_z(&self) -> i16;
}

#[repr(C)]
pub struct SimpleAccelerometer {
    pub id: SensorID,
    pub x: AtomicUsize,
    pub y: AtomicUsize,
    pub z: AtomicUsize,
}

impl SimpleAccelerometer {
    pub fn new(id: SensorID) -> Self {
        SimpleAccelerometer {
            id,
            x: AtomicUsize::new(0),
            y: AtomicUsize::new(0),
            z: AtomicUsize::new(1000),
        }
    }
}

impl Accelerometer for SimpleAccelerometer {
    fn id(&self) -> SensorID { self.id }
    fn read_x(&self) -> i16 { self.x.load(Ordering::SeqCst) as i16 }
    fn read_y(&self) -> i16 { self.y.load(Ordering::SeqCst) as i16 }
    fn read_z(&self) -> i16 { self.z.load(Ordering::SeqCst) as i16 }
}

pub trait IMUSensor {
    fn init(&mut self) -> Result<(), SensorError>;
    fn read_accel(&self) -> (i16, i16, i16);
    def read_gyro(&self) -> (i16, i16, i16);
}

#[repr(C)]
pub struct SimpleIMUSensor {
    pub accelerometer: SimpleAccelerometer,
    pub gyro_x: AtomicUsize,
    pub gyro_y: AtomicUsize,
    pub gyro_z: AtomicUsize,
}

impl SimpleIMUSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleIMUSensor {
            accelerometer: SimpleAccelerometer::new(id),
            gyro_x: AtomicUsize::new(0),
            gyro_y: AtomicUsize::new(0),
            gyro_z: AtomicUsize::new(0),
        }
    }
}

impl IMUSensor for SimpleIMUSensor {
    fn init(&mut self) -> Result<(), SensorError> {
        Ok(())
    }
    
    fn read_accel(&self) -> (i16, i16, i16) {
        (
            self.accelerometer.read_x(),
            self.accelerometer.read_y(),
            self.accelerometer.read_z(),
        )
    }
    
    fn read_gyro(&self) -> (i16, i16, i16) {
        (
            self.gyro_x.load(Ordering::SeqCst) as i16,
            self.gyro_y.load(Ordering::SeqCst) as i16,
            self.gyro_z.load(Ordering::SeqCst) as i16,
        )
    }
}

pub trait MotionDetection {
    def detect_motion(&self, threshold: i16) -> bool;
    def set_threshold(&mut self, threshold: i16);
}

#[repr(C)]
pub struct SimpleMotionDetection {
    pub imu: SimpleIMUSensor,
    pub threshold: AtomicUsize,
}

impl SimpleMotionDetection {
    pub fn new(imu: SimpleIMUSensor) -> Self {
        SimpleMotionDetection {
            imu,
            threshold: AtomicUsize::new(100),
        }
    }
}

impl MotionDetection for SimpleMotionDetection {
    fn detect_motion(&self, threshold: i16) -> {
        let (x, y, z) = self.imu.read_accel();
        x.abs() > threshold || y.abs() > threshold || z.abs() > threshold
    }
    
    fn set_threshold(&mut self, threshold: i16) {
        self.threshold.store(threshold as usize, Ordering::SeqCst);
    }
}
