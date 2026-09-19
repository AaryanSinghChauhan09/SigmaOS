# SigmaOS Sovereign EEVDF CPU Scheduler

## Overview

SigmaOS implements a **100% Safe Rust sovereign EEVDF (Earliest Eligible Virtual Deadline First) scheduler** (`src/kernel/eevdf_sovereign.rs`), absorbing the landmark CPU scheduling paradigm introduced by Peter Zijlstra in Linux 6.6 (replacing the classic CFS - Completely Fair Scheduler).

## Core Concepts

1. **Eligibility (`vruntime <= V`)**:
   - A task is eligible to run only when its accumulated virtual runtime does not exceed the system's average virtual runtime $V$ ($lag \ge 0$).
   - Prevents newly awakened or low-runtime tasks from monopolizing CPU resources.
2. **Virtual Deadline (`deadline = vruntime + slice / weight`)**:
   - Each task has an assigned or requested scheduling quantum/slice ($q_i$) and weight ($w_i$).
   - The virtual deadline dictates latency priority. Short-slice latency-sensitive tasks automatically obtain earlier deadlines without stealing long-term throughput capacity.
3. **Core Selection Algorithm**:
   - The scheduler selects the task with the **Earliest Virtual Deadline** among all **Eligible** runnable tasks.
   - If no task is currently eligible, it falls back to the lowest virtual runtime task (CFS-compatible safety invariant).

## Comparison: CFS vs. EEVDF

| Dimension | Linux CFS (Linux 2.6.23–6.5) | SigmaOS EEVDF (Linux 6.6+ Parity) |
|-----------|------------------------------|-----------------------------------|
| Selection Metric | Minimum `vruntime` | Earliest `deadline` among eligible |
| Latency Control | Coarse `sched_latency_ns` | Fine-grained per-task `slice_ns` |
| Interactive Response | Heuristic latency trade-offs | Deterministic deadline guarantees |
| Starvation Prevention | Virtual runtime scaling | Eligibility threshold enforcement |

## Test Verification

6 standalone unit tests verified in test runner suite `[15]`:
- `test_eevdf_deadline_calculation`: Latency task gets earlier deadline.
- `test_eevdf_priority_scheduling`: Higher weight tasks achieve earlier deadlines.
- `test_eevdf_virtual_runtime_tick`: Runtime and virtual runtime scaling.
- `test_eevdf_fair_alternation`: Fair preemption and alternation between equal tasks.
- `test_eevdf_eligibility_enforcement`: Ineligible future tasks deferred.
- `test_eevdf_empty_scheduler`: Graceful handling of idle system.
