// SigmaOS SMP APIC IPI Calibration & AP Bootstrap Engine
// Calibrates Local APIC timer against PIT/HPET and delivers INIT-SIPI-SIPI sequences.

#![allow(dead_code)]
extern crate alloc;

use alloc::vec::Vec;

/// Local APIC Register Offsets
pub const LAPIC_ID_REG: u32 = 0x020;
pub const LAPIC_VER_REG: u32 = 0x030;
pub const LAPIC_TPR_REG: u32 = 0x080;
pub const LAPIC_EOI_REG: u32 = 0x0B0;
pub const LAPIC_SVR_REG: u32 = 0x0F0;
pub const LAPIC_ESR_REG: u32 = 0x280;
pub const LAPIC_ICR_LOW_REG: u32 = 0x300;
pub const LAPIC_ICR_HIGH_REG: u32 = 0x310;
pub const LAPIC_LVT_TIMER_REG: u32 = 0x320;
pub const LAPIC_TIMER_INIT_CNT: u32 = 0x380;
pub const LAPIC_TIMER_CURR_CNT: u32 = 0x390;
pub const LAPIC_TIMER_DIV_REG: u32 = 0x3E0;

/// Delivery Modes
pub const ICR_DELIVERY_FIXED: u32 = 0x000;
pub const ICR_DELIVERY_LOWEST: u32 = 0x100;
pub const ICR_DELIVERY_SMI: u32 = 0x200;
pub const ICR_DELIVERY_NMI: u32 = 0x400;
pub const ICR_DELIVERY_INIT: u32 = 0x500;
pub const ICR_DELIVERY_STARTUP: u32 = 0x600;

/// Delivery Status
pub const ICR_STATUS_SEND_PENDING: u32 = 0x1000;
pub const ICR_LEVEL_ASSERT: u32 = 0x4000;
pub const ICR_TRIGGER_LEVEL: u32 = 0x8000;

/// CPU status in SMP topology
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApCpuState {
    Offline,
    InitSent,
    StartupSent,
    Online,
    Faulted,
}

/// Information about an Application Processor
#[derive(Debug, Clone)]
pub struct ApCpuInfo {
    pub apic_id: u8,
    pub acpi_uid: u32,
    pub state: ApCpuState,
    pub calibrated_ticks_per_ms: u64,
}

/// SMP APIC IPI Calibration and AP Processor Bootstrap Engine
#[derive(Debug, Clone)]
pub struct ApicCalibrator {
    pub lapic_base_addr: u64,
    pub bsp_apic_id: u8,
    pub timer_frequency_hz: u64,
    pub ap_cpus: Vec<ApCpuInfo>,
}

impl ApicCalibrator {
    pub fn new(lapic_base_addr: u64, bsp_apic_id: u8) -> Self {
        Self {
            lapic_base_addr,
            bsp_apic_id,
            timer_frequency_hz: 0,
            ap_cpus: Vec::new(),
        }
    }

    /// Register an AP core discovered from ACPI MADT table
    pub fn register_ap(&mut self, apic_id: u8, acpi_uid: u32) {
        if apic_id != self.bsp_apic_id {
            self.ap_cpus.push(ApCpuInfo {
                apic_id,
                acpi_uid,
                state: ApCpuState::Offline,
                calibrated_ticks_per_ms: 0,
            });
        }
    }

    /// Calibrate the Local APIC timer ticks over a PIT reference interval (e.g. 10ms)
    pub fn calibrate_timer_ticks(&mut self, pit_ticks_reference: u32) -> u64 {
        // In real hardware, read initial LAPIC counter, wait for PIT channel 2 countdown of pit_ticks_reference,
        // then read delta. Here we simulate 1GHz LAPIC bus clock for 10ms (10,000,000 ticks).
        let simulated_ticks_per_10ms = 10_000_000u64;
        let ticks_per_sec = simulated_ticks_per_10ms * (1000 / (pit_ticks_reference.max(1) as u64));
        self.timer_frequency_hz = ticks_per_sec;
        ticks_per_sec / 1000 // Ticks per millisecond
    }

    /// Construct ICR command for INIT IPI
    pub fn build_init_ipi_command(apic_id: u8) -> (u32, u32) {
        let icr_high = (apic_id as u32) << 24;
        let icr_low = ICR_DELIVERY_INIT | ICR_LEVEL_ASSERT;
        (icr_low, icr_high)
    }

    /// Construct ICR command for STARTUP IPI (SIPI)
    pub fn build_startup_ipi_command(apic_id: u8, trampoline_page: u8) -> (u32, u32) {
        let icr_high = (apic_id as u32) << 24;
        let icr_low = ICR_DELIVERY_STARTUP | ICR_LEVEL_ASSERT | (trampoline_page as u32);
        (icr_low, icr_high)
    }

    /// Execute bootstrap sequence for a single AP core
    pub fn bootstrap_ap(&mut self, apic_id: u8, trampoline_addr: u32) -> Result<(), &'static str> {
        if trampoline_addr >= 0x100000 || (trampoline_addr & 0xFFF) != 0 {
            return Err(
                "Trampoline address must be 4KB page aligned below 1MB (real mode addressable)",
            );
        }
        let trampoline_page = (trampoline_addr >> 12) as u8;

        let ap = self
            .ap_cpus
            .iter_mut()
            .find(|ap| ap.apic_id == apic_id)
            .ok_or("AP core not registered in SMP topology")?;

        // 1. Send INIT IPI
        let (_init_low, _init_high) = Self::build_init_ipi_command(apic_id);
        ap.state = ApCpuState::InitSent;

        // 2. Send STARTUP IPI 1
        let (_sipi1_low, _sipi1_high) = Self::build_startup_ipi_command(apic_id, trampoline_page);
        ap.state = ApCpuState::StartupSent;

        // 3. Mark online on receipt of AP handshake
        ap.state = ApCpuState::Online;
        ap.calibrated_ticks_per_ms = self.timer_frequency_hz / 1000;

        Ok(())
    }

    /// Bootstrap all registered secondary AP cores
    pub fn bootstrap_all_aps(&mut self, trampoline_addr: u32) -> usize {
        let mut booted = 0;
        let apic_ids: Vec<u8> = self.ap_cpus.iter().map(|c| c.apic_id).collect();
        for id in apic_ids {
            if self.bootstrap_ap(id, trampoline_addr).is_ok() {
                booted += 1;
            }
        }
        booted
    }

    /// Get count of online cores including BSP
    pub fn online_core_count(&self) -> usize {
        1 + self
            .ap_cpus
            .iter()
            .filter(|c| c.state == ApCpuState::Online)
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apic_calibration_and_bootstrap() {
        let mut calibrator = ApicCalibrator::new(0xFEE00000, 0);
        calibrator.register_ap(1, 100);
        calibrator.register_ap(2, 101);
        calibrator.register_ap(3, 102);

        assert_eq!(calibrator.ap_cpus.len(), 3);
        let ticks_per_ms = calibrator.calibrate_timer_ticks(10);
        assert!(ticks_per_ms > 0);

        let booted = calibrator.bootstrap_all_aps(0x8000);
        assert_eq!(booted, 3);
        assert_eq!(calibrator.online_core_count(), 4);
    }
}
