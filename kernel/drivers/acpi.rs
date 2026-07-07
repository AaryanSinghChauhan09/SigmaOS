//! SigmaOS — ACPI Table Parser
//! Discovers hardware topology from RSDP/RSDT/XSDT/MADT tables.
//! Pure no_std, zero-dependency implementation.

#![no_std]
#![allow(dead_code)]

type U8  = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type Usize = usize;

// ── RSDP (Root System Description Pointer) ──────────────────────────────────
const RSDP_SIG: [U8; 8] = *b"RSD PTR ";

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Rsdp {
    pub signature:  [U8; 8],
    pub checksum:   U8,
    pub oem_id:     [U8; 6],
    pub revision:   U8,
    pub rsdt_addr:  U32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Rsdp2 {
    pub v1:           Rsdp,
    pub length:       U32,
    pub xsdt_addr:    U64,
    pub ext_checksum: U8,
    pub reserved:     [U8; 3],
}

// ── SDT Header (common to all ACPI tables) ──────────────────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct SdtHeader {
    pub signature:    [U8; 4],
    pub length:       U32,
    pub revision:     U8,
    pub checksum:     U8,
    pub oem_id:       [U8; 6],
    pub oem_table_id: [U8; 8],
    pub oem_revision: U32,
    pub creator_id:   U32,
    pub creator_rev:  U32,
}

