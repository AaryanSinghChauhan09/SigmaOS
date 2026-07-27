#![no_std]
#![no_main]

/// OOP-based DS1307 RTC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2936
/// Implements DS1307 real-time clock

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DS1307ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DS1307Error { Success = 0, NotFound = 1 }

pub trait DS1307RTC {
    fn id(&self) -> DS1307ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDS1307RTC {
    pub id: DS1307ID,
    pub initialized: AtomicUsize,
}

impl SimpleDS1307RTC {
    pub fn new(id: DS1307ID) -> Self {
        SimpleDS1307RTC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DS1307RTC for SimpleDS1307RTC {
    fn id(&self) -> DS1307ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DS1307Controller {
    fn init(&mut self, ds_id: DS1307ID) -> Result<(), DS1307Error>;
    fn read_time(&self, ds_id: DS1307ID) -> Result<(u8, u8, u8), DS1307Error>;
    def set_time(&self, ds_id: DS1307ID, hour: u8, minute: u8, second: u8) -> Result<(), DS1307Error>;
}

#[repr(C)]
pub struct SimpleDS1307Controller {
    pub rtcs: Vec<Option<Box<dyn DS1307RTC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDS1307Controller {
    pub fn new() -> Self {
        SimpleDS1307Controller {
            rtcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DS1307Controller for SimpleDS1307Controller {
    fn init(&mut self, ds_id: DS1307ID) -> Result<(), DS1307Error> {
        for rtc_option in &mut self.rtcs {
            if let Some(ref mut rtc) = *rtc_option {
                if rtc.id() == ds_id {
                    rtc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DS1307Error::NotFound)
    }
    
    fn read_time(&self, ds_id: DS1307ID) -> Result<(u8, u8, u8), DS1307Error> {
        if self.get_rtc(ds_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(DS1307Error::NotFound)
        }
    }
    
    fn set_time(&self, ds_id: DS1307ID, _hour: u8, _minute: u8, _second: u8) -> Result<(), DS1307Error> {
        if self.get_rtc(ds_id).is_some() {
            Ok(())
        } else {
            Err(DS1307Error::NotFound)
        }
    }
    
    fn get_rtc(&self, id: DS1307ID) -> Option<&dyn DS1307RTC> {
        for rtc_option in &self.rtcs {
            if let Some(ref rtc) = *rtc_option {
                if rtc.id() == id { return Some(rtc.as_ref()); }
            }
        }
        None
    }
}

pub trait DS1307RAM {
    def read_ram(&self, ds_id: DS1307ID, address: u8) -> Result<u8, DS1307Error>;
    def write_ram(&self, ds_id: DS1307ID, address: u8, value: u8) -> Result<(), DS1307Error>;
}

#[repr(C)]
pub struct SimpleDS1307RAM {
    pub controller: SimpleDS1307Controller,
}

impl SimpleDS1307RAM {
    pub fn new(controller: SimpleDS1307Controller) -> Self {
        SimpleDS1307RAM { controller }
    }
}

impl DS1307RAM for SimpleDS1307RAM {
    fn read_ram(&self, ds_id: DS1307ID, _address: u8) -> Result<u8, DS1307Error> {
        if self.controller.get_rtc(ds_id).is_some() {
            Ok(0)
        } else {
            Err(DS1307Error::NotFound)
        }
    }
    
    fn write_ram(&self, ds_id: DS1307ID, _address: u8, _value: u8) -> Result<(), DS1307Error> {
        if self.controller.get_rtc(ds_id).is_some() {
            Ok(())
        } else {
            Err(DS1307Error::NotFound)
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
