// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/wifi/rtw88.rs — Realtek RTW88 Wi-Fi Driver
//
// Implements the Realtek RTW88 Wi-Fi driver.
// Supports RTW88 802.11ac (Wi-Fi 5) chipsets.
// Based on Linux kernel rtw88 driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::wifi_device_base::{WifiDevice, WifiMode, WifiBand, WifiChannel, WifiSecurity, WifiStats, WIFI_OK, WIFI_ERR_NO_DEVICE, WIFI_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Realtek Vendor ID ─────────────────────────────

pub const REALTEK_VENDOR_ID: U16 = 0x10EC;

// ─── RTW88 Device IDs ─────────────────────────

pub const RTW8822B_DEVICE_ID: U16 = 0xB822;
pub const RTW8822C_DEVICE_ID: U16 = 0xC822;
pub const RTW8821C_DEVICE_ID: U16 = 0xC821;
pub const RTW8822BE_DEVICE_ID: U16 = 0xB82B;
pub const RTW8822CE_DEVICE_ID: U16 = 0xC82B;

// ─── RTW88 Register Offsets ─────────────────────

pub const RTW_SYS_CFG: U32 = 0x0000;
pub const RTW_CR: U32 = 0x0010;
pub const RTW_HIMR: U32 = 0x00B0;
pub const RTW_HISR: U32 = 0x00B4;
pub const RTW_HIMRE: U32 = 0x00B8;
pub const RTW_HISRE: U32 = 0x00BC;
pub const RTW_CPWM: U32 = 0x00C7;
pub const RTW_FW_START: U32 = 0x0080;
pub const RTW_MCU: U32 = 0x00C0;
pub const RTW_DDMA_CH0CTRL: U32 = 0x0100;
pub const RTW_DDMA_CH0ADDR: U32 = 0x0104;
pub const RTW_DDMA_CH0CTRL1: U32 = 0x0108;
pub const RTW_HDMA_CH0CTRL: U32 = 0x0200;
pub const RTW_HDMA_CH0ADDR: U32 = 0x0204;
pub const RTW_HDMA_CH0CTRL1: U32 = 0x0208;
pub const RTW_HDMA_CH0CTRL2: U32 = 0x020C;
pub const RTW_HDMA_CH0CTRL3: U32 = 0x0210;
pub const RTW_RXPKT_NUM: U32 = 0x0270;
pub const RTW_FIFOPAGE: U32 = 0x0204;
pub const RTW_TXPKT_NUM: U32 = 0x0274;

// ─── RTW88 Control Register Bits ─────────────────

pub const RTW_CR_MACTEST: U32 = 0x00000001;
pub const RTW_CR_MACTEST_NORMAL: U32 = 0x00000002;
pub const RTW_CR_MACTEST_EN: U32 = 0x00000004;
pub const RTW_CR_MACTEST_MAC1: U32 = 0x00000008;
pub const RTW_CR_MACTEST_MAC2: U32 = 0x00000010;
pub const RTW_CR_MACTEST_MAC3: U32 = 0x00000020;
pub const RTW_CR_MACTEST_MAC4: U32 = 0x00000040;
pub const RTW_CR_MACTEST_MAC5: U32 = 0x00000080;
pub const RTW_CR_MACTEST_MAC6: U32 = 0x00000100;
pub const RTW_CR_MACTEST_MAC7: U32 = 0x00000200;
pub const RTW_CR_MACTEST_MAC8: U32 = 0x00000400;
pub const RTW_CR_MACTEST_MAC9: U32 = 0x00000800;
pub const RTW_CR_MACTEST_MAC10: U32 = 0x00001000;
pub const RTW_CR_MACTEST_MAC11: U32 = 0x00002000;
pub const RTW_CR_MACTEST_MAC12: U32 = 0x00004000;
pub const RTW_CR_MACTEST_MAC13: U32 = 0x00008000;
pub const RTW_CR_MACTEST_MAC14: U32 = 0x00010000;
pub const RTW_CR_MACTEST_MAC15: U32 = 0x00020000;
pub const RTW_CR_MACTEST_MAC16: U32 = 0x00040000;
pub const RTW_CR_MACTEST_MAC17: U32 = 0x00080000;
pub const RTW_CR_MACTEST_MAC18: U32 = 0x00100000;
pub const RTW_CR_MACTEST_MAC19: U32 = 0x00200000;
pub const RTW_CR_MACTEST_MAC20: U32 = 0x00400000;
pub const RTW_CR_MACTEST_MAC21: U32 = 0x00800000;
pub const RTW_CR_MACTEST_MAC22: U32 = 0x01000000;
pub const RTW_CR_MACTEST_MAC23: U32 = 0x02000000;
pub const RTW_CR_MACTEST_MAC24: U32 = 0x04000000;
pub const RTW_CR_MACTEST_MAC25: U32 = 0x08000000;
pub const RTW_CR_MACTEST_MAC26: U32 = 0x10000000;
pub const RTW_CR_MACTEST_MAC27: U32 = 0x20000000;
pub const RTW_CR_MACTEST_MAC28: U32 = 0x40000000;
pub const RTW_CR_MACTEST_MAC29: U32 = 0x80000000;

