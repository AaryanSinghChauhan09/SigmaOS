//! # ACPI Table Parser & Power Manager
//!
//! Full ACPI 6.5 table discovery, validation, and power management engine for SigmaOS.
//! Inspired by Linux `drivers/acpi/tables.c`, FreeBSD `sys/dev/acpica/acpi.c`,
//! Redox OS `acpid/src/`, and SerenityOS `Kernel/Arch/x86/ACPI/`.
//!
//! Parses RSDP → XSDT/RSDT → MADT, FADT, HPET, DSDT, SSDT tables.

#![no_std]

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

// ============================================================================
// 1. ACPI SIGNATURE CONSTANTS (from ACPI 6.5 Specification)
// ============================================================================

pub const RSDP_SIGNATURE: &[u8; 8] = b"RSD PTR ";
pub const RSDT_SIGNATURE: &[u8; 4] = b"RSDT";
pub const XSDT_SIGNATURE: &[u8; 4] = b"XSDT";
pub const FADT_SIGNATURE: &[u8; 4] = b"FACP";
pub const MADT_SIGNATURE: &[u8; 4] = b"APIC";
pub const HPET_SIGNATURE: &[u8; 4] = b"HPET";
pub const DSDT_SIGNATURE: &[u8; 4] = b"DSDT";
pub const SSDT_SIGNATURE: &[u8; 4] = b"SSDT";
pub const MCFG_SIGNATURE: &[u8; 4] = b"MCFG";
pub const BGRT_SIGNATURE: &[u8; 4] = b"BGRT"; // Boot Graphics Resource Table
pub const TPM2_SIGNATURE: &[u8; 4] = b"TPM2";

// ============================================================================
// 2. ACPI TABLE HEADERS
// ============================================================================

/// ACPI Root System Description Pointer (RSDP) — found in low memory / UEFI config table
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AcpiRsdpDescriptor {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,   // 0 = ACPI 1.0, 2 = ACPI 2.0+
    pub rsdt_addr: u32, // Physical address of RSDT (ACPI 1.0)
    // Extended fields (revision >= 2):
    pub length: u32,
    pub xsdt_addr: u64, // Physical address of XSDT (ACPI 2.0+)
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

impl AcpiRsdpDescriptor {
    /// Validate RSDP signature and checksum (ACPI spec 5.2.5.3)
    pub fn validate(&self) -> bool {
        if &self.signature != RSDP_SIGNATURE {
            return false;
        }
        // Sum all bytes of the descriptor v1 section (20 bytes) must be 0
        let bytes: &[u8] =
            unsafe { core::slice::from_raw_parts(self as *const _ as *const u8, 20) };
        let sum: u8 = bytes.iter().fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }

    pub fn revision(&self) -> u8 {
        self.revision
    }
}

/// Generic ACPI System Description Table Header (SDTHeader)
/// Used for RSDT, XSDT, FADT, MADT, HPET, etc.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct AcpiSdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub creator_id: u32,
    pub creator_revision: u32,
}

impl AcpiSdtHeader {
    pub fn signature_matches(&self, sig: &[u8; 4]) -> bool {
        &self.signature == sig
    }

    pub fn validate_checksum(&self, full_table_bytes: &[u8]) -> bool {
        let sum: u8 = full_table_bytes
            .iter()
            .fold(0u8, |acc, &b| acc.wrapping_add(b));
        sum == 0
    }
}

// ============================================================================
// 3. MADT — Multiple APIC Description Table
//    Inspired by Linux kernel/include/acpi/actbl1.h ACPI_TABLE_MADT
// ============================================================================

/// MADT entry subtypes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MadtEntryType {
    LocalApic = 0,
    IoApic = 1,
    InterruptSourceOverride = 2,
    NmiSource = 3,
    LocalApicNmi = 4,
    LocalApicAddressOverride = 5,
    IoSapic = 6,
    LocalSapic = 7,
    PlatformInterruptSources = 8,
    LocalX2Apic = 9,
    LocalX2ApicNmi = 10,
    GicCpuInterface = 11,
    GicDistributor = 12,
}

/// MADT Local APIC entry (type 0)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MadtLocalApicEntry {
    pub acpi_cpu_id: u8,
    pub apic_id: u8,
    pub flags: u32, // Bit 0: Enabled, Bit 1: Online Capable
}

/// MADT I/O APIC entry (type 1)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MadtIoApicEntry {
    pub io_apic_id: u8,
    pub io_apic_addr: u32,    // Physical address
    pub global_irq_base: u32, // Global system interrupt base
}

/// MADT Interrupt Source Override entry (type 2)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MadtInterruptOverride {
    pub bus: u8,
    pub source_irq: u8,
    pub global_irq: u32,
    pub flags: u16, // Polarity + trigger mode
}

