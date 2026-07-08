// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/bluetooth/bluetooth_device_base.rs — Base Device Trait for Bluetooth Drivers
//
// Defines the OOP base class for all Bluetooth devices using Rust traits.
// This provides a common interface for Bluetooth operations across different chipsets.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Error Codes ─────────────────────────────────────────────────────────────

pub const BT_OK: I32 = 0;
pub const BT_ERR_NO_DEVICE: I32 = -1;
pub const BT_ERR_INIT_FAILED: I32 = -2;
pub const BT_ERR_OUT_OF_MEM: I32 = -3;
pub const BT_ERR_NOT_SUPPORTED: I32 = -4;
pub const BT_ERR_SCAN_FAILED: I32 = -5;
pub const BT_ERR_CONNECT_FAILED: I32 = -6;
pub const BT_ERR_DISCONNECT_FAILED: I32 = -7;

// ─── Bluetooth Address ───────────────────────────────────────────────────────

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BluetoothAddress {
    pub bytes: [U8; 6],
}

impl BluetoothAddress {
    pub const fn new() -> Self {
        BluetoothAddress {
            bytes: [0; 6],
        }
    }
    
    pub const fn from_bytes(b0: U8, b1: U8, b2: U8, b3: U8, b4: U8, b5: U8) -> Self {
        BluetoothAddress {
            bytes: [b0, b1, b2, b3, b4, b5],
        }
    }
}

// ─── Bluetooth Device Class ───────────────────────────────────────────────

#[repr(C)]
pub struct BluetoothDeviceClass {
    pub major_class: U8,
    pub minor_class: U8,
    pub service_class: U16,
}

impl BluetoothDeviceClass {
    pub const fn new() -> Self {
        BluetoothDeviceClass {
            major_class: 0,
            minor_class: 0,
            service_class: 0,
        }
    }
}

// ─── Bluetooth Device Info ─────────────────────────────────────────────────

#[repr(C)]
pub struct BluetoothDeviceInfo {
    pub address: BluetoothAddress,
    pub name: [U8; 248],
    pub name_len: U8,
    pub device_class: BluetoothDeviceClass,
    pub rssi: I32,
    pub is_connected: bool,
    pub is_paired: bool,
}

impl BluetoothDeviceInfo {
    pub const fn new() -> Self {
        BluetoothDeviceInfo {
            address: BluetoothAddress::new(),
            name: [0; 248],
            name_len: 0,
            device_class: BluetoothDeviceClass::new(),
            rssi: -100,
            is_connected: false,
            is_paired: false,
        }
    }
}

// ─── Bluetooth Adapter State ───────────────────────────────────────────────

#[repr(C)]
pub struct BluetoothAdapterState {
    pub powered: bool,
    pub discoverable: bool,
    pub connectable: bool,
    pub address: BluetoothAddress,
    pub name: [U8; 248],
    pub name_len: U8,
}

impl BluetoothAdapterState {
    pub const fn new() -> Self {
        BluetoothAdapterState {
            powered: false,
            discoverable: false,
            connectable: false,
            address: BluetoothAddress::new(),
            name: [0; 248],
            name_len: 0,
        }
    }
}

// ─── Bluetooth Device Trait ───────────────────────────────────────────────

/// Trait for Bluetooth-specific operations
pub trait BluetoothDevice {
    /// Initialize the Bluetooth device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Power on the adapter
    fn power_on(&mut self) -> I32;
    
    /// Power off the adapter
    fn power_off(&mut self) -> I32;
    
    /// Start device discovery
    fn start_discovery(&mut self) -> I32;
    
    /// Stop device discovery
    fn stop_discovery(&mut self) -> I32;
    
    /// Get discovered devices
    fn get_discovered_devices(&self, devices: &mut [BluetoothDeviceInfo], count: &mut usize) -> I32;
    
    /// Connect to a device
    fn connect(&mut self, address: BluetoothAddress) -> I32;
    
    /// Disconnect from a device
    fn disconnect(&mut self, address: BluetoothAddress) -> I32;
    
    /// Pair with a device
    fn pair(&mut self, address: BluetoothAddress, pin: &[U8]) -> I32;
    
    /// Unpair a device
    fn unpair(&mut self, address: BluetoothAddress) -> I32;
    
    /// Get adapter state
    fn get_adapter_state(&self) -> BluetoothAdapterState;
    
    /// Set adapter name
    fn set_name(&mut self, name: &[U8]) -> I32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}
