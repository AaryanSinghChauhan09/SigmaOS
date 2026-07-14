#![no_std]
#![no_main]

/// OOP-based IoT Device Discovery for SigmaOS
/// Based on Ideas-999-Structured: IoT & Smart Home Item 1006
/// Implements device discovery and enumeration

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DiscoveryProtocol { mDNS = 0, UPnP = 1, SSDP = 2, BLE = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum DiscoveryError { Success = 0, NotFound = 1, ScanFailed = 2 }

pub trait DiscoveredDevice {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn address(&self) -> &[u8];
    fn protocol(&self) -> DiscoveryProtocol;
}

#[repr(C)]
pub struct SimpleDiscoveredDevice {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub address: [u8; 64],
    pub protocol: AtomicUsize,
}

impl SimpleDiscoveredDevice {
    pub fn new(id: DeviceID, name: &[u8], address: &[u8], protocol: DiscoveryProtocol) -> Self {
        let mut name_array = [0u8; 64];
        let mut addr_array = [0u8; 64];
        let name_len = name.len().min(63);
        let addr_len = address.len().min(63);
        for i in 0..name_len { name_array[i] = name[i]; }
        for i in 0..addr_len { addr_array[i] = address[i]; }
        SimpleDiscoveredDevice {
            id,
            name: name_array,
            address: addr_array,
            protocol: AtomicUsize::new(protocol as usize),
        }
    }
}

impl DiscoveredDevice for SimpleDiscoveredDevice {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn address(&self) -> &[u8] {
        let len = self.address.iter().position(|&b| b == 0).unwrap_or(64);
        &self.address[..len]
    }
    fn protocol(&self) -> DiscoveryProtocol { unsafe { core::mem::transmute(self.protocol.load(Ordering::SeqCst)) } }
}

pub trait DeviceDiscovery {
    fn start_scan(&mut self, protocol: DiscoveryProtocol) -> Result<(), DiscoveryError>;
    fn stop_scan(&mut self);
    fn get_devices(&self) -> Vec<&dyn DiscoveredDevice>;
}

#[repr(C)]
pub struct SimpleDeviceDiscovery {
    pub devices: Vec<Option<Box<dyn DiscoveredDevice>>>,
    pub scanning: AtomicUsize,
    pub next_id: AtomicUsize,
}

impl SimpleDeviceDiscovery {
    pub fn new() -> Self {
        SimpleDeviceDiscovery {
            devices: Vec::new(),
            scanning: AtomicUsize::new(0),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl DeviceDiscovery for SimpleDeviceDiscovery {
    fn start_scan(&mut self, _protocol: DiscoveryProtocol) -> Result<(), DiscoveryError> {
        self.scanning.store(1, Ordering::SeqCst);
        Ok(())
    }
    
    fn stop_scan(&mut self) {
        self.scanning.store(0, Ordering::SeqCst);
    }
    
    fn get_devices(&self) -> Vec<&dyn DiscoveredDevice> {
        let mut devices = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                devices.push(device.as_ref());
            }
        }
        devices
    }
}

pub trait DevicePairing {
    fn pair_device(&mut self, device_id: DeviceID) -> Result<(), DiscoveryError>;
    fn unpair_device(&mut self, device_id: DeviceID) -> Result<(), DiscoveryError>;
}

#[repr(C)]
pub struct SimpleDevicePairing {
    pub paired: Vec<DeviceID>,
}

impl SimpleDevicePairing {
    pub fn new() -> Self {
        SimpleDevicePairing {
            paired: Vec::new(),
        }
    }
}

impl DevicePairing for SimpleDevicePairing {
    fn pair_device(&mut self, device_id: DeviceID) -> Result<(), DiscoveryError> {
        self.paired.push(device_id);
        Ok(())
    }
    
    fn unpair_device(&mut self, device_id: DeviceID) -> Result<(), DiscoveryError> {
        for i in 0..self.paired.len() {
            if self.paired[i] == device_id {
                self.paired.remove(i);
                return Ok(());
            }
        }
        Err(DiscoveryError::NotFound)
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
    fn remove(&mut self, index: usize) -> T {
        unsafe {
            let item = core::ptr::read(self.data.add(index));
            for i in index..self.len - 1 {
                core::ptr::copy_nonoverlapping(self.data.add(i + 1), self.data.add(i), 1);
            }
            self.len -= 1;
            item
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
