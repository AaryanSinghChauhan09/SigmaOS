# Diagnostics & Tracing Architecture

The **SigmaOS Diagnostics & Tracing Subsystem** provides observability, profiling, and debugging facilities with minimal execution overhead (<1% CPU tax), allowing developers and kernel engineers to diagnose performance bottlenecks and memory anomalies on live production systems.

---

## 1. Subsystem Components

### 1.1 Ftrace Dynamic Kernel Function Tracer
- **Location**: `src/tracing/ftrace.rs`
- **Specification**: In-kernel function execution graph tracer. Dynamically patches function prologues with `mcount` / `fentry` call instructions. Tracks call depth, entry timestamps, exit durations, and produces hierarchical flamegraph-compatible trace buffers.

### 1.2 Perf PMU Hardware Counter Profiling
- **Location**: `src/tracing/perf_pmu.rs`
- **Specification**: Interfaces directly with CPU Performance Monitoring Units (PMU). Profiles hardware events:
  - Instructions Retired
  - CPU Cycles & Clock Ticks
  - L1/L2/L3 Cache Misses
  - Branch Mispredictions
  - TLB Misses & Page Faults

### 1.3 In-Kernel GDB Remote Serial Stub
- **Location**: `src/tracing/gdb_stub.rs`
- **Specification**: Implementation of the GDB Remote Serial Protocol (RSP) over UART serial or TCP. Supports remote breakpoints (`$Z0`), single-stepping (`$s`), memory read/write (`$m`/`$M`), register inspection (`$g`), and backtrace interrogation without stopping the entire operating system.

### 1.4 eBPF Runtime & Safety Verifier
- **Location**: `src/ebpf/`, `src/tracing/`
- **Specification**: Extended Berkeley Packet Filter in-kernel virtual machine. Includes a formal safety verifier that proves termination (no unbounded loops), memory bounds checking, and uninitialized register protection before executing bytecode in the kernel context.

---

## 2. Diagnostics Parity Matrix

| Tool / Facility | Linux 6.x | FreeBSD 14 | OpenBSD 7.x | SigmaOS Implementation |
|---|---|---|---|---|
| **Function Tracer** | `ftrace` | DTrace | None | In-kernel `ftrace.rs` |
| **PMU Hardware Profiler** | `perf` | `pmc` | Limited | Built-in `perf_pmu.rs` |
| **Kernel GDB Stub** | KGDB | KGDB | GDB Stub | Native `gdb_stub.rs` |
| **In-Kernel Safe VM** | eBPF | DTrace / eBPF | None | Native eBPF Verifier |

---

## ## AI Agent Maintenance Instructions for Future Releases

1. **Language Invariant**: All diagnostics, tracers, and debugging stubs must be written in **100% Rust** or **Zig**.
2. **Zero Probe Overhead**: When disabled, dynamic tracepoints must compile into a single 5-byte NOP instruction (`NOP5`), ensuring zero CPU cost during normal execution.
3. **Trace Buffer Ring Overflow Safety**: In-kernel tracing buffers MUST operate as lock-free circular ring buffers that drop or overwrite the oldest entries under overflow conditions rather than blocking kernel progress.
4. **Zero Predefined Dependencies**: Do not link external C tracing frameworks.
5. **Compilation Verification**: All diagnostics code must compile with:
   ```bash
   cargo check --lib
   ```
