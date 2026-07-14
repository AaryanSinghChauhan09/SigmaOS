#![no_std]
#![no_main]

/// OOP-based DS1302 RTC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3686
/// Implements DS1302 real-time clock

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DS1302ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DS1302Error { Success = 0, NotFound = 1 }

pub trait DS1302RTC {
    fn id(&self) -> DS1302ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleDS1302RTC {
    pub id: DS1302ID,
    pub initialized: AtomicUsize,
}

impl SimpleDS1302RTC {
    pub fn new(id: DS1302ID) -> Self {
        SimpleDS1302RTC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl DS1302RTC for SimpleDS1302RTC {
    fn id(&self) -> DS1302ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait DS1302Controller {
    fn init(&mut self, rtc_id: DS1302ID) -> Result<(), DS1302Error>;
    fn read_time(&self, rtc_id: DS1302ID) -> Result<(u8, u8, u8), DS1302Error>;
    def set_time(&self, rtc_id: DS1302ID, h: u8, m: u8, s: u8) -> Result<(), DS1302Error>;
}

#[repr(C)]
pub struct SimpleDS1302Controller {
    pub rtcs: Vec<Option<Box<dyn DS1302RTC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleDS1302Controller {
    pub fn new() -> Self {
        SimpleDS1302Controller {
            rtcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DS1302Controller for SimpleDS1302Controller {
    fn init(&mut self, rtc_id: DS1302ID) -> Result<(), DS1302Error> {
        for rtc_option in &mut self.rtcs {
            if let Some(ref mut rtc) = *rtc_option {
                if rtc.id() == rtc_id {
                    rtc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(DS1302Error::NotFound)
    }
    
    fn read_time(&self, rtc_id: DS1302ID) -> Result<(u8, u8, u8), DS1302Error> {
        if self.get_rtc(rtc_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(DS1302Error::NotFound)
        }
    }
    
    fn set_time(&self, rtc_id: DS1302ID, _h: u8, _m: u8, _s: u8) -> Result<(), DS1302Error> {
        if self.get_rtc(rtc_id).is_some() {
            Ok(())
        } else {
            Err(DS1302Error::NotFound)
        }
    }
    
    fn get_rtc(&self, id: DS1302ID) -> Option<&dyn DS1302RTC> {
        for rtc_option in &self.rtcs {
            if let Some(ref rtc) = *rtc_option {
                if rtc.id() == id { return Some(rtc.as_ref()); }
            }
        }
        None
    }
}

pub trait DS1302RAM {
    def read_ram(&self, rtc_id: DS1302ID, addr: u8) -> Result<u8, DS1302Error>;
    def write_ram(&self, rtc_id: DS1302ID, addr: u8, data: u8) -> Result<(), DS1302Error>;
}

#[repr(C)]
pub struct SimpleDS1302RAM {
    pub controller: SimpleDS1302Controller,
}

impl SimpleDS1302RAM {
    pub fn new(controller: SimpleDS1302Controller) -> Self {
        SimpleDS1302RAM { controller }
    }
}

impl DS1302RAM for SimpleDS1302RAM {
    fn read_ram(&self, rtc_id: DS1302ID, _addr: u8) -> Result<u8, DS1302Error> {
        if self.controller.get_rtc(rtc_id).is_some() {
            Ok(0)
        } else {
            Err(DS1302Error::NotFound)
        }
    }
    
    fn write_ram(&self, rtc_id: DS1302ID, _addr: u8, _data: u8) -> Result<(), DS1302Error> {
        if self.controller.get_rtc(rtc_id).is_some() {
            Ok(())
        } else {
            Err(DS1302Error::NotFound)
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
