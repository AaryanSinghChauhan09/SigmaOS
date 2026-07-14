#![no_std]
#![no_main]

/// OOP-based ADC for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 1096
/// Implements analog-to-digital conversion

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type ChannelID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ADCResolution { Bits8 = 0, Bits10 = 1, Bits12 = 2, Bits16 = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum ADCError { Success = 0, NotFound = 1 }

pub trait ADCChannel {
    fn id(&self) -> ChannelID;
    fn resolution(&self) -> ADCResolution;
    def read(&self) -> u16;
}

#[repr(C)]
pub struct SimpleADCChannel {
    pub id: ChannelID,
    pub resolution: AtomicUsize,
    pub value: AtomicUsize,
}

impl SimpleADCChannel {
    pub fn new(id: ChannelID, resolution: ADCResolution) -> Self {
        SimpleADCChannel {
            id,
            resolution: AtomicUsize::new(resolution as usize),
            value: AtomicUsize::new(0),
        }
    }
}

impl ADCChannel for SimpleADCChannel {
    fn id(&self) -> ChannelID { self.id }
    fn resolution(&self) -> ADCResolution { unsafe { core::mem::transmute(self.resolution.load(Ordering::SeqCst)) } }
    
    fn read(&self) -> u16 {
        self.value.load(Ordering::SeqCst) as u16
    }
}

pub trait ADCController {
    fn configure_channel(&mut self, channel_id: ChannelID, resolution: ADCResolution) -> Result<(), ADCError>;
    fn start_conversion(&mut self, channel_id: ChannelID) -> Result<(), ADCError>;
    fn get_value(&self, channel_id: ChannelID) -> Result<u16, ADCError>;
}

#[repr(C)]
pub struct SimpleADCController {
    pub channels: Vec<Option<Box<dyn ADCChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleADCController {
    pub fn new() -> Self {
        SimpleADCController {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl ADCController for SimpleADCController {
    fn configure_channel(&mut self, channel_id: ChannelID, resolution: ADCResolution) -> Result<(), ADCError> {
        for channel_option in &mut self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == channel_id {
                    return Ok(());
                }
            }
        }
        let channel = SimpleADCChannel::new(channel_id, resolution);
        self.channels.push(Some(Box::new(channel)));
        Ok(())
    }
    
    fn start_conversion(&mut self, channel_id: ChannelID) -> Result<(), ADCError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == channel_id {
                    if let SimpleADCChannel { ref mut value, .. } = **channel {
                        value.store(2048, Ordering::SeqCst);
                    }
                    return Ok(());
                }
            }
        }
        Err(ADCError::NotFound)
    }
    
    fn get_value(&self, channel_id: ChannelID) -> Result<u16, ADCError> {
        for channel_option in &self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == channel_id {
                    return Ok(channel.read());
                }
            }
        }
        Err(ADCError::NotFound)
    }
}

pub trait VoltageReference {
    fn set_reference(&mut self, voltage_mv: u32);
    fn get_reference(&self) -> u32;
}

#[repr(C)]
pub struct SimpleVoltageReference {
    pub reference_mv: AtomicUsize,
}

impl SimpleVoltageReference {
    pub fn new() -> Self {
        SimpleVoltageReference {
            reference_mv: AtomicUsize::new(3300),
        }
    }
}

impl VoltageReference for SimpleVoltageReference {
    fn set_reference(&mut self, voltage_mv: u32) {
        self.reference_mv.store(voltage_mv as usize, Ordering::SeqCst);
    }
    
    fn get_reference(&self) -> u32 {
        self.reference_mv.load(Ordering::SeqCst) as u32
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
