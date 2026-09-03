#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Flow Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1796
/// Implements flow sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type FlowID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum FlowError { Success = 0, NotFound = 1 }

pub trait FlowSensor {
    fn id(&self) -> FlowID;
    fn flow_rate(&self) -> f32;
}

#[repr(C)]
pub struct SimpleFlowSensor {
    pub id: FlowID,
    pub flow_rate: AtomicUsize,
}

impl SimpleFlowSensor {
    pub fn new(id: FlowID) -> Self {
        SimpleFlowSensor {
            id,
            flow_rate: AtomicUsize::new(0),
        }
    }
}

impl FlowSensor for SimpleFlowSensor {
    fn id(&self) -> FlowID { self.id }
    fn flow_rate(&self) -> f32 { self.flow_rate.load(Ordering::SeqCst) as f32 }
}

pub trait FlowController {
    fn read(&self, sensor_id: FlowID) -> Result<f32, FlowError>;
    def calibrate(&mut self, sensor_id: FlowID) -> Result<(), FlowError>;
}

#[repr(C)]
pub struct SimpleFlowController {
    pub sensors: Vec<Option<Box<dyn FlowSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleFlowController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleFlowController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl FlowController for SimpleFlowController {
    fn read(&self, sensor_id: FlowID) -> Result<f32, FlowError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.flow_rate());
                }
            }
        }
        Err(FlowError::NotFound)
    }
    
    fn calibrate(&mut self, _sensor_id: FlowID) -> Result<(), FlowError> {
        Ok(())
    }
}

pub trait VolumeCalc {
    def get_volume(&self, sensor_id: FlowID) -> Result<f32, FlowError>;
    def reset_volume(&mut self, sensor_id: FlowID) -> Result<(), FlowError>;
}

#[repr(C)]
pub struct SimpleVolumeCalc {
    pub controller: SimpleFlowController,
    pub volumes: Vec<(FlowID, AtomicUsize)>,
}

impl SimpleVolumeCalc {
    pub fn new(controller: SimpleFlowController) -> Self {
        SimpleVolumeCalc {
            controller,
            volumes: Vec::new(),
        }
    }
}

impl VolumeCalc for SimpleVolumeCalc {
    fn get_volume(&self, sensor_id: FlowID) -> Result<f32, FlowError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0.0)
        } else {
            Err(FlowError::NotFound)
        }
    }
    
    fn reset_volume(&mut self, sensor_id: FlowID) -> Result<(), FlowError> {
        self.volumes.push((sensor_id, AtomicUsize::new(0)));
        Ok(())
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
