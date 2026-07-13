// SPDX-License-Identifier: GPL-2.0-or-later
//! SigmaOS Sovereign Profiler — high-frequency, lock-free kernel telemetry.
//!
//! Collects per-shard CPU, IPC, memory, and page-fault metrics using
//! cache-line-aligned atomic counters. Metrics published to /sigma/metrics
//! shared memory at 10Hz. Zero-overhead in production (no allocator, no_std).

#![no_std]
#![allow(dead_code)]
#![allow(non_snake_case)]

use core::sync::atomic::{AtomicU64, AtomicU32, Ordering};

// ─── Kernel Primitive Types ─────────────────────────────────────────────────

type SigmaU8    = u8;
type SigmaU32   = u32;
type SigmaU64   = u64;
type SigmaBool  = bool;
type SigmaUsize = usize;

// ─── Constants ──────────────────────────────────────────────────────────────

/// Maximum number of shards tracked simultaneously
const MAX_SHARDS: usize = 256;
/// Metrics broadcast interval in milliseconds (10Hz = 100ms)
const BROADCAST_INTERVAL_MS: SigmaU64 = 100;
/// Latency histogram buckets (nanoseconds): 100, 250, 500, 1000, 2500, 5000, 10000, +inf
const LATENCY_BUCKETS: [SigmaU64; 8] = [100, 250, 500, 1_000, 2_500, 5_000, 10_000, SigmaU64::MAX];
const HISTOGRAM_BUCKETS: usize = 8;

// ─── Shard State ─────────────────────────────────────────────────────────────

#[repr(u64)]
#[derive(Clone, Copy, PartialEq)]
pub enum ShardState {
    Unregistered = 0,
    Initializing = 1,
    Running      = 2,
    Blocked      = 3,
    Idle         = 4,
    Panic        = 5,
    Quarantined  = 6,
}

// ─── Per-Shard Metrics Counter Set ──────────────────────────────────────────

/// Lock-free per-shard metric counters.
/// Cache-line aligned (64 bytes) to prevent false sharing between CPUs.
#[repr(C, align(64))]
pub struct ShardMetrics {
    /// Monotonic CPU nanoseconds consumed by this shard
    pub cpu_ns:          AtomicU64,
    /// sigma-bus messages sent
    pub ipc_msgs_sent:   AtomicU64,
    /// sigma-bus messages received
    pub ipc_msgs_recv:   AtomicU64,
    /// Current RSS memory in KB
    pub memory_kb:       AtomicU64,
    /// Major + minor page faults
    pub page_faults:     AtomicU64,
    /// Current shard lifecycle state
    pub state:           AtomicU64,
    /// Shard ID (immutable after registration)
    pub shard_id:        AtomicU32,
    /// Padding to exactly 64 bytes
    _pad:                [u8; 4],
}

// Verify size at compile time (can't use const_assert in no_std without a crate)
// const _: () = assert!(core::mem::size_of::<ShardMetrics>() == 64);

impl ShardMetrics {
    pub const fn new() -> Self {
        Self {
            cpu_ns:        AtomicU64::new(0),
            ipc_msgs_sent: AtomicU64::new(0),
            ipc_msgs_recv: AtomicU64::new(0),
            memory_kb:     AtomicU64::new(0),
            page_faults:   AtomicU64::new(0),
            state:         AtomicU64::new(ShardState::Unregistered as u64),
            shard_id:      AtomicU32::new(0),
            _pad:          [0u8; 4],
        }
    }

    /// Record CPU time spent in this shard (called on shard exit)
    #[inline(always)]
    pub fn record_cpu_ns(&self, ns: SigmaU64) {
        self.cpu_ns.fetch_add(ns, Ordering::Relaxed);
    }

    /// Record an outgoing IPC message
    #[inline(always)]
    pub fn record_ipc_send(&self) {
        self.ipc_msgs_sent.fetch_add(1, Ordering::Relaxed);
    }

    /// Record an incoming IPC message
    #[inline(always)]
    pub fn record_ipc_recv(&self) {
        self.ipc_msgs_recv.fetch_add(1, Ordering::Relaxed);
    }

    /// Update current memory usage
    #[inline(always)]
    pub fn set_memory_kb(&self, kb: SigmaU64) {
        self.memory_kb.store(kb, Ordering::Relaxed);
    }

    /// Record a page fault
    #[inline(always)]
    pub fn record_page_fault(&self) {
        self.page_faults.fetch_add(1, Ordering::Relaxed);
    }

