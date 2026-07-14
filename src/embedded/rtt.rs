#![no_std]
#![no_main]

/// OOP-based RTT for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2166
/// Implements RTT (Real-Time Transfer)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type RTTID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RTTError { Success = 0, NotFound = 1 }

pub trait RTTChannel {
    fn id(&self) -> RTTID;
    fn is_up(&self) -> bool;
}

#[repr(C)]
pub struct SimpleRTTChannel {
    pub id: RTTID,
    pub up: AtomicUsize,
}

impl SimpleRTTChannel {
    pub fn new(id: RTTID) -> Self {
        SimpleRTTChannel {
            id,
            up: AtomicUsize::new(0),
        }
    }
}

impl RTTChannel for SimpleRTTChannel {
    fn id(&self) -> RTTID { self.id }
    fn is_up(&self) -> bool { self.up.load(Ordering::SeqCst) == 1 }
}

pub trait RTTController {
    fn init(&mut self, rtt_id: RTTID) -> Result<(), RTTError>;
    fn write(&self, rtt_id: RTTID, data: &[u8]) -> Result<usize, RTTError>;
    def read(&self, rtt_id: RTTID, buffer: &mut [u8]) -> Result<usize, RTTError>;
}

#[repr(C)]
pub struct SimpleRTTController {
    pub channels: Vec<Option<Box<dyn RTTChannel>>>,
    pub next_id: AtomicUsize,
}

impl SimpleRTTController {
    pub fn new() -> Self {
        SimpleRTTController {
            channels: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl RTTController for SimpleRTTController {
    fn init(&mut self, rtt_id: RTTID) -> Result<(), RTTError> {
        for channel_option in &mut self.channels {
            if let Some(ref mut channel) = *channel_option {
                if channel.id() == rtt_id {
                    channel.up.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(RTTError::NotFound)
    }
    
    fn write(&self, rtt_id: RTTID, _data: &[u8]) -> Result<usize, RTTError> {
        if self.get_channel(rtt_id).is_some() {
            Ok(0)
        } else {
            Err(RTTError::NotFound)
        }
    }
    
    fn read(&self, rtt_id: RTTID, buffer: &mut [u8]) -> Result<usize, RTTError> {
        if self.get_channel(rtt_id).is_some() {
            for byte in buffer.iter_mut() { *byte = 0; }
            Ok(buffer.len())
        } else {
            Err(RTTError::NotFound)
        }
    }
    
    fn get_channel(&self, id: RTTID) -> Option<&dyn RTTChannel> {
        for channel_option in &self.channels {
            if let Some(ref channel) = *channel_option {
                if channel.id() == id { return Some(channel.as_ref()); }
            }
        }
        None
    }
}

pub trait RTTControl {
    def set_buffer_size(&mut self, rtt_id: RTTID, size: usize) -> Result<(), RTTError>;
    def get_available(&self, rtt_id: RTTID) -> Result<usize, RTTError>;
}

#[repr(C)]
pub struct SimpleRTTControl {
    pub controller: SimpleRTTController,
    pub buffer_sizes: Vec<(RTTID, AtomicUsize)>,
}

impl SimpleRTTControl {
    pub fn new(controller: SimpleRTTController) -> Self {
        SimpleRTTControl {
            controller,
            buffer_sizes: Vec::new(),
        }
    }
}

impl RTTControl for SimpleRTTControl {
    fn set_buffer_size(&mut self, rtt_id: RTTID, size: usize) -> Result<(), RTTError> {
        self.buffer_sizes.push((rtt_id, AtomicUsize::new(size)));
        Ok(())
    }
    
    fn get_available(&self, rtt_id: RTTID) -> Result<usize, RTTError> {
        for &(id, ref size) in &self.buffer_sizes {
            if id == rtt_id {
                return Ok(size.load(Ordering::SeqCst));
            }
        }
        Err(RTTError::NotFound)
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
