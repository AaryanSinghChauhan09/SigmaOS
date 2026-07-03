# SigmaOS Scheduler

The SigmaOS scheduler (`kernel/core/sigma_sched.rs`) combines three scheduling
algorithms in one unified implementation, selecting the right policy per task.

---

## Three Policies

### MLFQ — Interactive Tasks (default)

4-level Multi-Level Feedback Queue for general-purpose workloads.

```
Queue 0: Q0 — highest priority, 2-tick quantum (interactive, new tasks)
Queue 1: Q1 — medium-high,      4-tick quantum
Queue 2: Q2 — medium,           8-tick quantum  
Queue 3: Q3 — lowest,          16-tick quantum  (CPU-bound background)
```

Rules:
- New tasks start at Q0
- If a task uses its full quantum → demoted to next lower queue
- If a task blocks before quantum expires → stays at current level (I/O-bound = stays high)
- Every 200 ticks: **priority boost** — all tasks moved to Q0 (prevents starvation)

### CFS — Fair Sharing

Linux-style Completely Fair Scheduler for batch and daemon workloads.

- Tracks `vruntime` per task (total CPU time weighted by priority)
- Always picks the task with the smallest `vruntime` (leftmost in concept-tree)
- New tasks start at `min_vruntime` to prevent starvation

### EDF — Hard Real-Time

Earliest Deadline First for `PROFILE=rtos` and industrial tasks.

- Each task has an absolute deadline in nanoseconds
- Always schedules the task whose deadline is soonest
- Deadline miss detection: `check_deadline_misses(now_ns)` returns count
- IRQ latency target: < 10 µs with EDF

---

## Priority Order

```
EDF tasks     (must meet deadline)
  ↓
MLFQ tasks    (interactive, Q0 first)
  ↓
CFS tasks     (fair CPU sharing)
  ↓
Idle          (background, only when nothing else runnable)
```

---

## Adding a Task

```c
// C ABI (from kernel init or process manager)
sched_add_task(
    pid,           // process ID
    SCHED_MLFQ,    // 0=MLFQ, 1=CFS, 2=EDF, 3=FIFO
    0,             // deadline_ns (EDF only)
    0              // mlfq_level (0=Q0 default)
);

// EDF example (deadline 1 ms from now)
uint64_t deadline = sigma_clock_ns() + 1_000_000;
sched_add_task(pid, SCHED_EDF, deadline, 0);
```

---

## Timer Tick

```c
// Called from timer IRQ handler (e.g. PIT at 1000 Hz)
uint32_t next_pid = sched_tick(sigma_clock_ns());
// next_pid = 0 → run idle task
// next_pid > 0 → context switch to this PID
```

---

## NUMA-Aware Scheduling

Phase C adds CPU affinity:
```rust
task.cpu_affinity = 0x0000_000F; // allow CPUs 0-3
task.cpu_affinity = 0xFFFF_FFFF; // any CPU (default)
```

The scheduler checks `cpu_affinity` before placing a task on a CPU's runqueue.

---

## Performance Targets

| Metric | Target | Profile |
|--------|--------|---------|
| IRQ latency (EDF) | < 10 µs | `rtos` |
| Context switch time | < 500 ns | all |
| MLFQ boost period | 200 ticks | all |
| Max tasks | 512 | all |
| Max EDF tasks | 64 | `rtos` |

---

## Source

`kernel/core/sigma_sched.rs` — 280 lines, `#![no_std]`, no external crates.

*See also: [Real-Time Scheduler Shard](Real-Time-Scheduler-Shard) · [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md)*
