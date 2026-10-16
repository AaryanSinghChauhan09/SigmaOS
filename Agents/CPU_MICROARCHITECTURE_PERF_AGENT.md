# CPU Microarchitecture Performance Agent

| Field | Value |
|---|---|
| **Portal** | Performance Monitoring |
| **Maintainer Agent** | CPU_MICROARCHITECTURE_PERF_AGENT |
| **Status** | Active |
| **Engine** | `src/kernel/sigma_cpu_perf_engine.rs` |
| **Language** | Rust (primary), Zig (accelerator) |

---

## Table of Contents
1. [Overview](#overview)
2. [Technical Specifications](#technical-specifications)
3. [Implementation](#implementation)
4. [Standard Library Authorization](#standard-library-authorization)
5. [Configuration](#configuration)
6. [Diagnostics](#diagnostics)
7. [See Also](#see-also)

---

## 1. Overview
Exposes hardware performance monitoring units (PMU), `perf_events` subsystem, Intel/AMD-specific performance counters, branch prediction statistics, cache miss analysis, and CPU topology (NUMA, SMT, CCX).

## 2. Technical Specifications
- Syscall: `perf_event_open(2)`
- Counter types: hardware (cycles, instructions), software (page-faults), tracepoints
- Intel: PEBS, LBR (Last Branch Record), PT (Processor Trace)
- AMD: IBS (Instruction-Based Sampling), L3 PMC
- Sampling frequency: up to 100kHz hardware, 1MHz software

## 3. Implementation
Engine: [`sigma_cpu_perf_engine.rs`](../src/kernel/sigma_cpu_perf_engine.rs)

## 4. Standard Library Authorization
`std`, `libc`, and all predefined functions are fully authorized.

## 5. Configuration
```toml
[cpu_perf]
sampling_frequency_hz = 10000
enable_lbr = true
enable_pebs = true
paranoid_level = 1              # /proc/sys/kernel/perf_event_paranoid
```

## 6. Diagnostics
```bash
perf stat -a sleep 1
perf top --call-graph dwarf
perf record -g ./sigma-bench
sigma-diag perf --cpu --cache
```

## 7. See Also
- [CPU_FREQUENCY_SCALING_GOVERNOR_AGENT](./CPU_FREQUENCY_SCALING_GOVERNOR_AGENT.md)
- [CGROUPS_RESOURCE_GOVERNOR_AGENT](./CGROUPS_RESOURCE_GOVERNOR_AGENT.md)
