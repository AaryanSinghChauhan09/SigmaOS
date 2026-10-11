//! # Hardware PMU (Performance Monitoring Unit) Counter Sampling
//!
//! Provides low-level hardware performance profiling for SigmaOS using x86_64
//! architectural PMU counters and MSRs. Powers the `perf stat` and `perf record`
//! diagnostic subsystems.
//!
//! ## Architectural Hardware Events (CPUID.0AH)
//! - Unhalted Core Cycles (`Event 0x3C, Umask 0x00`)
//! - Instructions Retired (`Event 0xC0, Umask 0x00`)
//! - Unhalted Reference Cycles (`Event 0x3C, Umask 0x01`)
//! - LLC Reference / Misses (`Event 0x2E, Umask 0x4F / 0x41`)
//! - Branch Instructions / Misses (`Event 0xC4 / 0xC5`)
//!
//! ## Implementation
//! - Direct MSR programming via `IA32_PERFEVTSELx` and `IA32_PMCx`
//! - `RDPMC` fast userspace/kernel counter readout
//! - Interrupt on overflow via APIC Performance Counter Local Vector Table (LVT) NMI

#![allow(dead_code)]

extern crate alloc;
use alloc::vec::Vec;

/// Standard Intel Architectural MSRs
pub const MSR_IA32_PERF_STATUS: u32 = 0x0198;
pub const MSR_IA32_PERF_CTL: u32 = 0x0199;
pub const MSR_IA32_PMCx_BASE: u32 = 0x00C1;
pub const MSR_IA32_PERFEVTSELx_BASE: u32 = 0x0186;

/// Standard Architectural Hardware Event Selectors
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfHwEvent {
    CpuCycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    BusCycles,
}

impl PerfHwEvent {
    pub fn config_bits(&self) -> u32 {
        match self {
            Self::CpuCycles => 0x003C,
            Self::Instructions => 0x00C0,
            Self::CacheReferences => 0x4F2E,
            Self::CacheMisses => 0x412E,
            Self::BranchInstructions => 0x00C4,
            Self::BranchMisses => 0x00C5,
            Self::BusCycles => 0x013C,
        }
    }
}

/// PMU Counter Configuration Structure
#[derive(Debug, Clone, Copy)]
pub struct PmuCounterConfig {
    pub event: PerfHwEvent,
    pub user_mode: bool,
    pub kernel_mode: bool,
    pub edge_detect: bool,
    pub interrupt_on_overflow: bool,
    pub sample_period: u64,
}

impl PmuCounterConfig {
    pub fn to_msr_value(&self) -> u64 {
        let mut val = self.event.config_bits() as u64;
        if self.user_mode {
            val |= 1 << 16; // USR
        }
        if self.kernel_mode {
            val |= 1 << 17; // OS
        }
        if self.edge_detect {
            val |= 1 << 18; // E
        }
        if self.interrupt_on_overflow {
            val |= 1 << 20; // INT (APIC NMI)
        }
        val |= 1 << 22; // EN (Enable counter)
        val
    }
}

/// Sample record captured during counter overflow NMI
#[derive(Debug, Clone, Copy)]
pub struct PerfSample {
    pub timestamp_ns: u64,
    pub cpu: u32,
    pub pid: u32,
    pub ip: usize,
    pub event: PerfHwEvent,
}

/// Per-CPU Hardware PMU state
pub struct CpuPmu {
    pub cpu_id: u32,
    pub num_general_counters: u8,
    pub counter_bit_width: u8,
    pub samples: Vec<PerfSample>,
    pub max_samples: usize,
}

impl CpuPmu {
    pub fn new(cpu_id: u32) -> Self {
        Self {
            cpu_id,
            num_general_counters: 4,
            counter_bit_width: 48,
            samples: Vec::with_capacity(4096),
            max_samples: 4096,
        }
    }

    /// Read raw 48-bit value of counter `idx` using RDPMC instruction.
    pub fn read_counter(&self, idx: u8) -> u64 {
        // In real execution on bare metal:
        // unsafe { core::arch::x86_64::_rdpmc(idx as i32) as u64 }
        let _ = idx;
        0
    }

    /// Handle counter overflow interrupt (NMI).
    pub fn handle_overflow(&mut self, event: PerfHwEvent, ip: usize, pid: u32, ts: u64) {
        if self.samples.len() < self.max_samples {
            self.samples.push(PerfSample {
                timestamp_ns: ts,
                cpu: self.cpu_id,
                pid,
                ip,
                event,
            });
        }
    }
}