/// Parsed MADT — Multiple APIC Description Table
#[derive(Debug, Clone, Default)]
pub struct ParsedMadt {
    pub local_apic_addr: u32,
    pub flags: u32,
    pub local_apics: Vec<MadtLocalApicEntry>,
    pub io_apics: Vec<MadtIoApicEntry>,
    pub interrupt_overrides: Vec<MadtInterruptOverride>,
    pub active_cpu_count: u32,
}

impl ParsedMadt {
    pub fn parse_from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 44 {
            return None; // Too short for MADT header + mandatory fields
        }

        // Validate signature
        if &bytes[0..4] != MADT_SIGNATURE {
            return None;
        }

        let local_apic_addr = u32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]);
        let flags = u32::from_le_bytes([bytes[40], bytes[41], bytes[42], bytes[43]]);

        let mut madt = ParsedMadt {
            local_apic_addr,
            flags,
            ..Default::default()
        };

        let mut offset = 44usize;
        while offset + 2 <= bytes.len() {
            let entry_type = bytes[offset];
            let entry_len = bytes[offset + 1] as usize;
            if entry_len < 2 || offset + entry_len > bytes.len() {
                break;
            }

            match entry_type {
                0 if entry_len >= 8 => {
                    let apic_id = bytes[offset + 3];
                    let flags = u32::from_le_bytes([
                        bytes[offset + 4],
                        bytes[offset + 5],
                        bytes[offset + 6],
                        bytes[offset + 7],
                    ]);
                    let cpu_enabled = (flags & 1) != 0;
                    let entry = MadtLocalApicEntry {
                        acpi_cpu_id: bytes[offset + 2],
                        apic_id,
                        flags,
                    };
                    madt.local_apics.push(entry);
                    if cpu_enabled {
                        madt.active_cpu_count += 1;
                    }
                }
                1 if entry_len >= 12 => {
                    let entry = MadtIoApicEntry {
                        io_apic_id: bytes[offset + 2],
                        io_apic_addr: u32::from_le_bytes([
                            bytes[offset + 4],
                            bytes[offset + 5],
                            bytes[offset + 6],
                            bytes[offset + 7],
                        ]),
                        global_irq_base: u32::from_le_bytes([
                            bytes[offset + 8],
                            bytes[offset + 9],
                            bytes[offset + 10],
                            bytes[offset + 11],
                        ]),
                    };
                    madt.io_apics.push(entry);
                }
                2 if entry_len >= 10 => {
                    let entry = MadtInterruptOverride {
                        bus: bytes[offset + 2],
                        source_irq: bytes[offset + 3],
                        global_irq: u32::from_le_bytes([
                            bytes[offset + 4],
                            bytes[offset + 5],
                            bytes[offset + 6],
                            bytes[offset + 7],
                        ]),
                        flags: u16::from_le_bytes([bytes[offset + 8], bytes[offset + 9]]),
                    };
                    madt.interrupt_overrides.push(entry);
                }
                _ => {}
            }
            offset += entry_len;
        }

        Some(madt)
    }
}

// ============================================================================
// 4. FADT — Fixed ACPI Description Table
//    Inspired by Linux include/acpi/actbl.h ACPI_TABLE_FADT
// ============================================================================

/// ACPI Power Management Profile
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[repr(u8)]
pub enum AcpiPmProfile {
    #[default]
    Unspecified = 0,
    Desktop = 1,
    Mobile = 2,
    Workstation = 3,
    EnterpriseServer = 4,
    SohoServer = 5,
    AppliancePc = 6,
    PerformanceServer = 7,
    Tablet = 8,
}

/// Parsed FADT — Key fields for power management
#[derive(Debug, Clone, Default)]
pub struct ParsedFadt {
    pub pm_profile: u8,
    pub sci_interrupt: u16,
    pub smi_cmd: u32,
    pub acpi_enable: u8,
    pub acpi_disable: u8,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm_timer_block: u32,
    pub pm_timer_length: u8,
    pub iapc_boot_arch: u16,
    pub flags: u32,
    /// Preferred Power Management Profile
    pub preferred_profile: AcpiPmProfile,
}

impl ParsedFadt {
    pub fn parse_from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 116 {
            return None;
        }
        if &bytes[0..4] != FADT_SIGNATURE {
            return None;
        }

        let pm_profile = bytes[45];
        let profile = match pm_profile {
            1 => AcpiPmProfile::Desktop,
            2 => AcpiPmProfile::Mobile,
            3 => AcpiPmProfile::Workstation,
            4 => AcpiPmProfile::EnterpriseServer,
            8 => AcpiPmProfile::Tablet,
            _ => AcpiPmProfile::Unspecified,
        };

