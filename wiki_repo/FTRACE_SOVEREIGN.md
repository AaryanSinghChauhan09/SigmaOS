# SigmaOS Sovereign Kernel Function Tracer (ftrace)

## Overview

SigmaOS implements a **pure-Rust sovereign ftrace subsystem** (`src/kernel/ftrace_sovereign.rs`) modeled after the Linux kernel's internal function tracer.

Ftrace is crucial for performance analysis, debugging latency spikes, tracking system call execution, observing process scheduling transitions, and monitoring interrupt latencies.

## Core Capabilities

1. **Function Tracing & Graphing**:
   - `FunctionEntry` & `FunctionReturn` events with high-resolution nanosecond timestamps.
   - Call stack depth tracking.
   - Function duration computation (`duration_ns`).
2. **Scheduling Tracing**:
   - `SchedSwitch` and `SchedWakeup` events capturing context switch transitions (`prev_comm -> next_comm`).
3. **Interrupt & Syscall Tracing**:
   - `IrqEntry`, `IrqExit`, `SyscallEntry`, and `SyscallExit` events with return code capture.
4. **Per-CPU Ring Buffers**:
   - Lockless per-CPU circular buffers with automatic overwrite or tail-drop semantics (`TraceRingBuffer`).
5. **Latency Histograms (Hist Triggers)**:
   - Logarithmic latency bucketing (1µs, 10µs, 100µs, 1ms, 10ms, 100ms, ∞).
   - Real-time computation of sample count, min, max, and average execution times.
6. **Dynamic Filtering**:
   - Filter by target PID, function name prefix, minimum latency threshold, or specific event kinds.

## Usage Example

```rust
// Initialize 4-CPU tracer with 4096-event buffers per core
let mut tracer = SovereignFtracer::new(4, 4096);
tracer.add_histogram("vfs_read");
tracer.enable();

// Trace function entry & exit
tracer.trace(TraceEvent::function_entry(ts1, cpu, pid, "app", "vfs_read", 1));
tracer.trace(TraceEvent::function_return(ts2, cpu, pid, "app", "vfs_read", 1, ts2 - ts1, 4096));

// Read events for CPU 0
let events = tracer.read_events(0);
for ev in events {
    println!("{}", ev.format_line());
}
```

## Linux Parity

| Linux ftrace File | SigmaOS Equivalent |
|-------------------|-------------------|
| `/sys/kernel/tracing/tracing_on` | `tracer.enable()` / `tracer.disable()` |
| `/sys/kernel/tracing/set_ftrace_pid` | `filter.pid_filter = Some(pid)` |
| `/sys/kernel/tracing/set_ftrace_filter` | `filter.func_prefix = Some("vfs_")` |
| `/sys/kernel/tracing/trace_pipe` | `tracer.read_events(cpu)` |
| Function Graph Tracer | `TraceEventKind::FunctionEntry` & `FunctionReturn` |
| Hist Triggers (`hist:keys=...`) | `LatencyHistogram` |

## Test Verification

6 standalone unit tests verified in test runner suite `[13]`:
- `test_ftrace_enable_disable`
- `test_ftrace_pid_filter`
- `test_function_graph_tracing`
- `test_latency_histogram`
- `test_ring_buffer_overwrite`
- `test_sched_switch_event_format`
