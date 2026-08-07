# Σ perf/bench — Sovereign Benchmarking Suite

Micro- and macro-benchmarks for every critical SigmaOS subsystem. Results are
used to catch performance regressions before they reach `main`.

## Benchmark Categories

| Category | Scope | Key Metric |
|---|---|---|
| `kernel/syscall` | Syscall dispatcher round-trip | ns/call |
| `kernel/ipc` | Zero-copy IPC throughput | GB/s |
| `mm/alloc` | Slab allocator latency | ns/op |
| `mm/page_fault` | Minor page fault handling | µs |
| `fs/read_seq` | Sequential read (SovereignFS) | MB/s |
| `fs/write_rand` | Random write 4K blocks | IOPS |
| `net/tcp_tx` | TCP transmit throughput | Gb/s |
| `scheduler/switch` | Context switch latency | ns |

## Running Benchmarks

```bash

# Run all benchmarks

just bench

# Run only IPC benchmarks

just bench -- kernel/ipc

# Generate flamegraph

just bench-flamegraph

# Compare against baseline

just bench-compare baseline.json
```

## Benchmark Harness

Benchmarks use a minimal no-alloc harness built on `perf_event_open`:

```c
sigma_bench_start("syscall_roundtrip");
for (int i = 0; i < BENCH_ITERS; i++) {
    sigma_syscall(NR_SIGMA_NOOP, NULL);
}
sigma_bench_end();   // prints: "syscall_roundtrip: 120ns/op"
```

## CI Integration

All benchmarks run nightly on bare-metal CI (not in QEMU — timing is not
meaningful in a VM). A regression of > 5% triggers a blocking CI alert.

```yaml

# .github/workflows/bench.yml

on:
  schedule:
    - cron: '0 2 * * *'   # 02:00 UTC nightly

```

## Roadmap

- [ ] Syscall round-trip benchmark

- [ ] IPC throughput benchmark

- [ ] Memory allocator latency suite

- [ ] Filesystem IOPS benchmark

- [ ] Network TCP throughput benchmark

- [ ] Automated regression detection (± 5% threshold)

- [ ] Flamegraph generation pipeline

- [ ] Historical results dashboard (GitHub Pages)

## Related Modules

- [`modules/perf/scheduler`](../scheduler/README.md) — Scheduler performance

- [`modules/perf/mm`](../mm/README.md) — Memory manager benchmarks
