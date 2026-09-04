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
use std::boxed::Box;
use std::string::{String, ToString};
use std::vec::Vec;
use std::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

/// OOP-based PCF8523 RTC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2926
/// Implements PCF8523 real-time clock

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCF8523ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCF8523Error { Success = 0, NotFound = 1 }

pub trait PCF8523RTC {
    fn id(&self) -> PCF8523ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCF8523RTC {
    pub id: PCF8523ID,
    pub initialized: AtomicUsize,
}

impl SimplePCF8523RTC {
    pub fn new(id: PCF8523ID) -> Self {
        SimplePCF8523RTC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCF8523RTC for SimplePCF8523RTC {
    fn id(&self) -> PCF8523ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCF8523Controller {
    fn init(&mut self, pcf_id: PCF8523ID) -> Result<(), PCF8523Error>;
    fn read_time(&self, pcf_id: PCF8523ID) -> Result<(u8, u8, u8), PCF8523Error>;
    def set_time(&self, pcf_id: PCF8523ID, hour: u8, minute: u8, second: u8) -> Result<(), PCF8523Error>;
}

#[repr(C)]
pub struct SimplePCF8523Controller {
    pub rtcs: Vec<Option<Box<dyn PCF8523RTC>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCF8523Controller {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SimplePCF8523Controller {
            rtcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCF8523Controller for SimplePCF8523Controller {
    fn init(&mut self, pcf_id: PCF8523ID) -> Result<(), PCF8523Error> {
        for rtc_option in &mut self.rtcs {
            if let Some(ref mut rtc) = *rtc_option {
                if rtc.id() == pcf_id {
                    rtc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCF8523Error::NotFound)
    }
    
    fn read_time(&self, pcf_id: PCF8523ID) -> Result<(u8, u8, u8), PCF8523Error> {
        if self.get_rtc(pcf_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(PCF8523Error::NotFound)
        }
    }
    
    fn set_time(&self, pcf_id: PCF8523ID, _hour: u8, _minute: u8, _second: u8) -> Result<(), PCF8523Error> {
        if self.get_rtc(pcf_id).is_some() {
            Ok(())
        } else {
            Err(PCF8523Error::NotFound)
        }
    }
    
    fn get_rtc(&self, id: PCF8523ID) -> Option<&dyn PCF8523RTC> {
        for rtc_option in &self.rtcs {
            if let Some(ref rtc) = *rtc_option {
                if rtc.id() == id { return Some(rtc.as_ref()); }
            }
        }
        None
    }
}

pub trait PCF8523Alarm {
    def set_alarm(&mut self, pcf_id: PCF8523ID, hour: u8, minute: u8) -> Result<(), PCF8523Error>;
    def enable_alarm(&mut self, pcf_id: PCF8523ID, enable: bool) -> Result<(), PCF8523Error>;
}

#[repr(C)]
pub struct SimplePCF8523Alarm {
    pub controller: SimplePCF8523Controller,
    pub alarm_enables: Vec<(PCF8523ID, AtomicUsize)>,
}

impl SimplePCF8523Alarm {
    pub fn new(controller: SimplePCF8523Controller) -> Self {
        SimplePCF8523Alarm {
            controller,
            alarm_enables: Vec::new(),
        }
    }
}

impl PCF8523Alarm for SimplePCF8523Alarm {
    fn set_alarm(&mut self, _pcf_id: PCF8523ID, _hour: u8, _minute: u8) -> Result<(), PCF8523Error> {
        Ok(())
    }
    
    fn enable_alarm(&mut self, pcf_id: PCF8523ID, enable: bool) -> Result<(), PCF8523Error> {
        self.alarm_enables.push((pcf_id, AtomicUsize::new(if enable { 1 } else { 0 })));
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
