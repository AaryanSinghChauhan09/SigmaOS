//! Performance Events Subsystem
//!
//! Inspired by Linux perf_events for hardware and software performance monitoring.
//! Provides zero-overhead performance counters, tracing, and profiling capabilities.
//!
//! # Linux perf_events Features
//! - Hardware performance counters (CPU cycles, cache misses, branch predictions)
//! - Software events (page faults, context switches, migrations)
//! - Tracepoint events (kernel function entry/exit)
//! - Sampling and counting modes
//! - Per-CPU and per-task monitoring

#![cfg_attr(not(any(feature = "standalone_test", test)), no_std)]

extern crate alloc;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicU64, Ordering};

/// Performance event type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PerfEventType {
    /// Hardware CPU events
    Hardware(HardwareEvent),
    /// Software kernel events  
    Software(SoftwareEvent),
    /// Tracepoint events
    Tracepoint,
    /// Cache events
    Cache(CacheEvent),
}

/// Hardware performance events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwareEvent {
    /// CPU cycles
    CpuCycles,
    /// Instructions retired
    Instructions,
    /// Cache references
    CacheReferences,
    /// Cache misses
    CacheMisses,
    /// Branch instructions
    BranchInstructions,
    /// Branch mispredictions
    BranchMisses,
    /// Bus cycles
    BusCycles,
    /// Stalled cycles frontend
    StalledCyclesFrontend,
    /// Stalled cycles backend
    StalledCyclesBackend,
}

/// Software performance events
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareEvent {
    /// CPU clock time
    CpuClock,
    /// Task clock time
    TaskClock,
    /// Page faults
    PageFaults,
    /// Context switches
    ContextSwitches,
    /// CPU migrations
    CpuMigrations,
    /// Minor page faults
    PageFaultsMin,
    /// Major page faults
    PageFaultsMaj,
    /// Alignment faults
    AlignmentFaults,
    /// Emulation faults
    EmulationFaults,
}

/// Cache event operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CacheEvent {
    /// L1 data cache read
    L1DRead,
    /// L1 data cache write
    L1DWrite,
    /// L1 data cache prefetch
    L1DPrefetch,
    /// L1 instruction cache read
    L1IRead,
    /// LLC (last level cache) read
    LLCRead,
    /// LLC write
    LLCWrite,
    /// Data TLB read
    DTLBRead,
    /// Instruction TLB read
    ITLBRead,
}

/// Performance counter state
#[derive(Debug)]
pub struct PerfCounter {
    /// Event type being counted
    pub event_type: PerfEventType,
    /// Current count value
    count: AtomicU64,
    /// Enabled flag
    enabled: bool,
}

impl PerfCounter {
    /// Create new performance counter
    pub fn new(event_type: PerfEventType) -> Self {
        Self {
            event_type,
            count: AtomicU64::new(0),
            enabled: true,
        }
    }

    /// Increment counter
    #[inline(always)]
    pub fn increment(&self) {
        if self.enabled {
            self.count.fetch_add(1, Ordering::Relaxed);
        }
    }

    /// Add value to counter
    #[inline(always)]
    pub fn add(&self, value: u64) {
        if self.enabled {
            self.count.fetch_add(value, Ordering::Relaxed);
        }
    }

    /// Read current count
    #[inline(always)]
    pub fn read(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }

    /// Reset counter
    pub fn reset(&self) {
        self.count.store(0, Ordering::Relaxed);
    }

    /// Enable counter
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable counter
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

/// Performance event group for multiple related counters
pub struct PerfEventGroup {
    /// Group name
    pub name: &'static str,
    /// Counters in this group
    counters: Vec<PerfCounter>,
}

impl PerfEventGroup {
    /// Create new event group
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            counters: Vec::new(),
        }
    }

    /// Add counter to group
    pub fn add_counter(&mut self, counter: PerfCounter) {
        self.counters.push(counter);
    }

    /// Read all counters
    pub fn read_all(&self) -> Vec<u64> {
        self.counters.iter().map(|c| c.read()).collect()
    }

    /// Reset all counters
    pub fn reset_all(&self) {
        for counter in &self.counters {
            counter.reset();
        }
    }

    /// Get counter count
    pub fn counter_count(&self) -> usize {
        self.counters.len()
    }
}

/// Global system-wide performance counters
pub struct SystemPerfCounters {
    /// Total CPU cycles
    pub cpu_cycles: PerfCounter,
    /// Total instructions
    pub instructions: PerfCounter,
    /// Cache misses
    pub cache_misses: PerfCounter,
    /// Page faults
    pub page_faults: PerfCounter,
    /// Context switches
    pub context_switches: PerfCounter,
}

impl SystemPerfCounters {
    /// Create new system counters
    pub fn new() -> Self {
        Self {
            cpu_cycles: PerfCounter::new(PerfEventType::Hardware(HardwareEvent::CpuCycles)),
            instructions: PerfCounter::new(PerfEventType::Hardware(HardwareEvent::Instructions)),
            cache_misses: PerfCounter::new(PerfEventType::Hardware(HardwareEvent::CacheMisses)),
            page_faults: PerfCounter::new(PerfEventType::Software(SoftwareEvent::PageFaults)),
            context_switches: PerfCounter::new(PerfEventType::Software(
                SoftwareEvent::ContextSwitches,
            )),
        }
    }

