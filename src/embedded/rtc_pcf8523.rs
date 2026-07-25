#![no_std]
#![no_main]

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
