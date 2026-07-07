// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/power/sigma_perf_governor.rs — CPU Frequency Governor
//
// Implements CPU frequency scaling and power management governor.
// Inspired by Linux cpufreq and Intel SpeedStep/Turbo Boost.
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU32, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of CPU cores.
const MAX_CPUS: SigmaUsize = 128;
/// Governor name length.
const GOV_NAME_LEN: SigmaUsize = 16;

// ── Governor Types ─────────────────────────────────────────────────────────────
#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum GovernorType {
    /// Performance governor (max frequency).
    Performance = 0,
    /// Powersave governor (min frequency).
    Powersave  = 1,
    /// Ondemand governor (dynamic).
    Ondemand   = 2,
    /// Conservative governor (gradual).
    Conservative = 3,
    /// Userspace governor (manual).
    Userspace  = 4,
}

// ── CPUFeatures ───────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPUFeatures {
    pub avx512f:       SigmaBool,
    pub avx512bw:      SigmaBool,
    pub avx512vl:      SigmaBool,
    pub avx2:          SigmaBool,
    pub bmi2:          SigmaBool,
    pub rdtsc:         SigmaBool,
    pub invariant_tsc: SigmaBool,
    pub base_freq_mhz: SigmaU32,
    pub boost_freq_mhz: SigmaU32,
    pub physical_cores: SigmaU32,
    pub logical_cores:  SigmaU32,
    pub llc_size_kb:    SigmaU32,
}

// ── CPUState ─────────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct CPUState {
    pub cpu_id:       SigmaU32,
    pub current_freq: SigmaU32,
    pub min_freq:     SigmaU32,
    pub max_freq:     SigmaU32,
    pub governor:     GovernorType,
    pub utilization:  SigmaU32,
    pub temperature:  SigmaU32,
}

// ── PerfGovernor ─────────────────────────────────────────────────────────────
pub struct PerfGovernor {
    /// CPU states per core.
    cpu_states:  [CPUState; MAX_CPUS],
    /// CPU features.
    features:    CPUFeatures,
    /// TSC frequency (Hz).
    tsc_freq:    SigmaU64,
    /// Initialized flag.
    initialized: SigmaBool,
}

