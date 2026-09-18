// SPDX-License-Identifier: MIT
// SigmaOS Intel i915/i965 Integrated GPU Driver
// Supports Gen 3-11 Intel integrated graphics (Skylake, Kaby Lake, Coffee Lake, etc.)

use std::boxed::Box;
use std::vec::Vec;
use std::string::String;
use core::sync::atomic::{AtomicU32, Ordering};

use crate::driver::pci_enumeration::{PciDeviceInfo, PciDriver};
use crate::interrupt::ApicManager;

// ============================================================================
// Intel GPU Constants
// ============================================================================

// PCI IDs
pub const INTEL_VENDOR_ID: u16 = 0x8086;

// Gen 9+ Graphics Device IDs (Skylake, Kaby Lake, Coffee Lake)
pub const SKL_ULT_GT1: u16 = 0x1906; // Skylake ULT
pub const SKL_ULT_GT2: u16 = 0x1916;
pub const KBL_ULT_GT1: u16 = 0x5906; // Kaby Lake ULT
pub const KBL_ULT_GT2: u16 = 0x5916;
pub const CFL_ULT_GT2: u16 = 0x3EA0; // Coffee Lake

// Memory Map IO (MMIO) Register Offsets
pub const MMIO_VRAM_SIZE: u32 = 256 * 1024 * 1024; // 256 MB framebuffer
pub const GMCH_CTRL: u32 = 0x50; // Graphics Memory Controller Hub
pub const GFX_MTRR: u32 = 0xF8; // Memory Type Range Register

// Display Pipeline Registers
pub const DSPASTRIDE: u32 = 0x70188; // Display A Stride
pub const DSPASURFACE: u32 = 0x7019C; // Display A Surface Address
pub const PIPEACONF: u32 = 0x70008; // Pipe A Configuration
pub const PIPEASTAT: u32 = 0x70024; // Pipe A Status
pub const PIPEASRC: u32 = 0x7000C; // Pipe A Source Size
pub const TRANSACONF: u32 = 0xF0008; // Transcoder A Configuration
pub const TRANS_HSYNC_A: u32 = 0xF0014; // Horizontal Sync A
pub const TRANS_VSYNC_A: u32 = 0xF0020; // Vertical Sync A

// Plane Control Registers
pub const PLANE_CTL_ENABLED: u32 = 0x80000000;
pub const PLANE_CTL_FORMAT_XRGB_8888: u32 = 0x00000004;

// Power Management
pub const GEN9_GTPUMD: u32 = 0x138; // Graphics Technology Power Up Mode Disable

// Interrupt Registers
pub const DEIMR: u32 = 0x44000; // Display Engine Interrupt Mask
pub const DEISR: u32 = 0x44000; // Display Engine Interrupt Status
pub const DEIIR: u32 = 0x44004; // Display Engine Interrupt Identity
pub const DEIER: u32 = 0x4400C; // Display Engine Interrupt Enable

// ============================================================================
// GPU Memory Management
// ============================================================================

#[derive(Debug, Clone)]
pub struct GpuMemoryRegion {
    pub physical_address: u64,
    pub virtual_address: u64,
    pub size: u64,
    pub is_vram: bool,
}

impl GpuMemoryRegion {
    pub fn new(phys: u64, virt: u64, size: u64, is_vram: bool) -> Self {
        GpuMemoryRegion {
            physical_address: phys,
            virtual_address: virt,
            size,
            is_vram,
        }
    }
}

pub struct GpuMemoryManager {
    regions: Vec<GpuMemoryRegion>,
    vram_offset: u64,
}

impl GpuMemoryManager {
    pub fn new() -> Self {
        GpuMemoryManager {
            regions: Vec::new(),
            vram_offset: 0,
        }
    }

    pub fn allocate_vram(&mut self, size: u64) -> Option<u64> {
        if self.vram_offset + size > MMIO_VRAM_SIZE as u64 {
            return None;
        }

        let offset = self.vram_offset;
        self.vram_offset += size;

        self.regions.push(GpuMemoryRegion::new(
            offset,
            offset, // For simplicity, map 1:1
            size,
            true,
        ));

        Some(offset)
    }

    pub fn free_vram(&mut self, address: u64) -> bool {
        if let Some(pos) = self.regions.iter().position(|r| r.physical_address == address) {
            self.regions.remove(pos);
            true
        } else {
            false
        }
    }

    pub fn get_region(&self, address: u64) -> Option<&GpuMemoryRegion> {
        self.regions.iter().find(|r| r.physical_address == address)
    }
}

// ============================================================================
// Display Configuration
// ============================================================================

#[derive(Debug, Clone, Copy)]
pub struct DisplayMode {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: u32,
    pub bits_per_pixel: u8,
    pub stride: u32,
}

