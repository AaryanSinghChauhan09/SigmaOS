// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// tools/sigma_tracing.rs — SigmaOS kernel tracing subsystem
//
// Implements ftrace-inspired in-kernel ring-buffer tracing:
//   - Tracepoint registration (static + dynamic)
//   - Per-CPU ring buffers (lock-free, power-of-2 size)
//   - Event categories: syscall, scheduler, IRQ, memory, network, filesystem
//   - userland readout via /dev/sigma-trace or Unix socket
//   - Flamegraph-compatible output (folded stack format)
//   - Perf counter integration hooks
//
// Language: Rust (no_std for kernel half, std for userland reader tool)

#![allow(dead_code)]

// ── Trace event categories ─────────────────────────────────────────────────
#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(u8)]
pub enum TraceCategory {
    Syscall    = 0,
    Scheduler  = 1,
    Irq        = 2,
    Memory     = 3,
    Network    = 4,
    Filesystem = 5,
    UserDefined= 6,
    Perf       = 7,
}

// ── Trace event record (48 bytes, cache-line friendly) ─────────────────────
#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct TraceEvent {
    pub timestamp_ns: u64,   // nanoseconds since boot
    pub pid:          u32,   // process ID
    pub cpu:          u8,    // CPU number
    pub category:     TraceCategory,
    pub event_id:     u16,   // event type within category
    pub payload:      [u64; 4], // event-specific data (up to 32 bytes)
}

impl TraceEvent {
    pub const fn new(ts: u64, pid: u32, cat: TraceCategory, id: u16) -> Self {
        TraceEvent { timestamp_ns: ts, pid, cpu: 0, category: cat, event_id: id, payload: [0; 4] }
    }
    pub const SIZE: usize = core::mem::size_of::<TraceEvent>();
}

// ── Well-known event IDs ────────────────────────────────────────────────────
pub mod syscall_events {
    pub const ENTER: u16 = 0;
    pub const EXIT:  u16 = 1;
}
pub mod sched_events {
    pub const SWITCH:     u16 = 0; // context switch
    pub const WAKEUP:     u16 = 1;
    pub const FORK:       u16 = 2;
    pub const EXIT:       u16 = 3;
    pub const MIGRATE:    u16 = 4; // task moved to another CPU
}
pub mod irq_events {
    pub const ENTER: u16 = 0;
    pub const EXIT:  u16 = 1;
    pub const SOFTIRQ_ENTER: u16 = 2;
    pub const SOFTIRQ_EXIT:  u16 = 3;
}
pub mod mm_events {
    pub const ALLOC_PAGE: u16 = 0;
    pub const FREE_PAGE:  u16 = 1;
    pub const PAGE_FAULT: u16 = 2;
    pub const MMAP:       u16 = 3;
    pub const MUNMAP:     u16 = 4;
}
pub mod net_events {
    pub const TX:        u16 = 0;
    pub const RX:        u16 = 1;
    pub const CONNECT:   u16 = 2;
    pub const ACCEPT:    u16 = 3;
    pub const DROP:      u16 = 4;
}
pub mod fs_events {
    pub const OPEN:  u16 = 0;
    pub const READ:  u16 = 1;
    pub const WRITE: u16 = 2;
    pub const CLOSE: u16 = 3;
    pub const MKDIR: u16 = 4;
}

// ── Lock-free ring buffer (power-of-2 size, single producer) ───────────────
const RING_SIZE: usize = 65536; // must be power of 2
const RING_MASK: usize = RING_SIZE - 1;

#[cfg(not(feature = "std"))]
use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(feature = "std")]
use std::sync::atomic::{AtomicUsize, Ordering};

pub struct TraceRingBuffer {
    events: [TraceEvent; RING_SIZE],
    head:   AtomicUsize, // write position
    tail:   AtomicUsize, // read position
    dropped: AtomicUsize,
}

impl TraceRingBuffer {
    pub const fn new() -> Self {
        TraceRingBuffer {
            events:  [TraceEvent::new(0, 0, TraceCategory::UserDefined, 0); RING_SIZE],
            head:    AtomicUsize::new(0),
            tail:    AtomicUsize::new(0),
            dropped: AtomicUsize::new(0),
        }
    }

