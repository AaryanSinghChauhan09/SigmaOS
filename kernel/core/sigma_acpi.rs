// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_acpi.rs — ACPI table parser (RSDP→RSDT/XSDT→MADT/SRAT)
// Language: Rust #![no_std]
#![no_std]
#![allow(dead_code)]

/// RSDP (Root System Description Pointer)
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Rsdp {
    pub signature: [u8; 8],  // "RSD PTR "
    pub checksum:  u8,
    pub oem_id:    [u8; 6],
    pub revision:  u8,       // 0=ACPI 1.0, 2=ACPI 2.0+
    pub rsdt_addr: u32,
    // ACPI 2.0+ extension
    pub length:    u32,
    pub xsdt_addr: u64,
    pub ext_checksum: u8,
    _reserved: [u8; 3],
}

/// Generic ACPI System Description Table header
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct AcpiSdtHeader {
    pub signature: [u8; 4],
    pub length:    u32,
    pub revision:  u8,
    pub checksum:  u8,
    pub oem_id:    [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id:   u32,
    pub creator_rev:  u32,
}

/// MADT (Multiple APIC Description Table) header
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtHeader {
    pub header:       AcpiSdtHeader,
    pub lapic_addr:   u32,
    pub flags:        u32,
}

/// MADT entry header
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtEntry {
    pub entry_type: u8,
    pub length:     u8,
}

pub const MADT_LAPIC:  u8 = 0;
pub const MADT_IOAPIC: u8 = 1;
pub const MADT_ISO:    u8 = 2;   // Interrupt Source Override

/// Local APIC entry
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtLapic {
    pub header:     MadtEntry,
    pub acpi_id:    u8,
    pub apic_id:    u8,
    pub flags:      u32,
}

/// I/O APIC entry
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtIoApic {
    pub header:       MadtEntry,
    pub ioapic_id:    u8,
    _reserved:        u8,
    pub ioapic_addr:  u32,
    pub gsi_base:     u32,
}

pub const MAX_CPUS: usize = 64;

pub struct AcpiInfo {
    pub lapic_base:   u64,
    pub ioapic_base:  u64,
    pub cpu_count:    usize,
    pub apic_ids:     [u8; MAX_CPUS],
    pub initialized:  bool,
}

impl AcpiInfo {
    pub const fn new() -> Self {
        Self {
            lapic_base: 0xFEE00000,
            ioapic_base: 0xFEC00000,
            cpu_count: 1,
            apic_ids: [0u8; MAX_CPUS],
            initialized: false,
        }
    }
}

fn acpi_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

pub unsafe fn sigma_acpi_init(rsdp_phys: u64) -> AcpiInfo {
    let mut info = AcpiInfo::new();
    if rsdp_phys == 0 { info.initialized = true; return info; }

    let rsdp = &*(rsdp_phys as *const Rsdp);
    if &{ rsdp.signature } != b"RSD PTR " { return info; }

    // Use XSDT if ACPI 2.0+, else RSDT
    let (use_xsdt, table_addr) = if { rsdp.revision } >= 2 {
        (true, { rsdp.xsdt_addr })
    } else {
        (false, { rsdp.rsdt_addr } as u64)
    };

    let rsdt_hdr = &*(table_addr as *const AcpiSdtHeader);
    let rsdt_len = { rsdt_hdr.length } as usize;
    let entry_size = if use_xsdt { 8 } else { 4 };
    let n_entries = (rsdt_len - core::mem::size_of::<AcpiSdtHeader>()) / entry_size;

    let entries_base = table_addr as usize + core::mem::size_of::<AcpiSdtHeader>();

    for i in 0..n_entries {
        let entry_addr: u64 = if use_xsdt {
            core::ptr::read_unaligned((entries_base + i * 8) as *const u64)
        } else {
            core::ptr::read_unaligned((entries_base + i * 4) as *const u32) as u64
        };
        if entry_addr == 0 { continue; }

        let hdr = &*(entry_addr as *const AcpiSdtHeader);
        let sig = { hdr.signature };

        if &sig == b"APIC" {
            // Parse MADT
            let madt = &*(entry_addr as *const MadtHeader);
            info.lapic_base = { madt.lapic_addr } as u64;
            let madt_len = { madt.header.length } as usize;
            let mut off = core::mem::size_of::<MadtHeader>();
            while off + 2 <= madt_len {
                let e = &*((entry_addr as usize + off) as *const MadtEntry);
                let etype = { e.entry_type };
                let elen  = { e.length } as usize;
                if elen == 0 { break; }
                match etype {
                    MADT_LAPIC => {
                        let lapic = &*((entry_addr as usize + off) as *const MadtLapic);
                        if { lapic.flags } & 1 != 0 && info.cpu_count < MAX_CPUS {
                            info.apic_ids[info.cpu_count] = { lapic.apic_id };
                            info.cpu_count += 1;
                        }
                    }
                    MADT_IOAPIC => {
                        let ioapic = &*((entry_addr as usize + off) as *const MadtIoApic);
                        info.ioapic_base = { ioapic.ioapic_addr } as u64;
                    }
                    _ => {}
                }
                off += elen;
            }
        }
    }

    info.initialized = true;
    info
}

static mut G_ACPI: AcpiInfo = AcpiInfo::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_acpi_parse(rsdp_phys: u64) {
    G_ACPI = sigma_acpi_init(rsdp_phys);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_acpi_cpu_count() -> usize { G_ACPI.cpu_count }
#[no_mangle]
pub unsafe extern "C" fn sigma_acpi_lapic_base() -> u64 { G_ACPI.lapic_base }
#[no_mangle]
pub unsafe extern "C" fn sigma_acpi_ioapic_base() -> u64 { G_ACPI.ioapic_base }