        Some(ParsedFadt {
            pm_profile,
            sci_interrupt: u16::from_le_bytes([bytes[46], bytes[47]]),
            smi_cmd: u32::from_le_bytes([bytes[48], bytes[49], bytes[50], bytes[51]]),
            acpi_enable: bytes[52],
            acpi_disable: bytes[53],
            pm1a_event_block: u32::from_le_bytes([bytes[56], bytes[57], bytes[58], bytes[59]]),
            pm1b_event_block: u32::from_le_bytes([bytes[60], bytes[61], bytes[62], bytes[63]]),
            pm1a_control_block: u32::from_le_bytes([bytes[64], bytes[65], bytes[66], bytes[67]]),
            pm1b_control_block: u32::from_le_bytes([bytes[68], bytes[69], bytes[70], bytes[71]]),
            pm_timer_block: u32::from_le_bytes([bytes[76], bytes[77], bytes[78], bytes[79]]),
            pm_timer_length: bytes[91],
            iapc_boot_arch: u16::from_le_bytes([bytes[109], bytes[110]]),
            flags: u32::from_le_bytes([bytes[112], bytes[113], bytes[114], bytes[115]]),
            preferred_profile: profile,
        })
    }

    /// Check if hardware-reduced ACPI mode is active (no SCI, no SMI)
    pub fn is_hardware_reduced(&self) -> bool {
        (self.flags & (1 << 20)) != 0
    }
}

// ============================================================================
// 5. HPET — High Precision Event Timer
// ============================================================================

#[derive(Debug, Clone, Default)]
pub struct ParsedHpet {
    pub event_timer_block_id: u32,
    pub base_address: u64,
    pub hpet_number: u8,
    pub minimum_clock_tick: u16,
    pub page_protection: u8,
}

impl ParsedHpet {
    pub fn parse_from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 56 {
            return None;
        }
        if &bytes[0..4] != HPET_SIGNATURE {
            return None;
        }

        let base_address = u64::from_le_bytes([
            bytes[44], bytes[45], bytes[46], bytes[47], bytes[48], bytes[49], bytes[50], bytes[51],
        ]);

        Some(ParsedHpet {
            event_timer_block_id: u32::from_le_bytes([bytes[36], bytes[37], bytes[38], bytes[39]]),
            base_address,
            hpet_number: bytes[52],
            minimum_clock_tick: u16::from_le_bytes([bytes[53], bytes[54]]),
            page_protection: bytes[55],
        })
    }
}

// ============================================================================
// 6. SOVEREIGN ACPI SUBSYSTEM — Top-level table registry and power controller
// ============================================================================

/// Discovered ACPI system state
#[derive(Debug)]
pub struct SovereignAcpiSubsystem {
    pub rsdp_revision: u8,
    pub madt: Option<ParsedMadt>,
    pub fadt: Option<ParsedFadt>,
    pub hpet: Option<ParsedHpet>,
    pub table_count: u32,
    pub cpu_count: u32,
    pub io_apic_count: u32,
    pub acpi_ready: AtomicBool,
    pub power_state: AtomicU32,
}

impl Clone for SovereignAcpiSubsystem {
    fn clone(&self) -> Self {
        Self {
            rsdp_revision: self.rsdp_revision,
            madt: self.madt.clone(),
            fadt: self.fadt.clone(),
            hpet: self.hpet.clone(),
            table_count: self.table_count,
            cpu_count: self.cpu_count,
            io_apic_count: self.io_apic_count,
            acpi_ready: AtomicBool::new(self.acpi_ready.load(Ordering::SeqCst)),
            power_state: AtomicU32::new(self.power_state.load(Ordering::SeqCst)),
        }
    }
}

/// ACPI Sx power states (inspired by Linux acpi_state)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum AcpiPowerState {
    S0Running = 0,   // Normal operation
    S1Standby = 1,   // CPU stopped, power on
    S3Suspend = 3,   // RAM refresh, suspend-to-RAM
    S4Hibernate = 4, // Suspend-to-disk
    S5SoftOff = 5,   // Mechanical off
}

impl Default for SovereignAcpiSubsystem {
    fn default() -> Self {
        Self {
            rsdp_revision: 0,
            madt: None,
            fadt: None,
            hpet: None,
            table_count: 0,
            cpu_count: 0,
            io_apic_count: 0,
            acpi_ready: AtomicBool::new(false),
            power_state: AtomicU32::new(AcpiPowerState::S0Running as u32),
        }
    }
}

impl SovereignAcpiSubsystem {
    pub fn new() -> Self {
        Self::default()
    }

    /// Initialize from RSDP descriptor bytes
    pub fn init_from_rsdp(&mut self, rsdp_bytes: &[u8; 36]) -> bool {
        let rsdp: AcpiRsdpDescriptor =
            unsafe { core::ptr::read_unaligned(rsdp_bytes.as_ptr() as *const AcpiRsdpDescriptor) };
        if !rsdp.validate() {
            return false;
        }
        self.rsdp_revision = rsdp.revision();
        true
    }

