use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::format;
// SigmaOS — DTrace/eBPF-Inspired Kernel Tracing Engine
//
// Provides static probe points (like DTrace's USDT/SDT probes) and a
// structured record collector for runtime kernel observability — without
// the complexity of a full bytecode verifier (that is on the roadmap).
//
// References:
//   Bryan Cantrill et al., "Dynamic Instrumentation of Production Systems,"
//   USENIX ATC 2004.
//   Linux eBPF documentation — https://ebpf.io/
//
// This implementation is purely custom — no std, no libc, no external crates.

// ─────────────────────────────────────────────────────────────────────────────
// Probe type (mirrors DTrace probe naming: provider:module:function:name)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProbeType {
    /// Function entry (like kprobe / DTrace `entry`).
    Entry,
    /// Function return (like kretprobe / DTrace `return`).
    Return,
    /// Error / exception path.
    Error,
    /// User-defined custom probe.
    Custom,
}

// ─────────────────────────────────────────────────────────────────────────────
// Probe descriptor
// ─────────────────────────────────────────────────────────────────────────────

/// A registered observation point in the kernel.
///
/// Once registered, a probe is identified by its index in the `TraceEngine`'s
/// probe list.  Probes can be enabled/disabled at runtime.
#[derive(Debug)]
pub struct Probe {
    /// Provider name (e.g. `"syscall"`, `"vfs"`, `"net"`).
    pub provider: &'static str,
    /// Module / subsystem (e.g. `"kernel"`, `"scheduler"`).
    pub module: &'static str,
    /// Function name (e.g. `"read"`, `"open"`, `"fork"`).
    pub function: &'static str,
    /// Probe variant within the function (e.g. `"entry"`, `"return"`).
    pub name: &'static str,
    pub kind: ProbeType,
    /// Whether this probe is currently collecting records.
    pub enabled: bool,
    /// Total times this probe has fired since registration.
    pub fire_count: u64,
}

