//! SigmaTrace — System Tracing Framework
//! Inspired by Linux perf, DTrace (FreeBSD/Illumos), SystemTap, and eBPF.
//! Provides: static tracepoints, dynamic probes, syscall tracing, perf counters.
//!
//! References:
//! - Linux perf: https://perf.wiki.kernel.org/
//! - DTrace: https://illumos.org/books/dtrace/
//! - Linux tracepoints: https://www.kernel.org/doc/html/latest/trace/tracepoints.html
//! - eBPF: https://ebpf.io/

use core::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;

/// Tracepoint categories (inspired by Linux kernel tracing subsystem)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraceCategory {
    Syscall,
    Scheduler,
    Memory,
    Filesystem,
    Network,
    Irq,
    Block,
    Security,
    Custom,
}

/// A tracepoint event record
#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub id: u64,
    pub category: TraceCategory,
    pub name: String,
    pub timestamp_ns: u64,
    pub cpu: u32,
    pub pid: u32,
    pub comm: String,
    pub data: Vec<(String, TraceValue)>,
}

/// Typed trace value (for structured event fields)
#[derive(Debug, Clone)]
pub enum TraceValue {
    Int(i64),
    Uint(u64),
    Str(String),
    Bytes(Vec<u8>),
    Bool(bool),
}

/// Performance counter type (inspired by Linux perf_event_type)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PerfCounterType {
    CpuCycles,
    Instructions,
    CacheReferences,
    CacheMisses,
    BranchInstructions,
    BranchMisses,
    PageFaults,
    ContextSwitches,
    CpuMigrations,
    CustomEvent(u32),
}

impl PerfCounterType {
    pub fn name(&self) -> &'static str {
        match self {
            Self::CpuCycles => "cpu-cycles",
            Self::Instructions => "instructions",
            Self::CacheReferences => "cache-references",
            Self::CacheMisses => "cache-misses",
            Self::BranchInstructions => "branch-instructions",
            Self::BranchMisses => "branch-misses",
            Self::PageFaults => "page-faults",
            Self::ContextSwitches => "context-switches",
            Self::CpuMigrations => "cpu-migrations",
            Self::CustomEvent(_) => "custom",
        }
    }
}

/// A performance counter (hardware or software)
pub struct PerfCounter {
    pub counter_type: PerfCounterType,
    value: AtomicU64,
    pub enabled: bool,
    pub overflow_count: u64,
    pub sample_period: u64,
}

impl PerfCounter {
    pub fn new(t: PerfCounterType) -> Self {
        Self {
            counter_type: t,
            value: AtomicU64::new(0),
            enabled: true,
            overflow_count: 0,
            sample_period: 0,
        }
    }

    pub fn increment(&self) {
        if self.enabled {
            self.value.fetch_add(1, Ordering::Relaxed);
        }
    }

    pub fn add(&self, n: u64) {
        if self.enabled {
            self.value.fetch_add(n, Ordering::Relaxed);
        }
    }

    pub fn read(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn reset(&self) {
        self.value.store(0, Ordering::Relaxed);
    }
}

/// Probe type (inspired by DTrace probes and Linux kprobes/uprobes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeType {
    /// Static kernel tracepoint
    Tracepoint,
    /// Dynamic kernel probe (like kprobe)
    KProbe,
    /// Dynamic userspace probe (like uprobe)
    UProbe,
    /// System call entry probe
    SyscallEntry,
    /// System call exit probe
    SyscallExit,
    /// Hardware performance monitoring probe
    Pmu,
}

/// A registered probe
#[derive(Debug, Clone)]
pub struct Probe {
    pub id: u64,
    pub probe_type: ProbeType,
    pub name: String,
    pub target: String, // function name, syscall name, or tracepoint name
    pub enabled: bool,
    pub hit_count: u64,
}

/// SigmaTrace — the main tracing engine
pub struct SigmaTrace {
    /// Registered probes
    probes: HashMap<u64, Probe>,
    /// Performance counters
    counters: HashMap<PerfCounterType, PerfCounter>,
    /// Captured events (ring buffer simulation)
    event_buffer: Vec<TraceEvent>,
    /// Max events to buffer before oldest are dropped
    pub max_events: usize,
    next_probe_id: u64,
    next_event_id: AtomicU64,
    /// Total events captured
    pub total_events: u64,
    /// Total probes fired
    pub total_probe_hits: u64,
    /// Whether tracing is globally enabled
    pub enabled: bool,
}

impl SigmaTrace {
    pub fn new(max_events: usize) -> Self {
        let mut counters = HashMap::new();
        for ct in [
            PerfCounterType::CpuCycles,
            PerfCounterType::Instructions,
            PerfCounterType::CacheMisses,
            PerfCounterType::BranchMisses,
            PerfCounterType::PageFaults,
            PerfCounterType::ContextSwitches,
        ] {
            counters.insert(ct, PerfCounter::new(ct));
        }
        Self {
            probes: HashMap::new(),
            counters,
            event_buffer: Vec::with_capacity(max_events),
            max_events,
            next_probe_id: 1,
            next_event_id: AtomicU64::new(1),
            total_events: 0,
            total_probe_hits: 0,
            enabled: true,
        }
    }

    /// Register a probe. Returns the probe ID.
    pub fn register_probe(&mut self, probe_type: ProbeType, name: &str, target: &str) -> u64 {
        let id = self.next_probe_id;
        self.next_probe_id += 1;
        self.probes.insert(
            id,
            Probe {
                id,
                probe_type,
                name: name.to_string(),
                target: target.to_string(),
                enabled: true,
                hit_count: 0,
            },
        );
        id
    }