    /// Update shard lifecycle state
    #[inline(always)]
    pub fn set_state(&self, state: ShardState) {
        self.state.store(state as u64, Ordering::Release);
    }

    pub fn current_state(&self) -> ShardState {
        match self.state.load(Ordering::Acquire) {
            1 => ShardState::Initializing,
            2 => ShardState::Running,
            3 => ShardState::Blocked,
            4 => ShardState::Idle,
            5 => ShardState::Panic,
            6 => ShardState::Quarantined,
            _ => ShardState::Unregistered,
        }
    }

    /// Take a point-in-time snapshot of all counters
    pub fn snapshot(&self) -> MetricsSnapshot {
        MetricsSnapshot {
            shard_id:     self.shard_id.load(Ordering::Relaxed),
            cpu_ns:       self.cpu_ns.load(Ordering::Relaxed),
            ipc_sent:     self.ipc_msgs_sent.load(Ordering::Relaxed),
            ipc_recv:     self.ipc_msgs_recv.load(Ordering::Relaxed),
            memory_kb:    self.memory_kb.load(Ordering::Relaxed),
            page_faults:  self.page_faults.load(Ordering::Relaxed),
            state:        self.current_state(),
        }
    }

    /// Reset all counters (called at each broadcast interval for rate calculation)
    pub fn reset_rate_counters(&self) {
        self.cpu_ns.store(0, Ordering::Relaxed);
        self.ipc_msgs_sent.store(0, Ordering::Relaxed);
        self.ipc_msgs_recv.store(0, Ordering::Relaxed);
        self.page_faults.store(0, Ordering::Relaxed);
        // Note: memory_kb and state are NOT reset — they are point-in-time values
    }
}

// ─── IPC Latency Histogram ───────────────────────────────────────────────────

/// Lock-free latency histogram with pre-defined bucket boundaries
pub struct LatencyHistogram {
    /// Per-bucket counts (one per LATENCY_BUCKET boundary)
    buckets:  [AtomicU64; HISTOGRAM_BUCKETS],
    /// Running sum for mean calculation
    sum_ns:   AtomicU64,
    /// Total observations
    count:    AtomicU64,
}

impl LatencyHistogram {
    pub const fn new() -> Self {
        Self {
            buckets: [
                AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0),
                AtomicU64::new(0), AtomicU64::new(0),
            ],
            sum_ns:  AtomicU64::new(0),
            count:   AtomicU64::new(0),
        }
    }

    /// Record a single latency observation (nanoseconds)
    #[inline(always)]
    pub fn record(&self, ns: SigmaU64) {
        // Binary search for bucket (linear for 8 buckets)
        let mut bucket = HISTOGRAM_BUCKETS - 1;
        for (i, &boundary) in LATENCY_BUCKETS.iter().enumerate() {
            if ns <= boundary {
                bucket = i;
                break;
            }
        }
        self.buckets[bucket].fetch_add(1, Ordering::Relaxed);
        self.sum_ns.fetch_add(ns, Ordering::Relaxed);
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    /// Compute approximate percentile (P50, P95, P99, etc.)
    pub fn percentile(&self, p: SigmaU64) -> SigmaU64 {
        let total = self.count.load(Ordering::Relaxed);
        if total == 0 { return 0; }

        let target = (total * p) / 100;
        let mut cumulative: SigmaU64 = 0;

        for (i, bucket) in self.buckets.iter().enumerate() {
            cumulative += bucket.load(Ordering::Relaxed);
            if cumulative >= target {
                return LATENCY_BUCKETS[i];
            }
        }
        LATENCY_BUCKETS[HISTOGRAM_BUCKETS - 1]
    }

    pub fn mean_ns(&self) -> SigmaU64 {
        let count = self.count.load(Ordering::Relaxed);
        if count == 0 { return 0; }
        self.sum_ns.load(Ordering::Relaxed) / count
    }
}

// ─── Metrics Snapshot ────────────────────────────────────────────────────────

/// Point-in-time snapshot of a shard's metrics (for broadcast/export)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct MetricsSnapshot {
    pub shard_id:    SigmaU32,
    pub cpu_ns:      SigmaU64,
    pub ipc_sent:    SigmaU64,
    pub ipc_recv:    SigmaU64,
    pub memory_kb:   SigmaU64,
    pub page_faults: SigmaU64,
    pub state:       ShardState,
}

impl MetricsSnapshot {
    pub const fn zero() -> Self {
        Self {
            shard_id:    0,
            cpu_ns:      0,
            ipc_sent:    0,
            ipc_recv:    0,
            memory_kb:   0,
            page_faults: 0,
            state:       ShardState::Unregistered,
        }
    }

