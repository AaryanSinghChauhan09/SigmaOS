#![no_std]
#![no_main]

/// OOP-based Sound Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1346
/// Implements sound sensor and microphone

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait SoundSensor {
    fn id(&self) -> SensorID;
    fn read_decibels(&self) -> u8;
}

#[repr(C)]
pub struct SimpleSoundSensor {
    pub id: SensorID,
    pub decibels: AtomicUsize,
}

impl SimpleSoundSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleSoundSensor {
            id,
            decibels: AtomicUsize::new(60),
        }
    }
}

impl SoundSensor for SimpleSoundSensor {
    fn id(&self) -> SensorID { self.id }
    fn read_decibels(&self) -> u8 { self.decibels.load(Ordering::SeqCst) as u8 }
}

pub trait Microphone {
    def start_recording(&mut self) -> Result<(), SensorError>;
    def stop_recording(&mut self) -> Result<(), SensorError>;
    def read_sample(&self) -> i16;
}

#[repr(C)]
pub struct SimpleMicrophone {
    pub sensor: SimpleSoundSensor,
    pub recording: AtomicUsize,
}

impl SimpleMicrophone {
    pub fn new(sensor: SimpleSoundSensor) -> Self {
        SimpleMicrophone {
            sensor,
            recording: AtomicUsize::new(0),
        }
    }
}

impl Microphone for SimpleMicrophone {
    fn start_recording(&mut self) -> Result<(), SensorError> {
        self.recording.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop_recording(&mut self) -> Result<(), SensorError> {
        self.recording.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn read_sample(&self) -> i16 {
        if self.recording.load(Ordering::SeqCst) == 1 {
            (self.sensor.read_decibels() as i16) * 100
        } else {
            0
        }
    }
}

pub trait NoiseDetector {
    def set_threshold(&mut self, threshold: u8);
    def is_noisy(&self) -> bool;
    def trigger_alert(&mut self) -> bool;
}

#[repr(C)]
pub struct SimpleNoiseDetector {
    pub sensor: SimpleSoundSensor,
    pub threshold: AtomicUsize,
    pub alert_triggered: AtomicUsize,
}

impl SimpleNoiseDetector {
    pub fn new(sensor: SimpleSoundSensor) -> Self {
        SimpleNoiseDetector {
            sensor,
            threshold: AtomicUsize::new(80),
            alert_triggered: AtomicUsize::new(0),
        }
    }
}

impl NoiseDetector for SimpleNoiseDetector {
    fn set_threshold(&mut self, threshold: u8) {
        self.threshold.store(threshold as usize, Ordering::SeqCst);
    }
    
    fn is_noisy(&self) -> bool {
        self.sensor.read_decibels() > self.threshold.load(Ordering::SeqCst) as u8
    }
    
    fn trigger_alert(&mut self) -> bool {
        if self.is_noisy() {
            self.alert_triggered.store(1, Ordering::SeqCst);
            true
        } else {
            false
        }
    }
}