    /// Get instructions per cycle (IPC)
    pub fn get_ipc(&self) -> f64 {
        let cycles = self.cpu_cycles.read();
        if cycles == 0 {
            return 0.0;
        }
        self.instructions.read() as f64 / cycles as f64
    }

    /// Get cache miss rate
    pub fn get_cache_miss_rate(&self) -> f64 {
        let refs = self.cpu_cycles.read(); // Approximate with cycles
        if refs == 0 {
            return 0.0;
        }
        self.cache_misses.read() as f64 / refs as f64
    }
}

impl Default for SystemPerfCounters {
    fn default() -> Self {
        Self::new()
    }
}

/// Record page fault event (called by memory management)
#[inline(always)]
pub fn record_page_fault(counters: &SystemPerfCounters) {
    counters.page_faults.increment();
}

/// Record context switch event (called by scheduler)
#[inline(always)]
pub fn record_context_switch(counters: &SystemPerfCounters) {
    counters.context_switches.increment();
}

/// Record cache miss event (called by memory subsystem)
#[inline(always)]
pub fn record_cache_miss(counters: &SystemPerfCounters) {
    counters.cache_misses.increment();
}

#[cfg(test)]
#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_counter_creation() {
        let counter = PerfCounter::new(PerfEventType::Hardware(HardwareEvent::CpuCycles));
        assert_eq!(counter.read(), 0);
        assert!(counter.is_enabled());
    }

    #[test]
    fn test_counter_increment() {
        let counter = PerfCounter::new(PerfEventType::Hardware(HardwareEvent::Instructions));
        counter.increment();
        assert_eq!(counter.read(), 1);
        counter.increment();
        assert_eq!(counter.read(), 2);
    }

    #[test]
    fn test_counter_add() {
        let counter = PerfCounter::new(PerfEventType::Software(SoftwareEvent::PageFaults));
        counter.add(100);
        assert_eq!(counter.read(), 100);
        counter.add(50);
        assert_eq!(counter.read(), 150);
    }

    #[test]
    fn test_counter_reset() {
        let counter = PerfCounter::new(PerfEventType::Hardware(HardwareEvent::CacheMisses));
        counter.add(42);
        assert_eq!(counter.read(), 42);
        counter.reset();
        assert_eq!(counter.read(), 0);
    }

    #[test]
    fn test_counter_enable_disable() {
        let mut counter = PerfCounter::new(PerfEventType::Software(SoftwareEvent::CpuClock));
        counter.disable();
        assert!(!counter.is_enabled());
        counter.increment();
        assert_eq!(counter.read(), 0); // Should not increment when disabled

        counter.enable();
        counter.increment();
        assert_eq!(counter.read(), 1); // Should increment when enabled
    }

    #[test]
    fn test_event_group() {
        let mut group = PerfEventGroup::new("test_group");
        let c1 = PerfCounter::new(PerfEventType::Hardware(HardwareEvent::CpuCycles));
        let c2 = PerfCounter::new(PerfEventType::Hardware(HardwareEvent::Instructions));

        c1.add(100);
        c2.add(80);

        group.add_counter(c1);
        group.add_counter(c2);

        let values = group.read_all();
        assert_eq!(values.len(), 2);
        assert_eq!(values[0], 100);
        assert_eq!(values[1], 80);
    }

    #[test]
    fn test_group_reset() {
        let mut group = PerfEventGroup::new("reset_test");
        let c1 = PerfCounter::new(PerfEventType::Software(SoftwareEvent::PageFaults));
        c1.add(50);
        group.add_counter(c1);

        group.reset_all();
        let values = group.read_all();
        assert_eq!(values[0], 0);
    }

    #[test]
    fn test_system_counters() {
        let counters = SystemPerfCounters::new();
        counters.cpu_cycles.add(1000);
        counters.instructions.add(800);

        let ipc = counters.get_ipc();
        assert!((ipc - 0.8).abs() < 0.01); // IPC should be 0.8
    }

    #[test]
    fn test_cache_miss_rate() {
        let counters = SystemPerfCounters::new();
        counters.cpu_cycles.add(10000);
        counters.cache_misses.add(100);

        let miss_rate = counters.get_cache_miss_rate();
        assert!((miss_rate - 0.01).abs() < 0.001); // 1% miss rate
    }

    #[test]
    fn test_record_functions() {
        let counters = SystemPerfCounters::new();

        record_page_fault(&counters);
        assert_eq!(counters.page_faults.read(), 1);

        record_context_switch(&counters);
        assert_eq!(counters.context_switches.read(), 1);

        record_cache_miss(&counters);
        assert_eq!(counters.cache_misses.read(), 1);
    }

    #[test]
    fn test_ipc_zero_cycles() {
        let counters = SystemPerfCounters::new();
        let ipc = counters.get_ipc();
        assert_eq!(ipc, 0.0); // Should handle division by zero
    }

    #[test]
    fn test_multiple_increments() {
        let counter = PerfCounter::new(PerfEventType::Software(SoftwareEvent::ContextSwitches));
        for _ in 0..100 {
            counter.increment();
        }
        assert_eq!(counter.read(), 100);
    }
}
