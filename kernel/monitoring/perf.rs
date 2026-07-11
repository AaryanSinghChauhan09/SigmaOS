// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/monitoring/perf.rs — Performance Monitoring and Counters
//
// This module implements performance monitoring and counters inspired by Linux perf.
// It provides hardware and software performance counters for profiling and analysis.
//
// Key features:
// - Hardware performance counters (CPU cycles, instructions, cache misses)
// - Software performance counters (context switches, page faults, syscalls)
// - Event-based profiling
// - OOP principles with counter traits
// - No external dependencies

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};

// ─────────────────────────────────────────────────────────────────────────────
// Event Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum PerfEventType {
    // Hardware events
    CpuCycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    
    // Software events
    ContextSwitches,
    CpuMigrations,
    PageFaults,
    MajorPageFaults,
    MinorPageFaults,
    Syscalls,
    
    // Cache events
    L1DCacheLoads,
    L1DCacheLoadMisses,
    L1ICacheLoads,
    L1ICacheLoadMisses,
    LLCLoads,
    LLCLoadMisses,
    
    // TLB events
    DTlbLoads,
    DTlbLoadMisses,
    ITlbLoads,
    ITlbLoadMisses,
}

// ─────────────────────────────────────────────────────────────────────────────
// Counter Configuration
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct CounterConfig {
    pub event_type: PerfEventType,
    pub enabled: bool,
    pub user: bool,     // Count user space events
    pub kernel: bool,   // Count kernel space events
    pub inherit: bool,  // Inherit to child processes
    pub freq: u64,      // Sampling frequency (0 for counting mode)
}

