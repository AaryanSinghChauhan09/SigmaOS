// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/net/sigma_wifi_driver.rs — Wi-Fi driver framework (SDF)
// Supports: iwlwifi (Intel), ath9k/ath11k (Qualcomm), mt76 (MediaTek), rtw89 (Realtek)
// Architecture: Sovereign Driver Framework (SDF) — probe → init → shutdown
//
// Inspired by: Linux drivers/net/wireless/ (cleanroom study)
// Language: Rust (#![no_std])

#![no_std]
#![allow(dead_code)]

// ── Wi-Fi modes ────────────────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum WifiMode { Monitor, Station, AccessPoint, Mesh }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum WifiSecurity { Open, WPA2, WPA3, WPA3_SAE }

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum WifiState { Down, Scanning, Connecting, Connected, Disconnecting }

// ── Wi-Fi driver chip IDs ──────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(u16)]
pub enum WifiChip {
    IntelAX200   = 0x2723,   // Wi-Fi 6 AX200
    IntelAX201   = 0x02F0,   // Wi-Fi 6 AX201
    IntelAX210   = 0x2725,   // Wi-Fi 6E AX210
    QualcommQCA6390 = 0x1101,
    QualcommWCN6855 = 0x1103,
    MediaTekMT7921 = 0x7961,
    MediaTekMT7922 = 0x7922,
    RealtekRTW8822CE = 0xC822,
    RealtekRTW8852AE = 0x8852,
    Unknown = 0xFFFF,
}

impl WifiChip {
    pub fn from_pci_device(vendor: u16, device: u16) -> Self {
        match (vendor, device) {
            (0x8086, 0x2723) => Self::IntelAX200,
            (0x8086, 0x02F0) => Self::IntelAX201,
            (0x8086, 0x2725) => Self::IntelAX210,
            (0x168C, 0x0034) => Self::QualcommQCA6390,
            (0x17CB, 0x1103) => Self::QualcommWCN6855,
            (0x14C3, 0x7961) => Self::MediaTekMT7921,
            (0x14C3, 0x7922) => Self::MediaTekMT7922,
            (0x10EC, 0xC822) => Self::RealtekRTW8822CE,
            (0x10EC, 0x8852) => Self::RealtekRTW8852AE,
            _ => Self::Unknown,
        }
    }
    pub fn driver_name(self) -> &'static str {
        match self {
            Self::IntelAX200 | Self::IntelAX201 | Self::IntelAX210 => "iwlwifi",
            Self::QualcommQCA6390 | Self::QualcommWCN6855 => "ath11k",
            Self::MediaTekMT7921 | Self::MediaTekMT7922 => "mt76",
            Self::RealtekRTW8822CE | Self::RealtekRTW8852AE => "rtw89",
            Self::Unknown => "unknown",
        }
    }
    pub fn firmware_name(self) -> &'static str {
        match self {
            Self::IntelAX200   => "iwlwifi-cc-a0-72.ucode",
            Self::IntelAX201   => "iwlwifi-QuZ-a0-hr-b0-72.ucode",
            Self::IntelAX210   => "iwlwifi-ty-a0-gf-a0-72.ucode",
            Self::QualcommQCA6390 => "ath11k/QCA6390/hw2.0/amss.bin",
            Self::QualcommWCN6855 => "ath11k/WCN6855/hw2.0/amss.bin",
            Self::MediaTekMT7921 => "mt7921_firmware.bin",
            Self::MediaTekMT7922 => "mt7922_firmware.bin",
            Self::RealtekRTW8822CE => "rtw8822c_fw.bin",
            Self::RealtekRTW8852AE => "rtw8852a_fw.bin",
            Self::Unknown => "",
        }
    }
}

// ── BSS scan result ────────────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct BssEntry {
    pub bssid:    [u8; 6],
    pub ssid:     [u8; 32],
    pub ssid_len: u8,
    pub channel:  u8,
    pub rssi_dbm: i8,
    pub security: WifiSecurity,
    pub freq_mhz: u16,
}

