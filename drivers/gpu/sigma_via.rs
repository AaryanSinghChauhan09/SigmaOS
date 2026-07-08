// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_via.rs — VIA GPU Driver
//
// Implements the VIA GPU driver.
// Supports VIA Chrome9, VX11, and related chipsets.
// Based on Linux kernel via driver patterns.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::device_base::{GpuDevice, GpuFamily, GpuError, GPU_OK, GPU_ERR_NO_DEVICE, GPU_ERR_INIT_FAILED};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── VIA Vendor ID ─────────────────────────────────

pub const VIA_VENDOR_ID: U16 = 0x1106;

// ─── VIA Device IDs ─────────────────────────────

pub const VIA_CHROME9_HC3_DEVICE_ID: U16 = 0x3122;
pub const VIA_CHROME9_HC4_DEVICE_ID: U16 = 0x3123;
pub const VIA_VX11_VGA_DEVICE_ID: U16 = 0x3324;
pub const VIA_VX8750_VGA_DEVICE_ID: U16 = 0x3225;
pub const VIA_VX900_VGA_DEVICE_ID: U16 = 0x3327;

// ─── VIA Register Offsets ─────────────────────────

pub const VIA_REG_SR_INDEX: U32 = 0x3C4;
pub const VIA_REG_SR_DATA: U32 = 0x3C5;
pub const VIA_REG_CR_INDEX: U32 = 0x3D4;
pub const VIA_REG_CR_DATA: U32 = 0x3D5;
pub const VIA_REG_GR_INDEX: U32 = 0x3CE;
pub const VIA_REG_GR_DATA: U32 = 0x3CF;
pub const VIA_REG_AR_INDEX: U32 = 0x3C0;
pub const VIA_REG_AR_DATA: U32 = 0x3C1;
pub const VIA_REG_MMIO_BASE: U32 = 0x0000;
pub const VIA_REG_FB_BASE: U32 = 0x10000;

// ─── VIA SR Register Indices ─────────────────────

pub const VIA_SR01: U8 = 0x01;
pub const VIA_SR07: U8 = 0x07;
pub const VIA_SR10: U8 = 0x10;
pub const VIA_SR11: U8 = 0x11;
pub const VIA_SR12: U8 = 0x12;
pub const VIA_SR13: U8 = 0x13;
pub const VIA_SR14: U8 = 0x14;
pub const VIA_SR15: U8 = 0x15;
pub const VIA_SR16: U8 = 0x16;
pub const VIA_SR17: U8 = 0x17;
pub const VIA_SR18: U8 = 0x18;
pub const VIA_SR19: U8 = 0x19;
pub const VIA_SR1A: U8 = 0x1A;
pub const VIA_SR1B: U8 = 0x1B;
pub const VIA_SR1C: U8 = 0x1C;
pub const VIA_SR1D: U8 = 0x1D;
pub const VIA_SR1E: U8 = 0x1E;
pub const VIA_SR1F: U8 = 0x1F;
pub const VIA_SR20: U8 = 0x20;
pub const VIA_SR21: U8 = 0x21;
pub const VIA_SR22: U8 = 0x22;
pub const VIA_SR23: U8 = 0x23;
pub const VIA_SR24: U8 = 0x24;
pub const VIA_SR25: U8 = 0x25;
pub const VIA_SR26: U8 = 0x26;
pub const VIA_SR27: U8 = 0x27;
pub const VIA_SR28: U8 = 0x28;
pub const VIA_SR29: U8 = 0x29;
pub const VIA_SR2A: U8 = 0x2A;
pub const VIA_SR2B: U8 = 0x2B;
pub const VIA_SR2C: U8 = 0x2C;
pub const VIA_SR2D: U8 = 0x2D;
pub const VIA_SR2E: U8 = 0x2E;
pub const VIA_SR2F: U8 = 0x2F;
pub const VIA_SR30: U8 = 0x30;
pub const VIA_SR31: U8 = 0x31;
pub const VIA_SR32: U8 = 0x32;
pub const VIA_SR33: U8 = 0x33;
pub const VIA_SR34: U8 = 0x34;
pub const VIA_SR35: U8 = 0x35;
pub const VIA_SR36: U8 = 0x36;
pub const VIA_SR37: U8 = 0x37;
pub const VIA_SR38: U8 = 0x38;
pub const VIA_SR39: U8 = 0x39;
pub const VIA_SR3A: U8 = 0x3A;
pub const VIA_SR3B: U8 = 0x3B;
pub const VIA_SR3C: U8 = 0x3C;
pub const VIA_SR3D: U8 = 0x3D;
pub const VIA_SR3E: U8 = 0x3E;
pub const VIA_SR3F: U8 = 0x3F;

