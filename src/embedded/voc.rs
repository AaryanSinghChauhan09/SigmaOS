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

/// OOP-based VOC Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1906
/// Implements VOC (Volatile Organic Compounds) sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type VOCID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum VOCError { Success = 0, NotFound = 1 }

pub trait VOCSensor {
    fn id(&self) -> VOCID;
    fn voc_ppb(&self) -> u16;
}

#[repr(C)]
pub struct SimpleVOCSensor {
    pub id: VOCID,
    pub voc_ppb: AtomicUsize,
}

impl SimpleVOCSensor {
    pub fn new(id: VOCID) -> Self {
        SimpleVOCSensor {
            id,
            voc_ppb: AtomicUsize::new(0),
        }
    }
}

impl VOCSensor for SimpleVOCSensor {
    fn id(&self) -> VOCID { self.id }
    fn voc_ppb(&self) -> u16 { self.voc_ppb.load(Ordering::SeqCst) as u16 }
}

pub trait VOCController {
    fn read(&self, sensor_id: VOCID) -> Result<u16, VOCError>;
    def set_baseline(&mut self, sensor_id: VOCID, baseline: u16) -> Result<(), VOCError>;
}

#[repr(C)]
pub struct SimpleVOCController {
    pub sensors: Vec<Option<Box<dyn VOCSensor>>>,
    pub baselines: Vec<(VOCID, AtomicUsize)>,
    pub next_id: AtomicUsize,
}

impl SimpleVOCController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleVOCController {
            sensors: Vec::new(),
            baselines: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl VOCController for SimpleVOCController {
    fn read(&self, sensor_id: VOCID) -> Result<u16, VOCError> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    return Ok(sensor.voc_ppb());
                }
            }
        }
        Err(VOCError::NotFound)
    }
    
    fn set_baseline(&mut self, sensor_id: VOCID, baseline: u16) -> Result<(), VOCError> {
        self.baselines.push((sensor_id, AtomicUsize::new(baseline as usize)));
        Ok(())
    }
}

pub trait IAQIndex {
    def get_iaq(&self, sensor_id: VOCID) -> Result<u8, VOCError>;
    def get_accuracy(&self, sensor_id: VOCID) -> Result<u8, VOCError>;
}

#[repr(C)]
pub struct SimpleIAQIndex {
    pub controller: SimpleVOCController,
}

impl SimpleIAQIndex {
    pub fn new(controller: SimpleVOCController) -> Self {
        SimpleIAQIndex { controller }
    }
}

impl IAQIndex for SimpleIAQIndex {
    fn get_iaq(&self, sensor_id: VOCID) -> Result<u8, VOCError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(VOCError::NotFound)
        }
    }
    
    fn get_accuracy(&self, sensor_id: VOCID) -> Result<u8, VOCError> {
        if self.controller.read(sensor_id).is_ok() {
            Ok(0)
        } else {
            Err(VOCError::NotFound)
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
