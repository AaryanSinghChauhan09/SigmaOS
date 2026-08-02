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

/// OOP-based HL7800 Voltage Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3376
/// Implements HL7800 voltage sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HL7800ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HL7800Error { Success = 0, NotFound = 1 }

pub trait HL7800Sensor {
    fn id(&self) -> HL7800ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHL7800Sensor {
    pub id: HL7800ID,
    pub initialized: AtomicUsize,
}

impl SimpleHL7800Sensor {
    pub fn new(id: HL7800ID) -> Self {
        SimpleHL7800Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HL7800Sensor for SimpleHL7800Sensor {
    fn id(&self) -> HL7800ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HL7800Controller {
    fn init(&mut self, sensor_id: HL7800ID) -> Result<(), HL7800Error>;
    fn read_voltage(&self, sensor_id: HL7800ID) -> Result<f32, HL7800Error>;
    def calibrate(&mut self, sensor_id: HL7800ID) -> Result<(), HL7800Error>;
}

#[repr(C)]
pub struct SimpleHL7800Controller {
    pub sensors: Vec<Option<Box<dyn HL7800Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHL7800Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleHL7800Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HL7800Controller for SimpleHL7800Controller {
    fn init(&mut self, sensor_id: HL7800ID) -> Result<(), HL7800Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HL7800Error::NotFound)
    }
    
    fn read_voltage(&self, sensor_id: HL7800ID) -> Result<f32, HL7800Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(0.0)
        } else {
            Err(HL7800Error::NotFound)
        }
    }
    
    fn calibrate(&mut self, sensor_id: HL7800ID) -> Result<(), HL7800Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(HL7800Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: HL7800ID) -> Option<&dyn HL7800Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait HL7800Range {
    def set_range(&mut self, sensor_id: HL7800ID, range: u8) -> Result<(), HL7800Error>;
}

#[repr(C)]
pub struct SimpleHL7800Range {
    pub controller: SimpleHL7800Controller,
    pub ranges: Vec<(HL7800ID, AtomicUsize)>,
}

impl SimpleHL7800Range {
    pub fn new(controller: SimpleHL7800Controller) -> Self {
        SimpleHL7800Range {
            controller,
            ranges: Vec::new(),
        }
    }
}

impl HL7800Range for SimpleHL7800Range {
    fn set_range(&mut self, sensor_id: HL7800ID, range: u8) -> Result<(), HL7800Error> {
        self.ranges.push((sensor_id, AtomicUsize::new(range as usize)));
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
