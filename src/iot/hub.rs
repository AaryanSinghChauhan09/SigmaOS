#![no_std]
#![no_main]

/// OOP-based IoT Hub for SigmaOS
/// Based on Ideas-999-Structured: IoT & Smart Home Item 976
/// Implements IoT device management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DeviceType { Sensor = 0, Actuator = 1, Controller = 2, Gateway = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IoTError { Success = 0, NotFound = 1, ConnectionFailed = 2 }

pub trait IoTDevice {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn device_type(&self) -> DeviceType;
    fn is_online(&self) -> bool;
}

#[repr(C)]
pub struct SimpleIoTDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub device_type: AtomicUsize,
    pub online: AtomicUsize,
}

impl SimpleIoTDevice {
    pub fn new(id: DeviceID, name: &[u8], device_type: DeviceType) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
        }
        SimpleIoTDevice {
            id,
            name: name_array,
            device_type: AtomicUsize::new(device_type as usize),
            online: AtomicUsize::new(0),
        }
    }
}

impl IoTDevice for SimpleIoTDevice {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn device_type(&self) -> DeviceType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }
    fn is_online(&self) -> bool { self.online.load(Ordering::SeqCst) == 1 }
}

pub trait IoTHub {
    fn add_device(&mut self, device: Box<dyn IoTDevice>) -> Result<DeviceID, IoTError>;
    fn remove_device(&mut self, id: DeviceID) -> Result<(), IoTError>;
    fn get_device(&self, id: DeviceID) -> Option<&dyn IoTDevice>;
    def send_command(&self, id: DeviceID, command: &[u8]) -> Result<(), IoTError>;
}

#[repr(C)]
pub struct SimpleIoTHub {
    pub devices: Vec<Option<Box<dyn IoTDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleIoTHub {
    pub fn new() -> Self {
        SimpleIoTHub {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl IoTHub for SimpleIoTHub {
    fn add_device(&mut self, device: Box<dyn IoTDevice>) -> Result<DeviceID, IoTError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    
    fn remove_device(&mut self, id: DeviceID) -> Result<(), IoTError> {
        for device_option in &mut self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id {
                    return Ok(());
                }
            }
        }
        Err(IoTError::NotFound)
    }
    
    fn get_device(&self, id: DeviceID) -> Option<&dyn IoTDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
    
    fn send_command(&self, id: DeviceID, _command: &[u8]) -> Result<(), IoTError> {
        if self.get_device(id).is_some() {
            Ok(())
        } else {
            Err(IoTError::NotFound)
        }
    }
}

pub trait AutomationRule {
    fn add_rule(&mut self, trigger: &[u8], action: &[u8]);
    def execute_rules(&self, event: &[u8]) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleAutomationRule {
    pub rules: Vec<([u8; 64], [u8; 64])>,
}

impl SimpleAutomationRule {
    pub fn new() -> Self {
        SimpleAutomationRule {
            rules: Vec::new(),
        }
    }
}

impl AutomationRule for SimpleAutomationRule {
    fn add_rule(&mut self, trigger: &[u8], action: &[u8]) {
        let mut trigger_array = [0u8; 64];
        let mut action_array = [0u8; 64];
        let trigger_len = trigger.len().min(63);
        let action_len = action.len().min(63);
        for i in 0..trigger_len { trigger_array[i] = trigger[i]; }
        for i in 0..action_len { action_array[i] = action[i]; }
        self.rules.push((trigger_array, action_array));
    }
    
    fn execute_rules(&self, event: &[u8]) -> Vec<&[u8]> {
        let mut actions = Vec::new();
        for &(ref trigger, ref action) in &self.rules {
            let trigger_len = trigger.iter().position(|&b| b == 0).unwrap_or(64);
            if &trigger[..trigger_len] == event {
                let action_len = action.iter().position(|&b| b == 0).unwrap_or(64);
                actions.push(&action[..action_len]);
            }
        }
        actions
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
