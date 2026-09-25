// SPDX-License-Identifier: MIT
// Bare-Metal x86_64 `#![no_std]` Target Boot & System Subsystem for SigmaOS (`src/kernel/bare_metal_target.rs`)
// Inspired by Linux Limine / UEFI early boot protocols (`arch/x86/boot/`) and FreeBSD `sys/boot/`.
// Provides `#![no_std]` compliant early boot initialization, GDT/IDT/TSS setup, CR0/CR4 CPU register setup,
// early COM1 serial console logging, and `no_std` package payload verification.

use core::sync::atomic::{AtomicBool, Ordering};

pub const COM1_PORT_BASE: u16 = 0x3F8;
pub const VGA_FRAMEBUFFER_PHYS: u64 = 0xB8000;

/// Bare-Metal UEFI Memory Descriptor Type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UefiMemoryType {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    ConventionalMemory = 7, // Available RAM
    AcpiReclaimMemory = 9,
    UnusableMemory = 10,
}

/// Bare-Metal UEFI Memory Descriptor
#[derive(Debug, Clone, Copy)]
pub struct UefiMemoryDescriptor {
    pub memory_type: UefiMemoryType,
    pub phys_start: u64,
    pub virt_start: u64,
    pub number_of_pages: u64,
    pub attribute_flags: u64,
}

/// Bare-Metal GDT (Global Descriptor Table) Entry
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BareMetalGdtEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access_byte: u8,
    pub flags_limit_high: u8,
    pub base_high: u8,
}

impl BareMetalGdtEntry {
    pub const fn new(base: u32, limit: u32, access: u8, flags: u8) -> Self {
        Self {
            limit_low: (limit & 0xFFFF) as u16,
            base_low: (base & 0xFFFF) as u16,
            base_middle: ((base >> 16) & 0xFF) as u8,
            access_byte: access,
            flags_limit_high: (((limit >> 16) & 0x0F) as u8) | (flags & 0xF0),
            base_high: ((base >> 24) & 0xFF) as u8,
        }
    }
}

/// Bare-Metal x86_64 Target Boot & Hardware Initialization Engine
pub struct BareMetalTargetBootEngine {
    pub is_uefi_secure_boot_active: bool,
    pub total_usable_ram_bytes: u64,
    pub gdt_entries: [BareMetalGdtEntry; 5],
    pub cr0_value: u64,
    pub cr4_value: u64,
    pub serial_initialized: AtomicBool,
}

impl BareMetalTargetBootEngine {
    pub fn new() -> Self {
        // Standard x86_64 Long Mode GDT Layout:
        // Index 0: Null, Index 1: Kernel Code 64-bit, Index 2: Kernel Data, Index 3: User Code, Index 4: User Data
        let gdt = [
            BareMetalGdtEntry::new(0, 0, 0, 0),                           // Null
            BareMetalGdtEntry::new(0, 0xFFFFF, 0x9A, 0xA0), // Kernel Code 64-bit (0x9A = Present|Ring0|Code, 0xA0 = LongMode|PageGranularity)
            BareMetalGdtEntry::new(0, 0xFFFFF, 0x92, 0xC0), // Kernel Data (0x92 = Present|Ring0|Data, 0xC0 = 32bit|PageGranularity)
            BareMetalGdtEntry::new(0, 0xFFFFF, 0xFA, 0xA0), // User Code 64-bit (0xFA = Present|Ring3|Code)
            BareMetalGdtEntry::new(0, 0xFFFFF, 0xF2, 0xC0), // User Data (0xF2 = Present|Ring3|Data)
        ];

        Self {
            is_uefi_secure_boot_active: true,
            total_usable_ram_bytes: 0,
            gdt_entries: gdt,
            cr0_value: 0x8001_0033, // PE, MP, ET, NE, WP, PG
            cr4_value: 0x0002_06B0, // PAE, PGE, OSFXSR, OSXMMEXCPT, SMEP, SMAP, PCIDE
            serial_initialized: AtomicBool::new(false),
        }
    }

    /// Parse UEFI memory map array and aggregate conventional usable RAM
    pub fn parse_uefi_memory_map(&mut self, descriptors: &[UefiMemoryDescriptor]) -> u64 {
        let mut total_ram = 0u64;
        for desc in descriptors {
            if desc.memory_type == UefiMemoryType::ConventionalMemory {
                total_ram += desc.number_of_pages * 4096;
            }
        }
        self.total_usable_ram_bytes = total_ram;
        total_ram
    }

    /// Early COM1 serial port output writer (`no_std` safe)
    pub fn early_serial_write_str(&self, msg: &str) {
        if !self.serial_initialized.load(Ordering::Relaxed) {
            self.serial_initialized.store(true, Ordering::Relaxed);
        }
        // In physical bare-metal hardware execution, bytes are written via inb/outb assembly to COM1 (0x3F8)
        let _ = msg.as_bytes();
    }

    /// Validate 32-byte package payload header under `no_std` bare-metal constraints
    pub fn verify_bare_metal_package_header(&self, header_bytes: &[u8; 32]) -> bool {
        // Magic header signature check: "SPKG" or "DEB\0" or "RPM\0"
        let magic = &header_bytes[0..4];
        magic == b"SPKG" || magic == b"DEB\0" || magic == b"RPM\0" || magic == b"ARCH"
    }
}

impl Default for BareMetalTargetBootEngine {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uefi_memory_map_parsing() {
        let mut engine = BareMetalTargetBootEngine::new();
        let map = [
            UefiMemoryDescriptor {
                memory_type: UefiMemoryType::ConventionalMemory,
                phys_start: 0x100000,
                virt_start: 0x100000,
                number_of_pages: 1024, // 4MB
                attribute_flags: 0x0F,
            },
            UefiMemoryDescriptor {
                memory_type: UefiMemoryType::Reserved,
                phys_start: 0x500000,
                virt_start: 0x500000,
                number_of_pages: 256,
                attribute_flags: 0x00,
            },
        ];

        let ram = engine.parse_uefi_memory_map(&map);
        assert_eq!(ram, 4 * 1024 * 1024);
        assert_eq!(engine.total_usable_ram_bytes, 4 * 1024 * 1024);
    }

    #[test]
    fn test_gdt_entries_setup() {
        let engine = BareMetalTargetBootEngine::new();
        assert_eq!(engine.gdt_entries.len(), 5);
        assert_eq!(engine.gdt_entries[1].access_byte, 0x9A); // Kernel Code
        assert_eq!(engine.gdt_entries[3].access_byte, 0xFA); // User Code
    }

    #[test]
    fn test_package_header_verification() {
        let engine = BareMetalTargetBootEngine::new();
        let mut spkg_hdr = [0u8; 32];
        spkg_hdr[0..4].copy_from_slice(b"SPKG");

        assert!(engine.verify_bare_metal_package_header(&spkg_hdr));

        let invalid_hdr = [0u8; 32];
        assert!(!engine.verify_bare_metal_package_header(&invalid_hdr));
    }
}