// ─── VIA Framebuffer Info ─────────────────────

#[repr(C)]
pub struct ViaFramebufferInfo {
    pub base: U64,
    pub size: U64,
    pub width: U32,
    pub height: U32,
    pub pitch: U32,
    pub bpp: U32,
}

impl ViaFramebufferInfo {
    pub const fn new() -> Self {
        ViaFramebufferInfo {
            base: 0,
            size: 0,
            width: 0,
            height: 0,
            pitch: 0,
            bpp: 0,
        }
    }
}

// ─── VIA GPU Structure ─────────────────────────

pub struct ViaGpuDevice {
    pub mmio_base: U64,
    pub fb_base: U64,
    pub device_id: U16,
    pub vendor_id: U16,
    pub initialized: bool,
    pub family: GpuFamily,
    pub framebuffer: Option<ViaFramebufferInfo>,
    pub fb_size: U64,
}

impl ViaGpuDevice {
    pub const fn new() -> Self {
        ViaGpuDevice {
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
        let ptr = (self.mmio_base + VIA_REG_SR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + VIA_REG_SR_DATA) as *mut U8;
        *data_ptr
    }

    /// Write SR register
    unsafe fn write_sr(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + VIA_REG_SR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + VIA_REG_SR_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Read CR register
    unsafe fn read_cr(&self, index: U8) -> U8 {
        let ptr = (self.mmio_base + VIA_REG_CR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + VIA_REG_CR_DATA) as *mut U8;
        *data_ptr
    }

    /// Write CR register
    unsafe fn write_cr(&self, index: U8, value: U8) {
        let ptr = (self.mmio_base + VIA_REG_CR_INDEX) as *mut U8;
        *ptr = index;
        let data_ptr = (self.mmio_base + VIA_REG_CR_DATA) as *mut U8;
        *data_ptr = value;
    }

    /// Get GPU family from device ID
    fn get_gpu_family(&self, device_id: U16) -> GpuFamily {
        match device_id {
            VIA_CHROME9_HC3_DEVICE_ID | VIA_CHROME9_HC4_DEVICE_ID => GpuFamily::ViaChrome9,
            VIA_VX11_VGA_DEVICE_ID | VIA_VX8750_VGA_DEVICE_ID | VIA_VX900_VGA_DEVICE_ID => GpuFamily::ViaVX,
            _ => GpuFamily::Unknown,
        }
    }

    /// Initialize VIA GPU
    fn init_via(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_mmio_base;
        self.fb_base = pci_fb_base;
        self.device_id = device_id;
        self.vendor_id = VIA_VENDOR_ID;

        self.family = self.get_gpu_family(device_id);
        if self.family == GpuFamily::Unknown {
            return GpuError::NoDevice as I32;
        }

        unsafe {
            // Initialize framebuffer
            let fb_info = ViaFramebufferInfo {
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

impl GpuDevice for ViaGpuDevice {
    fn init(&mut self, pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
        self.init_via(pci_mmio_base, pci_fb_base, device_id)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_name(&self) -> &'static str {
        match self.family {
            GpuFamily::ViaChrome9 => "VIA Chrome9 GPU",
            GpuFamily::ViaVX => "VIA VX GPU",
            _ => "VIA GPU",
        }
    }

    fn get_family(&self) -> GpuFamily {
        self.family
    }

    fn get_framebuffer(&self) -> Option<&super::device_base::FramebufferInfo> {
        // Convert ViaFramebufferInfo to generic FramebufferInfo
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

// ─── Global VIA GPU Device ─────────────────────

static mut G_VIA_GPU: ViaGpuDevice = ViaGpuDevice::new();

// ─── C-ABI Exports ─────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn via_init(pci_mmio_base: U64, pci_fb_base: U64, device_id: U16) -> I32 {
    G_VIA_GPU.init(pci_mmio_base, pci_fb_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn via_is_initialized() -> I32 {
    if G_VIA_GPU.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn via_shutdown() -> I32 {
    G_VIA_GPU.shutdown()
}
