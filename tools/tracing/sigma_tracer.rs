// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/tracing/sigma_tracer.rs — sigma-trace: Syscall + Shard Event Tracer
// Language: Rust (std) — userland tracing tool
// Pattern: OOP via Tracer struct + TraceEvent enum

use std::collections::VecDeque;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt;

// ── Event Types ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum TraceEvent {
    Syscall {
        pid:      u32,
        syscall:  u32,
        ret:      i64,
        duration_ns: u64,
    },
    ShardLoad {
        shard_id: u32,
        name:     String,
        ts_ns:    u64,
    },
    ShardMsg {
        src:     u32,
        dst:     u32,
        type_id: u32,
        len:     usize,
        ts_ns:   u64,
    },
    IrqFired {
        vec:     u8,
        latency_ns: u64,
        ts_ns:   u64,
    },
    PledgeApplied {
        pid:    u32,
        caps:   u64,
        ts_ns:  u64,
    },
    PageFault {
        pid:    u32,
        va:     u64,
        write:  bool,
        ts_ns:  u64,
    },
}

impl fmt::Display for TraceEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syscall { pid, syscall, ret, duration_ns } =>
                write!(f, "SYSCALL pid={} nr={} ret={} dur={}ns",
                       pid, syscall, ret, duration_ns),
            Self::ShardLoad { shard_id, name, ts_ns } =>
                write!(f, "SHARD_LOAD id={} name={} ts={}", shard_id, name, ts_ns),
            Self::ShardMsg { src, dst, type_id, len, ts_ns } =>
                write!(f, "SHARD_MSG src={} dst={} type={} len={} ts={}",
                       src, dst, type_id, len, ts_ns),
            Self::IrqFired { vec, latency_ns, ts_ns } =>
                write!(f, "IRQ vec={} latency={}ns ts={}", vec, latency_ns, ts_ns),
            Self::PledgeApplied { pid, caps, ts_ns } =>
                write!(f, "PLEDGE pid={} caps=0x{:x} ts={}", pid, caps, ts_ns),
            Self::PageFault { pid, va, write, ts_ns } =>
                write!(f, "PAGEFAULT pid={} va=0x{:x} write={} ts={}",
                       pid, va, write, ts_ns),
        }
    }
}

// ── Tracer ────────────────────────────────────────────────────────────────────

pub struct Tracer {
    events:    VecDeque<TraceEvent>,
    max_depth: usize,
    enabled:   bool,
    filters:   TracerFilters,
}

#[derive(Default, Clone)]
pub struct TracerFilters {
    pub pid:      Option<u32>,
    pub syscall:  Option<u32>,
    pub min_duration_ns: Option<u64>,
}

impl Tracer {
    pub fn new(max_depth: usize) -> Self {
        Self {
            events:    VecDeque::with_capacity(max_depth),
            max_depth,
            enabled:   true,
            filters:   TracerFilters::default(),
        }
    }

    pub fn enable(&mut self)  { self.enabled = true;  }
    pub fn disable(&mut self) { self.enabled = false; }

    pub fn set_filter(&mut self, f: TracerFilters) { self.filters = f; }

    fn now_ns() -> u64 {
        SystemTime::now().duration_since(UNIX_EPOCH)
            .unwrap_or_default().as_nanos() as u64
    }

    fn matches(&self, ev: &TraceEvent) -> bool {
        if let Some(pid) = self.filters.pid {
            match ev {
                TraceEvent::Syscall { pid: p, .. }      if *p != pid => return false,
                TraceEvent::PledgeApplied { pid: p, .. } if *p != pid => return false,
                TraceEvent::PageFault { pid: p, .. }    if *p != pid => return false,
                _ => {}
            }
        }
        if let Some(min_dur) = self.filters.min_duration_ns {
            if let TraceEvent::Syscall { duration_ns, .. } = ev {
                if *duration_ns < min_dur { return false; }
            }
        }
        true
    }

    pub fn record(&mut self, event: TraceEvent) {
        if !self.enabled || !self.matches(&event) { return; }
        if self.events.len() >= self.max_depth { self.events.pop_front(); }
        self.events.push_back(event);
    }

    /// Record a syscall with automatic timestamp
    pub fn trace_syscall(&mut self, pid: u32, syscall: u32, ret: i64, dur_ns: u64) {
        self.record(TraceEvent::Syscall { pid, syscall, ret, duration_ns: dur_ns });
    }

    pub fn trace_shard_load(&mut self, id: u32, name: String) {
        self.record(TraceEvent::ShardLoad { shard_id: id, name, ts_ns: Self::now_ns() });
    }

    pub fn trace_irq(&mut self, vec: u8, latency_ns: u64) {
        self.record(TraceEvent::IrqFired { vec, latency_ns, ts_ns: Self::now_ns() });
    }

    /// Dump all recorded events to stdout
    pub fn dump(&self) {
        println!("=== sigma-trace: {} events ===", self.events.len());
        for ev in &self.events { println!("  {}", ev); }
    }

    /// Return a summary: syscall counts by nr, avg duration
    pub fn syscall_summary(&self) -> [(u32, u64, u64); 256] {
        let mut summary = [(0u32, 0u64, 0u64); 256]; // (count, total_ns, max_ns)
        for ev in &self.events {
            if let TraceEvent::Syscall { syscall, duration_ns, .. } = ev {
                let nr = (*syscall as usize).min(255);
                summary[nr].0 += 1;
                summary[nr].1 += duration_ns;
                if *duration_ns > summary[nr].2 { summary[nr].2 = *duration_ns; }
            }
        }
        summary
    }

    pub fn event_count(&self) -> usize { self.events.len() }

    pub fn iter(&self) -> impl Iterator<Item = &TraceEvent> { self.events.iter() }
}