    pub fn is_active(&self) -> bool {
        !matches!(self.state, ShardState::Unregistered)
    }
}

// ─── System-Wide Metrics ─────────────────────────────────────────────────────

/// System-level aggregated metrics — updated by the profiler broadcast loop
#[repr(C)]
pub struct SystemMetrics {
    pub timestamp_ms:       SigmaU64,
    pub total_cpu_ns:       SigmaU64,     // sum across all shards
    pub total_ipc_msgs:     SigmaU64,     // total IPC messages/interval
    pub active_shard_count: SigmaU32,
    pub ipc_ring_pressure:  SigmaU32,     // sigma-bus ring fill % (0-100)
    pub ipc_p50_ns:         SigmaU64,
    pub ipc_p99_ns:         SigmaU64,
}

// ─── Sovereign Profiler ──────────────────────────────────────────────────────

/// The Sovereign Profiler — central metrics aggregator and broadcaster.
/// Maintains one ShardMetrics per registered shard (max 256).
/// Runs a broadcast loop at 10Hz (100ms interval) to publish to /sigma/metrics.
pub struct SovereignProfiler {
    pub initialized:       SigmaBool,
    shards:                [ShardMetrics; MAX_SHARDS],
    ipc_latency:           LatencyHistogram,
    last_broadcast_ms:     SigmaU64,
    broadcast_count:       SigmaU64,
    active_count:          SigmaU32,
}

impl SovereignProfiler {
    pub const fn new() -> Self {
        // Can't use array repeat for non-Copy types in const context without unsafe
        // Safety: AtomicU64 is valid when zero-initialized
        unsafe {
            let mut p: Self = core::mem::zeroed();
            p.initialized = false;
            p.last_broadcast_ms = 0;
            p.broadcast_count = 0;
            p.active_count = 0;
            p
        }
    }

    /// Initialize the profiler — must be called before any shard registration
    pub unsafe fn init(&mut self) {
        if self.initialized { return; }

        // Register the profiler's own broadcast loop with the kernel timer
        kernel_timer_register_periodic(profiler_broadcast_tick, BROADCAST_INTERVAL_MS);

        self.last_broadcast_ms = kernel_monotonic_ms();
        self.initialized = true;
    }

    /// Register a new shard with the profiler
    ///
    /// Returns the shard's metrics slot index, or u32::MAX on error.
    pub fn register_shard(&mut self, shard_id: SigmaU32) -> SigmaU32 {
        // Find an unregistered slot
        for (i, metrics) in self.shards.iter().enumerate() {
            if metrics.current_state() == ShardState::Unregistered {
                metrics.shard_id.store(shard_id, Ordering::Relaxed);
                metrics.set_state(ShardState::Initializing);
                self.active_count.wrapping_add(1);
                return i as SigmaU32;
            }
        }
        SigmaU32::MAX // No slot available
    }

    /// Unregister a shard (on clean shutdown)
    pub fn unregister_shard(&mut self, slot: SigmaU32) {
        if slot < MAX_SHARDS as SigmaU32 {
            self.shards[slot as usize].set_state(ShardState::Unregistered);
        }
    }

    /// Get mutable reference to a shard's metrics by slot index
    #[inline(always)]
    pub fn metrics(&self, slot: SigmaU32) -> Option<&ShardMetrics> {
        self.shards.get(slot as usize)
    }

    /// Record an IPC latency observation
    #[inline(always)]
    pub fn record_ipc_latency(&self, ns: SigmaU64) {
        self.ipc_latency.record(ns);
    }

    /// Called by kernel timer every 100ms — broadcasts metrics snapshot
    pub unsafe fn broadcast(&mut self) {
        let now_ms = kernel_monotonic_ms();
        self.last_broadcast_ms = now_ms;
        self.broadcast_count += 1;

        // Build system-wide snapshot
        let system = SystemMetrics {
            timestamp_ms:       now_ms,
            total_cpu_ns:       self.sum_cpu_ns(),
            total_ipc_msgs:     self.sum_ipc_msgs(),
            active_shard_count: self.active_count,
            ipc_ring_pressure:  kernel_sigma_bus_ring_pressure(),
            ipc_p50_ns:         self.ipc_latency.percentile(50),
            ipc_p99_ns:         self.ipc_latency.percentile(99),
        };

        // Publish to /sigma/metrics shared memory page
        kernel_metrics_shm_write_system(&system as *const _);

        // Publish per-shard snapshots
        for (i, metrics) in self.shards.iter().enumerate() {
            if metrics.current_state() != ShardState::Unregistered {
                let snap = metrics.snapshot();
                kernel_metrics_shm_write_shard(i as SigmaU32, &snap as *const _);
                // Reset rate counters after publishing
                metrics.reset_rate_counters();
            }
        }

        // Broadcast to sigma-bus so Zenith HUD and CLI can subscribe
        kernel_sigma_bus_publish_metrics(&system as *const _, self.shards.as_ptr());
    }

