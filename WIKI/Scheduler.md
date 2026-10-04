# Scheduler

SigmaOS implements a multi-policy process scheduler written entirely in Rust. It combines a CFS-equivalent completely fair scheduler, a real-time FIFO/RR scheduler, an AI-augmented predictive tier, and a Deadline scheduler — all running in O(log n) time with per-CPU run queues and full NUMA awareness.

---

## Architecture Overview

```
 ┌─────────────────────────────────────────────────────┐
 │                   Task Wakeup / Enqueue              │
 └─────────────────────┬───────────────────────────────┘
                       │
          ┌────────────▼──────────────┐
          │     Scheduler Core        │
          │   (src/kernel/scheduler)  │
          │                           │
          │  ┌──────────┐  ┌───────┐  │
          │  │ RT Queue │  │ CFS   │  │
          │  │ FIFO/RR  │  │ RB-   │  │
          │  │ Priority │  │ Tree  │  │
          │  └────┬─────┘  └───┬───┘  │
          │       │            │       │
          │  ┌────▼────────────▼────┐  │
          │  │   pick_next_task()   │  │
          │  └──────────┬───────────┘  │
          └─────────────┼──────────────┘
                        │
          ┌─────────────▼──────────────┐
          │    Per-CPU Run Queue        │
          │  load balance │ migration   │
          └────────────────────────────┘
```

---

## Scheduling Policies

| Policy | Description | Use Case |
|--------|-------------|---------|
| `SCHED_NORMAL` | CFS — weight-based fair share | Default for all processes |
| `SCHED_FIFO` | RT — run until preempted or blocks | Hard real-time tasks |
| `SCHED_RR` | RT — time-sliced FIFO | Soft real-time tasks |
| `SCHED_DEADLINE` | EDF — earliest deadline first | Media, audio pipelines |
| `SCHED_IDLE` | Lowest priority, background only | `nice 19` equivalent |
| `SCHED_AI` | AI-predictive tier (SigmaOS exclusive) | Proactive prefetch |

---

## CFS Scheduler (`src/kernel/scheduler.rs`)

- **Virtual runtime** (`vruntime`): tracks CPU time weighted by `nice` value
- **Red-black tree**: O(log n) pick-next (leftmost node)
- **Min-granularity**: 1 ms (prevents starvation)
- **Target latency**: 6 ms for 1 process, scales with load
- **Priority range**: `nice -20` (highest) to `nice 19` (lowest)

### CFS Formula
```
weight = NICE_TO_WEIGHT[nice + 20]   // lookup table
vruntime += real_delta * (NICE0_WEIGHT / weight)
```

### Load Balancing
- Periodic load balance every 4 ms
- NUMA-aware: prefer local node tasks
- Migration threshold: 25% imbalance before moving tasks
- Work stealing from overloaded CPUs

---

## Real-Time Scheduler (`src/rt/`)

- Priority range: 1 (lowest RT) to 99 (highest RT)
- `SCHED_FIFO`: runs until it blocks or yields
- `SCHED_RR`: time slice = 100 ms (configurable)
- RT throttling: RT tasks limited to 95% CPU by default (`rt_period=1s, rt_runtime=0.95s`)

---

## Deadline Scheduler

Based on GRUB (Greedy Reclamation of Unused Bandwidth):
- Per-task: `runtime`, `deadline`, `period`
- EDF ordering on active deadline queue
- CBS (Constant Bandwidth Server) for isolation
- Example: audio thread `runtime=5ms, deadline=20ms, period=20ms`

---

## AI-Predictive Scheduler (SigmaOS Exclusive)

Unique feature not in Linux, Omarchy, or Mint:

1. **Workload profiler**: records `(task, cpu_time, wait_time, wakeup_pattern)` tuples
2. **Pattern classifier**: ML model identifies periodic vs bursty vs interactive tasks
3. **Proactive wakeup**: wakes tasks 1–2 ms before predicted next event
4. **Cache pre-warm**: migrates task to CPU whose L2/L3 already has its working set

---

## Timer Wheel (`src/kernel/timer_wheel.rs`)

Hierarchical timing wheel for kernel timers:
- 5-level wheel: 256 slots × 5 = ~3.4 years range at 1ms resolution
- O(1) add/remove for timers in the near future
- `HRTIMER_NOHZ`: tickless operation (no unnecessary wakeups)

---

## Process Priority (`src/kernel/process.rs`)

```rust
pub struct Priority {
    pub value: i32,   // -20 (highest) to 19 (lowest)
}

impl Priority {
    pub const HIGH:    Priority = Priority { value: -10 };
    pub const NORMAL:  Priority = Priority { value:   0 };
    pub const LOW:     Priority = Priority { value:  10 };
    pub const IDLE:    Priority = Priority { value:  19 };
}
```

---

## Comparison vs Linux / Omarchy / Mint Schedulers

| Feature | Linux | Omarchy | Mint | **SigmaOS** |
|---------|-------|---------|------|-------------|
| CFS | ✅ | ✅ | ✅ | ✅ Rust |
| RT FIFO/RR | ✅ | ✅ | ✅ | ✅ |
| SCHED_DEADLINE | ✅ | ✅ | ✅ | ✅ |
| AI-predictive | ❌ | ❌ | ❌ | ✅ |
| NUMA-aware LB | ✅ | ✅ | ✅ | ✅ |
| Tickless (NOHZ) | ✅ | ✅ | ✅ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/kernel/scheduler.rs` | CFS + RT scheduler core |
| `src/kernel/timer_wheel.rs` | Hierarchical timer wheel |
| `src/kernel/process.rs` | Process struct and Priority |
| `src/scheduler/` | Scheduler module directory |
| `src/rt/` | Real-time scheduler |
| `src/performance/smart_optimizer.rs` | AI scheduling integration |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source: `src/kernel/scheduler.rs`, `src/scheduler/`, `src/rt/`
> - Update latency/granularity values when tuning constants change
> - Document new scheduling policies as they are added
> - Keep AI-predictive section current with `src/performance/smart_optimizer.rs`