impl Probe {
    pub const fn new(
        provider: &'static str,
        module: &'static str,
        function: &'static str,
        name: &'static str,
        kind: ProbeType,
    ) -> Self {
        Self {
            provider,
            module,
            function,
            name,
            kind,
            enabled: false,
            fire_count: 0,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Trace record
// ─────────────────────────────────────────────────────────────────────────────

/// One collected event from a fired probe.
#[derive(Debug, Clone)]
pub struct TraceRecord {
    /// Kernel timestamp (nanoseconds since boot or epoch).
    pub timestamp: u64,
    /// CPU ID on which the probe fired.
    pub cpu_id: u32,
    /// PID of the process in whose context the probe fired.
    pub pid: u32,
    /// Index of the probe in the TraceEngine's probe list.
    pub probe_idx: usize,
    /// Up to 4 arguments passed by the probe site.
    pub args: [u64; 4],
}

// ─────────────────────────────────────────────────────────────────────────────
// TraceEngine
// ─────────────────────────────────────────────────────────────────────────────

/// The kernel tracing engine.
///
/// Stores registered probes and a ring buffer of collected records.
/// When the buffer is full, new records are dropped (like DTrace's `drop`
/// action).  The caller should drain records frequently.
pub struct TraceEngine {
    probes: Vec<Probe>,
    records: Vec<TraceRecord>,
    /// Maximum records before dropping.
    max_records: usize,
    /// Global enable/disable switch.
    enabled: bool,
    /// Number of records dropped due to buffer full.
    drop_count: u64,
    /// Total records ever produced (including dropped).
    total_fired: u64,
}

impl TraceEngine {
    /// Create a new engine with a fixed-capacity record buffer.
    pub fn new(max_records: usize) -> Self {
        Self {
            probes: Vec::new(),
            records: Vec::new(),
            max_records,
            enabled: false,
            drop_count: 0,
            total_fired: 0,
        }
    }

    // ── Probe management ──────────────────────────────────────────────────────

    /// Register a probe.  Returns the probe's index for use in `fire()`.
    pub fn register(&mut self, probe: Probe) -> usize {
        let idx = self.probes.len();
        self.probes.push(probe);
        idx
    }

    /// Enable a specific probe by index.
    pub fn enable_probe(&mut self, idx: usize) {
        if let Some(p) = self.probes.get_mut(idx) {
            p.enabled = true;
        }
    }

    /// Disable a specific probe by index.
    pub fn disable_probe(&mut self, idx: usize) {
        if let Some(p) = self.probes.get_mut(idx) {
            p.enabled = false;
        }
    }

    /// Enable all probes matching a provider name.
    pub fn enable_provider(&mut self, provider: &str) {
        for i in 0..self.probes.len() {
            if let Some(p) = self.probes.get_mut(i) {
                if p.provider == provider {
                    p.enabled = true;
                }
            }
        }
    }

    /// Disable all probes.
    pub fn disable_all(&mut self) {
        for i in 0..self.probes.len() {
            if let Some(p) = self.probes.get_mut(i) {
                p.enabled = false;
            }
        }
    }

    // ── Global switch ─────────────────────────────────────────────────────────

    /// Enable the global tracing engine.
    pub fn start(&mut self) {
        self.enabled = true;
    }
    /// Pause the global tracing engine (probes remain registered).
    pub fn stop(&mut self) {
        self.enabled = false;
    }
    pub fn is_running(&self) -> bool {
        self.enabled
    }

    // ── Probe firing (called from kernel hot paths) ───────────────────────────

    /// Fire probe at `idx` with the given arguments.
    ///
    /// This is designed to be as fast as possible:
    /// - Early return if global switch is off.
    /// - Early return if probe is disabled.
    /// - Drop record if buffer is full (no blocking).
    pub fn fire(&mut self, idx: usize, timestamp: u64, cpu_id: u32, pid: u32, args: [u64; 4]) {
        if !self.enabled {
            return;
        }
        let enabled = match self.probes.get_mut(idx) {
            Some(p) if p.enabled => {
                p.fire_count = p.fire_count.wrapping_add(1);
                true
            }
            _ => false,
        };
        if !enabled {
            return;
        }
        self.total_fired = self.total_fired.wrapping_add(1);
        if self.records.len() >= self.max_records {
            self.drop_count = self.drop_count.wrapping_add(1);
            return;
        }
        self.records.push(TraceRecord {
            timestamp,
            cpu_id,
            pid,
            probe_idx: idx,
            args,
        });
    }

    // ── Record collection ─────────────────────────────────────────────────────

    /// Drain all accumulated records.
    ///
    /// In a production system a user-space daemon would call this periodically
    /// through a ring-buffer shared-memory mapping.
    pub fn drain(&mut self) -> Vec<TraceRecord> {
        let mut out = Vec::new();
        for i in 0..self.records.len() {
            if let Some(r) = self.records.get(i) {
                out.push(r.clone());
            }
        }
        self.records = Vec::new();
        out
    }

    // ── Statistics ────────────────────────────────────────────────────────────

    pub fn probe_count(&self) -> usize {
        self.probes.len()
    }
    pub fn record_count(&self) -> usize {
        self.records.len()
    }
    pub fn drop_count(&self) -> u64 {
        self.drop_count
    }
    pub fn total_fired(&self) -> u64 {
        self.total_fired
    }

    /// Per-probe fire count.
    pub fn probe_fires(&self, idx: usize) -> u64 {
        self.probes.get(idx).map(|p| p.fire_count).unwrap_or(0)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Convenience macros
// ─────────────────────────────────────────────────────────────────────────────

/// Fire a probe with up to 4 arguments.
///
/// ```rust,ignore
/// use crate::sigma_trace;
/// sigma_trace!(engine, probe_idx, timestamp, cpu, pid);
/// sigma_trace!(engine, probe_idx, timestamp, cpu, pid, arg0);
/// sigma_trace!(engine, probe_idx, timestamp, cpu, pid, arg0, arg1);
/// ```
#[macro_export]
macro_rules! sigma_trace {
    ($eng:expr, $idx:expr, $ts:expr, $cpu:expr, $pid:expr) => {
        $eng.fire($idx, $ts, $cpu, $pid, [0u64; 4])
    };
    ($eng:expr, $idx:expr, $ts:expr, $cpu:expr, $pid:expr, $a0:expr) => {
        $eng.fire($idx, $ts, $cpu, $pid, [$a0 as u64, 0, 0, 0])
    };
    ($eng:expr, $idx:expr, $ts:expr, $cpu:expr, $pid:expr, $a0:expr, $a1:expr) => {
        $eng.fire($idx, $ts, $cpu, $pid, [$a0 as u64, $a1 as u64, 0, 0])
    };
    ($eng:expr, $idx:expr, $ts:expr, $cpu:expr, $pid:expr, $a0:expr, $a1:expr, $a2:expr) => {
        $eng.fire(
            $idx,
            $ts,
            $cpu,
            $pid,
            [$a0 as u64, $a1 as u64, $a2 as u64, 0],
        )
    };
    ($eng:expr, $idx:expr, $ts:expr, $cpu:expr, $pid:expr, $a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $eng.fire(
            $idx,
            $ts,
            $cpu,
            $pid,
            [$a0 as u64, $a1 as u64, $a2 as u64, $a3 as u64],
        )
    };
}

// ─────────────────────────────────────────────────────────────────────────────
// Well-known probe definitions (static table, like DTrace SDT probes)
// ─────────────────────────────────────────────────────────────────────────────

/// Pre-defined probe descriptors for common SigmaOS subsystems.
/// Register these with `engine.register(PROBE_SYSCALL_READ_ENTRY)` etc.
pub const PROBE_SYSCALL_READ_ENTRY: Probe =
    Probe::new("syscall", "kernel", "read", "entry", ProbeType::Entry);
pub const PROBE_SYSCALL_WRITE_ENTRY: Probe =
    Probe::new("syscall", "kernel", "write", "entry", ProbeType::Entry);
pub const PROBE_SYSCALL_OPEN_ENTRY: Probe =
    Probe::new("syscall", "kernel", "open", "entry", ProbeType::Entry);
pub const PROBE_SCHED_SWITCH: Probe =
    Probe::new("sched", "kernel", "switch", "fire", ProbeType::Custom);
pub const PROBE_NET_CONNECT: Probe = Probe::new("net", "tcp", "connect", "entry", ProbeType::Entry);
pub const PROBE_VFS_WRITE: Probe =
    Probe::new("vfs", "sigma_fs", "write", "entry", ProbeType::Entry);
pub const PROBE_SECURITY_PLEDGE: Probe =
    Probe::new("security", "pledge", "check", "entry", ProbeType::Entry);

/// Linux perf_event / DTrace ring buffer event collector
#[derive(Debug, Clone)]
pub struct PerfEvent {
    pub event_type: u32, // 0 = HW_CPU_CYCLES, 1 = HW_INSTRUCTIONS, 2 = SW_PAGE_FAULTS
    pub sample_period: u64,
    pub val: u64,
}

pub struct PerfEventRingBuffer {
    pub events: Vec<PerfEvent>,
    pub max_capacity: usize,
    pub overflow_count: u64,
}

impl PerfEventRingBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            events: Vec::new(),
            max_capacity: capacity,
            overflow_count: 0,
        }
    }

    pub fn record_event(&mut self, event: PerfEvent) -> bool {
        if self.events.len() >= self.max_capacity {
            self.overflow_count += 1;
            false
        } else {
            self.events.push(event);
            true
        }
    }
}

/// Dynamic DTrace / kprobe traceprobe manager
pub struct TraceprobeManager {
    pub engine: TraceEngine,
    pub perf_ring: PerfEventRingBuffer,
}

impl TraceprobeManager {
    pub fn new(capacity: usize) -> Self {
        Self {
            engine: TraceEngine::new(capacity),
            perf_ring: PerfEventRingBuffer::new(capacity),
        }
    }

    pub fn attach_kprobe(&mut self, provider: &'static str, module: &'static str, function: &'static str) -> usize {
        let probe = Probe::new(provider, module, function, "entry", ProbeType::Entry);
        let idx = self.engine.register(probe);
        self.engine.enable_probe(idx);
        idx
    }
}

impl Default for TraceprobeManager {
    fn default() -> Self {
        Self::new(128)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_fire() {
        let mut eng = TraceEngine::new(64);
        let idx = eng.register(PROBE_SYSCALL_READ_ENTRY);
        eng.start();
        eng.enable_probe(idx);
        eng.fire(idx, 1000, 0, 42, [512, 0, 0, 0]);
        assert_eq!(eng.record_count(), 1);
        assert_eq!(eng.probe_fires(idx), 1);
    }

    #[test]
    fn test_disabled_probe_not_recorded() {
        let mut eng = TraceEngine::new(64);
        let idx = eng.register(PROBE_NET_CONNECT);
        eng.start();
        // probe not enabled — should not record
        eng.fire(idx, 1000, 0, 1, [0; 4]);
        assert_eq!(eng.record_count(), 0);
    }

    #[test]
    fn test_global_stop_suppresses_all() {
        let mut eng = TraceEngine::new(64);
        let idx = eng.register(PROBE_SCHED_SWITCH);
        eng.enable_probe(idx);
        // engine not started — should not record
        eng.fire(idx, 1000, 0, 1, [0; 4]);
        assert_eq!(eng.record_count(), 0);
    }

    #[test]
    fn test_buffer_full_drops() {
        let mut eng = TraceEngine::new(2);
        let idx = eng.register(PROBE_VFS_WRITE);
        eng.start();
        eng.enable_probe(idx);
        eng.fire(idx, 1, 0, 1, [0; 4]);
        eng.fire(idx, 2, 0, 1, [0; 4]);
        eng.fire(idx, 3, 0, 1, [0; 4]); // should drop
        assert_eq!(eng.record_count(), 2);
        assert_eq!(eng.drop_count(), 1);
    }

    #[test]
    fn test_drain_clears_buffer() {
        let mut eng = TraceEngine::new(64);
        let idx = eng.register(PROBE_SYSCALL_WRITE_ENTRY);
        eng.start();
        eng.enable_probe(idx);
        eng.fire(idx, 100, 0, 7, [1024, 0, 0, 0]);
        let records = eng.drain();
        assert_eq!(records.len(), 1);
        assert_eq!(eng.record_count(), 0, "drain must clear buffer");
        assert_eq!(records.get(0).unwrap().args[0], 1024);
    }

    #[test]
    fn test_enable_provider() {
        let mut eng = TraceEngine::new(64);
        let _r = eng.register(PROBE_SYSCALL_READ_ENTRY);
        let _w = eng.register(PROBE_SYSCALL_WRITE_ENTRY);
        let _n = eng.register(PROBE_NET_CONNECT);
        eng.enable_provider("syscall");
        eng.start();
        // Only syscall probes (index 0, 1) are enabled; net (index 2) is not.
        eng.fire(0, 1, 0, 1, [0; 4]);
        eng.fire(1, 2, 0, 1, [0; 4]);
        eng.fire(2, 3, 0, 1, [0; 4]);
        assert_eq!(eng.record_count(), 2);
    }

    #[test]
    fn test_traceprobe_manager_and_perf_events() {
        let mut mgr = TraceprobeManager::new(10);
        let idx = mgr.attach_kprobe("vfs", "kernel", "sys_open");

        assert_eq!(mgr.engine.probe_count(), 1);
        mgr.engine.start();
        mgr.engine.fire(idx, 100, 0, 1, [1, 2, 3, 4]);
        assert_eq!(mgr.engine.record_count(), 1);

        let perf_ev = PerfEvent {
            event_type: 0,
            sample_period: 1000,
            val: 50000,
        };
        assert!(mgr.perf_ring.record_event(perf_ev));
        assert_eq!(mgr.perf_ring.events.len(), 1);
    }
}
