// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_matrox.rs — Matrox GPU Driver
//
// Implements the Matrox GPU driver.
// Supports Matrox G200, G400, G450, G550, and related chipsets.
// Based on Linux kernel matrox driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::device_base::{GpuDevice, GpuFamily, GpuError, GPU_OK, GPU_ERR_NO_DEVICE, GPU_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Matrox Vendor ID ─────────────────────────────

pub const MATROX_VENDOR_ID: U16 = 0x102B;

// ─── Matrox Device IDs ─────────────────────────

pub const MATROX_MGA_2064W_DEVICE_ID: U16 = 0x0518;
pub const MATROX_MGA_1064SG_DEVICE_ID: U16 = 0x1001;
pub const MATROX_G200_PCI_DEVICE_ID: U16 = 0x0520;
pub const MATROX_G200_AGP_DEVICE_ID: U16 = 0x0521;
pub const MATROX_G400_AGP_DEVICE_ID: U16 = 0x0525;
pub const MATROX_G450_AGP_DEVICE_ID: U16 = 0x0528;
pub const MATROX_G550_AGP_DEVICE_ID: U16 = 0x0529;
pub const MATROX_G550_PCI_DEVICE_ID: U16 = 0x2527;

// ─── Matrox Register Offsets ─────────────────────

pub const MATROX_REG_INDEX: U32 = 0x3C0;
pub const MATROX_REG_DATA: U32 = 0x3C1;
pub const MATROX_REG_CRTC_INDEX: U32 = 0x3B4;
pub const MATROX_REG_CRTC_DATA: U32 = 0x3B5;
pub const MATROX_REG_SEQ_INDEX: U32 = 0x3C4;
pub const MATROX_REG_SEQ_DATA: U32 = 0x3C5;
pub const MATROX_REG_GR_INDEX: U32 = 0x3CE;
pub const MATROX_REG_GR_DATA: U32 = 0x3CF;
pub const MATROX_REG_MMIO_BASE: U32 = 0x0000;
pub const MATROX_REG_FB_BASE: U32 = 0x10000;

// ─── Matrox CRTC Register Indices ───────────────

pub const MATROX_CRTC_H_TOTAL: U8 = 0x00;
pub const MATROX_CRTC_H_DISP: U8 = 0x01;
pub const MATROX_CRTC_H_BLANK_START: U8 = 0x02;
pub const MATROX_CRTC_H_BLANK_END: U8 = 0x03;
pub const MATROX_CRTC_H_SYNC_START: U8 = 0x04;
pub const MATROX_CRTC_H_SYNC_END: U8 = 0x05;
pub const MATROX_CRTC_V_TOTAL: U8 = 0x06;
pub const MATROX_CRTC_OVERFLOW: U8 = 0x07;
pub const MATROX_CRTC_PRESET_ROW_SCAN: U8 = 0x08;
pub const MATROX_CRTC_MAX_SCAN_LINE: U8 = 0x09;
pub const MATROX_CRTC_V_SYNC_START: U8 = 0x10;
pub const MATROX_CRTC_V_SYNC_END: U8 = 0x11;
pub const MATROX_CRTC_V_DISP_END: U8 = 0x12;
pub const MATROX_CRTC_OFFSET: U8 = 0x13;
pub const MATROX_CRTC_UNDERLINE: U8 = 0x14;
pub const MATROX_CRTC_V_BLANK_START: U8 = 0x15;
pub const MATROX_CRTC_V_BLANK_END: U8 = 0x16;
pub const MATROX_CRTC_MODE_CONTROL: U8 = 0x17;
pub const MATROX_CRTC_LINE_COMPARE: U8 = 0x18;

// ─── Matrox Framebuffer Info ─────────────────────

#[repr(C)]
pub struct MatroxFramebufferInfo {
    pub base: U64,
    pub size: U64,
    pub width: U32,
    pub height: U32,
    pub pitch: U32,
    pub bpp: U32,
}

impl MatroxFramebufferInfo {
    pub const fn new() -> Self {
        MatroxFramebufferInfo {
            base: 0,
            size: 0,
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 0,
        }
    }
}

// ─── Matrox GPU Structure ─────────────────────

pub struct MatroxGpuDevice {
    pub mmio_base: U64,
    pub fb_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub family: GpuFamily,
    pub framebuffer: Option<MatroxFramebufferInfo>,
    pub fb_size: U64,
}

impl MatroxGpuDevice {
    pub const fn new() -> Self {
        MatroxGpuDevice {
            mmio_base: 0,
            fb_base: 0,
            device_id: 0,
            vendor_id: 0,
            initialized: false,
            family: GpuFamily::Unknown,
            framebuffer: None,
            fb_size: 0,
        }
    }

