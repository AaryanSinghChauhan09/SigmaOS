#![no_std]
#![no_main]

/// OOP-based SAI for SigmaOS
/// Based on Ideas-999-Structured: Embedded & Firmware Item 2346
/// Implements SAI (Serial Audio Interface)

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type SAIID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum SAIError { Success = 0, NotFound = 1 }

pub trait SAIDevice {
    fn id(&self) -> SAIID;
    fn is_active(&self) -> bool;
}

#[repr(C)]
pub struct SimpleSAIDevice {
    pub id: SAIID,
    pub active: AtomicUsize,
}

impl SimpleSAIDevice {
    pub fn new(id: SAIID) -> Self {
        SimpleSAIDevice {
            id,
            active: AtomicUsize::new(0),
        }
    }
}

impl SAIDevice for SimpleSAIDevice {
    fn id(&self) -> SAIID { self.id }
    fn is_active(&self) -> bool { self.active.load(Ordering::SeqCst) == 1 }
}

pub trait SAIController {
    fn init(&mut self, sai_id: SAIID) -> Result<(), SAIError>;
    fn send(&self, sai_id: SAIID, data: &[i16]) -> Result<usize, SAIError>;
    def receive(&self, sai_id: SAIID, buffer: &mut [i16]) -> Result<usize, SAIError>;
}

#[repr(C)]
pub struct SimpleSAIController {
    pub devices: Vec<Option<Box<dyn SAIDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleSAIController {
    pub fn new() -> Self {
        SimpleSAIController {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl SAIController for SimpleSAIController {
    fn init(&mut self, sai_id: SAIID) -> Result<(), SAIError> {
        for device_option in &mut self.devices {
            if let Some(ref mut device) = *device_option {
                if device.id() == sai_id {
                    device.active.store(1, Ordering::SeqCst);
                    return Ok(());
                }
            }
        }
        Err(SAIError::NotFound)
    }
    
    fn send(&self, sai_id: SAIID, _data: &[i16]) -> Result<usize, SAIError> {
        if self.get_device(sai_id).is_some() {
            Ok(0)
        } else {
            Err(SAIError::NotFound)
        }
    }
    
    fn receive(&self, sai_id: SAIID, buffer: &mut [i16]) -> Result<usize, SAIError> {
        if self.get_device(sai_id).is_some() {
            for sample in buffer.iter_mut() { *sample = 0; }
            Ok(buffer.len())
        } else {
            Err(SAIError::NotFound)
        }
    }
    
    fn get_device(&self, id: SAIID) -> Option<&dyn SAIDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
}

pub trait SAISlot {
    def set_slot_count(&mut self, sai_id: SAIID, slots: u8) -> Result<(), SAIError>;
    def get_slot_count(&self, sai_id: SAIID) -> Result<u8, SAIError>;
}

#[repr(C)]
pub struct SimpleSAISlot {
    pub controller: SimpleSAIController,
    pub slot_counts: Vec<(SAIID, AtomicUsize)>,
}

impl SimpleSAISlot {
    pub fn new(controller: SimpleSAIController) -> Self {
        SimpleSAISlot {
            controller,
            slot_counts: Vec::new(),
        }
    }
}

impl SAISlot for SimpleSAISlot {
    fn set_slot_count(&mut self, sai_id: SAIID, slots: u8) -> Result<(), SAIError> {
        self.slot_counts.push((sai_id, AtomicUsize::new(slots as usize)));
        Ok(())
    }
    
    fn get_slot_count(&self, sai_id: SAIID) -> Result<u8, SAIError> {
        for &(id, ref count) in &self.slot_counts {
            if id == sai_id {
                return Ok(count.load(Ordering::SeqCst) as u8);
            }
        }
        Err(SAIError::NotFound)
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
