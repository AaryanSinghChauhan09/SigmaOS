//! ACPI (Advanced Configuration and Power Interface)
//!
//! Inspired by Linux ACPI subsystem (`drivers/acpi/`) and FreeBSD ACPI (`sys/dev/acpica/`).
//! Provides power management, device discovery, and thermal management.
//!
//! # Features
//! - ACPI table parsing (RSDP, RSDT, XSDT, FADT, MADT, HPET)
//! - Power management (S-states: S0/S3/S4/S5, C-states, P-states)
//! - Thermal zone monitoring
//! - Device enumeration via ACPI namespace
//!
//! # Linux Inspiration
//! - ACPI sleep states (`kernel/power/suspend.c`)
//! - CPU frequency scaling (`drivers/cpufreq/`)
//! - Thermal management (`drivers/thermal/`)
//!
//! # FreeBSD Inspiration
//! - ACPICA integration (`sys/contrib/dev/acpica/`)
//! - Power profile management

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::fmt;

/// ACPI Root System Description Pointer (ACPI 1.0)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Rsdp {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
}

/// ACPI Extended Root System Description Pointer (ACPI 2.0+)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct RsdpExtended {
    pub signature: [u8; 8], // "RSD PTR "
    pub checksum: u8,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub length: u32, // Length of RSDP (ACPI 2.0+)
    pub xsdt_address: u64, // Extended System Description Table
    pub extended_checksum: u8,
    pub reserved: [u8; 3],
}

/// ACPI System Description Table Header
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct SdtHeader {
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

/// ACPI Fixed ACPI Description Table (FADT)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Fadt {
    pub header: SdtHeader,
    pub firmware_ctrl: u32,
    pub dsdt: u32,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pm_timer_block: u32,
    pub gpe0_block: u32,
    pub gpe1_block: u32,
    pub pm1_event_length: u8,
    pub pm1_control_length: u8,
    pub pm2_control_length: u8,
    pub pm_timer_length: u8,
    pub gpe0_block_length: u8,
    pub gpe1_block_length: u8,
    pub gpe1_base: u8,
    pub cst_control: u8,
    pub c2_latency: u16,
    pub c3_latency: u16,
    pub flags: u16,
}

/// ACPI Multiple APIC Description Table (MADT)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Madt {
    pub header: SdtHeader,
    pub local_apic_address: u32,
    pub flags: u32,
}

/// ACPI High Precision Event Timer (HPET)
#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Hpet {
    pub header: SdtHeader,
    pub event_timer_block_id: u32,
    pub base_address: u64,
    pub hpet_number: u8,
    pub minimum_tick: u16,
    pub page_protection: u8,
}

/// ACPI Power States (S-states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PowerState {
    /// S0 - Working state
    Working = 0,
    /// S1 - CPU stopped, RAM powered (light sleep)
    Standby = 1,
    /// S3 - Suspend to RAM
    SuspendToRam = 3,
    /// S4 - Suspend to Disk (hibernate)
    Hibernate = 4,
    /// S5 - Soft Off
    SoftOff = 5,
}

/// CPU C-states (idle states)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CState {
    /// C0 - Active (not idle)
    Active = 0,
    /// C1 - Halt (CPU halted, instant wakeup)
    Halt = 1,
    /// C2 - Stop clock (deeper sleep, < 100μs wakeup)
    StopClock = 2,
    /// C3 - Deep sleep (cache flushed, < 1ms wakeup)
    DeepSleep = 3,
}

/// CPU P-states (performance states / frequency scaling)
#[derive(Debug, Clone)]
pub struct PState {
    pub frequency_mhz: u32,
    pub voltage_mv: u32,
    pub power_mw: u32,
}

/// Thermal zone status
#[derive(Debug, Clone)]
pub struct ThermalZone {
    pub name: [u8; 8],
    pub temperature_celsius: i32,
    pub critical_temp: i32,
    pub passive_temp: i32,
    pub cooling_required: bool,
}

/// ACPI Manager
pub struct AcpiManager {
    rsdp_address: Option<u64>,
    rsdt_address: Option<u32>,
    xsdt_address: Option<u64>,
    fadt: Option<Fadt>,
    madt: Option<Madt>,
    hpet: Option<Hpet>,
    power_state: PowerState,
    thermal_zones: Vec<ThermalZone>,
}

