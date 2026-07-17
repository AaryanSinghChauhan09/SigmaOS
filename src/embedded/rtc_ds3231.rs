#![no_std]
#![no_main]

/// OOP-based DS3231 RTC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2916
/// Implements DS3231 real-time clock

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DS3231ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DS3231Error { Success = 0, NotFound = 1 }

pub trait DS3231RTC {
    fn id(&self) -> DS3231ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDS3231RTC {
    pub id: DS3231ID,
    pub initialized: AtomicUsize,
}

impl SimpleDS3231RTC {
    pub fn new(id: DS3231ID) -> Self {
        SimpleDS3231RTC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DS3231RTC for SimpleDS3231RTC {
    fn id(&self) -> DS3231ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DS3231Controller {
    fn init(&mut self, ds_id: DS3231ID) -> Result<(), DS3231Error>;
    fn read_time(&self, ds_id: DS3231ID) -> Result<(u8, u8, u8), DS3231Error>;
    def set_time(&self, ds_id: DS3231ID, hour: u8, minute: u8, second: u8) -> Result<(), DS3231Error>;
}

#[repr(C)]
pub struct SimpleDS3231Controller {
    pub rtcs: Vec<Option<Box<dyn DS3231RTC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDS3231Controller {
    pub fn new() -> Self {
        SimpleDS3231Controller {
            rtcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DS3231Controller for SimpleDS3231Controller {
    fn init(&mut self, ds_id: DS3231ID) -> Result<(), DS3231Error> {
        for rtc_option in &mut self.rtcs {
            if let Some(ref mut rtc) = *rtc_option {
                if rtc.id() == ds_id {
                    rtc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DS3231Error::NotFound)
    }
    
    fn read_time(&self, ds_id: DS3231ID) -> Result<(u8, u8, u8), DS3231Error> {
        if self.get_rtc(ds_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(DS3231Error::NotFound)
        }
    }
    
    fn set_time(&self, ds_id: DS3231ID, _hour: u8, _minute: u8, _second: u8) -> Result<(), DS3231Error> {
        if self.get_rtc(ds_id).is_some() {
            Ok(())
        } else {
            Err(DS3231Error::NotFound)
        }
    }
    
    fn get_rtc(&self, id: DS3231ID) -> Option<&dyn DS3231RTC> {
        for rtc_option in &self.rtcs {
            if let Some(ref rtc) = *rtc_option {
                if rtc.id() == id { return Some(rtc.as_ref()); }
            }
        }
        None
    }
}

pub trait DS3231Temperature {
    def read_temp(&self, ds_id: DS3231ID) -> Result<f32, DS3231Error>;
}

#[repr(C)]
pub struct SimpleDS3231Temperature {
    pub controller: SimpleDS3231Controller,
}

impl SimpleDS3231Temperature {
    pub fn new(controller: SimpleDS3231Controller) -> Self {
        SimpleDS3231Temperature { controller }
    }
}

impl DS3231Temperature for SimpleDS3231Temperature {
    fn read_temp(&self, ds_id: DS3231ID) -> Result<f32, DS3231Error> {
        if self.controller.get_rtc(ds_id).is_some() {
            Ok(0.0)
        } else {
            Err(DS3231Error::NotFound)
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