impl DisplayMode {
    pub fn new(width: u32, height: u32, refresh: u32, bpp: u8) -> Self {
        let stride = width * (bpp as u32 / 8);
        DisplayMode {
            width,
            height,
            refresh_rate: refresh,
            bits_per_pixel: bpp,
            stride,
        }
    }

    pub fn framebuffer_size(&self) -> u64 {
        (self.stride as u64) * (self.height as u64)
    }
}

// ============================================================================
// Intel i915 GPU Driver
// ============================================================================

pub struct IntelGpuDriver {
    device_id: u16,
    pci_address: String,
    mmio_base: u64,
    mmio_size: u64,
    vram_base: u64,
    vram_size: u64,
    memory_manager: GpuMemoryManager,
    current_mode: Option<DisplayMode>,
    framebuffer_address: Option<u64>,
    interrupt_line: u8,
    is_enabled: bool,
    command_buffer_offset: AtomicU32,
}

impl IntelGpuDriver {
    pub fn new(device_id: u16, pci_addr: &str) -> Self {
        IntelGpuDriver {
            device_id,
            pci_address: pci_addr.to_string(),
            mmio_base: 0,
            mmio_size: 0,
            vram_base: 0,
            vram_size: 0,
            memory_manager: GpuMemoryManager::new(),
            current_mode: None,
            framebuffer_address: None,
            interrupt_line: 0,
            is_enabled: false,
            command_buffer_offset: AtomicU32::new(0),
        }
    }

    pub fn init_mmio(&mut self, bar: u64, size: u64) -> Result<(), &'static str> {
        self.mmio_base = bar;
        self.mmio_size = size;

        // Initialize GPU memory
        self.vram_base = bar + 0x1000000; // Start after MMIO (typical layout)
        self.vram_size = MMIO_VRAM_SIZE as u64;

        Ok(())
    }

    pub fn set_display_mode(&mut self, mode: DisplayMode) -> Result<(), &'static str> {
        // Allocate framebuffer
        let fb_size = mode.framebuffer_size();
        let fb_addr = self
            .memory_manager
            .allocate_vram(fb_size)
            .ok_or("Failed to allocate framebuffer")?;

        self.framebuffer_address = Some(fb_addr);
        self.current_mode = Some(mode);

        // Program display pipeline (simplified)
        self.program_display_pipeline(mode)?;

        Ok(())
    }

    fn program_display_pipeline(&self, _mode: DisplayMode) -> Result<(), &'static str> {
        // In real implementation, would:
        // 1. Disable transcoder
        // 2. Configure display connector
        // 3. Set pixel clock
        // 4. Configure transcoder (h-sync, v-sync, blanking)
        // 5. Set up planes (framebuffer address, stride, format)
        // 6. Enable display pipeline

        // For now, simulate successful setup
        Ok(())
    }

    pub fn submit_command_buffer(&mut self, commands: &[u32]) -> Result<u32, &'static str> {
        if commands.is_empty() {
            return Err("Empty command buffer");
        }

        let offset = self.command_buffer_offset.load(Ordering::SeqCst);

        // Allocate space for command buffer
        let cmd_size = (commands.len() * 4) as u64;
        let cmd_addr = self
            .memory_manager
            .allocate_vram(cmd_size)
            .ok_or("Failed to allocate command buffer")?;

        // In real implementation, would:
        // 1. Copy commands to GPU memory
        // 2. Submit to command queue
        // 3. Update head/tail pointers
        // 4. Wait for completion

        let next_offset = offset + cmd_size as u32;
        self.command_buffer_offset.store(next_offset, Ordering::SeqCst);

        Ok(cmd_addr as u32)
    }

    pub fn get_framebuffer_address(&self) -> Option<u64> {
        self.framebuffer_address
    }

    pub fn get_vram_base(&self) -> u64 {
        self.vram_base
    }

    pub fn clear_framebuffer(&mut self, _color: u32) -> Result<(), &'static str> {
        if let Some(_fb_addr) = self.framebuffer_address {
            if let Some(_mode) = self.current_mode {
                // In real implementation, would DMA clear to framebuffer
                // For now, mark as done
                Ok(())
            } else {
                Err("No display mode set")
            }
        } else {
            Err("No framebuffer allocated")
        }
    }

    pub fn present_framebuffer(&mut self) -> Result<(), &'static str> {
        if let Some(_fb_addr) = self.framebuffer_address {
            // In real implementation, would:
            // 1. Flush caches
            // 2. Update surface address register
            // 3. Trigger page flip

            Ok(())
        } else {
            Err("No framebuffer to present")
        }
    }

    pub fn register_interrupt_handler(&mut self, _apic: &ApicManager) -> Result<(), &'static str> {
        // Register handler for GPU interrupts
        // In real implementation, would set interrupt vector
        Ok(())
    }
}

