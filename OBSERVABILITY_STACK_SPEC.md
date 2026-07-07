# SigmaOS Observability Stack Specification

## 1. Overview
Observability in SigmaOS is designed to be low-overhead, native, and deeply integrated into the kernel's scheduler and memory allocator. Unlike traditional distros that rely on heavy userland daemons, SigmaOS embeds telemetry hooks directly into the core primitives.

## 2. Kernel Telemetry (LTTng / eBPF Alternatives)
SigmaOS avoids the complexity of eBPF, opting instead for a static tracepoint system integrated directly into the `sigma_rr_sched.rs` and `sigma_buddy_alloc.rs` modules.
- **Tracepoints:** Placed at context switches, page faults, and syscall boundaries.
- **Ring Buffer:** Trace events are dumped into a lockless per-CPU ring buffer in physical memory.

## 3. Metrics and the Prometheus Client
- A lightweight, native Prometheus exporter daemon (`sigma-prom-agent`) runs as a low-priority system service.
- It parses the kernel's lockless ring buffer and exposes standard Prometheus `/metrics` over HTTP.
- Monitored metrics include:
  - Cache hit ratios (`sigma_ubc.rs`).
  - Slab allocator fragmentation.
  - MicroVM CPU quota throttling events.

## 4. Telemetry and Auto-Rollback
If the `sigma-prom-agent` detects severe anomalies (e.g., kernel panic loops, out-of-memory cascades, or critical services failing to bind) immediately after a `sigpkg` update, the system triggers the **Auto-Rollback protocol**:
1. The failing staged snapshot is flagged as `FAILED`.
2. The bootloader is instructed to pivot back to the previous stable snapshot.
3. The system gracefully reboots.
