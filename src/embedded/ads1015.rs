#![no_std]
#![no_main]

/// OOP-based ADS1015 for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2886
/// Implements ADS1015 ADC

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ADS1015ID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ADS1015Error { Success = 0, NotFound = 1 }

pub trait ADS1015ADC {
    fn id(&self) -> ADS1015ID;
    fn is_initialized(&self) -> bool;
}

#[repr(C)]
pub struct SimpleADS1015ADC {
    pub id: ADS1015ID,
    pub initialized: AtomicUsize,
}

impl SimpleADS1015ADC {
    pub fn new(id: ADS1015ID) -> Self {
        SimpleADS1015ADC {
            id,
            initialized: AtomicUsize::new(0),
        }
    }
}

impl ADS1015ADC for SimpleADS1015ADC {
    fn id(&self) -> ADS1015ID { self.id }
    fn is_initialized(&self) -> bool { self.initialized.load(Ordering::SeqCst) == 1 }
}

pub trait ADS1015Controller {
    fn init(&mut self, ads_id: ADS1015ID) -> Result<(), ADS1015Error>;
    fn read(&self, ads_id: ADS1015ID, channel: u8) -> Result<i16, ADS1015Error>;
    def set_gain(&mut self, ads_id: ADS1015ID, gain: u8) -> Result<(), ADS1015Error>;
}

#[repr(C)]
pub struct SimpleADS1015Controller {
    pub adcs: Vec<Option<Box<dyn ADS1015ADC>>>,
    pub next_id: AtomicUsize,
}

impl SimpleADS1015Controller {
    pub fn new() -> Self {
        SimpleADS1015Controller {
            adcs: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ADS1015Controller for SimpleADS1015Controller {
    fn init(&mut self, ads_id: ADS1015ID) -> Result<(), ADS1015Error> {
        for adc_option in &mut self.adcs {
            if let Some(ref mut adc) = *adc_option {
                if adc.id() == ads_id {
                    adc.initialized.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(ADS1015Error::NotFound)
    }
    
    fn read(&self, ads_id: ADS1015ID, _channel: u8) -> Result<i16, ADS1015Error> {
        if self.get_adc(ads_id).is_some() {
            Ok(0)
        } else {
            Err(ADS1015Error::NotFound)
        }
    }
    
    fn set_gain(&mut self, _ads_id: ADS1015ID, _gain: u8) -> Result<(), ADS1015Error> {
        Ok(())
    }
    
    fn get_adc(&self, id: ADS1015ID) -> Option<&dyn ADS1015ADC> {
        for adc_option in &self.adcs {
            if let Some(ref adc) = *adc_option {
                if adc.id() == id { return Some(adc.as_ref()); }
            }
        }
        None
    }
}

pub trait ADS1015Config {
    def set_data_rate(&mut self, ads_id: ADS1015ID, rate: u8) -> Result<(), ADS1015Error>;
    def get_data_rate(&self, ads_id: ADS1015ID) -> Result<u8, ADS1015Error>;
}

#[repr(C)]
pub struct SimpleADS1015Config {
    pub controller: SimpleADS1015Controller,
    pub data_rates: Vec<(ADS1015ID, AtomicUsize)>,
}

impl SimpleADS1015Config {
    pub fn new(controller: SimpleADS1015Controller) -> Self {
        SimpleADS1015Config {
            controller,
            data_rates: Vec::new(),
        }
    }
}

impl ADS1015Config for SimpleADS1015Config {
    fn set_data_rate(&mut self, ads_id: ADS1015ID, rate: u8) -> Result<(), ADS1015Error> {
        self.data_rates.push((ads_id, AtomicUsize::new(rate as usize)));
        Ok(())
    }
    
    fn get_data_rate(&self, ads_id: ADS1015ID) -> Result<u8, ADS1015Error> {
        for &(id, ref rate) in &self.data_rates {
            if id == ads_id {
                return Ok(rate.load(Ordering::SeqCst) as u8);
            }
        }
        Err(ADS1015Error::NotFound)
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
