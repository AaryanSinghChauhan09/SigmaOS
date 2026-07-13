#![no_std]
#![no_main]

/// OOP-based Hardware Abstraction Layer for SigmaOS
/// Based on Roadmap Item 2: Hardware abstraction layer

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceType { CPU = 0, Memory = 1, Storage = 2, Network = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceState { Uninitialized = 0, Initialized = 1, Active = 2 }

pub trait Device {
    fn id(&self) -> DeviceID;
    fn device_type(&self) -> DeviceType;
    fn state(&self) -> DeviceState;
    fn initialize(&mut self) -> Result<(), HALError>;
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum HALError { Success = 0, InitFailed = 1 }

#[repr(C)]
pub struct SimpleDevice {
    pub id: DeviceID,
    pub device_type: DeviceType,
    pub state: AtomicUsize,
}

impl SimpleDevice {
    pub fn new(id: DeviceID, device_type: DeviceType) -> Self {
        SimpleDevice { id, device_type, state: AtomicUsize::new(DeviceState::Uninitialized as usize) }
    }
}

impl Device for SimpleDevice {
    fn id(&self) -> DeviceID { self.id }
    fn device_type(&self) -> DeviceType { self.device_type }
    fn state(&self) -> DeviceState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }
    fn initialize(&mut self) -> Result<(), HALError> {
        self.state.store(DeviceState::Initialized as usize, Ordering::SeqCst);
        Ok(())
    }
}

pub trait HAL {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, HALError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device>;
}

pub struct SimpleHAL {
    devices: Vec<Option<Box<dyn Device>>>,
    next_id: AtomicUsize,
}

impl SimpleHAL {
    pub fn new() -> Self { SimpleHAL { devices: Vec::new(), next_id: AtomicUsize::new(1) } }
}

impl HAL for SimpleHAL {
    fn register_device(&mut self, device: Box<dyn Device>) -> Result<DeviceID, HALError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    fn get_device(&self, id: DeviceID) -> Option<&dyn Device> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
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