// ─── RTW88 Wi-Fi Structure ─────────────────────

pub struct Rtw88Device {
    pub mmio_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub enabled: bool,
    pub current_mode: WifiMode,
    pub current_band: WifiBand,
    pub current_channel: WifiChannel,
    pub security: WifiSecurity,
    pub stats: WifiStats,
    pub tx_ring_base: U64,
    pub rx_ring_base: U64,
    pub ring_size: U32,
}

impl Rtw88Device {
    pub const fn new() -> Self {
        Rtw88Device {
            mmio_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            enabled: false,
            current_mode: WifiMode::Station,
            current_band: WifiBand::FiveGHz,
            current_channel: WifiChannel::new(),
            security: WifiSecurity::WPA2,
            stats: WifiStats::new(),
            tx_ring_base: 0,
            rx_ring_base: 0,
            ring_size: 256,
        }
    }

    /// Read MMIO register
    unsafe fn read_mmio(&self, offset: U32) -> U32 {
        let ptr = (self.mmio_base + offset as U64) as *const U32;
        *ptr
    }

    /// Write MMIO register
    unsafe fn write_mmio(&self, offset: U32, value: U32) {
        let ptr = (self.mmio_base + offset as U64) as *mut U32;
        *ptr = value
    }

    /// Initialize RTW88 device
    fn init_rtw88(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        unsafe {
            // Reset MAC
            let mut cr = self.read_mmio(RTW_CR);
            cr |= RTW_CR_MACTEST_NORMAL;
            self.write_mmio(RTW_CR, cr);

            // Set up TX/RX rings
            self.tx_ring_base = 0x1000000;
            self.rx_ring_base = 0x2000000;

            self.write_mmio(RTW_HDMA_CH0ADDR, self.tx_ring_base as U32);
            self.write_mmio(RTW_HDMA_CH0CTRL1, self.ring_size);
            self.write_mmio(RTW_DDMA_CH0ADDR, self.rx_ring_base as U32);
            self.write_mmio(RTW_DDMA_CH0CTRL1, self.ring_size);

            // Enable DMA
            let mut ctrl = self.read_mmio(RTW_HDMA_CH0CTRL);
            ctrl |= 0x01; // Enable
            self.write_mmio(RTW_HDMA_CH0CTRL, ctrl);
        }

        self.initialized = true;
        self.enabled = true;

        WIFI_OK
    }
}

// ─── Implement WifiDevice Trait ─────────────────

impl WifiDevice for Rtw88Device {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = REALTEK_VENDOR_ID;
        self.init_rtw88(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "Realtek RTW88 Wi-Fi 5 Adapter"
    }

    fn enable(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.enabled = true;
        WIFI_OK
    }

    fn disable(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.enabled = false;
        WIFI_OK
    }

    fn set_mode(&mut self, mode: WifiMode) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.current_mode = mode;
        WIFI_OK
    }

    fn get_mode(&self) -> WifiMode {
        self.current_mode
    }

    fn set_band(&mut self, band: WifiBand) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.current_band = band;
        WIFI_OK
    }

    fn get_band(&self) -> WifiBand {
        self.current_band
    }

    fn set_channel(&mut self, channel: WifiChannel) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.current_channel = channel;
        WIFI_OK
    }

    fn get_channel(&self) -> WifiChannel {
        self.current_channel
    }

    fn set_security(&mut self, security: WifiSecurity) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        self.security = security;
        WIFI_OK
    }

    fn get_security(&self) -> WifiSecurity {
        self.security
    }

    fn scan(&mut self) -> I32 {
        if !self.initialized || !self.enabled {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, perform Wi-Fi scan
        WIFI_OK
    }

    fn connect(&mut self, ssid: *const U8, password: *const U8) -> I32 {
        if !self.initialized || !self.enabled {
            return WIFI_ERR_INIT_FAILED;
        }

        // In a real implementation, connect to AP
        WIFI_OK
    }

    fn disconnect(&mut self) -> I32 {
        if !self.initialized || !self.enabled {
            return WIFI_ERR_INIT_FAILED;
        }

        WIFI_OK
    }

    fn get_stats(&self) -> WifiStats {
        self.stats
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return WIFI_ERR_INIT_FAILED;
        }

        WIFI_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.disable();
        self.initialized = false;
        WIFI_OK
    }
}

// ─── Global RTW88 Device ─────────────────────────

static mut G_RTW88: Rtw88Device = Rtw88Device::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn rtw88_init(pci_bar: U64, device_id: U16) -> I32 {
    G_RTW88.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn rtw88_is_initialized() -> I32 {
    if G_RTW88.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn rtw88_shutdown() -> I32 {
    G_RTW88.shutdown()
}
