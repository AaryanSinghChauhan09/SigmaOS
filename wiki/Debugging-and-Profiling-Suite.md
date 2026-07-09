# Debugging & Profiling Suite

Deterministic memory tracing and performance analysis tools for SigmaOS kernel and userland development.

## Overview

The Debugging & Profiling Suite provides deterministic, zero-allocation tools for analyzing kernel behavior, memory usage, and performance characteristics without introducing measurement overhead or perturbation.

## Components

### 1. Memory Tracer (`sigma-mem-trace`)

Deterministic memory allocation tracking with zero runtime overhead:

- **Allocation Tracking**: Records all heap allocations with call stack information
- **Leak Detection**: Identifies memory leaks through reference counting analysis
- **Double-Free Detection**: Prevents use-after-free and double-free errors
- **Fragmentation Analysis**: Tracks memory fragmentation patterns over time
- **Hot Path Identification**: Pinpoints frequently allocated/deallocated memory regions

### 2. Kernel Profiler (`sigma-kern-profil`)

Low-overhead kernel performance profiling:

- **Syscall Latency**: Measures syscall dispatch times with nanosecond precision
- **Scheduler Analysis**: Tracks task scheduling decisions and context switches
- **Interrupt Latency**: Measures interrupt handling latency across different IRQ sources
- **Cache Miss Analysis**: Hardware performance counter integration for cache behavior
- **Lock Contention**: Identifies lock contention points in kernel synchronization

### 3. Crash Analyzer (`sigma-crash-analyze`)

Post-mortem crash analysis and debugging:

- **Kernel Panic Analysis**: Parses kernel panic logs and identifies root causes
- **Stack Trace Reconstruction**: Reconstructs call stacks from crash dumps
- **Register State Inspection**: Examines CPU register state at crash time
- **Memory Dump Analysis**: Analyzes memory contents for corruption patterns
- **Automated Bug Report Generation**: Creates structured bug reports from crash data

### 4. Live Debugger (`sigma-dbg`)

Interactive kernel and userland debugging:

- **Breakpoint Management**: Hardware and software breakpoint support
- **Watchpoints**: Data watchpoints for monitoring memory locations
- **Step Execution**: Single-step through kernel and userland code
- **Symbol Resolution**: Resolves symbols from debug information
- **Remote Debugging**: GDB-compatible remote debugging protocol

## Usage

### Memory Tracing

```bash
# Enable memory tracing for a kernel module
sigma-mem-trace --module kernel/core/sigma_sched.rs --output trace.log

# Analyze memory trace
sigma-mem-trace --analyze trace.log --report memory-report.html

# Detect leaks
sigma-mem-trace --detect-leaks trace.log
```

### Kernel Profiling

```bash
# Profile kernel for 30 seconds
sigma-kern-profil --duration 30 --output profile.data

# Generate flame graph
sigma-kern-profil --flame-graph profile.data --output flamegraph.svg

# Analyze syscall latency
sigma-kern-profil --syscall-latency profile.data --top 20
```

### Crash Analysis

```bash
# Analyze kernel crash dump
sigma-crash-analyze --dump /var/crash/kernel-panic-2024-01-15.dump

# Generate detailed report
sigma-crash-analyze --report crash-report.md
```

### Live Debugging

```bash
# Start debug server
sigma-dbg --listen :1234

# Connect with GDB
gdb -ex "target remote localhost:1234" /path/to/kernel.elf
```

## Integration with CI/CD

```yaml
# .github/workflows/profiling.yml
- name: Run memory tracer
  run: |
    sigma-mem-trace --module kernel/core/sigma_sched.rs \
      --output trace-${{ github.sha }}.log

- name: Check for memory leaks
  run: |
    sigma-mem-trace --detect-leaks trace-${{ github.sha }}.log \
      --fail-on-leak

- name: Profile kernel performance
  run: |
    sigma-kern-profil --duration 60 --output profile-${{ github.sha }}.data

- name: Upload profiling data
  uses: actions/upload-artifact@v3
  with:
    name: profiling-data
    path: profile-${{ github.sha }}.data
```

## Architecture

### Zero-Overhead Design

All profiling tools use:

- **Static Instrumentation**: Compile-time instrumentation with no runtime overhead when disabled
- **Ring Buffer Logging**: Lock-free ring buffers for high-frequency event logging
- **Sampling**: Statistical sampling to reduce overhead while maintaining accuracy
- **Deferred Analysis**: Raw data collection with offline analysis to minimize impact

### Deterministic Execution

- **Reproducible Traces**: Traces are deterministic and reproducible across runs
- **No Heisenberg Effect**: Measurement does not significantly affect system behavior
- **Timestamp Synchronization**: Hardware TSC synchronization for accurate timing

## Roadmap

- [ ] GUI-based profiling dashboard
- [ ] Real-time profiling visualization
- [ ] Distributed system tracing
- [ ] Machine learning-based anomaly detection
- [ ] Integration with perf and eBPF tools
- [ ] Automated performance regression detection

## Documentation

- [Memory Tracing Guide](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Memory-Tracing)
- [Kernel Profiling](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Kernel-Profiling)
- [Crash Analysis](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Crash-Analysis)
- [Debugging Protocols](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Debugging-Protocols)