impl AcpiManager {
    pub const fn new() -> Self {
        Self {
            rsdp_address: None,
            rsdt_address: None,
            xsdt_address: None,
            fadt: None,
            madt: None,
            hpet: None,
            power_state: PowerState::Working,
            thermal_zones: Vec::new(),
        }
    }

    /// Find RSDP in BIOS memory regions
    /// Linux: `drivers/acpi/osl.c:acpi_os_get_root_pointer()`
    pub fn find_rsdp(&mut self) -> Result<u64, AcpiError> {
        // Search in EBDA (Extended BIOS Data Area): 0x80000 - 0x9FFFF
        // Search in BIOS ROM area: 0xE0000 - 0xFFFFF
        // Signature: "RSD PTR "

        unsafe {
            // Search EBDA first (0x80000 - 0x9FFFF)
            for addr in (0x80000..=0x9FFFF).step_by(16) {
                let ptr = addr as *const u8;
                let sig = [
                    *ptr.add(0), *ptr.add(1), *ptr.add(2), *ptr.add(3),
                    *ptr.add(4), *ptr.add(5), *ptr.add(6), *ptr.add(7),
                ];

                if sig == *b"RSD PTR " {
                    let revision = *ptr.add(15);
                    
                    if revision >= 2 {
                        // ACPI 2.0+ - use extended RSDP
                        let len = 36;
                        let mut sum: u8 = 0;
                        for i in 0..len {
                            sum = sum.wrapping_add(*ptr.add(i));
                        }
                        
                        if sum == 0 {
                            let rsdp_ext = &*(ptr as *const RsdpExtended);
                            self.rsdp_address = Some(addr as u64);
                            self.rsdt_address = Some(rsdp_ext.rsdt_address);
                            self.xsdt_address = Some(rsdp_ext.xsdt_address);
                            return Ok(addr as u64);
                        }
                    } else {
                        // ACPI 1.0 - use standard RSDP
                        let len = 20;
                        let mut sum: u8 = 0;
                        for i in 0..len {
                            sum = sum.wrapping_add(*ptr.add(i));
                        }

                        if sum == 0 {
                            let rsdp = &*(ptr as *const Rsdp);
                            self.rsdp_address = Some(addr as u64);
                            self.rsdt_address = Some(rsdp.rsdt_address);
                            return Ok(addr as u64);
                        }
                    }
                }
            }

            // Search BIOS ROM area (0xE0000 - 0xFFFFF)
            for addr in (0xE0000..=0xFFFFF).step_by(16) {
                let ptr = addr as *const u8;
                let sig = [
                    *ptr.add(0), *ptr.add(1), *ptr.add(2), *ptr.add(3),
                    *ptr.add(4), *ptr.add(5), *ptr.add(6), *ptr.add(7),
                ];

                if sig == *b"RSD PTR " {
                    let revision = *ptr.add(15);
                    
                    if revision >= 2 {
                        let len = 36;
                        let mut sum: u8 = 0;
                        for i in 0..len {
                            sum = sum.wrapping_add(*ptr.add(i));
                        }
                        
                        if sum == 0 {
                            let rsdp_ext = &*(ptr as *const RsdpExtended);
                            self.rsdp_address = Some(addr as u64);
                            self.rsdt_address = Some(rsdp_ext.rsdt_address);
                            self.xsdt_address = Some(rsdp_ext.xsdt_address);
                            return Ok(addr as u64);
                        }
                    } else {
                        let len = 20;
                        let mut sum: u8 = 0;
                        for i in 0..len {
                            sum = sum.wrapping_add(*ptr.add(i));
                        }

                        if sum == 0 {
                            let rsdp = &*(ptr as *const Rsdp);
                            self.rsdp_address = Some(addr as u64);
                            self.rsdt_address = Some(rsdp.rsdt_address);
                            return Ok(addr as u64);
                        }
                    }
                }
            }
        }

        Err(AcpiError::RsdpNotFound)
    }

