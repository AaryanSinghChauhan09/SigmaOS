# 🔍 SigmaOS Observability & Tracing (SigmaTrace, SigmaMetrics, SigmaDebug) Development Roadmap

This document establishes the system engineering and future development blueprint for **SigmaOS's Advanced Observability Stack**, taking design inspiration from Linux (`perf`, `eBPF`, `systemtap`) and BSD (`DTrace`).

---

## 🏗️ 1. Technical Vision & Architectural Supersets

Traditional OS observability relies on kernel-level execution hooks that introduce high latency and execution jitter. SigmaOS utilizes **Zero-Dependency, Multi-Language Hybrid Shards** to implement high-throughput tracking without sacrificing type safety or capability boundaries.

```
       +-------------------------------------------------------+
       |                  Userland Tracing API                 |
       +-------------------------------------------------------+
            |                        |                       |
            v (Rust)                 v (Zig)                 v (Nim)
   +-----------------+      +-----------------+      +-----------------+
   |   SigmaTrace    |      |  SigmaMetrics   |      |   SigmaDebug    |
   |   (eBPF VM)     |      |  (Ring Buffers) |      | (Symbol Parsing)|
   +-----------------+      +-----------------+      +-----------------+
```

---

## ⚡ 2. SigmaTrace: Next-Gen eBPF VM (Rust / Zig)

### 2.1 The Polymorphic Trace Engine
- **Inspiration**: Linux's eBPF and BSD's DTrace.
- **Implementation (Rust)**: High-safety execution sandbox inside `src/observability/stack.rs`.
- **Implementation (Zig)**: Low-level context-switch probing and registers interception. Zig's strict alignment controls and explicit memory allocators ensure no memory overhead during register snapshots.

### 2.2 User-Defined Function (UDF) Probes
- Developers register dynamic probes as short bytecodes.
- Probe scripts execute on the **UDF Interpreter Engine** with memory access constrained strictly to the thread's own assigned bounds, preventing page faults.

---

## 📊 3. SigmaMetrics: Zero-Copy Exporter (Nim / Rust)

### 3.1 Ring-Buffer Telemetry Exporter
- **Inspiration**: Prometheus, Grafana, and Linux sysfs.
- **Implementation (Nim)**: Highly expressive, compiled Nim code (`src/nim/`) acts as the user-space daemon to parse ring-buffers and export them to Prometheus format.
- **Implementation (Rust)**: Lock-free, zero-allocation ring-buffer implementations in kernel memory ensuring sub-millisecond metric updates.

### 3.2 Tracked Metrics Class
- CPU core utilization and frequency.
- Buddy Allocator order metrics and memory merge splits.
- Network driver queue depths and packet throughput.

---

## 🔍 4. SigmaDebug: Symbol & Core Dump Analysis (Rust)

### 4.1 Post-Mortem Core Dump Capture
- **Inspiration**: Linux `gdb` and macOS Crash Reporter.
- **Action**: When a kernel panic or exception occurs, the system registers the CPU dump in a pre-allocated crash vault sector on block storage.

### 4.2 Dynamic DWARF Parser
- Real-time symbol resolution directly in the kernel shell `repl` using cached Symbol maps to display function names, offsets, and line numbers.

---

## 📅 5. Step-by-Step Implementation Roadmap

- [ ] **Phase 1 (Stabilization)**: Complete standard tracing and metrics export traits in `src/observability/stack.rs`.
- [ ] **Phase 2 (Zig Core Integration)**: Add low-level interrupt hooks and register snapshot collectors in Zig.
- [ ] **Phase 3 (Nim Telemetry Daemon)**: Build the userland exporter in Nim to format metrics into Prometheus and Grafana schemas.
