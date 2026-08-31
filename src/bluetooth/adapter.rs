#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]
extern crate alloc;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;

// (no_std only applicable at crate root - removed)
// #![no_main]  // crate-root only

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
    pub name_len: u8,
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
            name_len: name_len as u8,
            address: addr_array,
            state: AtomicUsize::new(BluetoothState::Off as usize),
        }
    }
}

impl BluetoothAdapter for SimpleBluetoothAdapter {
    fn id(&self) -> DeviceID { self.id }
    fn name(&self) -> &[u8] {
        // Bolt ⚡ Optimization: Store explicit name length on instantiation to eliminate
        // O(N) zero-byte linear scanning (.position(|&b| b == 0)) on every Bluetooth device name access,
        // reducing slice lookup to instantaneous O(1) constant time.
        &self.name[..self.name_len as usize]
    }
    fn address(&self) -> &[u8] { &self.address }
    fn state(&self) -> BluetoothState {
        match self.state.load(Ordering::SeqCst) {
            0 => BluetoothState::Off,
            1 => BluetoothState::On,
            2 => BluetoothState::Scanning,
            3 => BluetoothState::Pairing,
            _ => BluetoothState::Off,
        }
    }

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
    #[allow(clippy::new_without_default)]
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
    #[allow(clippy::new_without_default)]
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
                devices.push(&addr[..]);
            }
        }
        devices
    }
}

/// HCI Socket Controller State (inspired by NetBSD/FreeBSD hciconfig & bthset)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciState {
    Down,
    Up,
    Resetting,
    Testing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HciInquiryMode {
    Standard,
    Rssi,
    Extended,
}

/// HCI Controller Configuration
#[derive(Debug, Clone)]
pub struct HciControllerConfig {
    pub dev_index: usize,
    pub bd_addr: [u8; 6],
    pub state: HciState,
    pub page_timeout: u16,
    pub inquiry_mode: HciInquiryMode,
    pub acl_mtu: u16,
    pub sco_mtu: u8,
}

impl Default for HciControllerConfig {
    fn default() -> Self {
        Self {
            dev_index: 0,
            bd_addr: [0x00, 0x1A, 0x7D, 0xDA, 0x71, 0x13],
            state: HciState::Down,
            page_timeout: 0x2000,
            inquiry_mode: HciInquiryMode::Extended,
            acl_mtu: 1021,
            sco_mtu: 64,
        }
    }
}

/// Linux rfkill Subsystem Integration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RfKillState {
    Unblocked = 0,
    SoftBlocked = 1,
    HardBlocked = 2,
}

#[derive(Debug, Clone)]
pub struct RfKillSwitch {
    pub id: usize,
    pub name: String,
    pub state: RfKillState,
}

impl RfKillSwitch {
    pub fn new(id: usize, name: &str) -> Self {
        Self {
            id,
            name: String::from(name),
            state: RfKillState::Unblocked,
        }
    }

    pub fn set_soft_block(&mut self, blocked: bool) {
        if self.state != RfKillState::HardBlocked {
            self.state = if blocked { RfKillState::SoftBlocked } else { RfKillState::Unblocked };
        }
    }

    pub fn is_blocked(&self) -> bool {
        self.state != RfKillState::Unblocked
    }
}
