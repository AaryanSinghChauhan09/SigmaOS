# Linux CFS and BORE Scheduler Integration in SigmaOS

## Overview

SigmaOS implements a hybrid scheduler inspired by Linux's Completely Fair Scheduler (CFS) and CachyOS's Burst-Oriented Response Enhancer (BORE) patch, providing both fairness and low-latency interactive response.

## Linux CFS Algorithm

### Virtual Runtime (vruntime)

CFS tracks a per-task `vruntime` - the amount of time a task "conceptually" used, weighted by priority:

    vruntime += delta_exec * (NICE_0_LOAD / task.weight)

Where `NICE_0_LOAD = 1024` is the default weight for nice-0 tasks.

### Red-Black Tree Scheduling

```rust
// Conceptual structure (SigmaOS uses BTreeMap as rb-tree substitute)
pub struct CfsSigmaScheduler {
    run_queue: BTreeMap<u64, TaskControl>,  // vruntime → task (red-black tree semantics)
    current_task: Option<u64>,              // Currently running task's vruntime key
    min_vruntime: u64,                      // Minimum vruntime in the tree
    target_latency_ns: u64,                 // CFS target scheduling latency
    min_granularity_ns: u64,               // CFS minimum granularity
}

impl CfsSigmaScheduler {
    /// Pick next task: leftmost node in red-black tree (smallest vruntime)
    pub fn pick_next_task(&mut self) -> Option<u64> {
        self.run_queue.keys().next().copied()
    }

    /// Update vruntime after running for delta nanoseconds
    pub fn update_vruntime(&mut self, task_id: u64, delta_ns: u64, weight: u64) {
        let vruntime_delta = delta_ns * 1024 / weight;  // Weighted time
        // Re-insert with new vruntime (tree re-sorts automatically)
        if let Some(task) = self.run_queue.remove(&task_id) {
            let new_vrt = task.vruntime.saturating_add(vruntime_delta);
            self.run_queue.insert(new_vrt, TaskControl { vruntime: new_vrt, ..task });
        }
    }
}
```

## BORE Patch Enhancement

### Burst Score

BORE adds a `burst_score` to track how "bursty" a task is:

```rust
pub struct BoreTask {
    pub burst_score: u64,    // Higher = more bursty (gets slight disadvantage)
    pub burst_time: u64,     // Accumulated burst time in ns
    pub prev_burst: u64,     // Previous burst for EWMA calculation
}

impl BoreTask {
    /// Update burst score using Exponential Weighted Moving Average
    pub fn update_burst_score(&mut self, burst_ns: u64) {
        // EWMA: new_score = alpha * burst_ns + (1-alpha) * old_score
        // With alpha = 1/8 (bit shift for efficiency)
        self.burst_score = (self.burst_score * 7 + burst_ns) / 8;
    }

    /// Calculate BORE-adjusted vruntime penalty
    pub fn vruntime_penalty(&self, base_penalty: u64) -> u64 {
        // Scale penalty by burst_score
        base_penalty + (self.burst_score >> 10)  // 10-bit right shift = /1024
    }
}
```

### Why BORE Improves Interactivity

    Standard CFS:
      Interactive GUI task │████░░░░░░░░░░░░░░│ waits for background compile
      Background compile   │░░░░████████████░░│ gets full time slice

    BORE-enhanced:
      Interactive GUI task │████░░████░░████░░│ gets frequent small bursts
      Background compile   │░░░░████░░████░░██│ slightly lower priority

Interactive tasks have low `burst_score` (they sleep often), so BORE gives them scheduling preference.

## SigmaOS Round-Robin + CFS Hybrid

```rust
pub struct SigmaHybridScheduler {
    /// CFS queue for normal tasks
    cfs_queue: CfsSigmaScheduler,
    /// Real-time FIFO queue (highest priority)
    rt_queue: Vec<u64>,
    /// Idle tasks queue
    idle_queue: Vec<u64>,
    /// CPU affinity map (task_id → allowed_cpus bitmask)
    affinity: BTreeMap<u64, u64>,
}

impl SigmaHybridScheduler {
    pub fn schedule(&mut self) -> Option<u64> {
        // 1. Real-time tasks always first (SCHED_FIFO/SCHED_RR)
        if let Some(&rt_task) = self.rt_queue.first() {
            return Some(rt_task);
        }

        // 2. CFS for normal tasks
        if let Some(cfs_task) = self.cfs_queue.pick_next_task() {
            return Some(cfs_task);
        }

        // 3. Idle task if nothing else
        self.idle_queue.first().copied()
    }

    /// Dynamic priority boost for interactive tasks (Linux-inspired)
    pub fn boost_interactive(&mut self, task_id: u64) {
        // Reduce vruntime to give interactive tasks a scheduling advantage
        if let Some(task) = self.cfs_queue.run_queue.remove(&task_id) {
            let boosted_vrt = task.vruntime.saturating_sub(1_000_000); // -1ms
            self.cfs_queue.run_queue.insert(boosted_vrt, TaskControl { vruntime: boosted_vrt, ..task });
        }
    }
}
```

## CPU Affinity Implementation

Inspired by Linux's `sched_setaffinity(2)`:

```rust
/// Set CPU affinity for a task (bitmask of allowed CPUs)
pub fn set_affinity(&mut self, task_id: u64, cpu_mask: u64) -> Result<(), SchedError> {
    if cpu_mask == 0 {
        return Err(SchedError::InvalidAffinity);
    }
    self.affinity.insert(task_id, cpu_mask);
    Ok(())
}

/// Check if task can run on this CPU
pub fn can_run_on_cpu(&self, task_id: u64, cpu_id: u8) -> bool {
    self.affinity
        .get(&task_id)
        .map(|&mask| mask & (1 << cpu_id) != 0)
        .unwrap_or(true) // No affinity = any CPU
}
```

## Scheduler Classes (Linux-inspired)

| Class | Priority | Use Case |
|-------|----------|----------|
| `SCHED_FIFO` | 0-99 (RT) | Hard real-time, interrupts |
| `SCHED_RR` | 0-99 (RT) | Soft real-time, round-robin |
| `SCHED_OTHER` (CFS+BORE) | 100-139 (nice) | Normal desktop/server tasks |
| `SCHED_BATCH` | CFS (lower) | Background batch processing |
| `SCHED_IDLE` | Lowest | Idle tasks only |
| `SCHED_DEADLINE` | EDF | Deadline-constrained tasks |

## References

*   [CachyOS BORE Scheduler Architecture](CachyOS-BORE-Scheduler-Architecture)
*   Linux kernel `kernel/sched/fair.c`
*   [BORE patch on GitHub](https://github.com/firelzrd/bore-scheduler)
*   `src/scheduler/` (SigmaOS implementation)
*   `sigmaos/core/src/atomic_scheduler_cfs.cpp`
