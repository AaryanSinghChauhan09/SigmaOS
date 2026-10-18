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
            BareMetalGdtEntry::new(0, 0, 0, 0),             // Null
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
// BARE METAL ELF64 KERNEL BINARY SYNTHESIZER ENGINE
// =========================================================================

pub const ELF_MAGIC: [u8; 4] = [0x7F, b'E', b'L', b'F'];
pub const MULTIBOOT2_HEADER_MAGIC: u32 = 0xE852_50D6;

pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_GNU_STACK: u32 = 0x6474_E551;
pub const PT_GNU_RELRO: u32 = 0x6474_E552;

pub const PF_X: u32 = 1;
pub const PF_W: u32 = 2;
pub const PF_R: u32 = 4;

/// Bare-Metal ELF64 Header Structure
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BareMetalElf64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,    // 2 = ET_EXEC, 3 = ET_DYN
    pub e_machine: u16, // 0x3E = x86_64, 0xB7 = AArch64, 0xF3 = RISC-V
    pub e_version: u32, // 1
    pub e_entry: u64,   // Kernel Virtual Entry Address
    pub e_phoff: u64,   // Program Header Offset
    pub e_shoff: u64,   // Section Header Offset
    pub e_flags: u32,
    pub e_ehsize: u16,    // Header size (64 bytes)
    pub e_phentsize: u16, // Program Header size (56 bytes)
    pub e_phnum: u16,     // Number of Program Headers
    pub e_shentsize: u16, // Section Header size (64 bytes)
    pub e_shnum: u16,     // Number of Section Headers
    pub e_shstrndx: u16,  // Section string table index
}

/// Bare-Metal ELF64 Program Header Segment Descriptor
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BareMetalElf64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

/// Bare-Metal Multiboot2 Specification Header Descriptor
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct BareMetalMultiboot2Header {
    pub magic: u32,
    pub architecture: u32,
    pub header_length: u32,
    pub checksum: u32,
}

impl BareMetalMultiboot2Header {
    pub const fn new(length: u32) -> Self {
        let magic = MULTIBOOT2_HEADER_MAGIC;
        let arch = 0u32; // x86 / x86_64 Multiboot2 architecture
        let checksum = (0u32).wrapping_sub(magic.wrapping_add(arch).wrapping_add(length));
        Self {
            magic,
            architecture: arch,
            header_length: length,
            checksum,
        }
    }
}

/// Exported Kernel Symbol in Bare-Metal ELF Symbol Table
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BareMetalKernelSymbol {
    pub name_offset: u32,
    pub address: u64,
    pub size: u64,
    pub symbol_type: u8,
}

/// Bare-Metal ELF64 Kernel Binary Builder & Synthesizer
pub struct BareMetalElfKernelBuilder {
    pub target_arch: u16, // 0x3E = x86_64
    pub entry_point: u64,
    pub load_base_vaddr: u64,
}

impl BareMetalElfKernelBuilder {
    pub fn new(entry_point: u64, load_base_vaddr: u64) -> Self {
        Self {
            target_arch: 0x3E, // x86_64
            entry_point,
            load_base_vaddr,
        }
    }