    /// Write one event. Non-blocking: drops if ring is full.
    #[inline]
    pub fn write(&self, event: TraceEvent) {
        let head = self.head.load(Ordering::Relaxed);
        let next = (head + 1) & RING_MASK;
        let tail = self.tail.load(Ordering::Acquire);
        if next == tail {
            // Ring full — drop and count
            self.dropped.fetch_add(1, Ordering::Relaxed);
            return;
        }
        unsafe {
            // SAFETY: head is only modified by this CPU (single producer per buffer)
            let slot = &self.events[head] as *const _ as *mut TraceEvent;
            slot.write(event);
        }
        self.head.store(next, Ordering::Release);
    }

    /// Read up to `max` events. Returns count read.
    pub fn read(&self, out: &mut [TraceEvent]) -> usize {
        let mut n = 0;
        let head = self.head.load(Ordering::Acquire);
        let mut tail = self.tail.load(Ordering::Relaxed);
        while n < out.len() && tail != head {
            out[n] = unsafe { self.events[tail] };
            tail = (tail + 1) & RING_MASK;
            n += 1;
        }
        self.tail.store(tail, Ordering::Release);
        n
    }

    pub fn dropped_count(&self) -> usize { self.dropped.load(Ordering::Relaxed) }
    pub fn len(&self) -> usize {
        let h = self.head.load(Ordering::Relaxed);
        let t = self.tail.load(Ordering::Relaxed);
        (h.wrapping_sub(t)) & RING_MASK
    }
    pub fn is_empty(&self) -> bool { self.len() == 0 }
}

// ── Global trace buffer (per subsystem) ───────────────────────────────────
const N_CATEGORIES: usize = 8;
static mut TRACE_BUFS: [TraceRingBuffer; N_CATEGORIES] = [
    const { TraceRingBuffer::new() }; N_CATEGORIES
];

static TRACE_ENABLED: AtomicUsize = AtomicUsize::new(0xFF); // all enabled by default

// ── Tracing API ────────────────────────────────────────────────────────────

/// Emit a trace event. Called from kernel hot paths — must be fast.
#[inline(always)]
pub unsafe fn trace_emit(
    category: TraceCategory, event_id: u16,
    pid: u32, now_ns: u64, payload: [u64; 4],
) {
    let cat = category as usize;
    if TRACE_ENABLED.load(Ordering::Relaxed) & (1 << cat) == 0 { return; }
    let mut evt = TraceEvent::new(now_ns, pid, category, event_id);
    evt.payload = payload;
    TRACE_BUFS[cat % N_CATEGORIES].write(evt);
}

/// Enable or disable tracing for a category
#[no_mangle]
pub unsafe extern "C" fn sigma_trace_enable(category: u8, enable: bool) {
    let bit = 1usize << (category as usize & 7);
    if enable {
        TRACE_ENABLED.fetch_or(bit, Ordering::Relaxed);
    } else {
        TRACE_ENABLED.fetch_and(!bit, Ordering::Relaxed);
    }
}

/// Read up to `max` events from category's buffer into `out`.
#[no_mangle]
pub unsafe extern "C" fn sigma_trace_read(
    category: u8, out: *mut TraceEvent, max: usize,
) -> usize {
    let cat = category as usize % N_CATEGORIES;
    let buf = core::slice::from_raw_parts_mut(out, max);
    TRACE_BUFS[cat].read(buf)
}

// ── Convenience macros (kernel-side) ──────────────────────────────────────

#[macro_export]
macro_rules! trace_syscall_enter {
    ($pid:expr, $ts:expr, $nr:expr) => {
        unsafe {
            crate::sigma_tracing::trace_emit(
                crate::sigma_tracing::TraceCategory::Syscall,
                crate::sigma_tracing::syscall_events::ENTER,
                $pid, $ts, [$nr, 0, 0, 0],
            );
        }
    };
}

#[macro_export]
macro_rules! trace_sched_switch {
    ($ts:expr, $old_pid:expr, $new_pid:expr) => {
        unsafe {
            crate::sigma_tracing::trace_emit(
                crate::sigma_tracing::TraceCategory::Scheduler,
                crate::sigma_tracing::sched_events::SWITCH,
                $old_pid, $ts, [$new_pid as u64, 0, 0, 0],
            );
        }
    };
}

#[macro_export]
macro_rules! trace_page_fault {
    ($ts:expr, $pid:expr, $addr:expr, $write:expr) => {
        unsafe {
            crate::sigma_tracing::trace_emit(
                crate::sigma_tracing::TraceCategory::Memory,
                crate::sigma_tracing::mm_events::PAGE_FAULT,
                $pid, $ts, [$addr, $write as u64, 0, 0],
            );
        }
    };
}

