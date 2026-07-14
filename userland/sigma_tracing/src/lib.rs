pub mod trace;
pub mod perf;
pub mod probe;

pub use trace::{TraceCollector, SyscallEvent};
pub use perf::{PerfCounter, PerfMetric};
pub use probe::ProbeManager;

/// SigmaTracing: Native System Profiling & Tracing
/// Displaces `strace`, `perf`, and eBPF with a unified, zero-overhead Rust daemon.
pub struct SigmaTracing {
    pub collector: TraceCollector,
    pub perf: PerfCounter,
    pub probes: ProbeManager,
}

impl Default for SigmaTracing {
    fn default() -> Self {
        Self::new()
    }
}

impl SigmaTracing {
    pub fn new() -> Self {
        Self {
            collector: TraceCollector::new(),
            perf: PerfCounter::new(),
            probes: ProbeManager::new(),
        }
    }
}
