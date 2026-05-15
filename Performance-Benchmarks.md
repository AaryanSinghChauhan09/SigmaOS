# SigmaOS Performance Benchmarks (v15.0)

This page tracks the official performance metrics of the SigmaOS Zenith v15.0 microkernel under different workloads and formats.

## 1. Inter-Shard IPC Latency
- **Baseline**: 85ns
- **v15.0 Optimized**: 42ns
- **Measurement**: Ring-0 asynchronous IPC using the sovereign lock-free ring buffer (S-IPC).

## 2. Kernel Memory Allocation (S-MM)
- **Slab Allocation (Small Objects)**: 12ns per allocation
- **Page Allocation (4KB)**: 55ns per allocation
- **Regression Limit**: 5% degradation threshold strictly enforced by CI.

## 3. Asynchronous Shard Ignition (Boot Time)
- **Standalone Edition (Embedded)**: 380ms
- **Dual-Boot Edition**: 520ms (includes EFI partition selection)
- **Cloud/Virtual Hypervisor**: 450ms

## 4. UI Responsiveness (App/Browser Layer)
- **Compositor Framerate**: Sustained 144Hz (locked, zero drops under 90% CPU load).
- **WASM Runtime Latency**: <15ms execution overhead for compiled shards.

## Continuous Monitoring
These metrics are constantly profiled via `scripts/regression_check.sh`. Any build failing to meet these thresholds is automatically blocked in the release pipelines.
