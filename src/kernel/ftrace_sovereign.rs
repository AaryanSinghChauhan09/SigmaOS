//! SigmaOS Sovereign Function Tracer (ftrace)
//! Implements Linux ftrace-style kernel function tracing in 100% safe Rust.
//!
//! Linux ftrace (merged in Linux 2.6.27, 2008) provides:
//!   - Function call tracing with timestamps
//!   - Function graph tracing (call + return)
//!   - Event tracing (tracepoints)
//!   - Latency histograms (hist triggers)
//!   - Per-CPU ring buffers
//!   - Filtering by function, pid, cpu

#![allow(dead_code)]
#![allow(clippy::new_without_default)]

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

// ─── Trace Event Types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum TraceEventKind {
    FunctionEntry,   // function tracer: func enter
    FunctionReturn,  // function graph: func return
    Tracepoint,      // static tracepoint (like TRACE_EVENT macro)
    KprobeHit,       // dynamic kprobe
    UprobeHit,       // dynamic uprobe
    SchedSwitch,     // sched:sched_switch tracepoint
    SchedWakeup,     // sched:sched_wakeup
    IrqEntry,        // irq:irq_handler_entry
    IrqExit,         // irq:irq_handler_exit
    SyscallEntry,    // raw_syscalls:sys_enter
    SyscallExit,     // raw_syscalls:sys_exit
}

impl TraceEventKind {
    pub fn name(&self) -> &'static str {
        match self {
            TraceEventKind::FunctionEntry  => "funcentry",
            TraceEventKind::FunctionReturn => "funcreturn",
            TraceEventKind::Tracepoint     => "tracepoint",
            TraceEventKind::KprobeHit      => "kprobe",
            TraceEventKind::UprobeHit      => "uprobe",
            TraceEventKind::SchedSwitch    => "sched_switch",
            TraceEventKind::SchedWakeup    => "sched_wakeup",
            TraceEventKind::IrqEntry       => "irq_entry",
            TraceEventKind::IrqExit        => "irq_exit",
            TraceEventKind::SyscallEntry   => "sys_enter",
            TraceEventKind::SyscallExit    => "sys_exit",
        }
    }
}

// ─── Trace Event Record ───────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TraceEvent {
    pub kind: TraceEventKind,
    pub timestamp_ns: u64,
    pub cpu: u8,
    pub pid: u32,
    pub comm: String,         // task name (up to 16 chars like Linux comm)
    pub func_name: String,    // function or tracepoint name
    pub duration_ns: u64,     // for FunctionReturn: call duration
    pub depth: u8,            // call stack depth (function graph)
    pub return_value: i64,    // for SyscallExit / FunctionReturn
}

impl TraceEvent {
    pub fn function_entry(ts: u64, cpu: u8, pid: u32, comm: &str, func: &str, depth: u8) -> Self {
        TraceEvent {
            kind: TraceEventKind::FunctionEntry,
            timestamp_ns: ts,
            cpu,
            pid,
            comm: comm[..comm.len().min(16)].to_string(),
            func_name: func.to_string(),
            duration_ns: 0,
            depth,
            return_value: 0,
        }
    }

    pub fn function_return(ts: u64, cpu: u8, pid: u32, comm: &str, func: &str, depth: u8, duration: u64, retval: i64) -> Self {
        TraceEvent {
            kind: TraceEventKind::FunctionReturn,
            timestamp_ns: ts,
            cpu,
            pid,
            comm: comm[..comm.len().min(16)].to_string(),
            func_name: func.to_string(),
            duration_ns: duration,
            depth,
            return_value: retval,
        }
    }

    pub fn sched_switch(ts: u64, cpu: u8, prev_pid: u32, next_pid: u32, prev_comm: &str, next_comm: &str) -> Self {
        let mut func = String::from(prev_comm);
        func.push_str("->");
        func.push_str(next_comm);
        TraceEvent {
            kind: TraceEventKind::SchedSwitch,
            timestamp_ns: ts,
            cpu,
            pid: prev_pid,
            comm: next_pid.to_string(), // store next_pid in comm for display
            func_name: func,
            duration_ns: 0,
            depth: 0,
            return_value: 0,
        }
    }

    pub fn format_line(&self) -> String {
        let mut line = String::new();
        line.push_str(&self.comm);
        line.push_str("-");
        line.push_str(&self.pid.to_string());
        line.push_str(" [cpu");
        line.push_str(&self.cpu.to_string());
        line.push_str("] ");
        line.push_str(&self.timestamp_ns.to_string());
        line.push_str("ns ");
        line.push_str(self.kind.name());
        line.push(' ');
        line.push_str(&self.func_name);
        if self.duration_ns > 0 {
            line.push_str(" (");
            line.push_str(&self.duration_ns.to_string());
            line.push_str("ns)");
        }
        line
    }
}