    fn sum_cpu_ns(&self) -> SigmaU64 {
        self.shards.iter()
            .filter(|m| m.current_state() != ShardState::Unregistered)
            .fold(0u64, |acc, m| acc + m.cpu_ns.load(Ordering::Relaxed))
    }

    fn sum_ipc_msgs(&self) -> SigmaU64 {
        self.shards.iter()
            .filter(|m| m.current_state() != ShardState::Unregistered)
            .fold(0u64, |acc, m| {
                acc + m.ipc_msgs_sent.load(Ordering::Relaxed)
                    + m.ipc_msgs_recv.load(Ordering::Relaxed)
            })
    }
}

// ─── Global Singleton ────────────────────────────────────────────────────────

static mut PROFILER: SovereignProfiler = SovereignProfiler::new();

// ─── C-ABI Exports ───────────────────────────────────────────────────────────

#[no_mangle]
pub unsafe extern "C" fn profiler_init() {
    PROFILER.init();
}

#[no_mangle]
pub unsafe extern "C" fn profiler_register_shard(shard_id: SigmaU32) -> SigmaU32 {
    PROFILER.register_shard(shard_id)
}

#[no_mangle]
pub unsafe extern "C" fn profiler_unregister_shard(slot: SigmaU32) {
    PROFILER.unregister_shard(slot);
}

#[no_mangle]
pub unsafe extern "C" fn profiler_record_cpu_ns(slot: SigmaU32, ns: SigmaU64) {
    if let Some(m) = PROFILER.metrics(slot) {
        m.record_cpu_ns(ns);
    }
}

#[no_mangle]
pub unsafe extern "C" fn profiler_record_ipc_send(slot: SigmaU32) {
    if let Some(m) = PROFILER.metrics(slot) {
        m.record_ipc_send();
    }
}

#[no_mangle]
pub unsafe extern "C" fn profiler_record_ipc_recv(slot: SigmaU32) {
    if let Some(m) = PROFILER.metrics(slot) {
        m.record_ipc_recv();
    }
}

#[no_mangle]
pub unsafe extern "C" fn profiler_set_shard_state(slot: SigmaU32, state: SigmaU64) {
    if let Some(m) = PROFILER.metrics(slot) {
        let s = match state {
            1 => ShardState::Initializing,
            2 => ShardState::Running,
            3 => ShardState::Blocked,
            4 => ShardState::Idle,
            5 => ShardState::Panic,
            6 => ShardState::Quarantined,
            _ => ShardState::Unregistered,
        };
        m.set_state(s);
    }
}

#[no_mangle]
pub unsafe extern "C" fn profiler_record_ipc_latency(ns: SigmaU64) {
    PROFILER.record_ipc_latency(ns);
}

#[no_mangle]
pub unsafe extern "C" fn profiler_ipc_p50_ns() -> SigmaU64 {
    PROFILER.ipc_latency.percentile(50)
}

#[no_mangle]
pub unsafe extern "C" fn profiler_ipc_p99_ns() -> SigmaU64 {
    PROFILER.ipc_latency.percentile(99)
}

#[no_mangle]
pub unsafe extern "C" fn profiler_broadcast_tick() {
    PROFILER.broadcast();
}

#[no_mangle]
pub unsafe extern "C" fn profiler_broadcast_count() -> SigmaU64 {
    PROFILER.broadcast_count
}

// ─── Kernel HAL Externs ───────────────────────────────────────────────────────

extern "C" {
    fn kernel_monotonic_ms() -> SigmaU64;
    fn kernel_timer_register_periodic(f: unsafe extern "C" fn(), interval_ms: SigmaU64);
    fn kernel_sigma_bus_ring_pressure() -> SigmaU32;
    fn kernel_metrics_shm_write_system(sys: *const SystemMetrics);
    fn kernel_metrics_shm_write_shard(slot: SigmaU32, snap: *const MetricsSnapshot);
    fn kernel_sigma_bus_publish_metrics(sys: *const SystemMetrics, shards: *const ShardMetrics);
}
