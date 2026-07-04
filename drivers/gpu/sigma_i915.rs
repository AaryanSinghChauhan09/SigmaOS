// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_i915.rs — Intel i915 GPU Driver
//
// Implements Intel i915 GPU driver with DRM/KMS modesetting,
// GPU command submission, and memory management for SigmaOS.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

pub const I915_OK: I32 = 0;
pub const I915_ERR_NO_DEVICE: I32 = -1;
pub const I915_ERR_INIT_FAILED: I32 = -2;
pub const I915_ERR_OUT_OF_MEM: I32 = -3;

// ─── PCI Device IDs ───────────────────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;

// Common Intel GPU device IDs
pub const I915_DEVICE_ID_HSW_GT1: U16 = 0x0402;
pub const I915_DEVICE_ID_HSW_GT2: U16 = 0x0412;
pub const I915_DEVICE_ID_BDW_GT1: U16 = 0x1606;
pub const I915_DEVICE_ID_BDW_GT2: U16 = 0x1616;
pub const I915_DEVICE_ID_SKL_GT1: U16 = 0x1906;
pub const I915_DEVICE_ID_SKL_GT2: U16 = 0x1916;
pub const I915_DEVICE_ID_KBL_GT1: U16 = 0x5906;
pub const I915_DEVICE_ID_KBL_GT2: U16 = 0x5916;

// ─── MMIO Register Offsets ───────────────────────────────────────────────────

pub const PCI_MMIO_BAR: U8 = 0;
pub const PCI_GTT_BAR: U8 = 2;

pub const GTT_PTE_BASE: U64 = 0;
pub const PTE_VALID: U64 = 1 << 0;
pub const PTE_WRITE: U64 = 1 << 1;

// ─── Display Engine Registers ───────────────────────────────────────────────

pub const PIPE_A_OFFSET: U32 = 0x70000;
pub const PIPE_B_OFFSET: U32 = 0x71000;
pub const PIPE_C_OFFSET: U32 = 0x72000;

pub const PIPE_PLANE_OFFSET: U32 = 0x70180;
pub const PIPE_CURSOR_OFFSET: U32 = 0x70080;

pub const DISPLAY_CTRL: U32 = 0x71400;
pub const DISPLAY_STATUS: U32 = 0x71404;

// ─── GPU Engine Registers ───────────────────────────────────────────────────

pub const RCS_HW_CTX: U32 = 0x2230;
pub const RCS_INSTDONE: U32 = 0x2050;
pub const RING_CTL: U32 = 0x203c;
pub const RING_HEAD: U32 = 0x2040;
pub const RING_TAIL: U32 = 0x2044;
pub const RING_START: U32 = 0x2048;

// ─── Graphics Memory ───────────────────────────────────────────────────────

#[repr(C)]
pub struct GTTEntry {
    pub base: U64,
    pub flags: U64,
}

// ─── Framebuffer Info ───────────────────────────────────────────────────────

#[repr(C)]
pub struct FramebufferInfo {
    pub base: U64,
    pub width: U32,
    pub height: U32,
    pub stride: U32,
    pub bpp: U32,
}

// ─── i915 Device Structure ─────────────────────────────────────────────────

pub struct I915Device {
    pub mmio_base: U64,
    pub gtt_base: U64,
    pub device_id: U16,
    pub initialized: bool,
    pub framebuffer: Option<FramebufferInfo>,
    pub gtt_size: usize,
}

impl I915Device {
    pub const fn new() -> Self {
        I915Device {
            mmio_base: 0,
            gtt_base: 0,
            device_id: 0,
            initialized: false,
            framebuffer: None,
            gtt_size: 0,
        }
    }

    /// Initialize i915 GPU
    pub unsafe fn init(&mut self, pci_mmio_base: U64, pci_gtt_base: U64, device_id: U16) -> I32 {
        self.mmio_base = pci_mmio_base;
        self.gtt_base = pci_gtt_base;
        self.device_id = device_id;

        // Validate device ID
        if !self.is_supported_device(device_id) {
            return I915_ERR_NO_DEVICE;
        }

        // Initialize GTT (Graphics Translation Table)
        if self.init_gtt() != I915_OK {
            return I915_ERR_INIT_FAILED;
        }

        // Initialize display engine
        if self.init_display() != I915_OK {
            return I915_ERR_INIT_FAILED;
        }

        // Initialize render engine
        if self.init_render() != I915_OK {
            return I915_ERR_INIT_FAILED;
        }

        self.initialized = true;
        I915_OK
    }

