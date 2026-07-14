#![no_std]
#![no_main]

/// OOP-based Wireless Network Driver for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 86
/// Implements WiFi device management and connection

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type WirelessDeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WirelessType { WiFi = 0, Bluetooth = 1, Cellular = 2 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum WirelessError { Success = 0, NotFound = 1, ConnectFailed = 2, ScanFailed = 3 }

pub trait WirelessDevice {
    fn id(&self) -> WirelessDeviceID;
    fn device_type(&self) -> WirelessType;
    fn mac_address(&self) -> &[u8];
    fn scan_networks(&mut self) -> Result<Vec<([u8; 32], i8)>, WirelessError>;
}

#[repr(C)]
pub struct SimpleWirelessDevice {
    pub id: WirelessDeviceID,
    pub device_type: AtomicUsize,
    pub mac_address: [u8; 6],
}

impl SimpleWirelessDevice {
    pub fn new(id: WirelessDeviceID, device_type: WirelessType, mac: &[u8]) -> Self {
        let mut mac_array = [0u8; 6];
        let mac_len = mac.len().min(6);
        unsafe {
            core::ptr::copy_nonoverlapping(mac.as_ptr(), mac_array.as_mut_ptr(), mac_len);
        }
        SimpleWirelessDevice {
            id,
            device_type: AtomicUsize::new(device_type as usize),
            mac_address: mac_array,
        }
    }
}

impl WirelessDevice for SimpleWirelessDevice {
    fn id(&self) -> WirelessDeviceID { self.id }
    fn device_type(&self) -> WirelessType { unsafe { core::mem::transmute(self.device_type.load(Ordering::SeqCst)) } }
    fn mac_address(&self) -> &[u8] { &self.mac_address }
    
    fn scan_networks(&mut self) -> Result<Vec<([u8; 32], i8)>, WirelessError> {
        let mut networks = Vec::new();
        networks.push((*b"SigmaOS-Network", -50));
        networks.push((*b"Guest-Network", -70));
        Ok(networks)
    }
}

pub trait WiFiConnection {
    fn connect(&mut self, ssid: &[u8], password: &[u8]) -> Result<(), WirelessError>;
    fn disconnect(&mut self) -> Result<(), WirelessError>;
    fn is_connected(&self) -> bool;
    fn get_signal_strength(&self) -> i8;
}

#[repr(C)]
pub struct SimpleWiFiConnection {
    pub connected: AtomicUsize,
    pub signal_strength: AtomicUsize,
}

impl SimpleWiFiConnection {
    pub fn new() -> Self {
        SimpleWiFiConnection {
            connected: AtomicUsize::new(0),
            signal_strength: AtomicUsize::new(0),
        }
    }
}

impl WiFiConnection for SimpleWiFiConnection {
    fn connect(&mut self, _ssid: &[u8], _password: &[u8]) -> Result<(), WirelessError> {
        self.connected.store(1, Ordering::SeqCst);
        self.signal_strength.store(60, Ordering::SeqCst);
        Ok(())
    }
    
    fn disconnect(&mut self) -> Result<(), WirelessError> {
        self.connected.store(0, Ordering::SeqCst);
        self.signal_strength.store(0, Ordering::SeqCst);
        Ok(())
    }
    
    fn is_connected(&self) -> bool { self.connected.load(Ordering::SeqCst) == 1 }
    
    fn get_signal_strength(&self) -> i8 { self.signal_strength.load(Ordering::SeqCst) as i8 }
}

pub trait WirelessManager {
    fn register_device(&mut self, device: Box<dyn WirelessDevice>) -> Result<WirelessDeviceID, WirelessError>;
    fn get_device(&self, id: WirelessDeviceID) -> Option<&dyn WirelessDevice>;
    fn list_devices(&self) -> Vec<WirelessDeviceID>;
}

#[repr(C)]
pub struct SimpleWirelessManager {
    pub devices: Vec<Option<Box<dyn WirelessDevice>>>,
    pub next_id: AtomicUsize,
}

impl SimpleWirelessManager {
    pub fn new() -> Self {
        SimpleWirelessManager {
            devices: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl WirelessManager for SimpleWirelessManager {
    fn register_device(&mut self, device: Box<dyn WirelessDevice>) -> Result<WirelessDeviceID, WirelessError> {
        let id = device.id();
        self.devices.push(Some(device));
        Ok(id)
    }
    
    fn get_device(&self, id: WirelessDeviceID) -> Option<&dyn WirelessDevice> {
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                if device.id() == id { return Some(device.as_ref()); }
            }
        }
        None
    }
    
    fn list_devices(&self) -> Vec<WirelessDeviceID> {
        let mut ids = Vec::new();
        for device_option in &self.devices {
            if let Some(ref device) = *device_option {
                ids.push(device.id());
            }
        }
        ids
    }
}

pub trait WirelessSecurity {
    fn set_security_mode(&mut self, mode: u8);
    fn get_security_mode(&self) -> u8;
    fn enable_wpa3(&mut self, enabled: bool);
}

#[repr(C)]
pub struct SimpleWirelessSecurity {
    pub security_mode: AtomicUsize,
    pub wpa3_enabled: AtomicUsize,
}

impl SimpleWirelessSecurity {
    pub fn new() -> Self {
        SimpleWirelessSecurity {
            security_mode: AtomicUsize::new(2),
            wpa3_enabled: AtomicUsize::new(1),
        }
    }
}

impl WirelessSecurity for SimpleWirelessSecurity {
    fn set_security_mode(&mut self, mode: u8) {
        self.security_mode.store(mode as usize, Ordering::SeqCst);
    }
    
    fn get_security_mode(&self) -> u8 { self.security_mode.load(Ordering::SeqCst) as u8 }
    
    fn enable_wpa3(&mut self, enabled: bool) {
        self.wpa3_enabled.store(if enabled { 1 } else { 0 }, Ordering::SeqCst);
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
    fn is_empty(&self) -> bool { self.len == 0 }
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