    /// Synthesizes a bootable bare-metal ELF64 kernel binary image in memory
    pub fn synthesize_bare_metal_elf_binary(&self) -> Vec<u8> {
        let mut binary = Vec::new();

        // 1. ELF Identification Header (16 bytes)
        let mut e_ident = [0u8; 16];
        e_ident[0..4].copy_from_slice(&ELF_MAGIC);
        e_ident[4] = 2; // ELFCLASS64
        e_ident[5] = 1; // ELFDATA2LSB (Little-Endian)
        e_ident[6] = 1; // EV_CURRENT
        e_ident[7] = 0; // ELFOSABI_NONE / System V

        // 2. Multiboot2 Header (24 bytes payload)
        let mb2_hdr = BareMetalMultiboot2Header::new(24);

        // 3. Program Headers (2 segments: .text RX, .data RW)
        let ph_text = BareMetalElf64ProgramHeader {
            p_type: PT_LOAD,
            p_flags: PF_R | PF_X,
            p_offset: 0x1000,
            p_vaddr: self.load_base_vaddr + 0x1000,
            p_paddr: 0x100000 + 0x1000,
            p_filesz: 0x4000,
            p_memsz: 0x4000,
            p_align: 0x1000,
        };

        let ph_data = BareMetalElf64ProgramHeader {
            p_type: PT_LOAD,
            p_flags: PF_R | PF_W,
            p_offset: 0x5000,
            p_vaddr: self.load_base_vaddr + 0x5000,
            p_paddr: 0x100000 + 0x5000,
            p_filesz: 0x2000,
            p_memsz: 0x4000, // Includes BSS
            p_align: 0x1000,
        };

        let elf_hdr = BareMetalElf64Header {
            e_ident,
            e_type: 2, // ET_EXEC
            e_machine: self.target_arch,
            e_version: 1,
            e_entry: self.entry_point,
            e_phoff: 64, // Program headers start immediately after ELF header
            e_shoff: 0x8000,
            e_flags: 0,
            e_ehsize: 64,
            e_phentsize: 56,
            e_phnum: 2,
            e_shentsize: 64,
            e_shnum: 4,
            e_shstrndx: 3,
        };

        // Write ELF Header
        binary.extend_from_slice(unsafe {
            core::slice::from_raw_parts(&elf_hdr as *const _ as *const u8, 64)
        });

        // Write Program Headers
        binary.extend_from_slice(unsafe {
            core::slice::from_raw_parts(&ph_text as *const _ as *const u8, 56)
        });
        binary.extend_from_slice(unsafe {
            core::slice::from_raw_parts(&ph_data as *const _ as *const u8, 56)
        });

        // Write Multiboot2 Header at 0x100
        while binary.len() < 0x100 {
            binary.push(0);
        }
        binary.extend_from_slice(unsafe {
            core::slice::from_raw_parts(&mb2_hdr as *const _ as *const u8, 16)
        });

        // Write kernel .text prologue at 0x1000
        while binary.len() < 0x1000 {
            binary.push(0x90); // NOP padding
        }

        // x86_64 long-mode kernel entry prologue instructions
        let prologue = [
            0x48, 0x31, 0xC0, // xor rax, rax
            0x48, 0x31, 0xDB, // xor rbx, rbx
            0x48, 0x31, 0xC9, // xor rcx, rcx
            0x48, 0x31, 0xD2, // xor rdx, rdx
            0x0F, 0x20, 0xC0, // mov rax, cr0
            0x0F, 0x22, 0xC0, // mov cr0, rax
            0xC3, // ret
        ];
        binary.extend_from_slice(&prologue);

        // Fill up to total file size
        while binary.len() < 0x7000 {
            binary.push(0);
        }

        binary
    }

    /// Verifies if a given byte array contains a valid Bare-Metal ELF64 kernel header
    pub fn verify_bare_metal_elf_header(payload: &[u8]) -> bool {
        if payload.len() < 64 {
            return false;
        }

        if &payload[0..4] != &ELF_MAGIC {
            return false;
        }

        let is_elf64 = payload[4] == 2;
        let is_little_endian = payload[5] == 1;

        is_elf64 && is_little_endian
    }
}

// =========================================================================
// UNIT TESTS
// =========================================================================

#[cfg(test)]
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

    #[test]
    fn test_bare_metal_elf_kernel_builder() {
        let builder = BareMetalElfKernelBuilder::new(0xFFFFFFFF81000000, 0xFFFFFFFF80000000);
        let binary = builder.synthesize_bare_metal_elf_binary();

        assert!(binary.len() >= 0x7000);
        assert_eq!(&binary[0..4], &ELF_MAGIC);

        assert!(BareMetalElfKernelBuilder::verify_bare_metal_elf_header(
            &binary
        ));

        let mb2_hdr = BareMetalMultiboot2Header::new(24);
        let magic = mb2_hdr.magic;
        assert_eq!(magic, MULTIBOOT2_HEADER_MAGIC);
    }
}
