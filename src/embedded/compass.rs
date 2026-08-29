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
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based Digital Compass for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1716
/// Implements digital compass

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type CompassID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CompassError { Success = 0, NotFound = 1 }

pub trait Compass {
    fn id(&self) -> CompassID;
    fn heading(&self) -> f32;
}

#[repr(C)]
pub struct SimpleCompass {
    pub id: CompassID,
    pub heading: AtomicUsize,
}

impl SimpleCompass {
    pub fn new(id: CompassID) -> Self {
        SimpleCompass {
            id,
            heading: AtomicUsize::new(0),
        }
    }
}

impl Compass for SimpleCompass {
    fn id(&self) -> CompassID { self.id }
    fn heading(&self) -> f32 { self.heading.load(Ordering::SeqCst) as f32 }
}

pub trait CompassController {
    fn read_heading(&self, compass_id: CompassID) -> Result<f32, CompassError>;
    def calibrate(&mut self, compass_id: CompassID) -> Result<(), CompassError>;
}

#[repr(C)]
pub struct SimpleCompassController {
    pub compasses: Vec<Option<Box<dyn Compass>>>,
    pub next_id: AtomicUsize,
}

impl SimpleCompassController {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleCompassController {
            compasses: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl CompassController for SimpleCompassController {
    fn read_heading(&self, compass_id: CompassID) -> Result<f32, CompassError> {
        for compass_option in &self.compasses {
            if let Some(ref compass) = *compass_option {
                if compass.id() == compass_id {
                    return Ok(compass.heading());
                }
            }
        }
        Err(CompassError::NotFound)
    }
    
    fn calibrate(&mut self, _compass_id: CompassID) -> Result<(), CompassError> {
        Ok(())
    }
}

pub trait TiltCompensation {
    def set_tilt(&mut self, compass_id: CompassID, pitch: f32, roll: f32) -> Result<(), CompassError>;
    def get_compensated_heading(&self, compass_id: CompassID) -> Result<f32, CompassError>;
}

#[repr(C)]
pub struct SimpleTiltCompensation {
    pub controller: SimpleCompassController,
    pub tilts: Vec<(CompassID, (AtomicUsize, AtomicUsize))>,
}

impl SimpleTiltCompensation {
    pub fn new(controller: SimpleCompassController) -> Self {
        SimpleTiltCompensation {
            controller,
            tilts: Vec::new(),
        }
    }
}

impl TiltCompensation for SimpleTiltCompensation {
    fn set_tilt(&mut self, compass_id: CompassID, pitch: f32, roll: f32) -> Result<(), CompassError> {
        self.tilts.push((compass_id, (AtomicUsize::new(pitch as usize), AtomicUsize::new(roll as usize))));
        Ok(())
    }
    
    fn get_compensated_heading(&self, compass_id: CompassID) -> Result<f32, CompassError> {
        if self.controller.read_heading(compass_id).is_ok() {
            Ok(0.0)
        } else {
            Err(CompassError::NotFound)
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
