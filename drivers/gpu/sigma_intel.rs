// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/sigma_intel.rs — Intel GPU Driver
//
// Implements Intel GPU driver with DRM/KMS modesetting,
// GPU command submission, and memory management for SigmaOS.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

use super::device_base::{Device, DeviceType, GpuDevice, FramebufferInfo, GpuMemoryInfo, PciDeviceInfo, DEVICE_OK, DEVICE_ERR_INIT_FAILED, DEVICE_ERR_NO_DEVICE, DEVICE_ERR_OUT_OF_MEM};

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Intel Device IDs ─────────────────────────────────────────────────────

pub const INTEL_VENDOR_ID: U16 = 0x8086;

// Intel GPU device IDs (selected common models)
pub const INTEL_DEVICE_ID_HD_GRAPHICS_620: U16 = 0x5916;
pub const INTEL_DEVICE_ID_HD_GRAPHICS_630: U16 = 0x591B;
pub const INTEL_DEVICE_ID_UHD_GRAPHICS_620: U16 = 0x5917;
pub const INTEL_DEVICE_ID_UHD_GRAPHICS_630: U16 = 0x3E9B;
pub const INTEL_DEVICE_ID_IRIS_XE: U16 = 0x9A49;
pub const INTEL_DEVICE_ID_IRIS_XE_MAX: U16 = 0x4907;
pub const INTEL_DEVICE_ID_ARC_A380: U16 = 0x7FD0;
pub const INTEL_DEVICE_ID_ARC_A750: U16 = 0x56A0;
pub const INTEL_DEVICE_ID_ARC_A770: U16 = 0x56A1;

// ─── Intel GPU Families ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IntelGpuFamily {
    Unknown,
    KabyLake,      // HD Graphics 620/630
    CoffeeLake,    // UHD Graphics 620/630
    TigerLake,     // Iris Xe
    DG2,           // Arc A-series
}

// ─── Intel Device Structure ───────────────────────────────────────────────

pub struct IntelDevice {
    pub mmio_base: U64,
    pub framebuffer_base: U64,
    pub device_id: U16,
    pub initialized: bool,
    pub framebuffer: Option<FramebufferInfo>,
    pub family: IntelGpuFamily,
    pub memory_info: GpuMemoryInfo,
    pub gtt_size: usize,
}

impl IntelDevice {
    pub const fn new() -> Self {
        IntelDevice {
            mmio_base: 0,
            framebuffer_base: 0,
            device_id: 0,
            initialized: false,
            framebuffer: None,
            family: IntelGpuFamily::Unknown,
            memory_info: GpuMemoryInfo::new(),
            gtt_size: 0,
        }
    }

    /// Get GPU family from device ID
    fn get_gpu_family(&self, device_id: U16) -> IntelGpuFamily {
        match device_id {
            INTEL_DEVICE_ID_HD_GRAPHICS_620 |
            INTEL_DEVICE_ID_HD_GRAPHICS_630 => IntelGpuFamily::KabyLake,
            
            INTEL_DEVICE_ID_UHD_GRAPHICS_620 |
            INTEL_DEVICE_ID_UHD_GRAPHICS_630 => IntelGpuFamily::CoffeeLake,
            
            INTEL_DEVICE_ID_IRIS_XE |
            INTEL_DEVICE_ID_IRIS_XE_MAX => IntelGpuFamily::TigerLake,
            
            INTEL_DEVICE_ID_ARC_A380 |
            INTEL_DEVICE_ID_ARC_A750 |
            INTEL_DEVICE_ID_ARC_A770 => IntelGpuFamily::DG2,
            
            _ => IntelGpuFamily::Unknown,
        }
    }