impl CounterConfig {
    pub const fn default() -> Self {
        Self {
            event_type: PerfEventType::CpuCycles,
            enabled: false,
            user: true,
            kernel: true,
            inherit: false,
            freq: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Counter Value
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct CounterValue {
    pub value: u64,
    pub enabled: bool,
    pub running: bool,
    pub time_enabled: u64,
    pub time_running: u64,
}

impl CounterValue {
    pub const fn empty() -> Self {
        Self {
            value: 0,
            enabled: false,
            running: false,
            time_enabled: 0,
            time_running: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Performance Counter Trait (OOP Principles)
// ─────────────────────────────────────────────────────────────────────────────

pub trait PerfCounter {
    fn get_config(&self) -> CounterConfig;
    fn get_value(&self) -> CounterValue;
    fn enable(&mut self) -> bool;
    fn disable(&mut self) -> bool;
    fn reset(&mut self) -> bool;
    fn read(&self) -> u64;
    fn is_enabled(&self) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// Hardware Counter Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct HardwareCounter {
    config: CounterConfig,
    value: CounterValue,
    event_id: u32,
}

impl HardwareCounter {
    pub const fn new(event_type: PerfEventType) -> Self {
        Self {
            config: CounterConfig {
                event_type,
                enabled: false,
                user: true,
                kernel: true,
                inherit: false,
                freq: 0,
            },
            value: CounterValue::empty(),
            event_id: event_type as u32,
        }
    }
}

impl PerfCounter for HardwareCounter {
    fn get_config(&self) -> CounterConfig {
        self.config
    }

    fn get_value(&self) -> CounterValue {
        self.value
    }

    fn enable(&mut self) -> bool {
        // In real implementation, this would program the hardware PMU
        self.config.enabled = true;
        self.value.enabled = true;
        self.value.running = true;
        true
    }

    fn disable(&mut self) -> bool {
        self.config.enabled = false;
        self.value.enabled = false;
        self.value.running = false;
        true
    }

    fn reset(&mut self) -> bool {
        self.value.value = 0;
        self.value.time_enabled = 0;
        self.value.time_running = 0;
        true
    }

    fn read(&self) -> u64 {
        // In real implementation, this would read from hardware PMU
        self.value.value
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Software Counter Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct SoftwareCounter {
    config: CounterConfig,
    value: CounterValue,
    counter: AtomicU64,
}

impl SoftwareCounter {
    pub const fn new(event_type: PerfEventType) -> Self {
        Self {
            config: CounterConfig {
                event_type,
                enabled: false,
                user: true,
                kernel: true,
                inherit: false,
                freq: 0,
            },
            value: CounterValue::empty(),
            counter: AtomicU64::new(0),
        }
    }

    pub fn increment(&self) {
        if self.config.enabled {
            self.counter.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn add(&self, value: u64) {
        if self.config.enabled {
            self.counter.fetch_add(value, Ordering::Relaxed);
        }
    }
}

impl PerfCounter for SoftwareCounter {
    fn get_config(&self) -> CounterConfig {
        self.config
    }

    fn get_value(&self) -> CounterValue {
        CounterValue {
            value: self.counter.load(Ordering::Relaxed),
            enabled: self.value.enabled,
            running: self.value.running,
            time_enabled: self.value.time_enabled,
            time_running: self.value.time_running,
        }
    }

    fn enable(&mut self) -> bool {
        self.config.enabled = true;
        self.value.enabled = true;
        self.value.running = true;
        true
    }

    fn disable(&mut self) -> bool {
        self.config.enabled = false;
        self.value.enabled = false;
        self.value.running = false;
        true
    }

    fn reset(&mut self) -> bool {
        self.counter.store(0, Ordering::Relaxed);
        self.value.value = 0;
        self.value.time_enabled = 0;
        self.value.time_running = 0;
        true
    }

    fn read(&self) -> u64 {
        self.counter.load(Ordering::Relaxed)
    }

    fn is_enabled(&self) -> bool {
        self.config.enabled
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Performance Event Manager
// ─────────────────────────────────────────────────────────────────────────────

pub struct PerfEventManager {
    hardware_counters: [Option<HardwareCounter>; 32],
    software_counters: [Option<SoftwareCounter>; 32],
    num_hw_counters: usize,
    num_sw_counters: usize,
}

impl PerfEventManager {
    pub const fn new() -> Self {
        Self {
            hardware_counters: [None; 32],
            software_counters: [None; 32],
            num_hw_counters: 0,
            num_sw_counters: 0,
        }
    }

    // Add a hardware counter
    pub fn add_hw_counter(&mut self, event_type: PerfEventType) -> Option<usize> {
        if self.num_hw_counters >= 32 { return None; }
        
        let counter = HardwareCounter::new(event_type);
        self.hardware_counters[self.num_hw_counters] = Some(counter);
        let idx = self.num_hw_counters;
        self.num_hw_counters += 1;
        Some(idx)
    }

    // Add a software counter
    pub fn add_sw_counter(&mut self, event_type: PerfEventType) -> Option<usize> {
        if self.num_sw_counters >= 32 { return None; }
        
        let counter = SoftwareCounter::new(event_type);
        self.software_counters[self.num_sw_counters] = Some(counter);
        let idx = self.num_sw_counters;
        self.num_sw_counters += 1;
        Some(idx)
    }

    // Enable a counter by index
    pub fn enable_counter(&mut self, idx: usize, is_hw: bool) -> bool {
        if is_hw {
            if idx < self.num_hw_counters {
                if let Some(ref mut counter) = self.hardware_counters[idx] {
                    return counter.enable();
                }
            }
        } else {
            if idx < self.num_sw_counters {
                if let Some(ref mut counter) = self.software_counters[idx] {
                    return counter.enable();
                }
            }
        }
        false
    }

    // Disable a counter by index
    pub fn disable_counter(&mut self, idx: usize, is_hw: bool) -> bool {
        if is_hw {
            if idx < self.num_hw_counters {
                if let Some(ref mut counter) = self.hardware_counters[idx] {
                    return counter.disable();
                }
            }
        } else {
            if idx < self.num_sw_counters {
                if let Some(ref mut counter) = self.software_counters[idx] {
                    return counter.disable();
                }
            }
        }
        false
    }

    // Read a counter by index
    pub fn read_counter(&self, idx: usize, is_hw: bool) -> Option<u64> {
        if is_hw {
            if idx < self.num_hw_counters {
                if let Some(ref counter) = self.hardware_counters[idx] {
                    return Some(counter.read());
                }
            }
        } else {
            if idx < self.num_sw_counters {
                if let Some(ref counter) = self.software_counters[idx] {
                    return Some(counter.read());
                }
            }
        }
        None
    }

    // Get counter value by index
    pub fn get_counter_value(&self, idx: usize, is_hw: bool) -> Option<CounterValue> {
        if is_hw {
            if idx < self.num_hw_counters {
                if let Some(ref counter) = self.hardware_counters[idx] {
                    return Some(counter.get_value());
                }
            }
        } else {
            if idx < self.num_sw_counters {
                if let Some(ref counter) = self.software_counters[idx] {
                    return Some(counter.get_value());
                }
            }
        }
        None
    }

    // Reset a counter by index
    pub fn reset_counter(&mut self, idx: usize, is_hw: bool) -> bool {
        if is_hw {
            if idx < self.num_hw_counters {
                if let Some(ref mut counter) = self.hardware_counters[idx] {
                    return counter.reset();
                }
            }
        } else {
            if idx < self.num_sw_counters {
                if let Some(ref mut counter) = self.software_counters[idx] {
                    return counter.reset();
                }
            }
        }
        false
    }

    // Get number of hardware counters
    pub fn num_hw_counters(&self) -> usize {
        self.num_hw_counters
    }

    // Get number of software counters
    pub fn num_sw_counters(&self) -> usize {
        self.num_sw_counters
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut PERF_MANAGER: PerfEventManager = PerfEventManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_init() {
    PERF_MANAGER = PerfEventManager::new();
    
    // Initialize common software counters
    PERF_MANAGER.add_sw_counter(PerfEventType::ContextSwitches);
    PERF_MANAGER.add_sw_counter(PerfEventType::PageFaults);
    PERF_MANAGER.add_sw_counter(PerfEventType::Syscalls);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_add_hw_counter(event_type: u8) -> i32 {
    let event_type = match event_type {
        0 => PerfEventType::CpuCycles,
        1 => PerfEventType::Instructions,
        2 => PerfEventType::CacheReferences,
        3 => PerfEventType::CacheMisses,
        4 => PerfEventType::BranchInstructions,
        5 => PerfEventType::BranchMisses,
        _ => return -1,
    };
    match PERF_MANAGER.add_hw_counter(event_type) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_add_sw_counter(event_type: u8) -> i32 {
    let event_type = match event_type {
        0 => PerfEventType::ContextSwitches,
        1 => PerfEventType::CpuMigrations,
        2 => PerfEventType::PageFaults,
        3 => PerfEventType::MajorPageFaults,
        4 => PerfEventType::MinorPageFaults,
        5 => PerfEventType::Syscalls,
        _ => return -1,
    };
    match PERF_MANAGER.add_sw_counter(event_type) {
        Some(idx) => idx as i32,
        None => -1,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_enable_counter(idx: usize, is_hw: bool) -> bool {
    PERF_MANAGER.enable_counter(idx, is_hw)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_disable_counter(idx: usize, is_hw: bool) -> bool {
    PERF_MANAGER.disable_counter(idx, is_hw)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_read_counter(idx: usize, is_hw: bool) -> u64 {
    PERF_MANAGER.read_counter(idx, is_hw).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_reset_counter(idx: usize, is_hw: bool) -> bool {
    PERF_MANAGER.reset_counter(idx, is_hw)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_increment_sw_counter(idx: usize) {
    if idx < PERF_MANAGER.num_sw_counters() {
        if let Some(ref counter) = PERF_MANAGER.software_counters[idx] {
            counter.increment();
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_add_to_sw_counter(idx: usize, value: u64) {
    if idx < PERF_MANAGER.num_sw_counters() {
        if let Some(ref counter) = PERF_MANAGER.software_counters[idx] {
            counter.add(value);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_get_num_hw_counters() -> usize {
    PERF_MANAGER.num_hw_counters()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_perf_get_num_sw_counters() -> usize {
    PERF_MANAGER.num_sw_counters()
}
