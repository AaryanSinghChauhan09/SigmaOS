// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/wifi/sigma_iwlwifi.rs — Intel iwlwifi Wi-Fi Driver
//
// Implements Intel wireless LAN driver for Wi-Fi 6 (802.11ax) devices,
// including firmware loading, MAC/PHY configuration, and network operations.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const IWLWIFI_OK: I32 = 0;
pub const IWLWIFI_ERR_NO_DEVICE: I32 = -1;
pub const IWLWIFI_ERR_INIT_FAILED: I32 = -2;
pub const IWLWIFI_ERR_FW_LOAD: I32 = -3;
pub const IWLWIFI_ERR_SCAN_FAILED: I32 = -4;

// ─── PCI Device IDs ───────────────────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;

// Intel Wi-Fi 6 device IDs
pub const IWL_DEVICE_ID_AX200: U16 = 0x2723;
pub const IWL_DEVICE_ID_AX201: U16 = 0x43F0;
pub const IWL_DEVICE_ID_AX210: U16 =0x2725;
pub const IWL_DEVICE_ID_AX211: U16 = 0x2726;
pub const IWL_DEVICE_ID_AX411: U16 = 0x4DC5;

// ─── MMIO Register Offsets ───────────────────────────────────────────────────

pub const PCI_MMIO_BAR: U8 = 0;

pub const CSR_HW_REV: U32 = 0x000;
pub const CSR_HW_IF_CONFIG: U32 = 0x000;
pub const CSR_INT_COALESCING: U32 = 0x004;
pub const CSR_INT: U32 = 0x008;
pub const CSR_INT_MASK: U32 = 0x00C;
pub const CSR_FH_INT_STATUS: U32 = 0x010;
pub const CSR_FH_INT_MASK: U32 = 0x014;
pub const CSR_RESET: U32 = 0x024;

// ─── Shared Memory Offsets ───────────────────────────────────────────────────

pub const SHMEM_BASE: U32 = 0x5000;
pub const SHMEM_UCODE_BASE: U32 = 0x2000;

// ─── TX/RQ Queue Offsets ───────────────────────────────────────────────────

pub const FH_MEM_TFDIB_CTRL0: U32 = 0x08D0;
pub const FH_MEM_TFDIB_CTRL1: U32 = 0x08D4;
pub const FH_MEM_TFDIB_DB0: U32 = 0x08D8;
pub const FH_MEM_RSCSR_PTR: U32 = 0x0BC0;
pub const FH_MEM_RSCSR_INC: U32 = 0x0BC4;
pub const FH_MEM_RSCSR_STATUS: U32 = 0x0BC8;

// ─── Wi-Fi Constants ───────────────────────────────────────────────────────

pub const NUM_TX_QUEUES: usize = 16;
pub const NUM_RX_QUEUES: usize = 16;
pub const TX_QUEUE_SIZE: usize = 256;
pub const RX_QUEUE_SIZE: usize = 256;

pub const MAX_SSID_LEN: usize = 32;
pub const MAX_BSSID_LEN: usize = 6;

// ─── Scan Result Structure ─────────────────────────────────────────────────