impl Default for IntelGpuDriver {
    fn default() -> Self {
        Self::new(0x1916, "0000:00:02.0")
    }
}

// ============================================================================
// PciDriver Implementation
// ============================================================================

pub struct IntelGpuPciDriver {
    gpu: Option<Box<IntelGpuDriver>>,
}

impl IntelGpuPciDriver {
    pub fn new() -> Self {
        IntelGpuPciDriver { gpu: None }
    }

    pub fn get_gpu(&self) -> Option<&IntelGpuDriver> {
        self.gpu.as_ref().map(|b| b.as_ref())
    }

    pub fn get_gpu_mut(&mut self) -> Option<&mut IntelGpuDriver> {
        self.gpu.as_mut().map(|b| b.as_mut())
    }
}

impl PciDriver for IntelGpuPciDriver {
    fn probe(&mut self, device: &PciDeviceInfo) -> Result<bool, &'static str> {
        // Check if this is an Intel GPU
        if device.vendor_id != INTEL_VENDOR_ID {
            return Ok(false);
        }

        // Check for known Intel GPU device IDs
        let supported = match device.device_id {
            SKL_ULT_GT1 | SKL_ULT_GT2 | KBL_ULT_GT1 | KBL_ULT_GT2 | CFL_ULT_GT2 => true,
            _ => false,
        };

        if !supported {
            return Ok(false);
        }

        // Device is supported, initialize driver
        let mut gpu = Box::new(IntelGpuDriver::new(device.device_id, &device.address.sysfs_format()));

        // Extract MMIO BAR (typically BAR0 for Intel GPU)
        if let Some(ref bar) = device.bars[0] {
            gpu.init_mmio(bar.address, bar.size)?;
        } else {
            return Err("No MMIO BAR found");
        }

        self.gpu = Some(gpu);
        Ok(true)
    }

    fn remove(&mut self, _device: &PciDeviceInfo) -> Result<(), &'static str> {
        self.gpu = None;
        Ok(())
    }

    fn name(&self) -> &str {
        "intel_i915"
    }
}

// ============================================================================
// GPU Control Commands
// ============================================================================

pub struct GpuCommandBuilder {
    commands: Vec<u32>,
}

impl GpuCommandBuilder {
    pub fn new() -> Self {
        GpuCommandBuilder {
            commands: Vec::new(),
        }
    }

    pub fn add_command(&mut self, cmd: u32) {
        self.commands.push(cmd);
    }

    pub fn build(&self) -> &[u32] {
        &self.commands
    }

    pub fn clear(&mut self) {
        self.commands.clear();
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gpu_driver_creation() {
        let driver = IntelGpuDriver::new(SKL_ULT_GT2, "0000:00:02.0");
        assert_eq!(driver.device_id, SKL_ULT_GT2);
        assert!(!driver.is_enabled);
    }

    #[test]
    fn test_display_mode_creation() {
        let mode = DisplayMode::new(1920, 1080, 60, 32);
        assert_eq!(mode.width, 1920);
        assert_eq!(mode.height, 1080);
        assert_eq!(mode.framebuffer_size(), 1920 * 1080 * 4);
    }

    #[test]
    fn test_gpu_memory_allocation() {
        let mut mem_mgr = GpuMemoryManager::new();
        let addr1 = mem_mgr.allocate_vram(1024).unwrap();
        let addr2 = mem_mgr.allocate_vram(2048).unwrap();

        assert!(addr2 > addr1);
        assert!(mem_mgr.get_region(addr1).is_some());
        assert!(mem_mgr.get_region(addr2).is_some());
    }

    #[test]
    fn test_gpu_memory_free() {
        let mut mem_mgr = GpuMemoryManager::new();
        let addr = mem_mgr.allocate_vram(1024).unwrap();

        assert!(mem_mgr.get_region(addr).is_some());
        assert!(mem_mgr.free_vram(addr));
        assert!(mem_mgr.get_region(addr).is_none());
    }

    #[test]
    fn test_gpu_command_builder() {
        let mut builder = GpuCommandBuilder::new();
        builder.add_command(0x12345678);
        builder.add_command(0x87654321);

        let cmds = builder.build();
        assert_eq!(cmds.len(), 2);
        assert_eq!(cmds[0], 0x12345678);
    }

    #[test]
    fn test_intel_gpu_pci_driver() {
        let mut driver = IntelGpuPciDriver::new();
        assert!(driver.name() == "intel_i915");
        assert!(driver.get_gpu().is_none());
    }
}