    /// Enable/disable a probe by ID.
    pub fn set_probe_enabled(&mut self, probe_id: u64, enabled: bool) -> bool {
        if let Some(p) = self.probes.get_mut(&probe_id) {
            p.enabled = enabled;
            true
        } else {
            false
        }
    }

    /// Fire a probe — records an event and increments hit count.
    pub fn fire_probe(
        &mut self,
        probe_id: u64,
        pid: u32,
        cpu: u32,
        comm: &str,
        fields: Vec<(String, TraceValue)>,
    ) -> Option<u64> {
        if !self.enabled {
            return None;
        }
        let probe = self.probes.get_mut(&probe_id)?;
        if !probe.enabled {
            return None;
        }
        probe.hit_count += 1;
        self.total_probe_hits += 1;
        let event_id = self.next_event_id.fetch_add(1, Ordering::Relaxed);
        let event = TraceEvent {
            id: event_id,
            category: match probe.probe_type {
                ProbeType::SyscallEntry | ProbeType::SyscallExit => TraceCategory::Syscall,
                ProbeType::KProbe | ProbeType::Tracepoint => TraceCategory::Custom,
                _ => TraceCategory::Custom,
            },
            name: probe.name.clone(),
            timestamp_ns: event_id * 1000, // simulated timestamps
            cpu,
            pid,
            comm: comm.to_string(),
            data: fields,
        };
        if self.event_buffer.len() >= self.max_events {
            self.event_buffer.remove(0); // drop oldest
        }
        self.event_buffer.push(event);
        self.total_events += 1;
        Some(event_id)
    }

    /// Increment a performance counter.
    pub fn perf_inc(&self, counter: PerfCounterType) {
        if let Some(c) = self.counters.get(&counter) {
            c.increment();
        }
    }

    /// Increment a performance counter by N.
    pub fn perf_add(&self, counter: PerfCounterType, n: u64) {
        if let Some(c) = self.counters.get(&counter) {
            c.add(n);
        }
    }

    /// Read a performance counter value.
    pub fn perf_read(&self, counter: PerfCounterType) -> u64 {
        self.counters.get(&counter).map(|c| c.read()).unwrap_or(0)
    }

    /// Reset all performance counters.
    pub fn perf_reset_all(&self) {
        for c in self.counters.values() {
            c.reset();
        }
    }

    /// Drain and return captured events.
    pub fn drain_events(&mut self) -> Vec<TraceEvent> {
        std::mem::take(&mut self.event_buffer)
    }

    /// Get events by category.
    pub fn events_by_category(&self, cat: TraceCategory) -> Vec<&TraceEvent> {
        self.event_buffer
            .iter()
            .filter(|e| e.category == cat)
            .collect()
    }

    /// Get probe statistics.
    pub fn probe_stats(&self) -> Vec<(u64, &str, u64, bool)> {
        self.probes
            .values()
            .map(|p| (p.id, p.name.as_str(), p.hit_count, p.enabled))
            .collect()
    }
}

/// Convenience macro-like function for emitting a syscall tracepoint.
pub fn trace_syscall(
    tracer: &mut SigmaTrace,
    probe_id: u64,
    pid: u32,
    syscall: &str,
    args: &[u64],
) {
    let fields: Vec<(String, TraceValue)> = args
        .iter()
        .enumerate()
        .map(|(i, &v)| (format!("arg{}", i), TraceValue::Uint(v)))
        .chain(std::iter::once((
            "syscall".to_string(),
            TraceValue::Str(syscall.to_string()),
        )))
        .collect();
    tracer.fire_probe(probe_id, pid, 0, "kernel", fields);
}

#[cfg(test)]
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_fire_probe() {
        let mut tracer = SigmaTrace::new(100);
        let pid = tracer.register_probe(ProbeType::SyscallEntry, "sys_read", "read");
        tracer.fire_probe(
            pid,
            1234,
            0,
            "bash",
            vec![("fd".to_string(), TraceValue::Int(3))],
        );
        assert_eq!(tracer.probes[&pid].hit_count, 1);
        assert_eq!(tracer.total_events, 1);
    }

    #[test]
    fn test_perf_counters() {
        let tracer = SigmaTrace::new(100);
        tracer.perf_add(PerfCounterType::Instructions, 1_000_000);
        assert_eq!(tracer.perf_read(PerfCounterType::Instructions), 1_000_000);
        tracer.perf_reset_all();
        assert_eq!(tracer.perf_read(PerfCounterType::Instructions), 0);
    }

    #[test]
    fn test_probe_disable() {
        let mut tracer = SigmaTrace::new(100);
        let pid = tracer.register_probe(ProbeType::KProbe, "page_fault", "do_page_fault");
        tracer.set_probe_enabled(pid, false);
        tracer.fire_probe(pid, 1, 0, "test", vec![]);
        assert_eq!(tracer.probes[&pid].hit_count, 0);
    }

    #[test]
    fn test_event_buffer_overflow() {
        let mut tracer = SigmaTrace::new(5);
        let pid = tracer.register_probe(ProbeType::Tracepoint, "tp", "test");
        for i in 0..10 {
            tracer.fire_probe(pid, i as u32, 0, "test", vec![]);
        }
        assert!(tracer.event_buffer.len() <= 5);
        assert_eq!(tracer.total_events, 10);
    }

    #[test]
    fn test_trace_syscall_helper() {
        let mut tracer = SigmaTrace::new(100);
        let pid = tracer.register_probe(ProbeType::SyscallEntry, "sys_write", "write");
        trace_syscall(&mut tracer, pid, 42, "write", &[1, 0x7FFF_0000, 1024]);
        assert_eq!(tracer.total_events, 1);
    }
}