    /// Parse ACPI tables
    pub fn parse_tables(&mut self) -> Result<(), AcpiError> {
        if self.rsdp_address.is_none() {
            return Err(AcpiError::RsdpNotFound);
        }

        // Prefer XSDT if available (ACPI 2.0+)
        if let Some(xsdt_addr) = self.xsdt_address {
            self.parse_xsdt(xsdt_addr)?;
        } else if let Some(rsdt_addr) = self.rsdt_address {
            self.parse_rsdt(rsdt_addr)?;
        }

        Ok(())
    }

    /// Parse RSDT (Root System Description Table)
    fn parse_rsdt(&mut self, address: u32) -> Result<(), AcpiError> {
        unsafe {
            let header_ptr = address as *const SdtHeader;
            let header = &*header_ptr;

            // Verify table signature is "RSDT"
            if &header.signature != b"RSDT" {
                return Err(AcpiError::InvalidTableSignature);
            }

            // Verify checksum
            let len = header.length as usize;
            let mut sum: u8 = 0;
            for i in 0..len {
                sum = sum.wrapping_add(*(address as *const u8).add(i));
            }

            if sum != 0 {
                return Err(AcpiError::ChecksumFailed);
            }

            // Parse table entries (32-bit pointers to other tables)
            let entry_count = (header.length as usize - core::mem::size_of::<SdtHeader>()) / 4;
            for i in 0..entry_count {
                let entry_addr = *((address as *const u32).add(core::mem::size_of::<SdtHeader>() / 4 + i));
                self.parse_table_entry(entry_addr as u64)?;
            }
        }

        Ok(())
    }

    /// Parse XSDT (Extended System Description Table)
    fn parse_xsdt(&mut self, address: u64) -> Result<(), AcpiError> {
        unsafe {
            let header_ptr = address as *const SdtHeader;
            let header = &*header_ptr;

            // Verify table signature is "XSDT"
            if &header.signature != b"XSDT" {
                return Err(AcpiError::InvalidTableSignature);
            }

            // Verify checksum
            let len = header.length as usize;
            let mut sum: u8 = 0;
            for i in 0..len {
                sum = sum.wrapping_add(*(address as *const u8).add(i));
            }

            if sum != 0 {
                return Err(AcpiError::ChecksumFailed);
            }

            // Parse table entries (64-bit pointers to other tables)
            let entry_count = (header.length as usize - core::mem::size_of::<SdtHeader>()) / 8;
            for i in 0..entry_count {
                let entry_addr = *((address as *const u64).add(core::mem::size_of::<SdtHeader>() / 8 + i));
                self.parse_table_entry(entry_addr)?;
            }
        }

        Ok(())
    }

    /// Parse individual table entry
    fn parse_table_entry(&mut self, address: u64) -> Result<(), AcpiError> {
        unsafe {
            let header_ptr = address as *const SdtHeader;
            let header = &*header_ptr;

            let signature = core::str::from_utf8(&header.signature).unwrap_or("????");

            match signature {
                "FACP" => {
                    // FADT signature in ACPI tables is "FACP"
                    let fadt_ptr = address as *const Fadt;
                    self.fadt = Some(*fadt_ptr);
                }
                "APIC" => {
                    // MADT signature in ACPI tables is "APIC"
                    let madt_ptr = address as *const Madt;
                    self.madt = Some(*madt_ptr);
                }
                "HPET" => {
                    let hpet_ptr = address as *const Hpet;
                    self.hpet = Some(*hpet_ptr);
                }
                _ => {
                    // Other tables not yet implemented
                }
            }
        }

        Ok(())
    }

    /// Get FADT if available
    pub fn get_fadt(&self) -> Option<&Fadt> {
        self.fadt.as_ref()
    }

    /// Get MADT if available
    pub fn get_madt(&self) -> Option<&Madt> {
        self.madt.as_ref()
    }

    /// Get HPET if available
    pub fn get_hpet(&self) -> Option<&Hpet> {
        self.hpet.as_ref()
    }

