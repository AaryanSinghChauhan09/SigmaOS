// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/wifi/wifi_device_base.rs — Base Device Trait for Wi-Fi Drivers
//
// Defines the OOP base class for all Wi-Fi devices using Rust traits.
// This provides a common interface for Wi-Fi operations across different chipsets.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Error Codes ─────────────────────────────────────────────────────────────

pub const WIFI_OK: I32 = 0;
pub const WIFI_ERR_NO_DEVICE: I32 = -1;
pub const WIFI_ERR_INIT_FAILED: I32 = -2;
pub const WIFI_ERR_OUT_OF_MEM: I32 = -3;
pub const WIFI_ERR_NOT_SUPPORTED: I32 = -4;
pub const WIFI_ERR_SCAN_FAILED: I32 = -5;
pub const WIFI_ERR_CONNECT_FAILED: I32 = -6;
pub const WIFI_ERR_DISCONNECT_FAILED: I32 = -7;

// ─── Wi-Fi Security Types ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WifiSecurity {
    Open,
    WEP,
    WPA_PSK,
    WPA2_PSK,
    WPA3_SAE,
    WPA2_ENTERPRISE,
    WPA3_ENTERPRISE,
}

// ─── Wi-Fi Channel ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy)]
pub struct WifiChannel {
    pub number: U8,
    pub frequency_mhz: U32,
}

impl WifiChannel {
    pub const fn new(number: U8) -> Self {
        let frequency_mhz = if number >= 1 && number <= 14 {
            2407 + (number as U32) * 5
        } else if number >= 36 && number <= 165 {
            5000 + (number as U32) * 5
        } else {
            0
        };
        
        WifiChannel {
            number,
            frequency_mhz,
        }
    }
}

// ─── Wi-Fi Network Info ────────────────────────────────────────────────────

#[repr(C)]
pub struct WifiNetwork {
    pub ssid: [U8; 32],
    pub ssid_len: U8,
    pub bssid: [U8; 6],
    pub security: WifiSecurity,
    pub channel: WifiChannel,
    pub signal_strength: I32, // dBm
    pub is_hidden: bool,
}

impl WifiNetwork {
    pub const fn new() -> Self {
        WifiNetwork {
            ssid: [0; 32],
            ssid_len: 0,
            bssid: [0; 6],
            security: WifiSecurity::Open,
            channel: WifiChannel::new(1),
            signal_strength: -100,
            is_hidden: false,
        }
    }
}

// ─── Wi-Fi Connection Info ─────────────────────────────────────────────────

#[repr(C)]
pub struct WifiConnection {
    pub connected: bool,
    pub ssid: [U8; 32],
    pub ssid_len: U8,
    pub bssid: [U8; 6],
    pub security: WifiSecurity,
    pub channel: WifiChannel,
    pub ip_address: U32,
    pub subnet_mask: U32,
    pub gateway: U32,
    pub dns_server: U32,
}

impl WifiConnection {
    pub const fn new() -> Self {
        WifiConnection {
            connected: false,
            ssid: [0; 32],
            ssid_len: 0,
            bssid: [0; 6],
            security: WifiSecurity::Open,
            channel: WifiChannel::new(1),
            ip_address: 0,
            subnet_mask: 0,
            gateway: 0,
            dns_server: 0,
        }
    }
}

// ─── Wi-Fi Device Trait ─────────────────────────────────────────────────────

/// Trait for Wi-Fi-specific operations
pub trait WifiDevice {
    /// Initialize the Wi-Fi device
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Scan for available networks
    fn scan_networks(&mut self, networks: &mut [WifiNetwork], count: &mut usize) -> I32;
    
    /// Connect to a network
    fn connect(&mut self, ssid: &[U8], password: &[U8], security: WifiSecurity) -> I32;
    
    /// Disconnect from current network
    fn disconnect(&mut self) -> I32;
    
    /// Get current connection info
    fn get_connection_info(&self) -> WifiConnection;
    
    /// Get signal strength
    fn get_signal_strength(&self) -> I32;
    
    /// Set operation mode (managed, monitor, ad-hoc, etc.)
    fn set_mode(&mut self, mode: WifiMode) -> I32;
    
    /// Set transmit power
    fn set_tx_power(&mut self, power_dbm: I32) -> I32;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
}

// ─── Wi-Fi Operation Modes ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WifiMode {
    Managed,    // Station mode (client)
    Monitor,    // Monitor mode (packet capture)
    AdHoc,      // Ad-hoc mode (peer-to-peer)
    Master,     // Access point mode
}

// ─── Wi-Fi Statistics ───────────────────────────────────────────────────────

#[repr(C)]
pub struct WifiStats {
    pub tx_packets: U64,
    pub rx_packets: U64,
    pub tx_bytes: U64,
    pub rx_bytes: U64,
    pub tx_errors: U64,
    pub rx_errors: U64,
    pub tx_dropped: U64,
    pub rx_dropped: U64,
}

impl WifiStats {
    pub const fn new() -> Self {
        WifiStats {
            tx_packets: 0,
            rx_packets: 0,
            tx_bytes: 0,
            rx_bytes: 0,
            tx_errors: 0,
            rx_errors: 0,
            tx_dropped: 0,
            rx_dropped: 0,
        }
    }
}