    /// Initialize Intel GPU
    fn init_intel(&mut self, pci_info: &PciDeviceInfo) -> I32 {
        self.mmio_base = pci_info.bar0;
        self.framebuffer_base = pci_info.bar2;
        self.device_id = pci_info.device_id;

        // Validate device ID and determine family
        self.family = self.get_gpu_family(pci_info.device_id);
        if self.family == IntelGpuFamily::Unknown {
            return DEVICE_ERR_NO_DEVICE;
        }

        // Initialize GTT (Graphics Translation Table)
        self.gtt_size = match self.family {
            IntelGpuFamily::KabyLake | IntelGpuFamily::CoffeeLake => 2 * 1024 * 1024, // 2MB
            IntelGpuFamily::TigerLake => 4 * 1024 * 1024, // 4MB
            IntelGpuFamily::DG2 => 8 * 1024 * 1024, // 8MB
            _ => 0,
        };

        // Initialize memory info based on family
        self.memory_info = match self.family {
            IntelGpuFamily::KabyLake | IntelGpuFamily::CoffeeLake => GpuMemoryInfo {
                total_vram: 0, // Integrated GPU uses system memory
                free_vram: 0,
                gart_size: self.gtt_size as U64,
                used_gart: 0,
            },
            IntelGpuFamily::TigerLake => GpuMemoryInfo {
                total_vram: 0,
                free_vram: 0,
                gart_size: self.gtt_size as U64,
                used_gart: 0,
            },
            IntelGpuFamily::DG2 => GpuMemoryInfo {
                total_vram: 8 * 1024 * 1024 * 1024, // 8GB dedicated VRAM
                free_vram: 8 * 1024 * 1024 * 1024,
                gart_size: self.gtt_size as U64,
                used_gart: 0,
            },
            _ => GpuMemoryInfo::new(),
        };

        // Initialize display engine
        if self.init_display() != DEVICE_OK {
            return DEVICE_ERR_INIT_FAILED;
        }

        // Initialize compute engine
        if self.init_compute() != DEVICE_OK {
            return DEVICE_ERR_INIT_FAILED;
        }

        self.initialized = true;
        DEVICE_OK
    }

    /// Initialize display engine
    fn init_display(&mut self) -> I32 {
        // In a real implementation, this would:
        // 1. Detect connected displays (DisplayPort, HDMI, eDP)
        // 2. Read EDID from display
        // 3. Configure display controller (pipe, plane, transcoder)
        // 4. Set up mode (resolution, refresh rate)

        // Stub framebuffer
        self.framebuffer = Some(FramebufferInfo {
            base: self.framebuffer_base,
            width: 1920,
            height: 1080,
            stride: 1920 * 4,
            bpp: 32,
            refresh_rate: 60,
        });

        DEVICE_OK
    }

    /// Initialize compute engine
    fn init_compute(&self) -> I32 {
        // In a real implementation, this would:
        // 1. Initialize ring buffers (render, blit, video)
        // 2. Set up compute context
        // 3. Enable compute engine
        // 4. Configure media engine for video decode/encode

        DEVICE_OK
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

// ─── Implement Device Trait ─────────────────────────────────────────────────

impl Device for IntelDevice {
    fn init(&mut self, pci_info: &PciDeviceInfo) -> I32 {
        if pci_info.vendor_id != INTEL_VENDOR_ID {
            return DEVICE_ERR_NO_DEVICE;
        }
        self.init_intel(pci_info)
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn get_device_type(&self) -> DeviceType {
        DeviceType::GPU
    }

    fn get_device_name(&self) -> &'static str {
        match self.family {
            IntelGpuFamily::KabyLake => "Intel Kaby Lake GPU",
            IntelGpuFamily::CoffeeLake => "Intel Coffee Lake GPU",
            IntelGpuFamily::TigerLake => "Intel Tiger Lake Iris Xe",
            IntelGpuFamily::DG2 => "Intel Arc GPU",
            IntelGpuFamily::Unknown => "Intel GPU",
        }
    }

    fn reset(&mut self) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, perform GPU reset
        DEVICE_OK
    }

    fn shutdown(&mut self) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, perform GPU shutdown
        self.initialized = false;
        DEVICE_OK
    }

