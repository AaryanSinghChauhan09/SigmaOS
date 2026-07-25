#![no_std]
#![no_main]

/// OOP-based ADS1115 ADC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 3646
/// Implements ADS1115 16-bit ADC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ADS1115ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ADS1115Error { Success = 0, NotFound = 1 }

pub trait ADS1115Device {
    fn id(&self) -> ADS1115ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleADS1115Device {
    pub id: ADS1115ID,
    pub initialized: AtomicUsize,
}

impl SimpleADS1115Device {
    pub fn new(id: ADS1115ID) -> Self {
        SimpleADS1115Device {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ADS1115Device for SimpleADS1115Device {
    fn id(&self) -> ADS1115ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ADS1115Controller {
    fn init(&mut self, adc_id: ADS1115ID) -> Result<(), ADS1115Error>;
    fn read(&self, adc_id: ADS1115ID, channel: u8) -> Result<i16, ADS1115Error>;
    def set_gain(&mut self, adc_id: ADS1115ID, gain: u8) -> Result<(), ADS1115Error>;
}

#[repr(C)]
pub struct SimpleADS1115Controller {
    pub adcs: Vec<Option<Box<dyn ADS1115Device>>>,
    pub next_id: AtomicUsize,
}

impl SimpleADS1115Controller {
    pub fn new() -> Self {
        SimpleADS1115Controller {
            adcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ADS1115Controller for SimpleADS1115Controller {
    fn init(&mut self, adc_id: ADS1115ID) -> Result<(), ADS1115Error> {
        for adc_option in &mut self.adcs {
            if let Some(ref mut adc) = *adc_option {
                if adc.id() == adc_id {
                    adc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ADS1115Error::NotFound)
    }
    
    fn read(&self, adc_id: ADS1115ID, _channel: u8) -> Result<i16, ADS1115Error> {
        if self.get_adc(adc_id).is_some() {
            Ok(0)
        } else {
            Err(ADS1115Error::NotFound)
        }
    }
    
    fn set_gain(&mut self, adc_id: ADS1115ID, _gain: u8) -> Result<(), ADS1115Error> {
        if self.get_adc(adc_id).is_some() {
            Ok(())
        } else {
            Err(ADS1115Error::NotFound)
        }
    }
    
    fn get_adc(&self, id: ADS1115ID) -> Option<&dyn ADS1115Device> {
        for adc_option in &self.adcs {
            if let Some(ref adc) = *adc_option {
                if adc.id() == id { return Some(adc.as_ref()); }
            }
        }
        None
    }
}

pub trait ADS1115Rate {
    def set_rate(&mut self, adc_id: ADS1115ID, rate: u8) -> Result<(), ADS1115Error>;
}

#[repr(C)]
pub struct SimpleADS1115Rate {
    pub controller: SimpleADS1115Controller,
    pub rates: Vec<(ADS1115ID, AtomicUsize)>,
}

impl SimpleADS1115Rate {
    pub fn new(controller: SimpleADS1115Controller) -> Self {
        SimpleADS1115Rate {
            controller,
            rates: Vec::new(),
        }
    }
}

impl ADS1115Rate for SimpleADS1115Rate {
    fn set_rate(&mut self, adc_id: ADS1115ID, rate: u8) -> Result<(), ADS1115Error> {
        self.rates.push((adc_id, AtomicUsize::new(rate as usize)));
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