impl PerfGovernor {
    pub const fn new() -> Self {
        Self {
            cpu_states: [CPUState {
                cpu_id: 0, current_freq: 0, min_freq: 0,
                max_freq: 0, governor: GovernorType::Ondemand,
                utilization: 0, temperature: 0,
            }; MAX_CPUS],
            features: CPUFeatures {
                avx512f: false, avx512bw: false, avx512vl: false,
                avx2: false, bmi2: false, rdtsc: false,
                invariant_tsc: false, base_freq_mhz: 0,
                boost_freq_mhz: 0, physical_cores: 0,
                logical_cores: 0, llc_size_kb: 0,
            },
            tsc_freq: 0,
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        self.detect_features();
        self.calibrate_tsc();
        self.init_cpu_states();
    }

    fn detect_features(&mut self) {
        self.features.rdtsc = true;
        self.features.invariant_tsc = true;
        self.features.base_freq_mhz = 2000;
        self.features.boost_freq_mhz = 4000;
        self.features.physical_cores = 4;
        self.features.logical_cores = 8;
    }

    fn calibrate_tsc(&mut self) {
        self.tsc_freq = (self.features.base_freq_mhz as SigmaU64) * 1_000_000;
    }

    fn init_cpu_states(&mut self) {
        for i in 0..MAX_CPUS {
            self.cpu_states[i].cpu_id = i as SigmaU32;
            self.cpu_states[i].min_freq = self.features.base_freq_mhz;
            self.cpu_states[i].max_freq = self.features.boost_freq_mhz;
            self.cpu_states[i].current_freq = self.features.base_freq_mhz;
            self.cpu_states[i].governor = GovernorType::Ondemand;
        }
    }

    pub fn set_governor(&mut self, cpu_id: SigmaU32, gov: GovernorType) -> SigmaI32 {
        if cpu_id as SigmaUsize >= MAX_CPUS {
            return -1;
        }
        self.cpu_states[cpu_id as SigmaUsize].governor = gov;
        0
    }

    pub fn get_governor(&self, cpu_id: SigmaU32) -> GovernorType {
        if cpu_id as SigmaUsize >= MAX_CPUS {
            GovernorType::Ondemand
        } else {
            self.cpu_states[cpu_id as SigmaUsize].governor
        }
    }

    pub fn set_frequency(&mut self, cpu_id: SigmaU32, freq_mhz: SigmaU32) -> SigmaI32 {
        if cpu_id as SigmaUsize >= MAX_CPUS {
            return -1;
        }
        let state = &mut self.cpu_states[cpu_id as SigmaUsize];
        if freq_mhz < state.min_freq || freq_mhz > state.max_freq {
            return -1;
        }
        state.current_freq = freq_mhz;
        0
    }

    pub fn get_frequency(&self, cpu_id: SigmaU32) -> SigmaU32 {
        if cpu_id as SigmaUsize >= MAX_CPUS {
            0
        } else {
            self.cpu_states[cpu_id as SigmaUsize].current_freq
        }
    }

    pub fn update_utilization(&mut self, cpu_id: SigmaU32, util: SigmaU32) {
        if cpu_id as SigmaUsize < MAX_CPUS {
            self.cpu_states[cpu_id as SigmaUsize].utilization = util.min(100);
            self.adjust_frequency(cpu_id);
        }
    }

    pub fn update_temperature(&mut self, cpu_id: SigmaU32, temp: SigmaU32) {
        if cpu_id as SigmaUsize < MAX_CPUS {
            self.cpu_states[cpu_id as SigmaUsize].temperature = temp;
            if temp > 90 {
                self.thermal_throttle(cpu_id);
            }
        }
    }

    fn adjust_frequency(&mut self, cpu_id: SigmaU32) {
        let idx = cpu_id as SigmaUsize;
        if idx >= MAX_CPUS {
            return;
        }

        let state = self.cpu_states[idx];
        match state.governor {
            GovernorType::Performance => {
                self.cpu_states[idx].current_freq = state.max_freq;
            }
            GovernorType::Powersave => {
                self.cpu_states[idx].current_freq = state.min_freq;
            }
            GovernorType::Ondemand => {
                if state.utilization > 80 {
                    self.cpu_states[idx].current_freq = state.max_freq;
                } else if state.utilization < 20 {
                    self.cpu_states[idx].current_freq = state.min_freq;
                }
            }
            GovernorType::Conservative => {
                let target = if state.utilization > 60 { state.max_freq } else { state.min_freq };
                let current = self.cpu_states[idx].current_freq;
                if target > current {
                    self.cpu_states[idx].current_freq = (current + 100).min(target);
                } else if target < current {
                    self.cpu_states[idx].current_freq = current.saturating_sub(100).max(target);
                }
            }
            GovernorType::Userspace => {}
        }
    }

    fn thermal_throttle(&mut self, cpu_id: SigmaU32) {
        let idx = cpu_id as SigmaUsize;
        if idx >= MAX_CPUS {
            return;
        }
        self.cpu_states[idx].current_freq = self.cpu_states[idx].min_freq;
    }

    pub fn has_avx512(&self) -> SigmaBool {
        self.features.avx512f
    }

    pub fn has_avx2(&self) -> SigmaBool {
        self.features.avx2
    }

    pub fn tsc_freq(&self) -> SigmaU64 {
        self.tsc_freq
    }

    pub fn rdtsc_ns(&self) -> SigmaU64 {
        0
    }
}

static mut G_GOVERNOR: PerfGovernor = PerfGovernor::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_governor_init() {
    G_GOVERNOR.init();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_set_governor(cpu_id: SigmaU32, gov: SigmaU32) -> SigmaI32 {
    let gov_type = match gov {
        0 => GovernorType::Performance,
        1 => GovernorType::Powersave,
        2 => GovernorType::Ondemand,
        3 => GovernorType::Conservative,
        4 => GovernorType::Userspace,
        _ => GovernorType::Ondemand,
    };
    G_GOVERNOR.set_governor(cpu_id, gov_type)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_get_governor(cpu_id: SigmaU32) -> SigmaU32 {
    G_GOVERNOR.get_governor(cpu_id) as SigmaU32
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_set_frequency(cpu_id: SigmaU32, freq_mhz: SigmaU32) -> SigmaI32 {
    G_GOVERNOR.set_frequency(cpu_id, freq_mhz)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_get_frequency(cpu_id: SigmaU32) -> SigmaU32 {
    G_GOVERNOR.get_frequency(cpu_id)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_has_avx512() -> SigmaU32 {
    if G_GOVERNOR.has_avx512() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_has_avx2() -> SigmaU32 {
    if G_GOVERNOR.has_avx2() { 1 } else { 0 }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_tsc_freq() -> SigmaU64 {
    G_GOVERNOR.tsc_freq()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_rdtsc_ns() -> SigmaU64 {
    G_GOVERNOR.rdtsc_ns()
}



