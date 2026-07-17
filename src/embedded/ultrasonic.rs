#![no_std]
#![no_main]

/// OOP-based Ultrasonic Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1366
/// Implements ultrasonic distance sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait UltrasonicSensor {
    fn id(&self) -> SensorID;
    fn measure_distance(&self) -> u16;
}

#[repr(C)]
pub struct SimpleUltrasonicSensor {
    pub id: SensorID,
    pub distance: AtomicUsize,
}

impl SimpleUltrasonicSensor {
    pub fn new(id: SensorID) -> Self {
        SimpleUltrasonicSensor {
            id,
            distance: AtomicUsize::new(100),
        }
    }
}

impl UltrasonicSensor for SimpleUltrasonicSensor {
    fn id(&self) -> SensorID { self.id }
    fn measure_distance(&self) -> u16 { self.distance.load(Ordering::SeqCst) as u16 }
}

pub trait DistanceSensor {
    def set_threshold(&mut self, threshold: u16);
    def is_within_range(&self) -> bool;
    def get_range_status(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleDistanceSensor {
    pub sensor: SimpleUltrasonicSensor,
    pub threshold: AtomicUsize,
}

impl SimpleDistanceSensor {
    pub fn new(sensor: SimpleUltrasonicSensor) -> Self {
        SimpleDistanceSensor {
            sensor,
            threshold: AtomicUsize::new(50),
        }
    }
}

impl DistanceSensor for SimpleDistanceSensor {
    fn set_threshold(&mut self, threshold: u16) {
        self.threshold.store(threshold as usize, Ordering::SeqCst);
    }
    
    fn is_within_range(&self) -> bool {
        self.sensor.measure_distance() < self.threshold.load(Ordering::SeqCst) as u16
    }
    
    fn get_range_status(&self) -> &[u8] {
        let dist = self.sensor.measure_distance();
        if dist < 10 { b"too_close" } else if dist > 200 { b"too_far" } else { b"in_range" }
    }
}

pub trait ObstacleDetector {
    def scan_for_obstacles(&self) -> Vec<(u16, u16)>;
    def get_nearest_obstacle(&self) -> Option<u16>;
}

#[repr(C)]
pub struct SimpleObstacleDetector {
    pub sensors: Vec<SimpleUltrasonicSensor>,
}

impl SimpleObstacleDetector {
    pub fn new() -> Self {
        SimpleObstacleDetector {
            sensors: Vec::new(),
        }
    }
}

impl ObstacleDetector for SimpleObstacleDetector {
    fn scan_for_obstacles(&self) -> Vec<(u16, u16)> {
        let mut obstacles = Vec::new();
        for (i, sensor) in self.sensors.iter().enumerate() {
            let dist = sensor.measure_distance();
            if dist < 100 {
                obstacles.push((i as u16, dist));
            }
        }
        obstacles
    }
    
    fn get_nearest_obstacle(&self) -> Option<u16> {
        let mut nearest = None;
        let mut min_dist = u16::MAX;
        for sensor in &self.sensors {
            let dist = sensor.measure_distance();
            if dist < min_dist {
                min_dist = dist;
                nearest = Some(dist);
            }
        }
        nearest
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