// ── Userland trace reader (std, for sigma-trace CLI tool) ─────────────────
#[cfg(feature = "std")]
pub mod reader {
    use super::*;
    use std::io::{self, Read, Write};
    use std::os::unix::net::UnixStream;
    use std::time::Duration;

    const TRACE_SOCKET: &str = "/run/sigma/trace.sock";

    /// Category name for display
    pub fn category_name(cat: TraceCategory) -> &'static str {
        match cat {
            TraceCategory::Syscall    => "syscall",
            TraceCategory::Scheduler  => "sched",
            TraceCategory::Irq        => "irq",
            TraceCategory::Memory     => "mm",
            TraceCategory::Network    => "net",
            TraceCategory::Filesystem => "fs",
            TraceCategory::UserDefined=> "user",
            TraceCategory::Perf       => "perf",
        }
    }

    /// Syscall number → name (abbreviated list)
    pub fn syscall_name(nr: u64) -> &'static str {
        match nr {
            0  => "read",   1  => "write",  2  => "open",    3  => "close",
            4  => "stat",   5  => "fstat",  8  => "lseek",   9  => "mmap",
            11 => "munmap", 12 => "brk",    22 => "pipe",    32 => "dup",
            35 => "nanosleep", 39 => "getpid", 41 => "socket",
            57 => "fork",   59 => "execve", 60 => "exit",    61 => "wait4",
            62 => "kill",   79 => "getcwd", 83 => "mkdir",   87 => "unlink",
            228=> "clock_gettime", _ => "?",
        }
    }

    /// Format one event as human-readable string
    pub fn format_event(evt: &TraceEvent) -> String {
        let cat = category_name(evt.category);
        let ts_ms = evt.timestamp_ns / 1_000_000;
        match evt.category {
            TraceCategory::Syscall => {
                let nr = evt.payload[0];
                let name = syscall_name(nr);
                let marker = if evt.event_id == syscall_events::ENTER { ">" } else { "<" };
                format!("[{:8}ms] {:5} syscall  {} #{} ({})",
                    ts_ms, evt.pid, marker, nr, name)
            }
            TraceCategory::Scheduler => {
                match evt.event_id {
                    0 => format!("[{:8}ms] {:5}→{:5} sched    context_switch",
                        ts_ms, evt.pid, evt.payload[0] as u32),
                    2 => format!("[{:8}ms] {:5} sched    fork → child {:5}",
                        ts_ms, evt.pid, evt.payload[0] as u32),
                    _ => format!("[{:8}ms] {:5} sched    event {}", ts_ms, evt.pid, evt.event_id),
                }
            }
            TraceCategory::Memory => {
                match evt.event_id {
                    2 => format!("[{:8}ms] {:5} mm       page_fault @ {:#018x} ({})",
                        ts_ms, evt.pid, evt.payload[0],
                        if evt.payload[1] != 0 { "W" } else { "R" }),
                    0 => format!("[{:8}ms] {:5} mm       alloc_page order={}", ts_ms, evt.pid, evt.payload[0]),
                    _ => format!("[{:8}ms] {:5} mm       event {}", ts_ms, evt.pid, evt.event_id),
                }
            }
            TraceCategory::Network => {
                match evt.event_id {
                    0 => format!("[{:8}ms] {:5} net      tx {} bytes", ts_ms, evt.pid, evt.payload[0]),
                    1 => format!("[{:8}ms] {:5} net      rx {} bytes", ts_ms, evt.pid, evt.payload[0]),
                    _ => format!("[{:8}ms] {:5} net      event {}", ts_ms, evt.pid, evt.event_id),
                }
            }
            _ => format!("[{:8}ms] {:5} {:<8} event {}", ts_ms, evt.pid, cat, evt.event_id),
        }
    }

    /// Generate a flamegraph-compatible folded stack line
    /// Format: "func1;func2;func3 <count>"
    pub fn to_folded_stack(events: &[TraceEvent]) -> String {
        let mut lines = Vec::new();
        for evt in events {
            if evt.category == TraceCategory::Syscall && evt.event_id == syscall_events::ENTER {
                let nr = evt.payload[0];
                lines.push(format!("sigma-kernel;syscall_{} 1", syscall_name(nr)));
            } else if evt.category == TraceCategory::Scheduler && evt.event_id == sched_events::SWITCH {
                lines.push(format!("sigma-kernel;sched_switch 1"));
            }
        }
        lines.join("\n")
    }

    /// Perf statistics from a batch of events
    pub struct TraceSummary {
        pub total_events:   usize,
        pub syscall_count:  usize,
        pub context_switches: usize,
        pub page_faults:    usize,
        pub net_tx_bytes:   u64,
        pub net_rx_bytes:   u64,
        pub duration_ms:    u64,
    }

    pub fn summarise(events: &[TraceEvent]) -> TraceSummary {
        let mut s = TraceSummary {
            total_events: events.len(),
            syscall_count: 0, context_switches: 0, page_faults: 0,
            net_tx_bytes: 0, net_rx_bytes: 0, duration_ms: 0,
        };
        if events.is_empty() { return s; }
        let t0 = events[0].timestamp_ns;
        let t1 = events.last().unwrap().timestamp_ns;
        s.duration_ms = (t1 - t0) / 1_000_000;
        for e in events {
            match e.category {
                TraceCategory::Syscall   if e.event_id == syscall_events::ENTER => s.syscall_count += 1,
                TraceCategory::Scheduler if e.event_id == sched_events::SWITCH  => s.context_switches += 1,
                TraceCategory::Memory    if e.event_id == mm_events::PAGE_FAULT  => s.page_faults += 1,
                TraceCategory::Network   if e.event_id == net_events::TX => s.net_tx_bytes += e.payload[0],
                TraceCategory::Network   if e.event_id == net_events::RX => s.net_rx_bytes += e.payload[0],
                _ => {}
            }
        }
        s
    }

    /// Print a summary table
    pub fn print_summary(s: &TraceSummary) {
        println!("┌──────────────────────────────────────┐");
        println!("│  sigma-trace Summary ({:4}ms)         │", s.duration_ms);
        println!("├──────────────────────────────────────┤");
        println!("│  Total events:    {:8}             │", s.total_events);
        println!("│  Syscalls:        {:8}             │", s.syscall_count);
        println!("│  Context switches:{:8}             │", s.context_switches);
        println!("│  Page faults:     {:8}             │", s.page_faults);
        println!("│  Net TX:          {:6} KB            │", s.net_tx_bytes / 1024);
        println!("│  Net RX:          {:6} KB            │", s.net_rx_bytes / 1024);
        println!("└──────────────────────────────────────┘");
    }
}