impl BssEntry {
    pub const fn zeroed() -> Self {
        Self { bssid:[0;6], ssid:[0;32], ssid_len:0, channel:0,
               rssi_dbm:0, security: WifiSecurity::Open, freq_mhz:0 }
    }
    pub fn ssid_str(&self) -> &[u8] {
        &self.ssid[..self.ssid_len as usize]
    }
}

// ── Association parameters ─────────────────────────────────────────────────
#[derive(Copy, Clone)]
pub struct AssocParams {
    pub ssid:     [u8; 32],
    pub ssid_len: u8,
    pub bssid:    [u8; 6],
    pub password: [u8; 64],
    pub pass_len: u8,
    pub security: WifiSecurity,
}

// ── Wi-Fi driver trait (SDF interface) ─────────────────────────────────────
pub trait WifiDriver: Send + Sync {
    fn name(&self)   -> &'static str;
    fn chip(&self)   -> WifiChip;
    fn probe(&mut self, pci_bar: u64) -> bool;
    fn init(&mut self)   -> bool;
    fn shutdown(&mut self);
    fn scan(&mut self)   -> usize;             // returns number of BSS found
    fn bss_at(&self, idx: usize) -> Option<BssEntry>;
    fn connect(&mut self, params: &AssocParams) -> bool;
    fn disconnect(&mut self);
    fn state(&self) -> WifiState;
    fn rssi(&self)  -> i8;
    fn mac_addr(&self) -> [u8; 6];
    fn set_mode(&mut self, mode: WifiMode) -> bool;
    fn set_channel(&mut self, channel: u8) -> bool;
    fn tx_frame(&mut self, data: &[u8]) -> bool;
}

// ── Generic Wi-Fi device (hardware-agnostic) ───────────────────────────────
pub struct WifiDevice {
    pub chip:       WifiChip,
    pub state:      WifiState,
    pub mode:       WifiMode,
    pub mac:        [u8; 6],
    pub channel:    u8,
    pub rssi_dbm:   i8,
    pub scan_results: [BssEntry; 32],
    pub scan_count:   usize,
    pub pci_bar:    u64,   // MMIO base address
    pub irq:        u8,
    pub fw_loaded:  bool,
    pub assoc_bssid: [u8; 6],
    pub assoc_ssid:  [u8; 32],
    pub assoc_len:   u8,
}

impl WifiDevice {
    pub const fn new(chip: WifiChip) -> Self {
        Self {
            chip, state: WifiState::Down,
            mode: WifiMode::Station,
            mac: [0u8; 6], channel: 1, rssi_dbm: -80,
            scan_results: [BssEntry::zeroed(); 32],
            scan_count: 0, pci_bar: 0, irq: 0,
            fw_loaded: false,
            assoc_bssid: [0u8;6], assoc_ssid: [0u8;32], assoc_len: 0,
        }
    }

    /// Load firmware blob — called during init
    pub fn load_firmware(&mut self) -> bool {
        let fw_name = self.chip.firmware_name();
        if fw_name.is_empty() { return false; }
        // In production: read from sigma-fw shard via sigma-bus
        // For now: mark as loaded if firmware name is known
        self.fw_loaded = true;
        true
    }

    /// Write to device MMIO register
    pub unsafe fn mmio_write32(&self, offset: u32, val: u32) {
        if self.pci_bar == 0 { return; }
        let addr = (self.pci_bar + offset as u64) as *mut u32;
        core::ptr::write_volatile(addr, val);
    }

    /// Read from device MMIO register
    pub unsafe fn mmio_read32(&self, offset: u32) -> u32 {
        if self.pci_bar == 0 { return 0; }
        let addr = (self.pci_bar + offset as u64) as *const u32;
        core::ptr::read_volatile(addr)
    }