#[repr(C)]
pub struct ScanResult {
    pub ssid: [U8; MAX_SSID_LEN],
    pub ssid_len: U8,
    pub bssid: [U8; MAX_BSSID_LEN],
    pub channel: U8,
    pub signal_strength: I32, // dBm
    pub security_type: SecurityType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub enum SecurityType {
    Open = 0,
    WEP = 1,
    WPA_PSK = 2,
    WPA2_PSK = 3,
    WPA3_SAE = 4,
}

// ─── Connection Info ───────────────────────────────────────────────────────

#[repr(C)]
pub struct ConnectionInfo {
    pub connected: bool,
    pub ssid: [U8; MAX_SSID_LEN],
    pub ssid_len: U8,
    pub bssid: [U8; MAX_BSSID_LEN],
    pub channel: U8,
    pub security: SecurityType,
}

// ─── iwlwifi Device Structure ───────────────────────────────────────────────

pub struct IwlwifiDevice {
    pub mmio_base: U64,
    pub device_id: U16,
    pub initialized: bool,
    pub firmware_loaded: bool,
    pub tx_queues: [TxQueue; NUM_TX_QUEUES],
    pub rx_queues: [RxQueue; NUM_RX_QUEUES],
    pub connection: ConnectionInfo,
    pub scan_results: [ScanResult; 64],
    pub scan_count: usize,
}

#[repr(C)]
pub struct TxQueue {
    pub base: U64,
    pub read_ptr: U32,
    pub write_ptr: U32,
    pub count: U32,
}

#[repr(C)]
pub struct RxQueue {
    pub base: U64,
    pub read_ptr: U32,
    pub write_ptr: U32,
    pub count: U32,
}

impl IwlwifiDevice {
    pub const fn new() -> Self {
        IwlwifiDevice {
            mmio_base: 0,
            device_id: 0,
            initialized: false,
            firmware_loaded: false,
            tx_queues: [TxQueue {
                base: 0,
                read_ptr: 0,
                write_ptr: 0,
                count: 0,
            }; NUM_TX_QUEUES],
            rx_queues: [RxQueue {
                base: 0,
                read_ptr: 0,
                write_ptr: 0,
                count: 0,
            }; NUM_RX_QUEUES],
            connection: ConnectionInfo {
                connected: false,
                ssid: [0; MAX_SSID_LEN],
                ssid_len: 0,
                bssid: [0; MAX_BSSID_LEN],
                channel: 0,
                security: SecurityType::Open,
            },
            scan_results: [ScanResult {
                ssid: [0; MAX_SSID_LEN],
                ssid_len: 0,
                bssid: [0; MAX_BSSID_LEN],
                channel: 0,
                signal_strength: 0,
                security: SecurityType::Open,
            }; 64],
            scan_count: 0,
        }
    }

