// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/wifi/iwlwifi.rs — Intel Wireless Wi-Fi Driver
//
// Implements Intel Wi-Fi driver (iwlwifi) for Intel wireless chipsets.
// Supports modern Intel Wi-Fi adapters (AX200, AX201, AX210, etc.)
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::wifi_device_base::{WifiDevice, WifiMode, WifiSecurity, WifiNetwork, WifiConnection, WifiStats, WifiChannel, WIFI_OK, WIFI_ERR_NO_DEVICE, WIFI_ERR_INIT_FAILED, WIFI_ERR_SCAN_FAILED, WIFI_ERR_CONNECT_FAILED, WIFI_ERR_DISCONNECT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Intel Wi-Fi Device IDs ───────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;

// Intel Wi-Fi device IDs
pub const IWL_DEVICE_ID_AX200: U16 = 0x2720;
pub const IWL_DEVICE_ID_AX201: U16 = 0xA0F0;
pub const IWL_DEVICE_ID_AX210: U16 = 0x271C;
pub const IWL_DEVICE_ID_AX211: U16 = 0x271D;
pub const IWL_DEVICE_ID_AX411: U16 = 0x2725;
pub const IWL_DEVICE_ID_9560: U16 = 0x2526;
pub const IWL_DEVICE_ID_9462: U16 = 0x02F0;
pub const IWL_DEVICE_ID_9260: U16 = 0x2520;

// ─── Intel Wi-Fi Families ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IwlFamily {
    Unknown,
    9000Series,    // 9260, 9560, 9462
    2000Series,    // AX200, AX201
    2100Series,    // AX210, AX211, AX411
}

// ─── Intel Wi-Fi Device Structure ─────────────────────────────────────────

pub struct IwlDevice {
    pub mmio_base: U64,
    pub device_id: U16,
    pub initialized: bool,
    pub family: IwlFamily,
    pub connection: WifiConnection,
    pub stats: WifiStats,
    pub current_mode: WifiMode,
    pub tx_power_dbm: I32,
}

impl IwlDevice {
    pub const fn new() -> Self {
        IwlDevice {
            mmio_base: 0,
            device_id: 0,
            initialized: false,
            family: IwlFamily::Unknown,
            connection: WifiConnection::new(),
            stats: WifiStats::new(),
            current_mode: WifiMode::Managed,
            tx_power_dbm: 20,
        }
    }

    /// Get Wi-Fi family from device ID
    fn get_wifi_family(&self, device_id: U16) -> IwlFamily {
        match device_id {
            IWL_DEVICE_ID_9560 |
            IWL_DEVICE_ID_9462 |
            IWL_DEVICE_ID_9260 => IwlFamily::9000Series,
            
            IWL_DEVICE_ID_AX200 |
            IWL_DEVICE_ID_AX201 => IwlFamily::2000Series,
            
            IWL_DEVICE_ID_AX210 |
            IWL_DEVICE_ID_AX211 |
            IWL_DEVICE_ID_AX411 => IwlFamily::2100Series,
            
            _ => IwlFamily::Unknown,
        }
    }

    /// Initialize Intel Wi-Fi device
    fn init_iwl(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;

        // Validate device ID and determine family
        self.family = self.get_wifi_family(device_id);
        if self.family == IwlFamily::Unknown {
            return WIFI_ERR_NO_DEVICE;
        }

        // In a real implementation, this would:
        // 1. Load firmware from /lib/firmware
        // 2. Initialize NIC (Network Interface Card)
        // 3. Set up TX/RX rings
        // 4. Configure interrupts
        // 5. Enable radio

        self.initialized = true;
        WIFI_OK
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: U32) -> U32 {
        let ptr = (self.mmio_base + offset as U64) as *const U32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: U32, value: U32) {
        let ptr = (self.mmio_base + offset as U64) as *mut U32;
        *ptr = value;
    }
}

// ─── Implement WifiDevice Trait ─────────────────────────────────────────────

impl WifiDevice for IwlDevice {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        self.init_iwl(pci_bar, device_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.family {
            IwlFamily::9000Series => "Intel Wireless 9000 Series",
            IwlFamily::2000Series => "Intel Wi-Fi 6 AX200/AX201",
            IwlFamily::2100Series => "Intel Wi-Fi 6E AX210/AX211/AX411",
            IwlFamily::Unknown => "Intel Wireless",
        }
    }

    fn scan_networks(&mut self, networks: &mut [WifiNetwork], count: &mut usize) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send scan command to firmware
        // 2. Wait for scan results
        // 3. Parse beacon frames
        // 4. Populate networks array

        // Stub: Return one fake network for testing
        if networks.len() > 0 {
            networks[0] = WifiNetwork {
                ssid: [b'S', b'i', b'g', b'm', b'a', b'O', b'S', 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                ssid_len: 7,
                bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
                security: WifiSecurity::WPA2_PSK,
                channel: WifiChannel::new(6),
                signal_strength: -45,
                is_hidden: false,
            };
            *count = 1;
        }

        WIFI_OK
    }

    fn connect(&mut self, ssid: &[U8], password: &[U8], security: WifiSecurity) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Authenticate with AP
        // 2. Associate with AP
        // 3. Perform 4-way handshake (for WPA/WPA2)
        // 4. Obtain IP via DHCP

        // Stub: Mark as connected
        self.connection.connected = true;
        self.connection.ssid_len = ssid.len().min(32) as U8;
        for i in 0..self.connection.ssid_len as usize {
            self.connection.ssid[i] = ssid[i];
        }
        self.connection.security = security;
        self.connection.channel = WifiChannel::new(6);
        self.connection.ip_address = 0xC0A8010A; // 192.168.1.10
        self.connection.subnet_mask = 0xFFFFFF00; // 255.255.255.0
        self.connection.gateway = 0xC0A80101; // 192.168.1.1
        self.connection.dns_server = 0xC0A80101; // 192.168.1.1

