# ⚙️ SigmaOS Scheduler

SigmaOS implements a **multi-class hierarchical scheduler** with Linux EEVDF, CachyOS BORE, MLFQ, Transformer-based prediction, and thermal-aware scheduling.

## Scheduler Classes (Priority Order)

| Class | Policy | Use Case |
|-------|--------|----------|
| Stop | Per-CPU stop | CPU hotplug, migration |
| Deadline | EDF (Earliest Deadline First) | Hard real-time periodic tasks |
| Realtime | FIFO + Round Robin | Soft real-time, audio, video |
| **Fair (EEVDF+BORE)** | Virtual deadline | Normal interactive & batch |
| Idle | Idle threads | Background work when CPU free |

## EEVDF Scheduler

**EEVDF** (Earliest Eligible Virtual Deadline First) was introduced in Linux 6.6 as the replacement for CFS.

### Key Concepts

*   **Virtual Runtime (vruntime)**: Normalized CPU time consumed by a process
*   **System Virtual Time (V)**: Minimum vruntime of all runnable processes
*   **Eligibility**: A process is eligible if `vruntime ≤ V`
*   **Virtual Deadline**: `virtual_deadline = vruntime + (time_slice / weight)`

### Selection Rule

1.  Filter processes where `vruntime ≤ system_vtime` → eligible set
2.  Among eligible, pick the one with **earliest virtual deadline**
3.  If no eligible (starvation prevention): pick minimum vruntime

### Priority Weights

| Nice Level | Weight |
|------------|--------|
| -20 (highest) | 88761 |
| 0 (normal) | 1024 |
| 19 (lowest) | 15 |

## BORE Enhancement

**BORE** (Burst-Oriented Response Enhancer) from CachyOS adds a **burst penalty** to CPU-bound processes, preventing them from starving interactive tasks.

```rust
// BORE burst penalty calculation
let bore_penalty = process.burst_score / 2;
process.virtual_deadline = current_time + (1000 / weight) + bore_penalty;
```

**Effect**: CPU-bound processes accumulate `burst_score`. This increases their virtual deadline, making interactive processes (low burst score) more likely to be selected.

## NUMA-Aware Work Stealing

SigmaOS supports **NUMA** (Non-Uniform Memory Access) topologies:

    NUMA Node 0 (CPUs 0-7)    NUMA Node 1 (CPUs 8-15)
        Run Queue 0                Run Queue 1
             ↑                          ↑
        Work Stealing ←→→→→→→→→ Work Stealing

**Work-stealing algorithm**:

1.  CPU checks its local run queue
2.  If empty, try to steal a task from another CPU's queue
3.  Prefer stealing from CPUs in the same NUMA node
4.  Only steal from remote NUMA nodes as last resort

## MLFQ Scheduler

**Multi-Level Feedback Queue** for workload classification:

| Queue Level | Priority | Time Quantum | Promotion |
|-------------|----------|--------------|-----------|
| 0 | Highest | 5ms | — |
| 1 | High | 10ms | After exhausting Q0 |
| 2 | Normal | 20ms | After exhausting Q1 |
| 3 | Low | 40ms | After exhausting Q2 |
| 4 | Idle | Unlimited | Background |

## Thermal Scheduler

Adapts CPU frequencies and task placement based on thermal state:

| Thermal Zone | Temperature | Action |
|-------------|-------------|--------|
| Normal | < 70°C | Full performance |
| Warm | 70-80°C | Reduce boost, avoid hotspots |
| Hot | 80-90°C | Throttle CPU, migrate tasks |
| Critical | > 90°C | Emergency frequency cap |

## Aperiodic Task Scheduling

For embedded/RTOS workloads, SigmaOS supports:

*   **Polling Server**: Dedicated bandwidth for aperiodic tasks
*   **Deferrable Server**: Budget rolls over if unused
*   **Sporadic Server**: Replenishment after each service period

## Workload Classification

```rust
pub enum TaskWorkloadType {
    CpuBound,          // High burst score, penalized by BORE
    IoBound,           // Frequently blocks on I/O
    Interactive,       // Low burst, fast response needed
    Batch,             // Background, best-effort
    RealTimePeriodic,  // Fixed-period deadline tasks
    RealTimeAperiodic, // Event-triggered deadline tasks
    SystemKernelDaemon,// PID 1, kernel threads
}
```
