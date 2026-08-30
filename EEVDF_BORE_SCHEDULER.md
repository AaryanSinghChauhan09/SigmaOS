# EEVDF + BORE Scheduler Implementation

## EEVDF Algorithm

EEVDF (Earliest Eligible Virtual Deadline First) — introduced in Linux 6.6.

### Core Concepts

*   **vruntime**: Normalized CPU time consumed, weighted by priority
*   **virtual\_deadline**: vruntime + (time\_slice / weight)
*   **eligibility**: process.vruntime <= system\_vtime

### Weight Table (Nice -20 to +19)

| Nice | Weight |
|------|--------|
| -20 | 88761 |
| -10 | 9548 |
| 0 | 1024 |
| 10 | 110 |
| 19 | 15 |

### Selection Rule

1.  Filter: eligible processes (vruntime <= system\_vtime)
2.  Pick: min(virtual\_deadline) among eligible
3.  Fallback: min(vruntime) if no eligible (starvation prevention)

### vruntime Update

```rust
let delta_exec = time_elapsed;
let delta_vruntime = delta_exec * NICE0_WEIGHT / process.weight;
process.vruntime += delta_vruntime;
system_vtime = min(vruntime across all runnable processes);
```

## BORE Enhancement

BORE (Burst-Oriented Response Enhancer) penalizes CPU-bound tasks.

### Burst Score Tracking

```rust
fn update_bore(process: &mut Process, ran_full_slice: bool) {
    if ran_full_slice {
        // CPU-bound: didn't voluntarily yield
        process.burst_score = process.burst_score.saturating_add(1);
    } else {
        // I/O-bound or interactive: yielded early
        process.burst_score = process.burst_score.saturating_sub(1);
    }
}
```

### Deadline Penalty

```rust
let base_deadline = process.vruntime + (time_slice / process.weight);
let bore_penalty = process.burst_score * BORE_PENALTY_FACTOR;
process.virtual_deadline = base_deadline + bore_penalty;
```

**Effect**: CPU-bound processes get pushed later in selection queue.
Interactive processes (low burst\_score) get selected more often.

## Implementation Details

### Data Structures

*   **Red-black tree**: Processes ordered by virtual\_deadline for O(log n) selection
*   **Per-NUMA run queues**: Each NUMA node has independent scheduler queue
*   **Work-stealing deque**: Lock-free queue for cross-CPU task migration

### Scheduler Classes (Priority)

1.  Stop (CPU hotplug)
2.  Deadline (EDF, hard real-time)
3.  Realtime (FIFO + RR, soft real-time)
4.  **Fair (EEVDF+BORE)** — normal processes
5.  Idle (background)

### Time Slices

| Nice | Default Slice |
|------|--------------|
| -20 | 200ms |
| 0 | 4ms |
| 19 | 0.75ms |
