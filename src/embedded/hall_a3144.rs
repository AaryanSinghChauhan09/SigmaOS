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

/// OOP-based A3144 Hall Sensor for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 4146
/// Implements A3144 Hall effect sensor

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type HallA3144ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HallA3144Error { Success = 0, NotFound = 1 }

pub trait HallA3144Sensor {
    fn id(&self) -> HallA3144ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleHallA3144Sensor {
    pub id: HallA3144ID,
    pub initialized: AtomicUsize,
}

impl SimpleHallA3144Sensor {
    pub fn new(id: HallA3144ID) -> Self {
        SimpleHallA3144Sensor {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl HallA3144Sensor for SimpleHallA3144Sensor {
    fn id(&self) -> HallA3144ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait HallA3144Controller {
    fn init(&mut self, sensor_id: HallA3144ID) -> Result<(), HallA3144Error>;
    fn read(&self, sensor_id: HallA3144ID) -> Result<bool, HallA3144Error>;
    def set_threshold(&mut self, sensor_id: HallA3144ID, threshold: u16) -> Result<(), HallA3144Error>;
}

#[repr(C)]
pub struct SimpleHallA3144Controller {
    pub sensors: Vec<Option<Box<dyn HallA3144Sensor>>>,
    pub next_id: AtomicUsize,
}

impl SimpleHallA3144Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimpleHallA3144Controller {
            sensors: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl HallA3144Controller for SimpleHallA3144Controller {
    fn init(&mut self, sensor_id: HallA3144ID) -> Result<(), HallA3144Error> {
        for sensor_option in &mut self.sensors {
            if let Some(ref mut sensor) = *sensor_option {
                if sensor.id() == sensor_id {
                    sensor.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(HallA3144Error::NotFound)
    }
    
    fn read(&self, sensor_id: HallA3144ID) -> Result<bool, HallA3144Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(false)
        } else {
            Err(HallA3144Error::NotFound)
        }
    }
    
    fn set_threshold(&mut self, sensor_id: HallA3144ID, _threshold: u16) -> Result<(), HallA3144Error> {
        if self.get_sensor(sensor_id).is_some() {
            Ok(())
        } else {
            Err(HallA3144Error::NotFound)
        }
    }
    
    fn get_sensor(&self, id: HallA3144ID) -> Option<&dyn HallA3144Sensor> {
        for sensor_option in &self.sensors {
            if let Some(ref sensor) = *sensor_option {
                if sensor.id() == id { return Some(sensor.as_ref()); }
            }
        }
        None
    }
}

pub trait HallA3144Interrupt {
    def enable_interrupt(&mut self, sensor_id: HallA3144ID, enable: bool) -> Result<(), HallA3144Error>;
}

#[repr(C)]
pub struct SimpleHallA3144Interrupt {
    pub controller: SimpleHallA3144Controller,
    pub interrupts: Vec<(HallA3144ID, AtomicUsize)>,
}

impl SimpleHallA3144Interrupt {
    pub fn new(controller: SimpleHallA3144Controller) -> Self {
        SimpleHallA3144Interrupt {
            controller,
            interrupts: Vec::new(),
        }
    }
}

impl HallA3144Interrupt for SimpleHallA3144Interrupt {
    fn enable_interrupt(&mut self, sensor_id: HallA3144ID, enable: bool) -> Result<(), HallA3144Error> {
        self.interrupts.push((sensor_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
