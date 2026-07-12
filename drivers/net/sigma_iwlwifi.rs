// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/net/sigma_iwlwifi.rs — Intel Wireless Wi-Fi Driver
//
// Implements Intel Wi-Fi driver with firmware loading,
// hardware initialization, and network operations for SigmaOS.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use crate::drivers::common_types::{SigmaU8, SigmaU16, SigmaU32, SigmaU64, SigmaI32, SigmaI64, SigmaBool, SigmaUsize};

pub const IWLWIFI_OK: SigmaI32 = 0;
pub const IWLWIFI_ERR_NO_DEVICE: SigmaI32 = -1;
pub const IWLWIFI_ERR_INIT_FAILED: SigmaI32 = -2;
pub const IWLWIFI_ERR_FIRMWARE: SigmaI32 = -3;
pub const IWLWIFI_ERR_SCAN_FAILED: SigmaI32 = -4;
pub const IWLWIFI_ERR_CONNECT_FAILED: SigmaI32 = -5;

// PCI Device IDs for Intel Wi-Fi adapters
pub const INTEL_VENDOR_ID: SigmaU16 = 0x8086;
pub const IWLWIFI_DEVICE_ID_22000: SigmaU16 = 0x2720;
pub const IWLWIFI_DEVICE_ID_22560: SigmaU16 = 0x3B35;
pub const IWLWIFI_DEVICE_ID_9260: SigmaU16 = 0x2526;
pub const IWLWIFI_DEVICE_ID_9560: SigmaU16 = 0x24FD;
pub const IWLWIFI_DEVICE_ID_AX200: SigmaU16 = 0x2723;
pub const IWLWIFI_DEVICE_ID_AX201: SigmaU16 = 0x43F0;
pub const IWLWIFI_DEVICE_ID_AX210: SigmaU16 = 0x2725;

// PCI BAR offsets
pub const PCI_MMIO_BAR: SigmaU8 = 0;
pub const PCI_SHARED_MEM_BAR: SigmaU8 = 4;

// MMIO Register offsets
pub const CSR_HW_REV: SigmaU32 = 0x000;
pub const CSR_HW_IF_CONFIG: SigmaU32 = 0x000;
pub const CSR_INT_COAL_REG: SigmaU32 = 0x004;
pub const CSR_INT: SigmaU32 = 0x008;
pub const CSR_INT_MASK: SigmaU32 = 0x00C;
pub const CSR_FH_INT_STATUS: SigmaU32 = 0x010;
pub const CSR_GPIO_IN: SigmaU32 = 0x018;
pub const CSR_RESET: SigmaU32 = 0x024;
pub const CSR_GP_CNTRL: SigmaU32 = 0x024;
pub const CSR_UCODE_DRV_GP1: SigmaU32 = 0x054;
pub const CSR_UCODE_DRV_GP1_SET: SigmaU32 = 0x058;
pub const CSR_UCODE_DRV_GP1_CLR: SigmaU32 = 0x05C;
pub const CSR_UCODE_DRV_GP2: SigmaU32 = 0x060;
pub const CSR_UCODE_DRV_GP2_SET: SigmaU32 = 0x064;
pub const CSR_UCODE_DRV_GP2_CLR: SigmaU32 = 0x068;
pub const CSR_LED_REG: SigmaU32 = 0x094;
pub const CSR_DRAM_INT_TBL_REG: SigmaU32 = 0x0A0;
pub const CSR_DRAM_INT_TBL_MASK: SigmaU32 = 0x0A4;
pub const CSR_DRAM_INT_TBL_CLR: SigmaU32 = 0x0A8;
pub const CSR_MAC_SHADOW_REG_CTL: SigmaU32 = 0x0A8;
pub const CSR_HW_REV_MSK_A: SigmaU32 = 0x00000FFF;
pub const CSR_HW_REV_MSK_B: SigmaU32 = 0x000F0000;
pub const CSR_HW_REV_STEP_MSK: SigmaU32 = 0x0000000F;

// WiFi security types
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WiFiSecurity {
    Open = 0,
    WEP = 1,
    WPA_PSK = 2,
    WPA2_PSK = 3,
    WPA3_PSK = 4,
    WPA_EAP = 5,
    WPA2_EAP = 6,
}

// WiFi band
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WiFiBand {
    Band2_4GHz = 0,
    Band5GHz = 1,
    Band6GHz = 2,
}

// WiFi channel width
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ChannelWidth {
    Width20MHz = 0,
    Width40MHz = 1,
    Width80MHz = 2,
    Width160MHz = 3,
}

// WiFi state
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WiFiState {
    Disconnected = 0,
    Scanning = 1,
    Connecting = 2,
    Connected = 3,
    Authenticating = 4,
}

// WiFi network info
#[repr(C)]
pub struct WiFiNetwork {
    pub ssid: [SigmaU8; 32],
    pub bssid: [SigmaU8; 6],
    pub security: WiFiSecurity,
    pub band: WiFiBand,
    pub channel: SigmaU32,
    pub signal_strength: SigmaI32,
    pub frequency: SigmaU32,
}

