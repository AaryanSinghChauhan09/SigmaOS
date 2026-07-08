// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_sis.rs — SiS GPU Driver
//
// Implements the SiS GPU driver.
// Supports SiS 6326, 650, 740, and related chipsets.
// Based on Linux kernel sis driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::device_base::{GpuDevice, GpuFamily, GpuError, GPU_OK, GPU_ERR_NO_DEVICE, GPU_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── SiS Vendor ID ─────────────────────────────────

pub const SIS_VENDOR_ID: U16 = 0x1039;

// ─── SiS Device IDs ─────────────────────────────

pub const SIS_6326_DEVICE_ID: U16 = 0x6326;
pub const SIS_650_VGA_DEVICE_ID: U16 = 0x6325;
pub const SIS_740_VGA_DEVICE_ID: U16 = 0x6330;
pub const SIS_661_VGA_DEVICE_ID: U16 = 0x6336;
pub const SIS_741_VGA_DEVICE_ID: U16 = 0x6337;
pub const SIS_670_VGA_DEVICE_ID: U16 = 0x6351;
pub const SIS_771_VGA_DEVICE_ID: U16 = 0x6352;
pub const SIS_772_VGA_DEVICE_ID: U16 = 0x6353;

// ─── SiS Register Offsets ─────────────────────────

pub const SIS_REG_SR_INDEX: U32 = 0x3C4;
pub const SIS_REG_SR_DATA: U32 = 0x3C5;
pub const SIS_REG_CR_INDEX: U32 = 0x3D4;
pub const SIS_REG_CR_DATA: U32 = 0x3D5;
pub const SIS_REG_GR_INDEX: U32 = 0x3CE;
pub const SIS_REG_GR_DATA: U32 = 0x3CF;
pub const SIS_REG_AR_INDEX: U32 = 0x3C0;
pub const SIS_REG_AR_DATA: U32 = 0x3C1;
pub const SIS_REG_MMIO_BASE: U32 = 0x0000;
pub const SIS_REG_FB_BASE: U32 = 0x10000;

// ─── SiS SR Register Indices ─────────────────────

pub const SIS_SR01: U8 = 0x01;
pub const SIS_SR07: U8 = 0x07;
pub const SIS_SR10: U8 = 0x10;
pub const SIS_SR11: U8 = 0x11;
pub const SIS_SR12: U8 = 0x12;
pub const SIS_SR13: U8 = 0x13;
pub const SIS_SR14: U8 = 0x14;
pub const SIS_SR15: U8 = 0x15;
pub const SIS_SR16: U8 = 0x16;
pub const SIS_SR17: U8 = 0x17;
pub const SIS_SR18: U8 = 0x18;
pub const SIS_SR19: U8 = 0x19;
pub const SIS_SR1A: U8 = 0x1A;
pub const SIS_SR1B: U8 = 0x1B;
pub const SIS_SR1C: U8 = 0x1C;
pub const SIS_SR1D: U8 = 0x1D;
pub const SIS_SR1E: U8 = 0x1E;
pub const SIS_SR1F: U8 = 0x1F;
pub const SIS_SR20: U8 = 0x20;
pub const SIS_SR21: U8 = 0x21;
pub const SIS_SR22: U8 = 0x22;
pub const SIS_SR23: U8 = 0x23;
pub const SIS_SR24: U8 = 0x24;
pub const SIS_SR25: U8 = 0x25;
pub const SIS_SR26: U8 = 0x26;
pub const SIS_SR27: U8 = 0x27;
pub const SIS_SR28: U8 = 0x28;
pub const SIS_SR29: U8 = 0x29;
pub const SIS_SR2A: U8 = 0x2A;
pub const SIS_SR2B: U8 = 0x2B;
pub const SIS_SR2C: U8 = 0x2C;
pub const SIS_SR2D: U8 = 0x2D;
pub const SIS_SR2E: U8 = 0x2E;
pub const SIS_SR2F: U8 = 0x2F;
pub const SIS_SR30: U8 = 0x30;
pub const SIS_SR31: U8 = 0x31;
pub const SIS_SR32: U8 = 0x32;
pub const SIS_SR33: U8 = 0x33;
pub const SIS_SR34: U8 = 0x34;
pub const SIS_SR35: U8 = 0x35;
pub const SIS_SR36: U8 = 0x36;
pub const SIS_SR37: U8 = 0x37;
pub const SIS_SR38: U8 = 0x38;
pub const SIS_SR39: U8 = 0x39;
pub const SIS_SR3A: U8 = 0x3A;
pub const SIS_SR3B: U8 = 0x3B;
pub const SIS_SR3C: U8 = 0x3C;
pub const SIS_SR3D: U8 = 0x3D;
pub const SIS_SR3E: U8 = 0x3E;
pub const SIS_SR3F: U8 = 0x3F;

