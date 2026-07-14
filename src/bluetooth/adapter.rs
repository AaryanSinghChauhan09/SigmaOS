#![no_std]
#![no_main]

/// OOP-based Bluetooth Adapter for SigmaOS
/// Based on Ideas-999-Structured: Kernel & Hardware Item 271
/// Implements Bluetooth device management

use core::sync::atomic::{AtomicUsize, Ordering};
use core::mem;

pub type DeviceID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BluetoothState { Off = 0, On = 1, Scanning = 2, Pairing = 3 }

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum BluetoothError { Success = 0, NotFound = 1, PairingFailed = 2 }

pub trait BluetoothAdapter {
    fn id(&self) -> DeviceID;
    fn name(&self) -> &[u8];
    fn address(&self) -> &[u8];
    fn state(&self) -> BluetoothState;
    fn set_state(&mut self, state: BluetoothState);
}

#[repr(C)]
pub struct SimpleBluetoothAdapter {
    pub id: DeviceID,
    pub name: [u8; 64],
    pub address: [u8; 6],
    pub state: AtomicUsize,
}

impl SimpleBluetoothAdapter {
    pub fn new(id: DeviceID, name: &[u8], address: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let mut addr_array = [0u8; 6];
        let name_len = name.len().min(63);
        let addr_len = address.len().min(6);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), name_len);
            core::ptr::copy_nonoverlapping(address.as_ptr(), addr_array.as_mut_ptr(), addr_len);
        }
        SimpleBluetoothAdapter {
            id,
            name: name_array,
            address: addr_array,
            state: AtomicUsize::new(BluetoothState::Off as usize),
        }
    }
}

impl BluetoothAdapter for SimpleBluetoothAdapter {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn address(&self) -> &[u8] { &self.address }
    fn state(&self) -> BluetoothState { unsafe { core::mem::transmute(self.state.load(Ordering::SeqCst)) } }

    fn set_state(&mut self, state: BluetoothState) {
        self.state.store(state as usize, Ordering::SeqCst);
    }
}

pub trait BluetoothManager {
    fn add_adapter(&mut self, adapter: Box<dyn BluetoothAdapter>) -> Result<DeviceID, BluetoothError>;
    fn remove_adapter(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
    fn get_adapter(&self, id: DeviceID) -> Option<&dyn BluetoothAdapter>;
    fn start_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
    fn stop_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError>;
}

#[repr(C)]
pub struct SimpleBluetoothManager {
    pub adapters: Vec<Option<Box<dyn BluetoothAdapter>>>,
    pub next_id: AtomicUsize,
}

impl SimpleBluetoothManager {
    pub fn new() -> Self {
        SimpleBluetoothManager {
            adapters: Vec::new(),
            next_id: AtomicUsize::new(1),
        }
    }
}

impl BluetoothManager for SimpleBluetoothManager {
    fn add_adapter(&mut self, adapter: Box<dyn BluetoothAdapter>) -> Result<DeviceID, BluetoothError> {
        let id = adapter.id();
        self.adapters.push(Some(adapter));
        Ok(id)
    }

    fn remove_adapter(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref adapter) = *adapter_option {
                if adapter.id() == id {
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn get_adapter(&self, id: DeviceID) -> Option<&dyn BluetoothAdapter> {
        for adapter_option in &self.adapters {
            if let Some(ref adapter) = *adapter_option {
                if adapter.id() == id { return Some(adapter.as_ref()); }
            }
        }
        None
    }

    fn start_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref mut adapter) = *adapter_option {
                if adapter.id() == id {
                    adapter.set_state(BluetoothState::Scanning);
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn stop_scan(&mut self, id: DeviceID) -> Result<(), BluetoothError> {
        for adapter_option in &mut self.adapters {
            if let Some(ref mut adapter) = *adapter_option {
                if adapter.id() == id {
                    adapter.set_state(BluetoothState::On);
                    return Ok(());
                }
            }
        }
        Err(BluetoothError::NotFound)
    }
}

pub trait DevicePairing {
    fn pair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError>;
    fn unpair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError>;
    fn get_paired_devices(&self, adapter_id: DeviceID) -> Vec<&[u8]>;
}

#[repr(C)]
pub struct SimpleDevicePairing {
    pub paired: Vec<(DeviceID, [u8; 6])>,
}

impl SimpleDevicePairing {
    pub fn new() -> Self {
        SimpleDevicePairing {
            paired: Vec::new(),
        }
    }
}

impl DevicePairing for SimpleDevicePairing {
    fn pair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError> {
        let mut addr_array = [0u8; 6];
        let addr_len = device_address.len().min(6);
        for i in 0..addr_len {
            addr_array[i] = device_address[i];
        }
        self.paired.push((adapter_id, addr_array));
        Ok(())
    }

    fn unpair_device(&mut self, adapter_id: DeviceID, device_address: &[u8]) -> Result<(), BluetoothError> {
        for i in 0..self.paired.len() {
            if self.paired[i].0 == adapter_id && &self.paired[i].1[..device_address.len()] == device_address {
                self.paired.remove(i);
                return Ok(());
            }
        }
        Err(BluetoothError::NotFound)
    }

    fn get_paired_devices(&self, adapter_id: DeviceID) -> Vec<&[u8]> {
        let mut devices = Vec::new();
        for &(id, ref addr) in &self.paired {
            if id == adapter_id {
                devices.push(addr);
            }
        }
        devices
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
