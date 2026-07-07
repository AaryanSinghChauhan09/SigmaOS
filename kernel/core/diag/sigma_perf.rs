// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/diag/sigma_perf.rs — Performance Events Monitoring
//
// Implements hardware performance counter monitoring for CPU cycles,
// cache misses, branch predictions, and other microarchitectural events.
// Inspired by Linux perf_events and x86 PMU.
//
// Language: Rust #![no_std] — no alloc, no external crates.

#![no_std]
#![allow(dead_code)]

use core::sync::atomic::{AtomicU64, Ordering};

// ── Types ─────────────────────────────────────────────────────────────────────
type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaI32   = i32;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ── Constants ─────────────────────────────────────────────────────────────────
/// Maximum number of performance events.
const MAX_EVENTS: SigmaUsize = 32;
/// Maximum event name length.
const EVENT_NAME_LEN: SigmaUsize = 32;

// ── Event Types ─────────────────────────────────────────────────────────────
#[repr(u32)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PerfEventType {
    /// CPU cycles (not halted).
    Cycles           = 0,
    /// Instructions retired.
    Instructions     = 1,
    /// Cache references (all levels).
    CacheReferences  = 2,
    /// Cache misses (all levels).
    CacheMisses      = 3,
    /// Branch instructions retired.
    Branches         = 4,
    /// Branch mispredictions.
    BranchMisses     = 5,
    /// Bus cycles.
    BusCycles        = 6,
    /// Stalled cycles frontend.
    StalledFrontend  = 7,
    /// Stalled cycles backend.
    StalledBackend   = 8,
}

// ── PerfEvent ───────────────────────────────────────────────────────────────
#[repr(C)]
#[derive(Copy, Clone)]
pub struct PerfEvent {
    /// Event type.
    pub event_type:  PerfEventType,
    /// Event name.
    pub name:         [SigmaU8; EVENT_NAME_LEN],
    /// Raw counter value.
    pub count:        SigmaU64,
    /// Enabled flag.
    pub enabled:      SigmaBool,
    /// Event ID (PMU register).
    pub event_id:     SigmaU32,
    pub _pad:         [SigmaU8; 7],
}

impl PerfEvent {
    pub const fn zeroed() -> Self {
        Self {
            event_type:  PerfEventType::Cycles,
            name:         [0u8; EVENT_NAME_LEN],
            count:        0,
            enabled:      false,
            event_id:     0,
            _pad:         [0u8; 7],
        }
    }
}

// ── PerfMonitor ─────────────────────────────────────────────────────────────
pub struct PerfMonitor {
    /// Registered events.
    events:      [PerfEvent; MAX_EVENTS],
    /// Number of active events.
    event_count: SigmaUsize,
    /// Global cycle counter.
    cycles:      AtomicU64,
    /// Initialized flag.
    initialized: SigmaBool,
}

impl PerfMonitor {
    pub const fn new() -> Self {
        Self {
            events:      [PerfEvent::zeroed(); MAX_EVENTS],
            event_count: 0,
            cycles:      AtomicU64::new(0),
            initialized: false,
        }
    }

    pub fn init(&mut self) {
        self.initialized = true;
        // Initialize PMU hardware
        // Configure fixed counters
    }

    fn find_slot(&self) -> Option<SigmaUsize> {
        for i in 0..MAX_EVENTS {
            if !self.events[i].enabled {
                return Some(i);
            }
        }
        None
    }

    fn copy_str(dst: &mut [SigmaU8], src: &[SigmaU8]) {
        let len = src.len().min(dst.len() - 1);
        let mut i = 0;
        while i < len { dst[i] = src[i]; i += 1; }
        dst[len] = 0;
    }

    // ── Public API ────────────────────────────────────────────────────────────

    /// Register a performance event.
    /// Returns event ID on success, -1 on failure.
    pub fn register_event(
        &mut self,
        event_type: PerfEventType,
        name:       &[SigmaU8],
    ) -> SigmaI32 {
        let slot = match self.find_slot() { Some(s) => s, None => return -1 };

        let mut event = PerfEvent::zeroed();
        event.event_type = event_type;
        event.enabled = true;
        event.event_id = slot as SigmaU32;

        Self::copy_str(&mut event.name, name);

        self.events[slot] = event;
        self.event_count += 1;
        slot as SigmaI32
    }

    /// Start counting for an event.
    pub fn event_enable(&mut self, event_id: SigmaI32) -> SigmaI32 {
        let idx = event_id as SigmaUsize;
        if idx >= MAX_EVENTS {
            return -1;
        }
        self.events[idx].enabled = true;
        // Enable PMU counter
        0
    }

    /// Stop counting for an event.
    pub fn event_disable(&mut self, event_id: SigmaI32) -> SigmaI32 {
        let idx = event_id as SigmaUsize;
        if idx >= MAX_EVENTS {
            return -1;
        }
        self.events[idx].enabled = false;
        // Disable PMU counter
        0
    }

    /// Read the current counter value.
    pub fn event_read(&mut self, event_id: SigmaI32) -> SigmaU64 {
        let idx = event_id as SigmaUsize;
        if idx < MAX_EVENTS && self.events[idx].enabled {
            // Read PMU counter
            self.events[idx].count
        } else {
            0
        }
    }

    /// Reset all counters.
    pub fn reset_all(&mut self) {
        for i in 0..MAX_EVENTS {
            self.events[i].count = 0;
        }
        self.cycles.store(0, Ordering::SeqCst);
    }

    /// Get global cycle count.
    pub fn get_cycles(&self) -> SigmaU64 {
        self.cycles.load(Ordering::Relaxed)
    }

    /// Increment global cycle counter (called by timer interrupt).
    pub fn increment_cycles(&self) {
        self.cycles.fetch_add(1, Ordering::Relaxed);
    }
}

// ── Global Instance ───────────────────────────────────────────────────────────
static mut G_PERF_MONITOR: PerfMonitor = PerfMonitor::new();

// ── C-ABI Exports ─────────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn perf_events_init() {
    G_PERF_MONITOR.init();
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_register(
    event_type: SigmaU32,
    name:      *const SigmaU8,
    name_len:  SigmaUsize,
) -> SigmaI32 {
    let n = core::slice::from_raw_parts(name, name_len.min(EVENT_NAME_LEN));
    let et = match event_type {
        0 => PerfEventType::Cycles,
        1 => PerfEventType::Instructions,
        2 => PerfEventType::CacheReferences,
        3 => PerfEventType::CacheMisses,
        4 => PerfEventType::Branches,
        5 => PerfEventType::BranchMisses,
        6 => PerfEventType::BusCycles,
        7 => PerfEventType::StalledFrontend,
        8 => PerfEventType::StalledBackend,
        _ => PerfEventType::Cycles,
    };
    G_PERF_MONITOR.register_event(et, n)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_enable(event_id: SigmaI32) -> SigmaI32 {
    G_PERF_MONITOR.event_enable(event_id)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_disable(event_id: SigmaI32) -> SigmaI32 {
    G_PERF_MONITOR.event_disable(event_id)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_read(event_id: SigmaI32) -> SigmaU64 {
    G_PERF_MONITOR.event_read(event_id)
}

#[no_mangle]
pub unsafe extern "C" fn perf_event_close(event_id: SigmaI32) -> SigmaI32 {
    G_PERF_MONITOR.event_disable(event_id)
}

#[no_mangle]
pub unsafe extern "C" fn perf_reset_all() {
    G_PERF_MONITOR.reset_all();
}

#[no_mangle]
pub unsafe extern "C" fn perf_get_cycles() -> SigmaU64 {
    G_PERF_MONITOR.get_cycles()
}