    fn get_info(&self, buffer: &mut [U8]) -> I32 {
        if buffer.is_empty() {
            return -1;
        }

        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        let family_name = match self.family {
            IntelGpuFamily::KabyLake => "Intel HD Graphics 620/630 (Kaby Lake)",
            IntelGpuFamily::CoffeeLake => "Intel UHD Graphics 620/630 (Coffee Lake)",
            IntelGpuFamily::TigerLake => "Intel Iris Xe (Tiger Lake)",
            IntelGpuFamily::DG2 => "Intel Arc A-series (DG2)",
            IntelGpuFamily::Unknown => "Unknown Intel GPU",
        };

        let name_bytes = family_name.as_bytes();
        let copy_len = name_bytes.len().min(buffer.len() - 1);

        for i in 0..copy_len {
            buffer[i] = name_bytes[i];
        }
        if copy_len < buffer.len() {
            buffer[copy_len] = 0;
        }

        DEVICE_OK
    }
}

// ─── Implement GpuDevice Trait ───────────────────────────────────────────────

impl GpuDevice for IntelDevice {
    fn set_mode(&mut self, width: U32, height: U32, refresh: U32) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        if let Some(ref mut fb) = self.framebuffer {
            fb.width = width;
            fb.height = height;
            fb.stride = width * 4;
            fb.refresh_rate = refresh;
        }

        // In a real implementation, configure display controller
        DEVICE_OK
    }

    fn enable_display(&mut self) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, enable display output
        DEVICE_OK
    }

    fn disable_display(&mut self) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, disable display output
        DEVICE_OK
    }

    fn get_framebuffer_info(&self) -> Option<FramebufferInfo> {
        self.framebuffer.as_ref().copied()
    }

    fn submit_command(&mut self, cmd: U32, data: U64) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, write to ring buffer
        DEVICE_OK
    }

    fn map_page(&mut self, physical: U64, virtual_addr: U64) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, map page to GTT
        if self.memory_info.used_gart + 4096 > self.memory_info.gart_size {
            return DEVICE_ERR_OUT_OF_MEM;
        }

        self.memory_info.used_gart += 4096;
        DEVICE_OK
    }

    fn unmap_page(&mut self, virtual_addr: U64) -> I32 {
        if !self.initialized {
            return DEVICE_ERR_INIT_FAILED;
        }

        // In a real implementation, unmap page from GTT
        if self.memory_info.used_gart >= 4096 {
            self.memory_info.used_gart -= 4096;
        }

        DEVICE_OK
    }

    fn get_memory_info(&self) -> GpuMemoryInfo {
        self.memory_info
    }
}

// ─── Global Intel Device ───────────────────────────────────────────────────

static mut G_INTEL: IntelDevice = IntelDevice::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn intel_init(pci_bar0: U64, pci_bar2: U64, device_id: U16) -> I32 {
    let pci_info = PciDeviceInfo {
        vendor_id: INTEL_VENDOR_ID,
        device_id: device_id,
        class_code: 0x03,
        subclass: 0x00,
        bar0: pci_bar0,
        bar1: 0,
        bar2: pci_bar2,
        bar3: 0,
        bar4: 0,
        bar5: 0,
    };
    G_INTEL.init(&pci_info)
}

#[no_mangle]
pub unsafe extern "C" fn intel_is_initialized() -> I32 {
    if G_INTEL.is_initialized() {
        1
    } else {
        0
    }
}

#[no_mangle]
pub unsafe extern "C" fn intel_set_mode(width: U32, height: U32, refresh: U32) -> I32 {
    G_INTEL.set_mode(width, height, refresh)
}

#[no_mangle]
pub unsafe extern "C" fn intel_enable_display() -> I32 {
    G_INTEL.enable_display()
}