        WIFI_OK
    }

    fn disconnect(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send deauthentication frame
        // 2. Clear connection state

        self.connection = WifiConnection::new();
        WIFI_OK
    }

    fn get_connection_info(&self) -> WifiConnection {
        self.connection
    }

    fn get_signal_strength(&self) -> I32 {
        if self.connection.connected {
            -45 // Stub: return fake signal strength
        } else {
            -100
        }
    }

    fn set_mode(&mut self, mode: WifiMode) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Send mode change command to firmware
        // 2. Reconfigure device for new mode

        self.current_mode = mode;
        WIFI_OK
    }

    fn set_tx_power(&mut self, power_dbm: I32) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Validate power range
        // 2. Send power command to firmware

        self.tx_power_dbm = power_dbm.max(0).min(30);
        WIFI_OK
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, perform NIC reset
        WIFI_OK
    }

    fn shutdown(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, perform NIC shutdown
        self.initialized = false;
        WIFI_OK
    }
}

// ─── Global Intel Wi-Fi Device ───────────────────────────────────────────

static mut G_IWL: IwlDevice = IwlDevice::new();

// ─── C-ABI Exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_init(pci_bar: U64, device_id: U16) -> I32 {
    G_IWL.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_is_initialized() -> I32 {
    if G_IWL.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_scan(networks: *mut WifiNetwork, max_count: usize, count: *mut usize) -> I32 {
    if networks.is_null() || count.is_null() {
        return WIFI_ERR_INIT_FAILED;
    }
    
    let networks_slice = core::slice::from_raw_parts_mut(networks, max_count);
    let count_mut = &mut *count;
    G_IWL.scan_networks(networks_slice, count_mut)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_connect(ssid: *const U8, ssid_len: usize, password: *const U8, password_len: usize, security: U32) -> I32 {
    if ssid.is_null() || password.is_null() {
        return WIFI_ERR_CONNECT_FAILED;
    }
    
    let ssid_slice = core::slice::from_raw_parts(ssid, ssid_len);
    let password_slice = core::slice::from_raw_parts(password, password_len);
    
    let security_type = match security {
        0 => WifiSecurity::Open,
        1 => WifiSecurity::WEP,
        2 => WifiSecurity::WPA_PSK,
        3 => WifiSecurity::WPA2_PSK,
        4 => WifiSecurity::WPA3_SAE,
        _ => WifiSecurity::WPA2_PSK,
    };
    
    G_IWL.connect(ssid_slice, password_slice, security_type)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_disconnect() -> I32 {
    G_IWL.disconnect()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_get_connection_info() -> WifiConnection {
    G_IWL.get_connection_info()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_get_signal_strength() -> I32 {
    G_IWL.get_signal_strength()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_set_mode(mode: U32) -> I32 {
    let wifi_mode = match mode {
        0 => WifiMode::Managed,
        1 => WifiMode::Monitor,
        2 => WifiMode::AdHoc,
        3 => WifiMode::Master,
        _ => WifiMode::Managed,
    };
    G_IWL.set_mode(wifi_mode)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_set_tx_power(power_dbm: I32) -> I32 {
    G_IWL.set_tx_power(power_dbm)
}

/// Probe for Intel Wi-Fi devices
#[no_mangle]
pub unsafe extern "C" fn iwlwifi_probe() -> I32 {
    // Scan PCI bus for Intel Wi-Fi devices
    let mut found_devices = 0;
    
    for bus in 0..256u8 {
        for device in 0..32u8 {
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                
                if vendor_id == INTEL_VENDOR_ID && is_iwl_device(device_id) {
                    let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                    let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                    
                    let result = G_IWL.init(mmio_base, device_id);
                    
                    if result == WIFI_OK {
                        found_devices += 1;
                        return WIFI_OK;
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        WIFI_OK
    } else {
        WIFI_ERR_NO_DEVICE
    }
}

/// Check if device ID is a supported Intel Wi-Fi device
unsafe fn is_iwl_device(device_id: U16) -> bool {
    match device_id {
        IWL_DEVICE_ID_AX200 |
        IWL_DEVICE_ID_AX201 |
        IWL_DEVICE_ID_AX210 |
        IWL_DEVICE_ID_AX211 |
        IWL_DEVICE_ID_AX411 |
        IWL_DEVICE_ID_9560 |
        IWL_DEVICE_ID_9462 |
        IWL_DEVICE_ID_9260 => true,
        _ => false,
    }
}

/// Read 16-bit value from PCI configuration space
unsafe fn read_pci_config_u16(bus: U8, device: U8, function: U8, offset: U8) -> U16 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    let value = inl(0xCFC);
    let shift = ((offset & 2) as u32) * 8;
    ((value >> shift) & 0xFFFF) as U16
}

/// Read 32-bit value from PCI configuration space
unsafe fn read_pci_config_u32(bus: U8, device: U8, function: U8, offset: U8) -> U32 {
    let config_address = ((1u32 << 31) | 
                          ((bus as u32) << 16) | 
                          ((device as u32) << 11) | 
                          ((function as u32) << 8) | 
                          ((offset as u32) & 0xFC)) as u32;
    
    outl(0xCF8, config_address);
    inl(0xCFC)
}

unsafe fn outl(port: U16, value: U32) {
    // Placeholder
}

unsafe fn inl(port: U16) -> U32 {
    // Placeholder
    0
}
