# SigmaOS EEVDF Scheduler

## Overview

The SigmaOS EEVDF (Earliest Eligible Virtual Deadline First) scheduler is the
default CPU scheduling policy for SigmaOS, implemented in
`src/kernel/sigma_scheduler_eevdf.rs`.  It is a SigmaOS-native design inspired
by the Linux 6.6 EEVDF scheduler and the BSD ULE (UltraLight Execution) scheduler.

---

## Design Goals

| Goal | Mechanism |
|------|-----------|
| CPU fairness | Weighted virtual runtime (vruntime) per task |
| Low latency for interactive workloads | Earliest-deadline-first preemption |
| Predictable throughput for batch jobs | Time-slice control via `slice_ns` |
| Multi-CPU scalability | Per-CPU runqueues + load balancer |
| No external dependencies | Pure Rust; `no_std`-compatible logic |

---

## Algorithm

### Virtual Runtime (vruntime)

Each task accumulates *virtual runtime* at a rate inversely proportional to its weight:

```
vruntime_delta = real_delta_ns * DEFAULT_WEIGHT / task.weight
```

Heavier tasks (higher `weight`) accumulate vruntime more slowly and therefore
receive a larger share of CPU time.

### Eligible Time and Deadline

When a task wakes up or is first enqueued, its fields are initialised:

```
eligible_time = max(task.vruntime, rq.min_vruntime)   -- lag compensation
deadline      = eligible_time + (slice_ns << 10) / weight
```

`min_vruntime` is a monotonic floor that advances as the current task runs,
preventing newly-woken tasks from claiming CPU time they have not earned (lag
compensation, analogous to the EEVDF *lag* formula).

### Task Selection (`pick_next_task`)

1. Scan the runqueue for **eligible** tasks (`eligible_time ≤ min_vruntime`).
2. Among eligible tasks, pick the one with the **smallest deadline**.
3. If no task is currently eligible (all tasks have positive lag), fall back to
   the smallest-deadline task regardless of eligibility.

This two-pass algorithm matches the Linux 6.6 EEVDF pick logic.

### Preemption (`should_preempt`)

A running task is preempted when:
- It has consumed its full time slice (`elapsed_ns ≥ slice_ns`), **or**
- An eligible task with an earlier virtual deadline becomes runnable.

---

## Data Structures

### `SigmaTask`

```rust
pub struct SigmaTask {
    pub id:             TaskId,    // unique u64 identifier
    pub vruntime:       u64,       // accumulated virtual runtime (ns)
    pub eligible_time:  u64,       // earliest time task may run
    pub deadline:       u64,       // virtual deadline
    pub weight:         u64,       // scheduling weight (1 – 88761)
    pub slice_ns:       u64,       // time slice (default 4 ms)
    pub on_rq:          bool,      // currently on runqueue?
    pub cpu_affinity:   u64,       // bitmask of allowed CPUs (0 = any)
    pub name:           String,    // debug name
}
```

### `SigmaEevdfRunqueue`

```rust
pub struct SigmaEevdfRunqueue {
    pub cpu:           u32,
    tasks:             BTreeMap<(u64, TaskId), SigmaTask>, // sorted by (deadline, id)
    pub min_vruntime:  u64,
    pub curr:          Option<TaskId>,
    pub nr_running:    usize,
    pub total_weight:  u64,
    pub clock_task:    u64,
}
```

The `BTreeMap<(deadline, id), task>` ordering gives O(log n) enqueue/dequeue and
O(1) access to the smallest-deadline task (`first_entry()`).

---

## Configuration

| Parameter | Default | Description |
|-----------|---------|-------------|
| `DEFAULT_SLICE_NS` | 4,000,000 ns (4 ms) | Per-task time slice |
| `MIN_WEIGHT` | 1 | Minimum task weight (nice +19) |
| `DEFAULT_WEIGHT` | 1024 | Weight at nice 0 |
| `MAX_WEIGHT` | 88,761 | Maximum weight (nice -20) |
| `IMBALANCE_THRESHOLD` | 2 | Tasks difference to trigger load migration |

Weight values follow the Linux `prio_to_weight[]` table.

---

## Comparison

### vs Linux CFS (Completely Fair Scheduler, Linux ≤ 6.5)

| Feature | Linux CFS | SigmaOS EEVDF |
|---------|-----------|---------------|
| Algorithm | Red-black tree on vruntime | BTreeMap on (deadline, id) |
| Latency | `sched_latency` / nr_running | Earliest-eligible-deadline-first |
| Wakeup preemption | `wakeup_gran` heuristic | Strict deadline comparison |
| Lag tracking | No | Yes (eligible_time = vruntime) |

### vs Linux EEVDF (Linux 6.6+)

SigmaOS EEVDF is a clean-room re-implementation in safe Rust.  Differences:

- Uses `BTreeMap` instead of a per-CPU red-black tree for simplicity.
- No group scheduling or cgroup integration yet.
- Load balancer is a stub (proportional to task count, not task weight).

### vs BSD ULE (DragonFly/FreeBSD)

| Feature | BSD ULE | SigmaOS EEVDF |
|---------|---------|---------------|
| Runqueue structure | Per-priority bitmap queue | Virtual-deadline BTreeMap |
| Interactivity boost | Yes (sleep/run ratio) | Via weight (future) |
| Real-time classes | Yes | Planned |
| Load balancing | Pull-based | Push + pull (stub) |

---

## Multi-CPU Load Balancing

`SigmaLoadBalancer` holds one `SigmaEevdfRunqueue` per logical CPU.  Balancing
is triggered when the busiest CPU has at least `IMBALANCE_THRESHOLD` more tasks
than the idlest CPU.  One migratable task (not currently running, affinity allows
destination CPU) is moved per balance call.

Full weighted load-balancing (NUMA-aware, cache-topology-aware) is planned for
a future milestone.

---

## API Summary

```rust
// Create a 4-CPU system.
let mut lb = SigmaLoadBalancer::new(4);

// Enqueue a task on CPU 0.
let t = SigmaTask::new(42, "my-task", DEFAULT_WEIGHT);
lb.runqueues[0].enqueue_task(t);

// Pick the next task.
let next = lb.runqueues[0].pick_next_task();

// Advance virtual clock after 2 ms of execution.
lb.runqueues[0].update_curr(2_000_000);

// Check if we should preempt.
if lb.runqueues[0].should_preempt(2_000_000) {
    // trigger reschedule
}

// Balance load.
lb.balance();
```

---

## Source Location

`src/kernel/sigma_scheduler_eevdf.rs`