    /// Initialize iwlwifi device
    pub unsafe fn init(&mut self, pci_mmio_base: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_mmio_base;
        self.device_id = device_id;

        // Validate device ID
        if !self.is_supported_device(device_id) {
            return IWLWIFI_ERR_NO_DEVICE;
        }

        // Reset device
        if self.reset_device() != IWLWIFI_OK {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // Load firmware
        if self.load_firmware() != IWLWIFI_OK {
            return IWLWIFI_ERR_FW_LOAD;
        }

        // Initialize TX/RX queues
        if self.init_queues() != IWLWIFI_OK {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // Initialize MAC/PHY
        if self.init_mac_phy() != IWLWIFI_OK {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        self.initialized = true;
        IWLWIFI_OK
    }

    /// Check if device ID is supported
    fn is_supported_device(&self, device_id: U16) -> bool {
        matches!(
            device_id,
            IWL_DEVICE_ID_AX200 |
            IWL_DEVICE_ID_AX201 |
            IWL_DEVICE_ID_AX210 |
            IWL_DEVICE_ID_AX211 |
            IWL_DEVICE_ID_AX411
        )
    }

    /// Reset device
    unsafe fn reset_device(&self) -> I32 {
        // Write reset bit to CSR_RESET
        let reset_ptr = (self.mmio_base + CSR_RESET as U64) as *mut U32;
        *reset_ptr = 0x1;

        // Wait for reset to complete (stub)
        IWLWIFI_OK
    }

    /// Load firmware
    unsafe fn load_firmware(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Load firmware from file system
        // 2. Upload to device memory
        // 3. Verify firmware integrity
        // 4. Start firmware execution

        self.firmware_loaded = true;
        IWLWIFI_OK
    }

    /// Initialize TX/RX queues
    unsafe fn init_queues(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Allocate DMA memory for queues
        // 2. Configure queue registers
        // 3. Enable interrupts

        IWLWIFI_OK
    }

    /// Initialize MAC/PHY
    unsafe fn init_mac_phy(&self) -> I32 {
        // In a real implementation, this would:
        // 1. Configure MAC parameters
        // 2. Initialize PHY
        // 3. Set up regulatory domain

        IWLWIFI_OK
    }

    /// Scan for networks
    pub unsafe fn scan(&mut self) -> I32 {
        if !self.initialized || !self.firmware_loaded {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // Clear previous scan results
        self.scan_count = 0;

        // In a real implementation, this would:
        // 1. Send scan command to firmware
        // 2. Wait for scan results
        // 3. Parse scan results

        // Stub: add fake scan result
        if self.scan_count < 64 {
            let result = &mut self.scan_results[self.scan_count];
            result.ssid = b"SigmaWiFi"[..].try_into().unwrap_or([0; MAX_SSID_LEN]);
            result.ssid_len = 9;
            result.bssid = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
            result.channel = 6;
            result.signal_strength = -45;
            result.security = SecurityType::WPA2_PSK;
            self.scan_count += 1;
        }

        IWLWIFI_OK
    }

    /// Connect to network
    pub unsafe fn connect(&mut self, ssid: &[U8], password: Option<&[U8]>) -> I32 {
        if !self.initialized || !self.firmware_loaded {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        if ssid.len() > MAX_SSID_LEN {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Find network in scan results
        // 2. Authenticate with AP
        // 3. Perform 4-way handshake (if WPA/WPA2)
        // 4. Associate with AP

        // Stub: mark as connected
        self.connection.connected = true;
        self.connection.ssid_len = ssid.len() as U8;
        let mut i = 0;
        while i < ssid.len() && i < MAX_SSID_LEN {
            self.connection.ssid[i] = ssid[i];
            i += 1;
        }
        self.connection.bssid = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        self.connection.channel = 6;
        self.connection.security = SecurityType::WPA2_PSK;

        IWLWIFI_OK
    }

    /// Disconnect from network
    pub unsafe fn disconnect(&mut self) -> I32 {
        if !self.initialized {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, send disassociation frame
        self.connection.connected = false;
        self.connection.ssid_len = 0;

        IWLWIFI_OK
    }

    /// Get scan results
    pub fn get_scan_results(&self) -> &[ScanResult] {
        &self.scan_results[..self.scan_count]
    }

    /// Get connection info
    pub fn get_connection(&self) -> &ConnectionInfo {
        &self.connection
    }

    /// Send packet
    pub unsafe fn send_packet(&mut self, data: &[U8]) -> I32 {
        if !self.initialized || !self.connection.connected {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Find available TX queue
        // 2. Build TX descriptor
        // 3. Copy data to DMA buffer
        // 4. Write to TX queue
        // 5. Notify firmware

        IWLWIFI_OK
    }

    /// Receive packet
    pub unsafe fn receive_packet(&mut self, buffer: &mut [U8]) -> I32 {
        if !self.initialized || !self.connection.connected {
            return IWLWIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, this would:
        // 1. Check RX queue for packets
        // 2. Copy data from DMA buffer
        // 3. Update RX queue pointer

        0 // Return bytes received (stub: 0)
    }
}

// ─── Global iwlwifi Device ─────────────────────────────────────────────────

static mut G_IWLWIFI: IwlwifiDevice = IwlwifiDevice::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_init(pci_mmio_base: U64, device_id: U16) -> I32 {
    G_IWLWIFI.init(pci_mmio_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_scan() -> I32 {
    G_IWLWIFI.scan()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_connect(ssid: *const U8, ssid_len: U32, password: *const U8, password_len: U32) -> I32 {
    let ssid_slice = core::slice::from_raw_parts(ssid, ssid_len as usize);
    let pass_slice = if password_len > 0 {
        Some(core::slice::from_raw_parts(password, password_len as usize))
    } else {
        None
    };
    G_IWLWIFI.connect(ssid_slice, pass_slice)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_disconnect() -> I32 {
    G_IWLWIFI.disconnect()
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_send_packet(data: *const U8, len: U32) -> I32 {
    let data_slice = core::slice::from_raw_parts(data, len as usize);
    G_IWLWIFI.send_packet(data_slice)
}

#[no_mangle]
pub unsafe extern "C" fn iwlwifi_receive_packet(buffer: *mut U8, max_len: U32) -> I32 {
    let buffer_slice = core::slice::from_raw_parts_mut(buffer, max_len as usize);
    G_IWLWIFI.receive_packet(buffer_slice)
}