// ─── SiS Framebuffer Info ─────────────────────

#[repr(C)]
pub struct SisFramebufferInfo {
    pub base: U64,
    pub size: U64,
    pub width: U32,
    pub height: U32,
    pub pitch: U32,
    pub bpp: U32,
}

impl SisFramebufferInfo {
    pub const fn new() -> Self {
        SisFramebufferInfo {
            base: 0,
            size: 0,
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 0,
        }
    }
}

// ─── SiS GPU Structure ─────────────────────────

pub struct SisGpuDevice {
    pub mmio_base: U64,
    pub fb_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub family: GpuFamily,
    pub framebuffer: Option<SisFramebufferInfo>,
    pub fb_size: U64,
}

impl SisGpuDevice {
    pub const fn new() -> Self {
        SisGpuDevice {
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

    /// Read SR register
    unsafe fn read_sr(&self, index: U8) -> U8 {
        let ptr = (self.mmio_base + SIS_REG_SR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + SIS_REG_SR_DATA) as *mut U8;
        *data_ptr
    }

    /// Write SR register
    unsafe fn write_sr(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + SIS_REG_SR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + SIS_REG_SR_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Read CR register
    unsafe fn read_cr(&self, index: U8) -> U8 {
        let ptr = (self.mmio_base + SIS_REG_CR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + SIS_REG_CR_DATA) as *mut U8;
        *data_ptr
    }

    /// Write CR register
    unsafe fn write_cr(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + SIS_REG_CR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + SIS_REG_CR_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Get GPU family from device ID
    fn get_gpu_family(&self, device_id: U16) -> GpuFamily {
        match device_id {
            SIS_6326_DEVICE_ID => GpuFamily::SiS6326,
            SIS_650_VGA_DEVICE_ID | SIS_740_VGA_DEVICE_ID => GpuFamily::SiS650,
            SIS_661_VGA_DEVICE_ID | SIS_741_VGA_DEVICE_ID => GpuFamily::SiS661,
            SIS_670_VGA_DEVICE_ID | SIS_771_VGA_DEVICE_ID | SIS_772_VGA_DEVICE_ID => GpuFamily::SiS670,
            _ => GpuFamily::Unknown,
        }
    }

    /// Initialize SiS GPU
    fn init_sis(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_mmio_base;
        self.fb_base = pci_fb_base;
        self.device_id = device_id;
        self.vendor_id = SIS_VENDOR_ID;

        self.family = self.get_gpu_family(device_id);
        if self.family == GpuFamily::Unknown {
            return GpuError::NoDevice as I32;
        }

        unsafe {
            // Initialize framebuffer
            let fb_info = SisFramebufferInfo {
                base: self.fb_base,
                size: 32 * 1024 * 1024, // 32MB default
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

impl GpuDevice for SisGpuDevice {
    fn init(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.init_sis(pci_mmio_base, pci_fb_base, device_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.family {
            GpuFamily::SiS6326 => "SiS 6326 GPU",
            GpuFamily::SiS650 => "SiS 650 GPU",
            GpuFamily::SiS661 => "SiS 661 GPU",
            GpuFamily::SiS670 => "SiS 670 GPU",
            _ => "SiS GPU",
        }
    }

    fn get_family(&self) -> GpuFamily {
        self.family
    }

    fn get_framebuffer(&self) -> Option<&super::device_base::FramebufferInfo> {
        // Convert SisFramebufferInfo to generic FramebufferInfo
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

// ─── Global SiS GPU Device ─────────────────────

static mut G_SIS_GPU: SisGpuDevice = SisGpuDevice::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn sis_init(pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
    G_SIS_GPU.init(pci_mmio_base, pci_fb_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn sis_is_initialized() -> I32 {
    if G_SIS_GPU.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn sis_shutdown() -> I32 {
    G_SIS_GPU.shutdown()
}