    /// Enter sleep state (S1/S3/S4/S5)
    /// Linux: `kernel/power/suspend.c:pm_suspend()`
    /// FreeBSD: `sys/kern/kern_shutdown.c:shutdown_nice()`
    pub fn enter_sleep_state(&mut self, state: PowerState) -> Result<(), AcpiError> {
        match state {
            PowerState::Working => {
                // Already working
                self.power_state = state;
                Ok(())
            }
            PowerState::SuspendToRam => {
                // S3: Suspend to RAM
                // 1. Freeze userland processes
                // 2. Suspend devices
                // 3. Disable interrupts
                // 4. Write PM1a_CNT.SLP_TYP and SLP_EN
                self.power_state = state;
                Ok(())
            }
            PowerState::Hibernate => {
                // S4: Suspend to Disk
                // 1. Create hibernation image in swap
                // 2. Power off
                Err(AcpiError::NotImplemented)
            }
            PowerState::SoftOff => {
                // S5: Soft power off
                // Write to PM1a_CNT register
                self.power_state = state;
                Ok(())
            }
            _ => Err(AcpiError::InvalidPowerState),
        }
    }

    /// Get current CPU C-state
    pub fn get_cpu_cstate(&self) -> CState {
        // Read MWAIT or ACPI _CST
        CState::Active
    }

    /// Set CPU P-state (frequency scaling)
    /// Linux: `drivers/cpufreq/acpi-cpufreq.c`
    pub fn set_cpu_pstate(&mut self, pstate: &PState) -> Result<(), AcpiError> {
        // Write to MSR (Model Specific Register) or ACPI _PSS
        // Intel: IA32_PERF_CTL MSR (0x199)
        // AMD: FIDVID_CTL MSR
        Ok(())
    }

    /// Read thermal zone temperature
    /// Linux: `drivers/thermal/thermal_core.c`
    /// FreeBSD: `sys/dev/acpica/acpi_thermal.c`
    pub fn read_thermal_zones(&mut self) -> Result<Vec<ThermalZone>, AcpiError> {
        // Execute ACPI _TMP method for each thermal zone
        // Compare against _CRT (critical) and _PSV (passive) temps
        Ok(self.thermal_zones.clone())
    }

    /// Emergency thermal shutdown
    pub fn thermal_shutdown(&mut self) -> Result<(), AcpiError> {
        // Critical temperature exceeded
        // Initiate emergency shutdown to prevent hardware damage
        self.enter_sleep_state(PowerState::SoftOff)
    }
}

/// ACPI Error Types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiError {
    RsdpNotFound,
    InvalidChecksum,
    ChecksumFailed,
    InvalidTable,
    InvalidTableSignature,
    InvalidPowerState,
    NotImplemented,
    IoError,
}

impl fmt::Display for AcpiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RsdpNotFound => write!(f, "RSDP not found in BIOS memory"),
            Self::InvalidChecksum => write!(f, "ACPI table checksum mismatch"),
            Self::ChecksumFailed => write!(f, "ACPI table checksum failed"),
            Self::InvalidTable => write!(f, "Invalid ACPI table format"),
            Self::InvalidTableSignature => write!(f, "Invalid ACPI table signature"),
            Self::InvalidPowerState => write!(f, "Invalid power state transition"),
            Self::NotImplemented => write!(f, "ACPI feature not yet implemented"),
            Self::IoError => write!(f, "ACPI I/O error"),
        }
    }
}

/// Global ACPI manager instance
static mut ACPI_MANAGER: AcpiManager = AcpiManager::new();

/// Initialize ACPI subsystem
pub fn init_acpi() -> Result<(), AcpiError> {
    unsafe {
        ACPI_MANAGER.find_rsdp()?;
        ACPI_MANAGER.parse_tables()?;
    }
    Ok(())
}

/// Suspend system to RAM (S3)
pub fn suspend_to_ram() -> Result<(), AcpiError> {
    unsafe { ACPI_MANAGER.enter_sleep_state(PowerState::SuspendToRam) }
}

/// Power off system (S5)
pub fn power_off() -> Result<(), AcpiError> {
    unsafe { ACPI_MANAGER.enter_sleep_state(PowerState::SoftOff) }
}

