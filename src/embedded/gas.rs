#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Gas Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1326
/// Implements gas sensor for air quality

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SensorID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GasType { CO2 = 0, CO = 1, CH4 = 2, NO2 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SensorError { Success = 0, NotFound = 1 }

pub trait GasSensor {
    fn id(&self) -> SensorID;
    fn gas_type(&self) -> GasType;
    fn read_ppm(&self) -> u32;
}

#[repr(C)]
pub struct SimpleGasSensor {
    pub id: SensorID,
    pub gas_type: AtomicUsize,
    pub ppm: AtomicUsize,
}

impl SimpleGasSensor {
    pub fn new(id: SensorID, gas_type: GasType) -> Self {
        SimpleGasSensor {
            id,
            gas_type: AtomicUsize::new(gas_type as usize),
            ppm: AtomicUsize::new(400),
        }
    }
}

impl GasSensor for SimpleGasSensor {
    fn id(&self) -> SensorID { self.id }
    fn gas_type(&self) -> GasType { unsafe { core::mem::transmute(self.gas_type.load(Ordering::SeqCst)) } }
    fn read_ppm(&self) -> u32 { self.ppm.load(Ordering::SeqCst) as u32 }
}

pub trait AirQualityMonitor {
    fn get_aqi(&self) -> u8;
    def get_status(&self) -> &[u8];
}

#[repr(C)]
pub struct SimpleAirQualityMonitor {
    pub sensors: Vec<Option<Box<dyn GasSensor>>>,
}

impl SimpleAirQualityMonitor {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleAirQualityMonitor {
            sensors: Vec::new(),
        }
    }
}

impl AirQualityMonitor for SimpleAirQualityMonitor {
    fn get_aqi(&self) -> u8 {
        let mut total = 0u32;
        let mut count = 0u32;
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                total += sensor.read_ppm();
                count += 1;
            }
        }
        if count > 0 {
            ((total / count) as u8).min(255)
        } else {
            0
        }
    }
    
    fn get_status(&self) -> &[u8] {
        let aqi = self.get_aqi();
        if aqi < 50 { b"good" } else if aqi < 100 { b"moderate" } else if aqi < 150 { b"unhealthy_sensitive" } else { b"unhealthy" }
    }
}

pub trait GasLeakDetector {
    def set_threshold(&mut self, gas_type: GasType, threshold: u32);
    def detect_leak(&self, gas_type: GasType) -> bool;
}

#[repr(C)]
pub struct SimpleGasLeakDetector {
    pub monitor: SimpleAirQualityMonitor,
    pub thresholds: Vec<(GasType, AtomicUsize)>,
}

impl SimpleGasLeakDetector {
    pub fn new(monitor: SimpleAirQualityMonitor) -> Self {
        SimpleGasLeakDetector {
            monitor,
            thresholds: Vec::new(),
        }
    }
}

impl GasLeakDetector for SimpleGasLeakDetector {
    fn set_threshold(&mut self, gas_type: GasType, threshold: u32) {
        self.thresholds.push((gas_type, AtomicUsize::new(threshold as usize)));
    }
    
    fn detect_leak(&self, gas_type: GasType) -> bool {
        for sensor_option in &self.monitor.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.gas_type() == gas_type {
                    for &(gt, ref threshold) in &self.thresholds {
                        if gt == gas_type {
                            return sensor.read_ppm() > threshold.load(Ordering::SeqCst) as u32;
                        }
                    }
                }
            }
        }
        false
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


impl<T> core::ops::Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &Self::Target {
        if self.data.is_null() {
            &[]
        } else {
            unsafe { core::slice::from_raw_parts(self.data, self.len) }
        }
    }
}

impl<T> core::ops::DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        if self.data.is_null() {
            &mut []
        } else {
            unsafe { core::slice::from_raw_parts_mut(self.data, self.len) }
        }
    }
}

impl<'a, T> IntoIterator for &'a Vec<T> {
    type Item = &'a T;
    type IntoIter = core::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::Deref;
        self.deref().iter()
    }
}


impl<'a, T> IntoIterator for &'a mut Vec<T> {
    type Item = &'a mut T;
    type IntoIter = core::slice::IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        use core::ops::DerefMut;
        self.deref_mut().iter_mut()
    }
}
