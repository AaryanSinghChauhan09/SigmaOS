# Σ perf/scheduler — Hybrid Sovereign Scheduler

Extends the base Round-Robin and EDF schedulers with **NUMA awareness,
real-time lanes, energy efficiency, and AI-driven workload prediction**.

## Scheduler Class Hierarchy

```
SovereignScheduler (abstract)
  ├─ RoundRobin     — fair-share for interactive tasks
  ├─ EDF            — Earliest Deadline First for real-time shards
  ├─ RTLane         — hard real-time, preempts all other lanes
  ├─ NUMAFair       — CFS analogue, NUMA-topology aware
  └─ EcoLane        — battery/power-optimised (ARM big.LITTLE)
```

## Scheduling Policy Selection

A shard declares its scheduling class in the spawn request:

```rust
shard_spawn(SpawnRequest {
    name: "audio_daemon",
    sched_class: SchedClass::RTLane { deadline_us: 5_000 },
    cpu_affinity: CpuSet::node(0),
    ..Default::default()
});
```

## AI Prediction Engine

`modules/core/kernel/res_alloc_ai.rs` feeds scheduling hints:

- Lightweight LSTM trained on historical shard CPU/memory patterns
- Runs in a sandboxed inference shard (no GPU required at boot)
- Inference latency < 50 µs on baseline x86_64
- Signals pre-warming of cache lines for known bursty workloads

## API Interface

```c
// Yield the current CPU timeslice
void sigma_sched_yield(void);

// Set real-time deadline for a shard
int sigma_sched_set_rt(sigma_shard_id_t id, uint64_t deadline_us);

// Get CPU usage statistics for a shard
sigma_cpu_stats_t sigma_sched_stats(sigma_shard_id_t id);

// Initialise the scheduler subsystem
void init_perf_scheduler(void);
```

## Context Switch Latency Targets

| Class | Target Latency |
|---|---|
| RTLane | < 5 µs |
| EDF | < 50 µs |
| RoundRobin | < 500 µs |
| EcoLane | Best-effort |

## Roadmap

- [x] Round-Robin base scheduler
- [x] EDF scheduler with deadline enforcement
- [ ] RTLane preemption guarantees (< 5 µs verified)
- [ ] NUMA topology detector integration
- [ ] CPU frequency governor (P-state / DVFS)
- [ ] AI prediction hook from `res_alloc_ai.rs`
- [ ] Formal scheduling analysis (response-time analysis)
- [ ] `schedtool`-compatible CLI for shard priority adjustment

## Related Modules

- [`modules/core/kernel`](../../core/kernel/README.md) — Kernel scheduler host
- [`modules/perf/bench`](../bench/README.md) — Context-switch benchmark
