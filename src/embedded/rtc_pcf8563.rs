#![no_std]
#![no_main]

/// OOP-based PCF8563 RTC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3696
/// Implements PCF8563 real-time clock

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCF8563ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCF8563Error { Success = 0, NotFound = 1 }

pub trait PCF8563RTC {
    fn id(&self) -> PCF8563ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCF8563RTC {
    pub id: PCF8563ID,
    pub initialized: AtomicUsize,
}

impl SimplePCF8563RTC {
    pub fn new(id: PCF8563ID) -> Self {
        SimplePCF8563RTC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl PCF8563RTC for SimplePCF8563RTC {
    fn id(&self) -> PCF8563ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait PCF8563Controller {
    fn init(&mut self, rtc_id: PCF8563ID) -> Result<(), PCF8563Error>;
    fn read_time(&self, rtc_id: PCF8563ID) -> Result<(u8, u8, u8), PCF8563Error>;
    def set_time(&self, rtc_id: PCF8563ID, h: u8, m: u8, s: u8) -> Result<(), PCF8563Error>;
}

#[repr(C)]
pub struct SimplePCF8563Controller {
    pub rtcs: Vec<Option<Box<dyn PCF8563RTC>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCF8563Controller {
    pub fn new() -> Self {
        SimplePCF8563Controller {
            rtcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCF8563Controller for SimplePCF8563Controller {
    fn init(&mut self, rtc_id: PCF8563ID) -> Result<(), PCF8563Error> {
        for rtc_option in &mut self.rtcs {
            if let Some(ref mut rtc) = *rtc_option {
                if rtc.id() == rtc_id {
                    rtc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCF8563Error::NotFound)
    }
    
    fn read_time(&self, rtc_id: PCF8563ID) -> Result<(u8, u8, u8), PCF8563Error> {
        if self.get_rtc(rtc_id).is_some() {
            Ok((0, 0, 0))
        } else {
            Err(PCF8563Error::NotFound)
        }
    }
    
    fn set_time(&self, rtc_id: PCF8563ID, _h: u8, _m: u8, _s: u8) -> Result<(), PCF8563Error> {
        if self.get_rtc(rtc_id).is_some() {
            Ok(())
        } else {
            Err(PCF8563Error::NotFound)
        }
    }
    
    fn get_rtc(&self, id: PCF8563ID) -> Option<&dyn PCF8563RTC> {
        for rtc_option in &self.rtcs {
            if let Some(ref rtc) = *rtc_option {
                if rtc.id() == id { return Some(rtc.as_ref()); }
            }
        }
        None
    }
}

pub trait PCF8563Alarm {
    def set_alarm(&mut self, rtc_id: PCF8563ID, h: u8, m: u8) -> Result<(), PCF8563Error>;
}

#[repr(C)]
pub struct SimplePCF8563Alarm {
    pub controller: SimplePCF8563Controller,
    pub alarms: Vec<(PCF8563ID, AtomicUsize, AtomicUsize)>,
}

impl SimplePCF8563Alarm {
    pub fn new(controller: SimplePCF8563Controller) -> Self {
        SimplePCF8563Alarm {
            controller,
            alarms: Vec::new(),
        }
    }
}

impl PCF8563Alarm for SimplePCF8563Alarm {
    fn set_alarm(&mut self, rtc_id: PCF8563ID, h: u8, m: u8) -> Result<(), PCF8563Error> {
        self.alarms.push((rtc_id, AtomicUsize::new(h as usize), AtomicUsize::new(m as usize)));
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