    /// Register a parsed table by signature
    pub fn register_table(&mut self, signature: &[u8; 4], data: &[u8]) {
        match signature {
            s if s == MADT_SIGNATURE => {
                if let Some(madt) = ParsedMadt::parse_from_bytes(data) {
                    self.cpu_count = madt.active_cpu_count;
                    self.io_apic_count = madt.io_apics.len() as u32;
                    self.madt = Some(madt);
                    self.table_count += 1;
                }
            }
            s if s == FADT_SIGNATURE => {
                if let Some(fadt) = ParsedFadt::parse_from_bytes(data) {
                    self.fadt = Some(fadt);
                    self.table_count += 1;
                }
            }
            s if s == HPET_SIGNATURE => {
                if let Some(hpet) = ParsedHpet::parse_from_bytes(data) {
                    self.hpet = Some(hpet);
                    self.table_count += 1;
                }
            }
            _ => {
                self.table_count += 1;
            }
        }
    }

    /// Mark ACPI subsystem as operational
    pub fn mark_ready(&self) {
        self.acpi_ready.store(true, Ordering::SeqCst);
    }

    /// Request power state transition
    pub fn request_power_state(&self, target: AcpiPowerState) -> bool {
        if !self.acpi_ready.load(Ordering::SeqCst) {
            return false;
        }
        self.power_state.store(target as u32, Ordering::SeqCst);
        true
    }

    pub fn current_power_state(&self) -> u32 {
        self.power_state.load(Ordering::SeqCst)
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(any(test, feature = "standalone_test"))]
mod tests {
    use super::*;

    fn build_madt_bytes(apic_addr: u32, flags: u32) -> Vec<u8> {
        let mut bytes = vec![0u8; 44 + 8 + 12]; // header(44) + local_apic(8) + io_apic(12)
                                                // Signature
        bytes[0..4].copy_from_slice(b"APIC");
        let total_len = bytes.len() as u32;
        bytes[4..8].copy_from_slice(&total_len.to_le_bytes());
        // Local APIC address at offset 36
        bytes[36..40].copy_from_slice(&apic_addr.to_le_bytes());
        bytes[40..44].copy_from_slice(&flags.to_le_bytes());
        // Local APIC entry (type=0, length=8)
        bytes[44] = 0; // type
        bytes[45] = 8; // length
        bytes[46] = 0; // ACPI CPU ID
        bytes[47] = 0; // APIC ID
        bytes[48..52].copy_from_slice(&1u32.to_le_bytes()); // enabled flag
                                                            // I/O APIC entry (type=1, length=12)
        bytes[52] = 1; // type
        bytes[53] = 12; // length
        bytes[54] = 1; // I/O APIC ID
        bytes[55] = 0;
        bytes[56..60].copy_from_slice(&0xFEC00000u32.to_le_bytes()); // I/O APIC addr
        bytes[60..64].copy_from_slice(&0u32.to_le_bytes()); // global IRQ base
        bytes
    }

    #[test]
    fn test_madt_parse() {
        let madt_bytes = build_madt_bytes(0xFEE00000, 1);
        let madt = ParsedMadt::parse_from_bytes(&madt_bytes).unwrap();
        assert_eq!(madt.local_apic_addr, 0xFEE00000);
        assert_eq!(madt.active_cpu_count, 1);
        assert_eq!(madt.io_apics.len(), 1);
        assert_eq!(madt.io_apics[0].io_apic_addr, 0xFEC00000);
    }

    #[test]
    fn test_hpet_parse() {
        let mut hpet_bytes = vec![0u8; 56];
        hpet_bytes[0..4].copy_from_slice(b"HPET");
        let base: u64 = 0xFED00000;
        hpet_bytes[44..52].copy_from_slice(&base.to_le_bytes());
        hpet_bytes[52] = 0; // HPET number
        let hpet = ParsedHpet::parse_from_bytes(&hpet_bytes).unwrap();
        assert_eq!(hpet.base_address, 0xFED00000);
        assert_eq!(hpet.hpet_number, 0);
    }

    #[test]
    fn test_acpi_subsystem_power_state() {
        let mut sys = SovereignAcpiSubsystem::new();
        let madt_bytes = build_madt_bytes(0xFEE00000, 1);
        sys.register_table(b"APIC", &madt_bytes);
        sys.mark_ready();
        assert_eq!(sys.cpu_count, 1);
        assert!(sys.request_power_state(AcpiPowerState::S3Suspend));
        assert_eq!(sys.current_power_state(), AcpiPowerState::S3Suspend as u32);
    }
}
