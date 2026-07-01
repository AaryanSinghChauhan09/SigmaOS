// SPDX-License-Identifier: GPL-2.0-or-later
//! kernel/core/metrics.rs — SigmaOS Kernel Metrics Exporter
//!
//! Provides a zero-overhead procfs-style virtual interface at /sigma/metrics.
//! Kernel subsystems call `metrics::record_*()` which writes into a lock-free
//! ring buffer. A VFS read handler serialises the latest snapshot on demand.
//! No heap allocation occurs in the hot path — all counters are atomic u64.

use core::sync::atomic::{AtomicU64, Ordering};

/// Global metrics snapshot — all fields updated atomically by kernel subsystems.
pub struct KernelMetrics {
    // ── Scheduler ──────────────────────────────────────────────────────────
    pub context_switches:    AtomicU64,
    pub runqueue_depth:      AtomicU64,
    pub preemptions:         AtomicU64,

    // ── Memory ────────────────────────────────────────────────────────────
    pub pages_allocated:     AtomicU64,
    pub pages_freed:         AtomicU64,
    pub slab_allocs:         AtomicU64,
    pub page_faults:         AtomicU64,

    // ── Interrupts ────────────────────────────────────────────────────────
    pub irq_count:           AtomicU64,
    pub spurious_irqs:       AtomicU64,
    pub softirq_count:       AtomicU64,

    // ── Syscalls ─────────────────────────────────────────────────────────
    pub syscall_total:       AtomicU64,
    pub syscall_denied:      AtomicU64,
    pub syscall_latency_ns:  AtomicU64,   // rolling average (ns)

    // ── Network ──────────────────────────────────────────────────────────
    pub net_rx_bytes:        AtomicU64,
    pub net_tx_bytes:        AtomicU64,
    pub net_drops:           AtomicU64,
}

/// Static global instance — no heap, no allocator.
pub static METRICS: KernelMetrics = KernelMetrics {
    context_switches:   AtomicU64::new(0),
    runqueue_depth:     AtomicU64::new(0),
    preemptions:        AtomicU64::new(0),
    pages_allocated:    AtomicU64::new(0),
    pages_freed:        AtomicU64::new(0),
    slab_allocs:        AtomicU64::new(0),
    page_faults:        AtomicU64::new(0),
    irq_count:          AtomicU64::new(0),
    spurious_irqs:      AtomicU64::new(0),
    softirq_count:      AtomicU64::new(0),
    syscall_total:      AtomicU64::new(0),
    syscall_denied:     AtomicU64::new(0),
    syscall_latency_ns: AtomicU64::new(0),
    net_rx_bytes:       AtomicU64::new(0),
    net_tx_bytes:       AtomicU64::new(0),
    net_drops:          AtomicU64::new(0),
};

// ── Hot-path recording helpers ────────────────────────────────────────────

#[inline(always)]
pub fn record_context_switch() {
    METRICS.context_switches.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_page_alloc(count: u64) {
    METRICS.pages_allocated.fetch_add(count, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_page_free(count: u64) {
    METRICS.pages_freed.fetch_add(count, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_irq() {
    METRICS.irq_count.fetch_add(1, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_syscall(denied: bool, latency_ns: u64) {
    METRICS.syscall_total.fetch_add(1, Ordering::Relaxed);
    if denied {
        METRICS.syscall_denied.fetch_add(1, Ordering::Relaxed);
    }
    // Exponential moving average: new = 0.875*old + 0.125*sample
    let old = METRICS.syscall_latency_ns.load(Ordering::Relaxed);
    let avg = old.wrapping_sub(old >> 3).wrapping_add(latency_ns >> 3);
    METRICS.syscall_latency_ns.store(avg, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_net_rx(bytes: u64) {
    METRICS.net_rx_bytes.fetch_add(bytes, Ordering::Relaxed);
}

#[inline(always)]
pub fn record_net_tx(bytes: u64) {
    METRICS.net_tx_bytes.fetch_add(bytes, Ordering::Relaxed);
}

// ── VFS read handler: /sigma/metrics ─────────────────────────────────────
//
// Called by the virtual filesystem when userspace reads /sigma/metrics.
// Writes a key=value snapshot into `buf` and returns bytes written.
// No syscall overhead — the data is already in atomics.
//
// Format (line-delimited, compatible with Prometheus text exposition):
//   sigma_context_switches <N>
//   sigma_pages_allocated  <N>
//   ...

pub fn vfs_read_handler(buf: &mut [u8]) -> usize {
    use core::fmt::Write;

    struct SliceBuf<'a> {
        buf: &'a mut [u8],
        pos: usize,
    }

    impl<'a> Write for SliceBuf<'a> {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            let bytes = s.as_bytes();
            let remaining = self.buf.len() - self.pos;
            let n = bytes.len().min(remaining);
            self.buf[self.pos..self.pos + n].copy_from_slice(&bytes[..n]);
            self.pos += n;
            Ok(())
        }
    }

    let m = &METRICS;
    let mut w = SliceBuf { buf, pos: 0 };

    let _ = writeln!(w, "# SigmaOS Kernel Metrics — /sigma/metrics");
    let _ = writeln!(w, "sigma_context_switches {}",  m.context_switches.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_runqueue_depth {}",    m.runqueue_depth.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_preemptions {}",       m.preemptions.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_pages_allocated {}",   m.pages_allocated.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_pages_freed {}",       m.pages_freed.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_slab_allocs {}",       m.slab_allocs.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_page_faults {}",       m.page_faults.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_irq_count {}",         m.irq_count.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_spurious_irqs {}",     m.spurious_irqs.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_syscall_total {}",     m.syscall_total.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_syscall_denied {}",    m.syscall_denied.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_syscall_latency_ns {}", m.syscall_latency_ns.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_net_rx_bytes {}",      m.net_rx_bytes.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_net_tx_bytes {}",      m.net_tx_bytes.load(Ordering::Relaxed));
    let _ = writeln!(w, "sigma_net_drops {}",         m.net_drops.load(Ordering::Relaxed));

    w.pos
}