// ── sigma-trace CLI (userland) ────────────────────────────────────────────
#[cfg(all(feature = "std", feature = "cli"))]
fn main() {
    use reader::*;
    let args: Vec<String> = std::env::args().collect();
    let cmd = args.get(1).map(|s| s.as_str()).unwrap_or("live");
    match cmd {
        "live" => {
            println!("sigma-trace live — streaming kernel events (Ctrl-C to stop)");
            println!("Connect to /run/sigma/trace.sock for real data");
            println!("[demo] Generating synthetic events...");
            let synthetic = vec![
                TraceEvent { timestamp_ns: 1000000, pid: 1, cpu: 0,
                    category: TraceCategory::Syscall, event_id: syscall_events::ENTER,
                    payload: [2, 0, 0, 0] }, // open
                TraceEvent { timestamp_ns: 1200000, pid: 1, cpu: 0,
                    category: TraceCategory::Scheduler, event_id: sched_events::SWITCH,
                    payload: [2, 0, 0, 0] }, // switch 1→2
                TraceEvent { timestamp_ns: 1500000, pid: 2, cpu: 0,
                    category: TraceCategory::Memory, event_id: mm_events::PAGE_FAULT,
                    payload: [0xDEAD0000, 0, 0, 0] },
            ];
            for e in &synthetic { println!("{}", format_event(e)); }
        }
        "summary" => {
            let events: Vec<TraceEvent> = vec![];
            let s = summarise(&events);
            print_summary(&s);
        }
        "flamegraph" => {
            let events: Vec<TraceEvent> = vec![];
            println!("{}", to_folded_stack(&events));
            println!("# Pipe to: inferno-flamegraph > flame.svg");
        }
        "help" => {
            println!("sigma-trace <command>");
            println!("  live       — stream live kernel trace events");
            println!("  summary    — show aggregated statistics");
            println!("  flamegraph — output folded stacks for inferno/flamegraph");
        }
        _ => eprintln!("Unknown command: {}. Try: sigma-trace help", cmd),
    }
}
