// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// drivers/gpu/device_base.rs — Base Device Trait for GPU Drivers
//
// Defines the OOP base class for all GPU devices using Rust traits.
// This provides a common interface for GPU operations across different vendors.
//
// Language: Rust (no_std for kernel driver)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;

// ─── Error Codes ─────────────────────────────────────────────────────────────

pub const DEVICE_OK: I32 = 0;
pub const DEVICE_ERR_NO_DEVICE: I32 = -1;
pub const DEVICE_ERR_INIT_FAILED: I32 = -2;
pub const DEVICE_ERR_OUT_OF_MEM: I32 = -3;
pub const DEVICE_ERR_NOT_SUPPORTED: I32 = -4;

// ─── Device Types ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DeviceType {
    Unknown,
    GPU,
    Network,
    Storage,
    Audio,
    Bluetooth,
    USB,
}

// ─── PCI Device Info ────────────────────────────────────────────────────────

#[repr(C)]
pub struct PciDeviceInfo {
    pub vendor_id: U16,
    pub device_id: U16,
    pub class_code: U8,
    pub subclass: U8,
    pub bar0: U64,
    pub bar1: U64,
    pub bar2: U64,
    pub bar3: U64,
    pub bar4: U64,
    pub bar5: U64,
}

impl PciDeviceInfo {
    pub const fn new() -> Self {
        PciDeviceInfo {
            vendor_id: 0,
            device_id: 0,
            class_code: 0,
            subclass: 0,
            bar0: 0,
            bar1: 0,
            bar2: 0,
            bar3: 0,
            bar4: 0,
            bar5: 0,
        }
    }
}

// ─── Base Device Trait ─────────────────────────────────────────────────────

/// Base trait for all hardware devices in SigmaOS
/// This provides a common interface for device operations
pub trait Device {
    /// Initialize the device
    fn init(&mut self, pci_info: &PciDeviceInfo) -> I32;
    
    /// Check if device is initialized
    fn is_initialized(&self) -> bool;
    
    /// Get device type
    fn get_device_type(&self) -> DeviceType;
    
    /// Get device name
    fn get_device_name(&self) -> &'static str;
    
    /// Reset the device
    fn reset(&mut self) -> I32;
    
    /// Shutdown the device
    fn shutdown(&mut self) -> I32;
    
    /// Get device info string
    fn get_info(&self, buffer: &mut [U8]) -> I32;
}

// ─── GPU Device Trait ─────────────────────────────────────────────────────

/// Trait for GPU-specific operations
/// Extends the base Device trait with GPU-specific functionality
pub trait GpuDevice: Device {
    /// Set display mode (resolution, refresh rate)
    fn set_mode(&mut self, width: U32, height: U32, refresh: U32) -> I32;
    
    /// Enable display output
    fn enable_display(&mut self) -> I32;
    
    /// Disable display output
    fn disable_display(&mut self) -> I32;
    
    /// Get framebuffer information
    fn get_framebuffer_info(&self) -> Option<FramebufferInfo>;
    
    /// Submit command to GPU
    fn submit_command(&mut self, cmd: U32, data: U64) -> I32;
    
    /// Map physical page to GPU address space
    fn map_page(&mut self, physical: U64, virtual_addr: U64) -> I32;
    
    /// Unmap page from GPU address space
    fn unmap_page(&mut self, virtual_addr: U64) -> I32;
    
    /// Get GPU memory info
    fn get_memory_info(&self) -> GpuMemoryInfo;
}

// ─── Framebuffer Info ───────────────────────────────────────────────────────

#[repr(C)]
pub struct FramebufferInfo {
    pub base: U64,
    pub width: U32,
    pub height: U32,
    pub stride: U32,
    pub bpp: U32,
    pub refresh_rate: U32,
}

impl FramebufferInfo {
    pub const fn new() -> Self {
        FramebufferInfo {
            base: 0,
            width: 0,
            height: 0,
            stride: 0,
            bpp: 0,
            refresh_rate: 60,
        }
    }
}

// ─── GPU Memory Info ───────────────────────────────────────────────────────

#[repr(C)]
pub struct GpuMemoryInfo {
    pub total_vram: U64,
    pub free_vram: U64,
    pub gart_size: U64,
    pub used_gart: U64,
}

impl GpuMemoryInfo {
    pub const fn new() -> Self {
        GpuMemoryInfo {
            total_vram: 0,
            free_vram: 0,
            gart_size: 0,
            used_gart: 0,
        }
    }
}

// ─── Device Manager ─────────────────────────────────────────────────────────

/// Manages all registered devices in the system
pub struct DeviceManager {
    devices: [Option<&'static dyn Device>; 256],
    device_count: usize,
}

impl DeviceManager {
    pub const fn new() -> Self {
        DeviceManager {
            devices: [None; 256],
            device_count: 0,
        }
    }
    
    /// Register a device with the manager
    pub fn register_device(&mut self, device: &'static dyn Device) -> I32 {
        if self.device_count >= 256 {
            return DEVICE_ERR_OUT_OF_MEM;
        }
        
        self.devices[self.device_count] = Some(device);
        self.device_count += 1;
        DEVICE_OK
    }
    
    /// Get device by index
    pub fn get_device(&self, index: usize) -> Option<&'static dyn Device> {
        if index < self.device_count {
            self.devices[index]
        } else {
            None
        }
    }
    
    /// Get device count
    pub fn get_device_count(&self) -> usize {
        self.device_count
    }
    
    /// Find device by type
    pub fn find_device_by_type(&self, device_type: DeviceType) -> Option<&'static dyn Device> {
        for i in 0..self.device_count {
            if let Some(device) = self.devices[i] {
                if device.get_device_type() == device_type {
                    return Some(device);
                }
            }
        }
        None
    }
}

// ─── Global Device Manager ─────────────────────────────────────────────────

static mut DEVICE_MANAGER: DeviceManager = DeviceManager::new();

/// Get global device manager
pub unsafe fn get_device_manager() -> &'static mut DeviceManager {
    &mut DEVICE_MANAGER
}

/// Initialize device manager
pub unsafe fn init_device_manager() {
    DEVICE_MANAGER = DeviceManager::new();
}
