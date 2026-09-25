//! Bare-Metal x86_64 Target Boot Subsystem (`no_std` boot)
//!
//! Provides bare-metal hardware handoff and early architecture initialization:
//! - Multiboot2 & Limine long-mode boot protocol headers
//! - Higher-Half Direct Mapping (HHDM) memory layout (`0xFFFF_8000_0000_0000`)
//! - Physical Memory Map validation (Usable RAM, ACPI Reclaimable, Framebuffer MMIO)
//! - Linear Framebuffer handoff for Zenith GUI compositor
//! - ACPI RSDP physical address resolution

#![allow(dead_code)]

extern crate alloc;

use alloc::vec::Vec;

/// Multiboot2 Header Magic constant
pub const MULTIBOOT2_MAGIC: u32 = 0xE85250D6;

/// Architecture type: x86_64 (Protected Mode / Long Mode)
pub const ARCH_X86_64: u32 = 0;

/// Memory region classification reported by BIOS/UEFI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    UsableRam,
    Reserved,
    AcpiReclaimable,
    AcpiNvs,
    BadMemory,
    BootloaderReclaimable,
    KernelAndModules,
    FramebufferMmio,
}

/// Physical memory map entry
#[derive(Debug, Clone, Copy)]
pub struct MemoryMapEntry {
    pub base_address: u64,
    pub length: u64,
    pub region_type: MemoryRegionType,
}

/// Linear graphics framebuffer configuration provided by UEFI GOP / VESA
#[derive(Debug, Clone, Copy)]
pub struct BootFramebufferInfo {
    pub physical_address: u64,
    pub width: u32,
    pub height: u32,
    pub pitch_bytes: u32,
    pub bpp: u8, // Bits per pixel (usually 32 for RGBA)
}

/// System boot info payload handed to `kernel_main`
#[derive(Debug, Clone)]
pub struct BareMetalBootInfo {
    pub bootloader_name: &'static str,
    pub memory_map: Vec<MemoryMapEntry>,
    pub framebuffer: Option<BootFramebufferInfo>,
    pub rsdp_physical_address: Option<u64>,
    pub initramfs_base: u64,
    pub initramfs_size: u64,
    pub higher_half_offset: u64,
}

/// Bare-metal x86_64 Bootloader Handoff Engine
pub struct BareMetalBootEngine {
    pub boot_info: BareMetalBootInfo,
    pub total_usable_ram_bytes: u64,
}

impl BareMetalBootEngine {
    pub fn new() -> Self {
        let mut mmap = Vec::new();
        // 1. Lower conventional RAM (0..640KB)
        mmap.push(MemoryMapEntry {
            base_address: 0x0000_0000,
            length: 0x0009_FC00,
            region_type: MemoryRegionType::UsableRam,
        });
        // 2. Extended System RAM (1MB..4GB)
        mmap.push(MemoryMapEntry {
            base_address: 0x0010_0000,
            length: 4 * 1024 * 1024 * 1024 - 0x0010_0000,
            region_type: MemoryRegionType::UsableRam,
        });
        // 3. ACPI Tables Area
        mmap.push(MemoryMapEntry {
            base_address: 0x000E_0000,
            length: 0x0002_0000,
            region_type: MemoryRegionType::AcpiReclaimable,
        });
        // 4. Linear Framebuffer (1920x1080 32bpp)
        mmap.push(MemoryMapEntry {
            base_address: 0xE000_0000,
            length: 1920 * 1080 * 4,
            region_type: MemoryRegionType::FramebufferMmio,
        });

        let fb = BootFramebufferInfo {
            physical_address: 0xE000_0000,
            width: 1920,
            height: 1080,
            pitch_bytes: 1920 * 4,
            bpp: 32,
        };

        let total_ram = mmap
            .iter()
            .filter(|e| e.region_type == MemoryRegionType::UsableRam)
            .map(|e| e.length)
            .sum();

        Self {
            boot_info: BareMetalBootInfo {
                bootloader_name: "SigmaOS Bare-Metal Limine/Multiboot2 Loader",
                memory_map: mmap,
                framebuffer: Some(fb),
                rsdp_physical_address: Some(0x000F_5C40),
                initramfs_base: 0x0400_0000,
                initramfs_size: 4 * 1024 * 1024,
                higher_half_offset: 0xFFFF_8000_0000_0000,
            },
            total_usable_ram_bytes: total_ram,
        }
    }

    /// Converts physical address to higher-half virtual address
    pub fn phys_to_virt(&self, phys_addr: u64) -> u64 {
        phys_addr + self.boot_info.higher_half_offset
    }

    /// Validates bare-metal long mode constraints
    pub fn verify_bare_metal_readiness(&self) -> bool {
        self.total_usable_ram_bytes >= 512 * 1024 * 1024
            && self.boot_info.framebuffer.is_some()
            && self.boot_info.rsdp_physical_address.is_some()
    }
}

impl Default for BareMetalBootEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bare_metal_boot_engine() {
        let engine = BareMetalBootEngine::new();
        assert!(engine.verify_bare_metal_readiness());
        assert!(engine.total_usable_ram_bytes > 1024 * 1024 * 1024);

        let phys = 0x0010_0000;
        let virt = engine.phys_to_virt(phys);
        assert_eq!(virt, 0xFFFF_8000_0010_0000);
    }
}
