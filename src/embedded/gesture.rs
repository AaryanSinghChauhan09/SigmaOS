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

/// OOP-based Gesture Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1776
/// Implements gesture sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type GestureID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GestureType { None = 0, Up = 1, Down = 2, Left = 3, Right = 4, Near = 5, Far = 6 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GestureError { Success = 0, NotFound = 1 }

pub trait GestureSensor {
    fn id(&self) -> GestureID;
    fn gesture(&self) -> GestureType;
}

#[repr(C)]
pub struct SimpleGestureSensor {
    pub id: GestureID,
    pub gesture: AtomicUsize,
}

impl SimpleGestureSensor {
    pub fn new(id: GestureID) -> Self {
        SimpleGestureSensor {
            id,
            gesture: AtomicUsize::new(GestureType::None as usize),
        }
    }
}

impl GestureSensor for SimpleGestureSensor {
    fn id(&self) -> GestureID { self.id }
    fn gesture(&self) -> GestureType { unsafe { core::mem::transmute(self.gesture.load(Ordering::SeqCst)) } }
}

pub trait GestureController {
    fn read(&self, sensor_id: GestureID) -> Result<GestureType, GestureError>;
    def enable(&mut self, sensor_id: GestureID) -> Result<(), GestureError>;
}

#[repr(C)]
pub struct SimpleGestureController {
    pub sensors: Vec<Option<Box<dyn GestureSensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleGestureController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleGestureController {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl GestureController for SimpleGestureController {
    fn read(&self, sensor_id: GestureID) -> Result<GestureType, GestureError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.gesture());
                }
            }
        }
        Err(GestureError::NotFound)
    }
    
    fn enable(&mut self, _sensor_id: GestureID) -> Result<(), GestureError> {
        Ok(())
    }
}

pub trait GestureRecognition {
    def set_threshold(&mut self, sensor_id: GestureID, threshold: u8) -> Result<(), GestureError>;
    def get_confidence(&self, sensor_id: GestureID) -> Result<u8, GestureError>;
}

#[repr(C)]
pub struct SimpleGestureRecognition {
    pub controller: SimpleGestureController,
    pub thresholds: Vec<(GestureID, AtomicUsize)>,
}

impl SimpleGestureRecognition {
    pub fn new(controller: SimpleGestureController) -> Self {
        SimpleGestureRecognition {
            controller,
            thresholds: Vec::new(),
        }
    }
}

impl GestureRecognition for SimpleGestureRecognition {
    fn set_threshold(&mut self, sensor_id: GestureID, threshold: u8) -> Result<(), GestureError> {
        self.thresholds.push((sensor_id, AtomicUsize::new(threshold as usize)));
        Ok(())
    }
    
    fn get_confidence(&self, sensor_id: GestureID) -> Result<u8, GestureError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(GestureError::NotFound)
        }
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