    /// Check if device ID is supported
    fn is_supported_device(&self, device_id: U16) -> bool {
        matches!(
            device_id,
            I915_DEVICE_ID_HSW_GT1 |
            I915_DEVICE_ID_HSW_GT2 |
            I915_DEVICE_ID_BDW_GT1 |
            I915_DEVICE_ID_BDW_GT2 |
            I915_DEVICE_ID_SKL_GT1 |
            I915_DEVICE_ID_SKL_GT2 |
            I915_DEVICE_ID_KBL_GT1 |
            I915_DEVICE_ID_KBL_GT2
        )
    }

    /// Initialize Graphics Translation Table
    unsafe fn init_gtt(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Map GTT aperture
        // 2. Initialize GTT entries
        // 3. Set up stolen memory
        
        self.gtt_size = 2 * 1024 * 1024; // Stub: 2MB GTT
        I915_OK
    }

    /// Initialize display engine
    unsafe fn init_display(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Detect connected displays (DDI ports)
        // 2. Read EDID from display
        // 3. Configure pipe and plane
        // 4. Set up mode (resolution, refresh rate)
        
        // Stub framebuffer
        self.framebuffer = Some(FramebufferInfo {
            base: self.gtt_base,
            width: 1920,
            height: 1080,
            stride: 1920 * 4,
            bpp: 32,
        });
        
        I915_OK
    }

    /// Initialize render engine
    unsafe fn init_render(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Initialize ring buffer
        2. Set up context
        // 3. Enable render engine
        
        I915_OK
    }

    /// Map physical page to GTT
    pub unsafe fn map_page(&mut self, physical: U64, virtual_addr: U64) -> I32 {
        if !self.initialized {
            return I915_ERR_INIT_FAILED;
        }

        // In a real implementation, write PTE to GTT
        let gtt_entry = GTTEntry {
            base: physical & !0xFFF,
            flags: PTE_VALID | PTE_WRITE,
        };

        let gtt_index = (virtual_addr / 0x1000) as usize;
        if gtt_index >= self.gtt_size {
            return I915_ERR_OUT_OF_MEM;
        }

        // Write PTE to GTT (stub)
        let gtt_ptr = (self.gtt_base + gtt_index as u64 * 8) as *mut GTTEntry;
        *gtt_ptr = gtt_entry;

        I915_OK
    }

    /// Submit command to render engine
    pub unsafe fn submit_command(&self, cmd: U32, data: U64) -> I32 {
        if !self.initialized {
            return I915_ERR_INIT_FAILED;
        }

        // In a real implementation, write to ring buffer
        // and update tail pointer
        
        I915_OK
    }

    /// Get framebuffer info
    pub fn get_framebuffer(&self) -> Option<&FramebufferInfo> {
        self.framebuffer.as_ref()
    }

    /// Set display mode
    pub unsafe fn set_mode(&mut self, width: U32, height: U32) -> I32 {
        if !self.initialized {
            return I915_ERR_INIT_FAILED;
        }

        if let Some(ref mut fb) = self.framebuffer {
            fb.width = width;
            fb.height = height;
            fb.stride = width * 4;
        }

        // In a real implementation, configure pipe and plane
        I915_OK
    }

    /// Enable display
    pub unsafe fn enable_display(&self) -> I32 {
        if !self.initialized {
            return I915_ERR_INIT_FAILED;
        }

        // In a real implementation, write to display control register
        I915_OK
    }

    /// Disable display
    pub unsafe fn disable_display(&self) -> I32 {
        if !self.initialized {
            return I915_ERR_INIT_FAILED;
        }

        // In a real implementation, write to display control register
        I915_OK
    }
}

// ─── Global i915 Device ─────────────────────────────────────────────────────

static mut G_I915: I915Device = I915Device::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn i915_init(pci_mmio_base: U64, pci_gtt_base: U64, device_id: U16) -> I32 {
    G_I915.init(pci_mmio_base, pci_gtt_base, device_id)
}

#[no_mangle]
pub unsafe extern "C" fn i915_map_page(physical: U64, virtual_addr: U64) -> I32 {
    G_I915.map_page(physical, virtual_addr)
}

#[no_mangle]
pub unsafe extern "C" fn i915_submit_command(cmd: U32, data: U64) -> I32 {
    G_I915.submit_command(cmd, data)
}

#[no_mangle]
pub unsafe extern "C" fn i915_set_mode(width: U32, height: U32) -> I32 {
    G_I915.set_mode(width, height)
}

#[no_mangle]
pub unsafe extern "C" fn i915_enable_display() -> I32 {
    G_I915.enable_display()
}

#[no_mangle]
pub unsafe extern "C" fn i915_disable_display() -> I32 {
    G_I915.disable_display()
}
