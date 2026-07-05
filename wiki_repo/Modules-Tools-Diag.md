# Σ tools/diag — Sovereign Diagnostics Toolkit

Provides **structured logging, profiling, and syscall tracing** for the SigmaOS
kernel and user-space shards.

## Source Files

| File | Description |
|---|---|
| `logger.rs` | Ring-buffer structured logger (`SIGMA_LOG_*` macros) |
| `profiler.rs` | CPU cycle + PMU counter-based profiler |
| `syscall_tracer.rs` | Strace-equivalent syscall intercept + timeline recorder |

## Logger

Zero-allocation ring-buffer logger using a lock-free SPSC queue:

```rust
// Log levels: TRACE, DEBUG, INFO, WARN, ERROR, FATAL
sigma_log!(INFO, "kernel", "shard {id} started in {elapsed_us}µs");

// Structured key-value pairs
sigma_log!(WARN, "net", "tcp_retransmit";
    "shard" => id, "seq" => seq_num, "count" => retries);
```

Output format (journal-compatible):
```
2026-07-05T14:30:00.000Z INFO  kernel  shard 42 started in 1200µs
```

## Profiler

Hardware PMU-based profiler using `perf_event_open` equivalents:

```c
// Start profiling a shard
profiler_start(shard_id, PROFILER_CPU_CYCLES | PROFILER_CACHE_MISS);

// Snapshot current counters
profiler_snapshot_t snap = profiler_snapshot(shard_id);
// snap.cpu_cycles, snap.cache_misses, snap.branch_mispredicts

// Stop and emit flamegraph data
profiler_stop(shard_id, "output.fg");
```

## Syscall Tracer

```c
// Attach to a running shard
tracer_attach(shard_id);

// All syscalls are now recorded to a ring buffer
// sigma_vfs_open("/etc/config", O_RDONLY) → 3 [1.2µs]
// sigma_net_send(3, buf, 1024, 0) → 1024 [8.4µs]

// Detach and dump trace
tracer_detach(shard_id);
tracer_dump("trace.json");
```

## API Interface

```c
void init_tools_diag(void);

// Logger
void sigma_log(log_level_t level, const char *module, const char *msg, ...);

// Profiler
void profiler_start(shard_id_t id, uint32_t events);
profiler_snapshot_t profiler_snapshot(shard_id_t id);
void profiler_stop(shard_id_t id, const char *output_path);

// Tracer
int tracer_attach(shard_id_t id);
void tracer_dump(const char *path);
```

## Roadmap

- [x] Ring-buffer structured logger (`logger.rs`)
- [x] PMU-based profiler stub (`profiler.rs`)
- [x] Syscall tracer stub (`syscall_tracer.rs`)
- [ ] Flamegraph generation (Brendan Gregg format)
- [ ] Log shipping to Sovereign Audit Chain
- [ ] Distributed tracing (OpenTelemetry-compatible spans)
- [ ] Interactive TUI dashboard (`sigma-top`)

## Related Modules

- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel log sources
- [`modules/security/access_control`](../../security/access_control/README.md) — Audit chain