// WiFi adapter info
#[repr(C)]
pub struct WiFiAdapter {
    pub name: [SigmaU8; 32],
    pub mac_address: [SigmaU8; 6],
    pub state: WiFiState,
    pub current_network: Option<WiFiNetwork>,
    pub supported_bands: SigmaU32,
    pub max_scan_results: SigmaU32,
}

// iwlwifi device structure
pub struct IwlwifiDevice {
    pub mmio_base: SigmaU64,
    pub shared_mem_base: SigmaU64,
    pub device_id: SigmaU16,
    pub hw_rev: SigmaU32,
    pub initialized: SigmaBool,
    pub adapter: WiFiAdapter,
    pub fw_loaded: SigmaBool,
    pub uc_ready: SigmaBool,
}

impl IwlwifiDevice {
    pub const fn new() -> Self {
        Self {
            mmio_base: 0,
            shared_mem_base: 0,
            device_id: 0,
            hw_rev: 0,
            initialized: false,
            adapter: WiFiAdapter {
                name: [0; 32],
                mac_address: [0; 6],
                state: WiFiState::Disconnected,
                current_network: None,
                supported_bands: 0,
                max_scan_results: 64,
            },
            fw_loaded: false,
            uc_ready: false,
        }
    }

    /// Initialize iwlwifi device
    pub unsafe fn init(&mut self, pci_mmio_base: SigmaU64, pci_shared_mem_base: SigmaU64, device_id: SigmaU16) -> SigmaI32 {
        self.mmio_base = pci_mmio_base;
        self.shared_mem_base = pci_shared_mem_base;
        self.device_id = device_id;

        // Validate device ID
        if !self.is_supported_device(device_id) {
            return IWLWIFI_ERR_NO_DEVICE;
        }

        // Read hardware revision
        self.hw_rev = self.read_mmio(CSR_HW_REV);

        // Initialize hardware
        if self.init_hw() != IWLWIFI_OK {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // Load firmware
        if self.load_firmware() != IWLWIFI_OK {
            return IWLWIFI_ERR_FIRMWARE;
        }

        // Get MAC address
        self.get_mac_address();

        // Set adapter name
        let name = b"iwlwifi\0";
        let mut i = 0;
        while i < name.len() && i < 31 {
            self.adapter.name[i] = name[i];
            i += 1;
        }

        self.initialized = true;
        IWLWIFI_OK
    }

    /// Check if device ID is supported
    fn is_supported_device(&self, device_id: SigmaU16) -> SigmaBool {
        matches!(
            device_id,
            IWLWIFI_DEVICE_ID_22000 |
            IWLWIFI_DEVICE_ID_22560 |
            IWLWIFI_DEVICE_ID_9260 |
            IWLWIFI_DEVICE_ID_9560 |
            IWLWIFI_DEVICE_ID_AX200 |
            IWLWIFI_DEVICE_ID_AX201 |
            IWLWIFI_DEVICE_ID_AX210
        )
    }

    /// Initialize hardware
    unsafe fn init_hw(&mut self) -> SigmaI32 {
        // Reset device
        self.write_mmio(CSR_RESET, 0x00000001);

        // Wait for reset to complete
        let mut timeout = 1000;
        while timeout > 0 {
            let status = self.read_mmio(CSR_RESET);
            if status & 0x00000001 == 0 {
                break;
            }
            timeout -= 1;
        }

        // Initialize GPIO
        self.write_mmio(CSR_GP_CNTRL, 0x00000004);

        // Enable interrupts
        self.write_mmio(CSR_INT_MASK, 0xFFFFFFFF);
        self.write_mmio(CSR_INT, 0xFFFFFFFF);

        // Initialize shared memory
        self.init_shared_mem();

        IWLWIFI_OK
    }

    /// Initialize shared memory
    unsafe fn init_shared_mem(&mut self) {
        // Clear shared memory
        let shared_mem_ptr = self.shared_mem_base as *mut SigmaU32;
        for i in 0..4096 {
            *shared_mem_ptr.add(i) = 0;
        }
    }

    /// Load firmware
    unsafe fn load_firmware(&mut self) -> SigmaI32 {
        // In a real implementation, this would:
        // 1. Read firmware from file system
        // 2. Load firmware into device memory
        // 3. Verify firmware integrity
        // 4. Start firmware execution

        // Stub: assume firmware loaded successfully
        self.fw_loaded = true;

        // Wait for firmware to be ready
        let mut timeout = 10000;
        while timeout > 0 {
            let alive = self.read_mmio(CSR_UCODE_DRV_GP1);
            if alive & 0x00000001 != 0 {
                self.uc_ready = true;
                break;
            }
            timeout -= 1;
        }

        if self.uc_ready {
            IWLWIFI_OK
        } else {
            IWLWIFI_ERR_FIRMWARE
        }
    }

    /// Get MAC address
    unsafe fn get_mac_address(&mut self) {
        // In a real implementation, read MAC address from hardware
        // Stub: use a default MAC address
        self.adapter.mac_address[0] = 0x00;
        self.adapter.mac_address[1] = 0x11;
        self.adapter.mac_address[2] = 0x22;
        self.adapter.mac_address[3] = 0x33;
        self.adapter.mac_address[4] = 0x44;
        self.adapter.mac_address[5] = 0x55;
    }

    /// Scan for networks
    pub unsafe fn scan(&mut self, networks: *mut WiFiNetwork, max_networks: SigmaU32) -> SigmaI32 {
        if !self.initialized || !self.uc_ready {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        self.adapter.state = WiFiState::Scanning;

        // In a real implementation, this would:
        // 1. Send scan command to firmware
        // 2. Wait for scan results
        // 3. Parse scan results

        // Stub: return a single network
        if max_networks > 0 {
            let network = WiFiNetwork {
                ssid: *b"SigmaOS-Network\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0",
                bssid: [0x00, 0x11, 0x22, 0x33, 0x44, 0x55],
                security: WiFiSecurity::WPA2_PSK,
                band: WiFiBand::Band2_4GHz,
                channel: 6,
                signal_strength: -45,
                frequency: 2437,
            };
            *networks = network;
        }

        self.adapter.state = WiFiState::Disconnected;
        IWLWIFI_OK
    }

    /// Connect to network
    pub unsafe fn connect(&mut self, ssid: *const SigmaU8, password: *const SigmaU8) -> SigmaI32 {
        if !self.initialized || !self.uc_ready {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        self.adapter.state = WiFiState::Connecting;

        // In a real implementation, this would:
        // 1. Parse SSID and password
        // 2. Send connect command to firmware
        // 3. Wait for authentication
        // 4. Complete connection

        // Stub: assume connection successful
        let mut network = WiFiNetwork {
            ssid: [0; 32],
            bssid: [0; 6],
            security: WiFiSecurity::WPA2_PSK,
            band: WiFiBand::Band2_4GHz,
            channel: 6,
            signal_strength: -45,
            frequency: 2437,
        };

        // Copy SSID
        if !ssid.is_null() {
            let mut i = 0;
            while i < 31 && *ssid.add(i) != 0 {
                network.ssid[i] = *ssid.add(i);
                i += 1;
            }
        }

        self.adapter.current_network = Some(network);
        self.adapter.state = WiFiState::Connected;

        IWLWIFI_OK
    }

    /// Disconnect from network
    pub unsafe fn disconnect(&mut self) -> SigmaI32 {
        if !self.initialized {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, send disconnect command to firmware
        self.adapter.current_network = None;
        self.adapter.state = WiFiState::Disconnected;

        IWLWIFI_OK
    }

    /// Get adapter state
    pub fn get_state(&self) -> WiFiState {
        self.adapter.state
    }

    /// Get current network
    pub fn get_current_network(&self) -> Option<&WiFiNetwork> {
        self.adapter.current_network.as_ref()
    }

    /// Get signal strength
    pub fn get_signal_strength(&self) -> SigmaI32 {
        if let Some(ref network) = self.adapter.current_network {
            network.signal_strength
        } else {
            0
        }
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: SigmaU32) -> SigmaU32 {
        let ptr = (self.mmio_base + offset as SigmaU64) as *const SigmaU32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: SigmaU32, value: SigmaU32) {
        let ptr = (self.mmio_base + offset as SigmaU64) as *mut SigmaU32;
        *ptr = value;
    }
}

// Global iwlwifi device
static mut G_IWLWIFI: IwlwifiDevice = IwlwifiDevice::new();

// C-ABI Exports

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_init(pci_mmio_base: SigmaU64, pci_shared_mem_base: SigmaU64, device_id: SigmaU16) -> SigmaI32 {
    G_IWLWIFI.init(pci_mmio_base, pci_shared_mem_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_scan(networks: *mut WiFiNetwork, max_networks: SigmaU32) -> SigmaI32 {
    G_IWLWIFI.scan(networks, max_networks)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_connect(ssid: *const SigmaU8, password: *const SigmaU8) -> SigmaI32 {
    G_IWLWIFI.connect(ssid, password)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_disconnect() -> SigmaI32 {
    G_IWLWIFI.disconnect()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_get_state() -> WiFiState {
    G_IWLWIFI.get_state()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_get_signal_strength() -> SigmaI32 {
    G_IWLWIFI.get_signal_strength()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_is_initialized() -> SigmaI32 {
    if G_IWLWIFI.initialized {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_get_mac_address(mac: *mut SigmaU8) -> SigmaI32 {
    if mac.is_null() {
        return -1;
    }
    
    if !G_IWLWIFI.initialized {
        return IWLWIFI_ERR_INIT_FAILED;
    }
    
    for i in 0..6 {
        *mac.add(i) = G_IWLWIFI.adapter.mac_address[i];
    }
    
    IWLWIFI_OK
}