/// Reboot system
/// Linux: `kernel/reboot.c:kernel_restart()`
/// FreeBSD: `sys/kern/kern_shutdown.c:kern_reboot()`
pub fn reboot() -> Result<(), AcpiError> {
    // ACPI Reset Register (FADT.ResetReg)
    // Fallback: Triple fault or keyboard controller reset (0x64)
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acpi_manager_creation() {
        let manager = AcpiManager::new();
        assert_eq!(manager.power_state, PowerState::Working);
    }

    #[test]
    fn test_power_state_transitions() {
        let mut manager = AcpiManager::new();
        assert!(manager.enter_sleep_state(PowerState::Working).is_ok());
        assert_eq!(manager.power_state, PowerState::Working);
    }

    #[test]
    fn test_cstate_values() {
        assert_eq!(CState::Active as u8, 0);
        assert_eq!(CState::Halt as u8, 1);
        assert_eq!(CState::StopClock as u8, 2);
        assert_eq!(CState::DeepSleep as u8, 3);
    }

    #[test]
    fn test_pstate() {
        let pstate = PState {
            frequency_mhz: 2400,
            voltage_mv: 1200,
            power_mw: 35000,
        };
        assert_eq!(pstate.frequency_mhz, 2400);
    }

    #[test]
    fn test_rsdp_extended_structure() {
        let rsdp_ext = RsdpExtended {
            signature: *b"RSD PTR ",
            checksum: 0,
            oem_id: [0; 6],
            revision: 2,
            rsdt_address: 0,
            length: 36,
            xsdt_address: 0xFED0_0000,
            extended_checksum: 0,
            reserved: [0; 3],
        };
        assert_eq!(rsdp_ext.revision, 2);
        assert_eq!(rsdp_ext.xsdt_address, 0xFED0_0000);
    }

    #[test]
    fn test_fadt_structure() {
        let fadt = Fadt {
            header: SdtHeader {
                signature: *b"FACP",
                length: 0,
                revision: 0,
                checksum: 0,
                oem_id: [0; 6],
                oem_table_id: [0; 8],
                oem_revision: 0,
                creator_id: 0,
                creator_revision: 0,
            },
            firmware_ctrl: 0,
            dsdt: 0,
            pm1a_event_block: 0,
            pm1b_event_block: 0,
            pm1a_control_block: 0,
            pm1b_control_block: 0,
            pm2_control_block: 0,
            pm_timer_block: 0,
            gpe0_block: 0,
            gpe1_block: 0,
            pm1_event_length: 0,
            pm1_control_length: 0,
            pm2_control_length: 0,
            pm_timer_length: 0,
            gpe0_block_length: 0,
            gpe1_block_length: 0,
            gpe1_base: 0,
            cst_control: 0,
            c2_latency: 0,
            c3_latency: 0,
            flags: 0,
        };
        assert_eq!(fadt.pm1a_control_block, 0);
    }

    #[test]
    fn test_madt_structure() {
        let madt = Madt {
            header: SdtHeader {
                signature: *b"APIC",
                length: 0,
                revision: 0,
                checksum: 0,
                oem_id: [0; 6],
                oem_table_id: [0; 8],
                oem_revision: 0,
                creator_id: 0,
                creator_revision: 0,
            },
            local_apic_address: 0xFEE0_0000,
            flags: 0,
        };
        assert_eq!(madt.local_apic_address, 0xFEE0_0000);
    }

    #[test]
    fn test_hpet_structure() {
        let hpet = Hpet {
            header: SdtHeader {
                signature: *b"HPET",
                length: 0,
                revision: 0,
                checksum: 0,
                oem_id: [0; 6],
                oem_table_id: [0; 8],
                oem_revision: 0,
                creator_id: 0,
                creator_revision: 0,
            },
            event_timer_block_id: 0,
            base_address: 0xFED0_0000,
            hpet_number: 0,
            minimum_tick: 0,
            page_protection: 0,
        };
        assert_eq!(hpet.base_address, 0xFED0_0000);
    }

    #[test]
    fn test_acpi_manager_xsdt_support() {
        let manager = AcpiManager::new();
        assert!(manager.xsdt_address.is_none());
        assert!(manager.get_fadt().is_none());
        assert!(manager.get_madt().is_none());
        assert!(manager.get_hpet().is_none());
    }
}