#[no_mangle]
pub unsafe extern "C" fn intel_disable_display() -> I32 {
    G_INTEL.disable_display()
}

#[no_mangle]
pub unsafe extern "C" fn intel_submit_command(cmd: U32, data: U64) -> I32 {
    G_INTEL.submit_command(cmd, data)
}

#[no_mangle]
pub unsafe extern "C" fn intel_map_page(physical: U64, virtual_addr: U64) -> I32 {
    G_INTEL.map_page(physical, virtual_addr)
}

#[no_mangle]
pub unsafe extern "C" fn intel_unmap_page(virtual_addr: U64) -> I32 {
    G_INTEL.unmap_page(virtual_addr)
}

#[no_mangle]
pub unsafe extern "C" fn intel_get_device_info(buffer: *mut U8, buffer_size: usize) -> I32 {
    if buffer.is_null() || buffer_size == 0 {
        return -1;
    }
    
    let slice = core::slice::from_raw_parts_mut(buffer, buffer_size);
    G_INTEL.get_info(slice)
}

/// Probe for Intel GPU devices
#[no_mangle]
pub unsafe extern "C" fn intel_probe() -> I32 {
    // Scan PCI bus for Intel GPU devices
    let mut found_devices = 0;
    
    // Scan PCI buses 0-255
    for bus in 0..256u8 {
        // Scan devices 0-31
        for device in 0..32u8 {
            // Scan functions 0-7
            for function in 0..8u8 {
                let device_id = read_pci_config_u16(bus, device, function, 0x02);
                let vendor_id = read_pci_config_u16(bus, device, function, 0x00);
                
                // Check if this is an Intel GPU
                if vendor_id == INTEL_VENDOR_ID && is_intel_gpu_device(device_id) {
                    let bar0 = read_pci_config_u32(bus, device, function, 0x10);
                    let bar2 = read_pci_config_u32(bus, device, function, 0x18);
                    
                    let mmio_base = (bar0 & 0xFFFFFFF0) as U64;
                    let framebuffer_base = (bar2 & 0xFFFFFFF0) as U64;
                    
                    let result = G_INTEL.init(&PciDeviceInfo {
                        vendor_id: INTEL_VENDOR_ID,
                        device_id: device_id,
                        class_code: 0x03,
                        subclass: 0x00,
                        bar0: mmio_base,
                        bar1: 0,
                        bar2: framebuffer_base,
                        bar3: 0,
                        bar4: 0,
                        bar5: 0,
                    });
                    
                    if result == DEVICE_OK {
                        found_devices += 1;
                        // For now, just use the first found device
                        return DEVICE_OK;
                    }
                }
            }
        }
    }
    
    if found_devices > 0 {
        DEVICE_OK
    } else {
        DEVICE_ERR_NO_DEVICE
    }
}

/// Check if device ID is a supported Intel GPU
unsafe fn is_intel_gpu_device(device_id: U16) -> bool {
    match device_id {
        INTEL_DEVICE_ID_HD_GRAPHICS_620 |
        INTEL_DEVICE_ID_HD_GRAPHICS_630 |
        INTEL_DEVICE_ID_UHD_GRAPHICS_620 |
        INTEL_DEVICE_ID_UHD_GRAPHICS_630 |
        INTEL_DEVICE_ID_IRIS_XE |
        INTEL_DEVICE_ID_IRIS_XE_MAX |
        INTEL_DEVICE_ID_ARC_A380 |
        INTEL_DEVICE_ID_ARC_A750 |
        INTEL_DEVICE_ID_ARC_A770 => true,
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

/// Write 32-bit value to IO port
unsafe fn outl(port: U16, value: U32) {
    // Placeholder - would be implemented with inline assembly
}

/// Read 32-bit value from IO port
unsafe fn inl(port: U16) -> U32 {
    // Placeholder - would be implemented with inline assembly
    0
}
