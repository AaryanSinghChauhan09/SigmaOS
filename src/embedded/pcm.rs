#![no_std]
#![no_main]

/// OOP-based PCM for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2326
/// Implements PCM audio interface

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type PCMID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum PCMError { Success = 0, NotFound = 1 }

pub trait PCMDevice {
    fn id(&self) -> PCMID;
    fn is_active(&self) -> bool;
}

#[repr(C)]
pub struct SimplePCMDevice {
    pub id: PCMID,
    pub active: AtomicUsize,
}

impl SimplePCMDevice {
    pub fn new(id: PCMID) -> Self {
        SimplePCMDevice {
            id,
            active: AtomicUsize::new(0),
        }
    }
}

impl PCMDevice for SimplePCMDevice {
    fn id(&self) -> PCMID { self.id }
    fn is_active(&self) -> bool { self.active.load(Ordering::SeqCst) == 1 }
}

pub trait PCMController {
    fn init(&mut self, pcm_id: PCMID) -> Result<(), PCMError>;
    fn send(&self, pcm_id: PCMID, data: &[i16]) -> Result<usize, PCMError>;
    def receive(&self, pcm_id: PCMID, buffer: &mut [i16]) -> Result<usize, PCMError>;
}

#[repr(C)]
pub struct SimplePCMController {
    pub devices: Vec<Option<Box<dyn PCMDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimplePCMController {
    pub fn new() -> Self {
        SimplePCMController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl PCMController for SimplePCMController {
    fn init(&mut self, pcm_id: PCMID) -> Result<(), PCMError> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == pcm_id {
                    device.active.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(PCMError::NotFound)
    }
    
    fn send(&self, pcm_id: PCMID, _data: &[i16]) -> Result<usize, PCMError> {
        if self.get_device(pcm_id).is_some() {
            Ok(0)
        } else {
            Err(PCMError::NotFound)
        }
    }
    
    fn receive(&self, pcm_id: PCMID, buffer: &mut [i16]) -> Result<usize, PCMError> {
        if self.get_device(pcm_id).is_some() {
            for sample in buffer.iter_mut() { *sample = 0; }
            Ok(buffer.len())
        } else {
            Err(PCMError::NotFound)
        }
    }
    
    fn get_device(&self, id: PCMID) -> Option<&dyn PCMDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait PCMSync {
    def set_sync_mode(&mut self, pcm_id: PCMID, mode: u8) -> Result<(), PCMError>;
    def get_sync_mode(&self, pcm_id: PCMID) -> Result<u8, PCMError>;
}

#[repr(C)]
pub struct SimplePCMSync {
    pub controller: SimplePCMController,
    pub sync_modes: Vec<(PCMID, AtomicUsize)>,
}

impl SimplePCMSync {
    pub fn new(controller: SimplePCMController) -> Self {
        SimplePCMSync {
            controller,
            sync_modes: Vec::new(),
        }
    }
}

impl PCMSync for SimplePCMSync {
    fn set_sync_mode(&mut self, pcm_id: PCMID, mode: u8) -> Result<(), PCMError> {
        self.sync_modes.push((pcm_id, AtomicUsize::new(mode as usize)));
        Ok(())
    }
    
    fn get_sync_mode(&self, pcm_id: PCMID) -> Result<u8, PCMError> {
        for &(id, ref mode) in &self.sync_modes {
            if id == pcm_id {
                return Ok(mode.load(Ordering::SeqCst) as u8);
            }
        }
        Err(PCMError::NotFound)
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