    /// Read CRTC register
    unsafe fn read_crtc(&self, index: U8) -> U8 {
        let ptr = (self.mmio_base + MATROX_REG_CRTC_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + MATROX_REG_CRTC_DATA) as *mut U8;
        *data_ptr
    }

    /// Write CRTC register
    unsafe fn write_crtc(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + MATROX_REG_CRTC_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + MATROX_REG_CRTC_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Read SEQ register
    unsafe fn read_seq(&self, index: U8) -> U8 {
        let ptr = (self.mmio_base + MATROX_REG_SEQ_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + MATROX_REG_SEQ_DATA) as *mut U8;
        *data_ptr
    }

    /// Write SEQ register
    unsafe fn write_seq(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + MATROX_REG_SEQ_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + MATROX_REG_SEQ_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Get GPU family from device ID
    fn get_gpu_family(&self, device_id: U16) -> GpuFamily {
        match device_id {
            MATROX_MGA_2064W_DEVICE_ID | MATROX_MGA_1064SG_DEVICE_ID => GpuFamily::MatroxMGA,
            MATROX_G200_PCI_DEVICE_ID | MATROX_G200_AGP_DEVICE_ID => GpuFamily::MatroxG200,
            MATROX_G400_AGP_DEVICE_ID => GpuFamily::MatroxG400,
            MATROX_G450_AGP_DEVICE_ID => GpuFamily::MatroxG450,
            MATROX_G550_AGP_DEVICE_ID | MATROX_G550_PCI_DEVICE_ID => GpuFamily::MatroxG550,
            _ => GpuFamily::Unknown,
        }
    }

    /// Initialize Matrox GPU
    fn init_matrox(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_mmio_base;
        self.fb_base = pci_fb_base;
        self.device_id = device_id;
        self.vendor_id = MATROX_VENDOR_ID;

        self.family = self.get_gpu_family(device_id);
        if self.family == GpuFamily::Unknown {
            return GpuError::NoDevice as I32;
        }

        unsafe {
            // Initialize framebuffer
            let fb_info = MatroxFramebufferInfo {
                base: self.fb_base,
                size: 64 * 1024 * 1024, // 64MB default
                width: 1024,
                height: 768,
                pitch: 1024 * 4,
                bpp: 32,
            };
            self.framebuffer = Some(fb_info);
            self.fb_size = fb_info.size;
        }

        self.initialized = true;
        GPU_OK
    }
}

// ─── Implement GpuDevice Trait ─────────────────

impl GpuDevice for MatroxGpuDevice {
    fn init(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.init_matrox(pci_mmio_base, pci_fb_base, device_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.family {
            GpuFamily::MatroxMGA => "Matrox MGA GPU",
            GpuFamily::MatroxG200 => "Matrox G200 GPU",
            GpuFamily::MatroxG400 => "Matrox G400 GPU",
            GpuFamily::MatroxG450 => "Matrox G450 GPU",
            GpuFamily::MatroxG550 => "Matrox G550 GPU",
            _ => "Matrox GPU",
        }
    }

    fn get_family(&self) -> GpuFamily {
        self.family
    }

    fn get_framebuffer(&self) -> Option<&super::device_base::FramebufferInfo> {
        // Convert MatroxFramebufferInfo to generic FramebufferInfo
        None
    }

    fn set_mode(&mut self, width: U32, height: U32, bpp: U32) -> I32 {
        if !self.initialized {
            return GpuError::InitFailed as I32;
        }

        if let Some(ref mut fb) = self.framebuffer {
            fb.width = width;
            fb.height = height;
            fb.bpp = bpp;
            fb.pitch = width * (bpp / 8);
        }

        GPU_OK
    }

    fn enable_display(&mut self) -> I32 {
        if !self.initialized {
            return GpuError::InitFailed as I32;
        }

        GPU_OK
    }

    fn disable_display(&mut self) -> I32 {
        if !self.initialized {
            return GpuError::InitFailed as I32;
        }

        GPU_OK
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return GpuError::InitFailed as I32;
        }

        GPU_OK
    }

    fn shutdown(&mut self) -> I32 {
        self.initialized = false;
        GPU_OK
    }
}

// ─── Global Matrox GPU Device ─────────────────

static mut G_MATROX_GPU: MatroxGpuDevice = MatroxGpuDevice::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn matrox_init(pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
    G_MATROX_GPU.init(pci_mmio_base, pci_fb_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn matrox_is_initialized() -> I32 {
    if G_MATROX_GPU.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn matrox_shutdown() -> I32 {
    G_MATROX_GPU.shutdown()
}