    /// Add a scan result (called by chip-specific scan routine)
    pub fn add_scan_result(&mut self, entry: BssEntry) {
        if self.scan_count < 32 {
            self.scan_results[self.scan_count] = entry;
            self.scan_count += 1;
        }
    }

    /// WPA3-SAE Dragonfly key exchange stub
    pub fn wpa3_sae_commit(&self, password: &[u8], ssid: &[u8]) -> [u8; 32] {
        // Dragonfly: H2E (Hash to Element) on SSID + password
        // Production: full P-256 curve operations
        let mut out = [0u8; 32];
        for (i, (&p, &s)) in password.iter().zip(ssid.iter()).enumerate() {
            if i >= 32 { break; }
            out[i] = p ^ s ^ (i as u8);
        }
        out
    }
}

impl WifiDriver for WifiDevice {
    fn name(&self)  -> &'static str { self.chip.driver_name() }
    fn chip(&self)  -> WifiChip     { self.chip }
    fn state(&self) -> WifiState    { self.state }
    fn rssi(&self)  -> i8           { self.rssi_dbm }
    fn mac_addr(&self) -> [u8; 6]   { self.mac }

    fn probe(&mut self, pci_bar: u64) -> bool {
        self.pci_bar = pci_bar;
        self.chip != WifiChip::Unknown
    }

    fn init(&mut self) -> bool {
        if !self.load_firmware() { return false; }
        self.state = WifiState::Down;
        // Chip-specific init would go here via MMIO
        true
    }

    fn shutdown(&mut self) {
        self.disconnect();
        self.state = WifiState::Down;
        self.fw_loaded = false;
    }

    fn scan(&mut self) -> usize {
        if self.state == WifiState::Down { return 0; }
        self.scan_count = 0;
        // In production: send SCAN command to firmware via CMD ring
        // Firmware posts scan results via notification ring
        self.scan_count
    }

    fn bss_at(&self, idx: usize) -> Option<BssEntry> {
        if idx < self.scan_count { Some(self.scan_results[idx]) } else { None }
    }

    fn connect(&mut self, params: &AssocParams) -> bool {
        if self.state != WifiState::Down { return false; }
        self.state = WifiState::Connecting;
        self.assoc_ssid[..params.ssid_len as usize]
            .copy_from_slice(&params.ssid[..params.ssid_len as usize]);
        self.assoc_len  = params.ssid_len;
        self.assoc_bssid = params.bssid;
        // In production: send ASSOCIATE command to firmware
        // After 4-way handshake completes: state = Connected
        self.state = WifiState::Connected;
        true
    }

    fn disconnect(&mut self) {
        if self.state == WifiState::Connected {
            self.state = WifiState::Disconnecting;
            // Send DEAUTH frame
            self.state = WifiState::Down;
        }
    }

    fn set_mode(&mut self, mode: WifiMode) -> bool {
        if self.state != WifiState::Down { return false; }
        self.mode = mode; true
    }

    fn set_channel(&mut self, channel: u8) -> bool {
        if channel == 0 || channel > 177 { return false; }
        self.channel = channel; true
    }

    fn tx_frame(&mut self, _data: &[u8]) -> bool {
        self.state == WifiState::Connected
    }
}

// ── SDF registration ───────────────────────────────────────────────────────
#[no_mangle]
pub extern "C" fn sigma_wifi_probe(vendor: u16, device: u16, pci_bar: u64) -> *mut WifiDevice {
    let chip = WifiChip::from_pci_device(vendor, device);
    if chip == WifiChip::Unknown { return core::ptr::null_mut(); }
    // In production: allocate from slab allocator
    let _ = (chip, pci_bar);
    core::ptr::null_mut()
}

#[no_mangle]
pub extern "C" fn sigma_wifi_connect(
    _drv: *mut WifiDevice, _ssid: *const u8, _ssid_len: u8,
    _pass: *const u8, _pass_len: u8, _security: u8,
) -> bool { false }