// ── MADT (Multiple APIC Description Table) ──────────────────────────────────
const MADT_SIG: [U8; 4] = *b"APIC";

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct Madt {
    pub header:           SdtHeader,
    pub local_apic_addr:  U32,
    pub flags:            U32,
    // Variable-length MADT entries follow
}

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MadtEntryType {
    LocalApic       = 0,
    IoApic          = 1,
    IntSourceOvr    = 2,
    NmiSource       = 3,
    LocalApicNmi    = 4,
    LocalApicOvr    = 5,
    IoSapic         = 6,
    LocalSapic      = 7,
    PlatformIntSrc  = 8,
    LocalX2Apic     = 9,
    LocalX2ApicNmi  = 10,
    GicCpu          = 11,
    GicDist         = 12,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtEntryHeader {
    pub entry_type: U8,
    pub length:     U8,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtLocalApic {
    pub header:    MadtEntryHeader,
    pub acpi_id:   U8,
    pub apic_id:   U8,
    pub flags:     U32,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct MadtIoApic {
    pub header:         MadtEntryHeader,
    pub io_apic_id:     U8,
    pub reserved:       U8,
    pub io_apic_addr:   U32,
    pub gsi_base:       U32,
}

// ── Discovered hardware info ────────────────────────────────────────────────
pub const MAX_CPUS: Usize = 256;
pub const MAX_IO_APICS: Usize = 16;

#[repr(C)]
pub struct AcpiInfo {
    pub cpu_count:       U32,
    pub cpu_apic_ids:    [U8; MAX_CPUS],
    pub io_apic_count:   U32,
    pub io_apic_addrs:   [U32; MAX_IO_APICS],
    pub local_apic_addr: U32,
    pub has_xsdt:        bool,
}

impl AcpiInfo {
    pub const fn zero() -> Self {
        AcpiInfo {
            cpu_count: 0,
            cpu_apic_ids: [0; MAX_CPUS],
            io_apic_count: 0,
            io_apic_addrs: [0; MAX_IO_APICS],
            local_apic_addr: 0,
            has_xsdt: false,
        }
    }
}

static mut ACPI: AcpiInfo = AcpiInfo::zero();

// ── Checksum validation ─────────────────────────────────────────────────────
unsafe fn acpi_checksum(ptr: *const U8, len: Usize) -> bool {
    let mut sum: U8 = 0;
    for i in 0..len {
        sum = sum.wrapping_add(*ptr.add(i));
    }
    sum == 0
}

// ── RSDP validation ─────────────────────────────────────────────────────────
unsafe fn validate_rsdp(rsdp: *const Rsdp) -> bool {
    if rsdp.is_null() { return false; }
    let r = &*rsdp;
    if r.signature != RSDP_SIG { return false; }
    acpi_checksum(rsdp as *const U8, core::mem::size_of::<Rsdp>())
}

// ── Parse MADT entries ──────────────────────────────────────────────────────
unsafe fn parse_madt(madt_ptr: *const Madt) {
    let madt = &*madt_ptr;
    let info = &mut ACPI;
    info.local_apic_addr = madt.local_apic_addr;

    let table_end = (madt_ptr as *const U8).add(madt.header.length as Usize);
    let mut ptr = (madt_ptr as *const U8).add(core::mem::size_of::<Madt>());

    while ptr < table_end {
        let entry = &*(ptr as *const MadtEntryHeader);
        if entry.length < 2 { break; }

        match entry.entry_type {
            0 => {
                // Local APIC — represents a CPU core
                let lapic = &*(ptr as *const MadtLocalApic);
                if lapic.flags & 1 != 0 || lapic.flags & 2 != 0 {
                    let idx = info.cpu_count as Usize;
                    if idx < MAX_CPUS {
                        info.cpu_apic_ids[idx] = lapic.apic_id;
                        info.cpu_count += 1;
                    }
                }
            }
            1 => {
                // I/O APIC
                let ioapic = &*(ptr as *const MadtIoApic);
                let idx = info.io_apic_count as Usize;
                if idx < MAX_IO_APICS {
                    info.io_apic_addrs[idx] = ioapic.io_apic_addr;
                    info.io_apic_count += 1;
                }
            }
            _ => {}
        }
        ptr = ptr.add(entry.length as Usize);
    }
}

// ── Walk RSDT/XSDT to find MADT ────────────────────────────────────────────
unsafe fn walk_rsdt(rsdt_addr: U64) {
    let hdr = &*(rsdt_addr as *const SdtHeader);
    let entry_count = (hdr.length as Usize - core::mem::size_of::<SdtHeader>()) / 4;
    let entries = (rsdt_addr as *const U8).add(core::mem::size_of::<SdtHeader>()) as *const U32;

    for i in 0..entry_count {
        let table_addr = *entries.add(i) as U64;
        let table_hdr = &*(table_addr as *const SdtHeader);
        if table_hdr.signature == MADT_SIG {
            parse_madt(table_addr as *const Madt);
        }
    }
}

unsafe fn walk_xsdt(xsdt_addr: U64) {
    let hdr = &*(xsdt_addr as *const SdtHeader);
    let entry_count = (hdr.length as Usize - core::mem::size_of::<SdtHeader>()) / 8;
    let entries = (xsdt_addr as *const U8).add(core::mem::size_of::<SdtHeader>()) as *const U64;

    for i in 0..entry_count {
        let table_addr = *entries.add(i);
        let table_hdr = &*(table_addr as *const SdtHeader);
        if table_hdr.signature == MADT_SIG {
            parse_madt(table_addr as *const Madt);
        }
    }
}

// ── Public API ──────────────────────────────────────────────────────────────

/// Initialize ACPI subsystem from RSDP address (provided by bootloader).
#[no_mangle]
pub unsafe extern "C" fn acpi_init(rsdp_phys: U64) -> i32 {
    ACPI = AcpiInfo::zero();

    let rsdp = rsdp_phys as *const Rsdp;
    if !validate_rsdp(rsdp) { return -1; }

    let r = &*rsdp;
    if r.revision >= 2 {
        let rsdp2 = &*(rsdp_phys as *const Rsdp2);
        if rsdp2.xsdt_addr != 0 {
            ACPI.has_xsdt = true;
            walk_xsdt(rsdp2.xsdt_addr);
            return 0;
        }
    }

    if r.rsdt_addr != 0 {
        walk_rsdt(r.rsdt_addr as U64);
    }
    0
}

/// Return number of discovered CPU cores.
#[no_mangle]
pub unsafe extern "C" fn acpi_cpu_count() -> U32 {
    ACPI.cpu_count
}

/// Return number of I/O APICs.
#[no_mangle]
pub unsafe extern "C" fn acpi_ioapic_count() -> U32 {
    ACPI.io_apic_count
}

/// Return the local APIC base address.
#[no_mangle]
pub unsafe extern "C" fn acpi_lapic_addr() -> U32 {
    ACPI.local_apic_addr
}
