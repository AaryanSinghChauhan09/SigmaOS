// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/wifi/mt7921.rs — MediaTek MT7921 Wi-Fi Driver
//
// Implements the MediaTek MT7921 Wi-Fi driver.
// Supports MT7921 802.11ax (Wi-Fi 6) chipsets.
// Based on Linux kernel mt7921 driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::wifi_device_base::{WifiDevice, WifiMode, WifiBand, WifiChannel, WifiSecurity, WifiStats, WIFI_OK, WIFI_ERR_NO_DEVICE, WIFI_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── MediaTek Vendor ID ─────────────────────────────

pub const MEDIATEK_VENDOR_ID: U16 = 0x14C3;

// ─── MT7921 Device IDs ─────────────────────────

pub const MT7921_DEVICE_ID: U16 = 0x7961;
pub const MT7921S_DEVICE_ID: U16 = 0x0608;
pub const MT7921K_DEVICE_ID: U16 = 0x0616;

// ─── MT7921 Register Offsets ─────────────────────

pub const MT_WFDMA0_GLO_CFG: U32 = 0x0020;
pub const MT_WFDMA0_RST_DTX_PTR: U32 = 0x002C;
pub const MT_WFDMA0_HOST_INT_STA: U32 = 0x00A0;
pub const MT_WFDMA0_HOST_INT_ENA: U32 = 0x00A4;
pub const MT_WFDMA0_WPDMA_GLO_CFG: U32 = 0x0200;
pub const MT_WFDMA0_WPDMA_RST_IDX: U32 = 0x0208;
pub const MT_WFDMA0_TX_RING0_BASE: U32 = 0x0300;
pub const MT_WFDMA0_TX_RING0_CNT: U32 = 0x0304;
pub const MT_WFDMA0_RX_RING0_BASE: U32 = 0x0400;
pub const MT_WFDMA0_RX_RING0_CNT: U32 = 0x0404;
pub const MT_WFDMA0_TX_RING1_BASE: U32 = 0x0500;
pub const MT_WFDMA0_TX_RING1_CNT: U32 = 0x0504;

// ─── MT7921 WFDMA Configuration ─────────────────

pub const MT_WFDMA_GLO_CFG_TX_DMA_EN: U32 = 0x00000001;
pub const MT_WFDMA_GLO_CFG_RX_DMA_EN: U32 = 0x00000002;
pub const MT_WFDMA_GLO_CFG_OMIT_TX_INFO: U32 = 0x00000004;
pub const MT_WFDMA_GLO_CFG_OMIT_RX_INFO: U32 = 0x00000008;
pub const MT_WFDMA_GLO_CFG_OMIT_RX_INFO_PFET2: U32 = 0x00000010;

// ─── MT7921 Wi-Fi Structure ─────────────────────

pub struct Mt7921Device {
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

impl Mt7921Device {
    pub const fn new() -> Self {
        Mt7921Device {
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

    /// Initialize MT7921 device
    fn init_mt7921(&mut self, pci_bar: U64, device_id: U16, vendor_id: U16) -> I32 {
        self.mmio_base = pci_bar;
        self.device_id = device_id;
        self.vendor_id = vendor_id;

        unsafe {
            // Reset WFDMA
            let mut glo_cfg = self.read_mmio(MT_WFDMA0_GLO_CFG);
            glo_cfg &= !(MT_WFDMA_GLO_CFG_TX_DMA_EN | MT_WFDMA_GLO_CFG_RX_DMA_EN);
            self.write_mmio(MT_WFDMA0_GLO_CFG, glo_cfg);

            // Set up TX/RX rings
            self.tx_ring_base = 0x1000000;
            self.rx_ring_base = 0x2000000;

            self.write_mmio(MT_WFDMA0_TX_RING0_BASE, self.tx_ring_base as U32);
            self.write_mmio(MT_WFDMA0_TX_RING0_CNT, self.ring_size);
            self.write_mmio(MT_WFDMA0_RX_RING0_BASE, self.rx_ring_base as U32);
            self.write_mmio(MT_WFDMA0_RX_RING0_CNT, self.ring_size);

            // Enable DMA
            glo_cfg = self.read_mmio(MT_WFDMA0_GLO_CFG);
            glo_cfg |= MT_WFDMA_GLO_CFG_TX_DMA_EN | MT_WFDMA_GLO_CFG_RX_DMA_EN;
            self.write_mmio(MT_WFDMA0_GLO_CFG, glo_cfg);
        }

        self.initialized = true;
        self.enabled = true;

        WIFI_OK
    }
}

// ─── Implement WifiDevice Trait ─────────────────

impl WifiDevice for Mt7921Device {
    fn init(&mut self, pci_bar: U64, device_id: U16) -> I32 {
        let vendor_id = MEDIATEK_VENDOR_ID;
        self.init_mt7921(pci_bar, device_id, vendor_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        "MediaTek MT7921 Wi-Fi 6 Adapter"
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

// ─── Global MT7921 Device ─────────────────────────

static mut G_MT7921: Mt7921Device = Mt7921Device::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn mt7921_init(pci_bar: U64, device_id: U16) -> I32 {
    G_MT7921.init(pci_bar, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn mt7921_is_initialized() -> I32 {
    if G_MT7921.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn mt7921_shutdown() -> I32 {
    G_MT7921.shutdown()
}