// ─── Per-CPU Ring Buffer ──────────────────────────────────────────────────────

pub struct TraceRingBuffer {
    pub cpu: u8,
    pub events: Vec<TraceEvent>,
    pub capacity: usize,
    pub total_written: u64,
    pub total_dropped: u64,
    pub overwrite: bool,  // true = overwrite oldest (like trace_pipe)
}

impl TraceRingBuffer {
    pub fn new(cpu: u8, capacity: usize, overwrite: bool) -> Self {
        TraceRingBuffer {
            cpu,
            events: Vec::new(),
            capacity,
            total_written: 0,
            total_dropped: 0,
            overwrite,
        }
    }

    pub fn write(&mut self, event: TraceEvent) {
        if self.events.len() >= self.capacity {
            if self.overwrite {
                self.events.remove(0); // Drop oldest
            } else {
                self.total_dropped = self.total_dropped.saturating_add(1);
                return;
            }
        }
        self.events.push(event);
        self.total_written = self.total_written.saturating_add(1);
    }

    pub fn read_all(&mut self) -> Vec<TraceEvent> {
        let mut out = Vec::new();
        core::mem::swap(&mut out, &mut self.events);
        out
    }

    pub fn available(&self) -> usize { self.events.len() }
}

// ─── Tracer Filter ────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct TracerFilter {
    pub pid_filter: Option<u32>,          // only trace this PID
    pub func_prefix: Option<String>,      // only trace funcs starting with prefix
    pub min_duration_ns: Option<u64>,     // only record if duration >= N ns
    pub event_kinds: Vec<TraceEventKind>, // only these event types
}

impl TracerFilter {
    pub fn none() -> Self {
        TracerFilter {
            pid_filter: None,
            func_prefix: None,
            min_duration_ns: None,
            event_kinds: Vec::new(),
        }
    }

    pub fn passes(&self, event: &TraceEvent) -> bool {
        if let Some(pid) = self.pid_filter {
            if event.pid != pid { return false; }
        }
        if let Some(ref prefix) = self.func_prefix {
            if !event.func_name.starts_with(prefix.as_str()) { return false; }
        }
        if let Some(min_dur) = self.min_duration_ns {
            if event.duration_ns < min_dur { return false; }
        }
        if !self.event_kinds.is_empty() && !self.event_kinds.contains(&event.kind) {
            return false;
        }
        true
    }
}

// ─── Latency Histogram (hist trigger) ────────────────────────────────────────

pub struct LatencyHistogram {
    pub func_name: String,
    pub buckets_us: Vec<u64>, // bucket upper bounds in µs
    pub counts: Vec<u64>,
    pub total_samples: u64,
    pub sum_ns: u64,
    pub max_ns: u64,
    pub min_ns: u64,
}

impl LatencyHistogram {
    /// Create histogram with logarithmic buckets: 1µs, 10µs, 100µs, 1ms, 10ms, 100ms, ∞
    pub fn new(func_name: &str) -> Self {
        let buckets_us = vec![1, 10, 100, 1_000, 10_000, 100_000, u64::MAX];
        let counts = vec![0u64; buckets_us.len()];
        LatencyHistogram {
            func_name: func_name.to_string(),
            buckets_us,
            counts,
            total_samples: 0,
            sum_ns: 0,
            max_ns: 0,
            min_ns: u64::MAX,
        }
    }

    pub fn record(&mut self, duration_ns: u64) {
        let duration_us = duration_ns / 1000;
        for (i, &bucket) in self.buckets_us.iter().enumerate() {
            if duration_us <= bucket {
                self.counts[i] = self.counts[i].saturating_add(1);
                break;
            }
        }
        self.total_samples = self.total_samples.saturating_add(1);
        self.sum_ns = self.sum_ns.saturating_add(duration_ns);
        self.max_ns = self.max_ns.max(duration_ns);
        self.min_ns = self.min_ns.min(duration_ns);
    }

    pub fn avg_ns(&self) -> u64 {
        if self.total_samples == 0 { return 0; }
        self.sum_ns / self.total_samples
    }
}

// ─── Sovereign ftrace Tracer ──────────────────────────────────────────────────

pub struct SovereignFtracer {
    pub enabled: bool,
    pub buffers: Vec<TraceRingBuffer>,   // one per CPU
    pub filter: TracerFilter,
    pub histograms: Vec<LatencyHistogram>,
    pub total_events: u64,
}

impl SovereignFtracer {
    pub fn new(cpu_count: u8, buffer_size: usize) -> Self {
        let buffers = (0..cpu_count)
            .map(|cpu| TraceRingBuffer::new(cpu, buffer_size, true))
            .collect();
        SovereignFtracer {
            enabled: false,
            buffers,
            filter: TracerFilter::none(),
            histograms: Vec::new(),
            total_events: 0,
        }
    }

    pub fn enable(&mut self)  { self.enabled = true; }
    pub fn disable(&mut self) { self.enabled = false; }

    pub fn trace(&mut self, event: TraceEvent) {
        if !self.enabled { return; }
        if !self.filter.passes(&event) { return; }

        // Record in histogram if applicable
        if event.kind == TraceEventKind::FunctionReturn && event.duration_ns > 0 {
            let func = event.func_name.clone();
            let dur  = event.duration_ns;
            if let Some(hist) = self.histograms.iter_mut().find(|h| h.func_name == func) {
                hist.record(dur);
            }
        }

        let cpu = event.cpu as usize;
        if let Some(buf) = self.buffers.get_mut(cpu) {
            buf.write(event);
            self.total_events = self.total_events.saturating_add(1);
        }
    }

    pub fn add_histogram(&mut self, func_name: &str) {
        self.histograms.push(LatencyHistogram::new(func_name));
    }

    pub fn read_events(&mut self, cpu: u8) -> Vec<TraceEvent> {
        if let Some(buf) = self.buffers.get_mut(cpu as usize) {
            buf.read_all()
        } else { Vec::new() }
    }

    pub fn total_available(&self) -> usize {
        self.buffers.iter().map(|b| b.available()).sum()
    }

    pub fn clear_all(&mut self) {
        for buf in &mut self.buffers {
            buf.events.clear();
        }
        self.total_events = 0;
    }
}

// ─── Tests ────────────────────────────────────────────────────────────────────
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ftrace_enable_disable() {
        let mut tracer = SovereignFtracer::new(4, 1024);
        let ev = TraceEvent::function_entry(1000, 0, 100, "kworker", "schedule", 0);
        tracer.trace(ev.clone()); // disabled → not recorded
        assert_eq!(tracer.total_available(), 0);
        tracer.enable();
        tracer.trace(ev);
        assert_eq!(tracer.total_available(), 1);
    }

    #[test]
    fn test_ftrace_pid_filter() {
        let mut tracer = SovereignFtracer::new(4, 1024);
        tracer.filter.pid_filter = Some(42);
        tracer.enable();
        tracer.trace(TraceEvent::function_entry(1000, 0, 99, "bash", "vfs_read", 0));
        assert_eq!(tracer.total_available(), 0); // wrong PID filtered
        tracer.trace(TraceEvent::function_entry(2000, 0, 42, "bash", "vfs_read", 0));
        assert_eq!(tracer.total_available(), 1); // correct PID passes
    }

    #[test]
    fn test_ring_buffer_overwrite() {
        let mut buf = TraceRingBuffer::new(0, 3, true);
        for i in 0..5u64 {
            buf.write(TraceEvent::function_entry(i * 100, 0, i as u32, "proc", "func", 0));
        }
        assert_eq!(buf.available(), 3);   // oldest overwritten
        assert_eq!(buf.total_written, 5); // 5 written total
    }

    #[test]
    fn test_latency_histogram() {
        let mut hist = LatencyHistogram::new("vfs_read");
        hist.record(500);          // 0.5µs — first bucket (≤1µs)
        hist.record(5_000);        // 5µs — second bucket (≤10µs)
        hist.record(50_000);       // 50µs — third bucket (≤100µs)
        hist.record(5_000_000);    // 5ms — fifth bucket (≤10ms)
        assert_eq!(hist.total_samples, 4);
        assert_eq!(hist.max_ns, 5_000_000);
        assert_eq!(hist.min_ns, 500);
        assert!(hist.avg_ns() > 0);
    }

    #[test]
    fn test_function_graph_tracing() {
        let mut tracer = SovereignFtracer::new(2, 512);
        tracer.add_histogram("do_sys_open");
        tracer.enable();
        tracer.trace(TraceEvent::function_entry(1000, 0, 1, "bash", "do_sys_open", 1));
        tracer.trace(TraceEvent::function_return(5000, 0, 1, "bash", "do_sys_open", 1, 4000, 3));
        let evs = tracer.read_events(0);
        assert_eq!(evs.len(), 2);
        assert_eq!(evs[1].duration_ns, 4000);
        // Histogram was recorded
        let hist = tracer.histograms.iter().find(|h| h.func_name == "do_sys_open").unwrap();
        assert_eq!(hist.total_samples, 1);
    }

    #[test]
    fn test_sched_switch_event_format() {
        let ev = TraceEvent::sched_switch(1_000_000, 1, 100, 200, "nginx", "kworker");
        assert_eq!(ev.kind, TraceEventKind::SchedSwitch);
        let line = ev.format_line();
        assert!(line.contains("sched_switch"));
        assert!(line.contains("nginx->kworker"));
    }
}
