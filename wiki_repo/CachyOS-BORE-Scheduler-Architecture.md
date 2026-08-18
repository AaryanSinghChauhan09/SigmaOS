# CachyOS BORE Scheduler Architecture in SigmaOS

## Overview

SigmaOS incorporates scheduling heuristics inspired by **CachyOS's BORE (Burst-Oriented Response Enhancer)** scheduler to drastically lower desktop latency and prevent UI frame drops under heavy compilation or batch computing workloads.

---

## Key Modules

- [`src/kernel/sched/sigma_mlfq.rs`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/kernel/sched/sigma_mlfq.rs): Dynamic burst tracking and burst-penalty scaling.
- [`src/docs/CachyOS-BORE-Scheduler-Spec.md`](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/src/docs/CachyOS-BORE-Scheduler-Spec.md): Formal mathematical specification of the BORE heuristic algorithm.

---

## Core Mechanisms

### 1. Burst Tracking
- Measures continuous execution slice without voluntary yield (`sleep`, `epoll_wait`, `read`).
- Tasks with low burst times (interactive desktop GUI, audio rendering, gaming input loops) are dynamically elevated in priority.

### 2. Burst-Penalty Normalization
- Long-running batch compute tasks accumulate a burst penalty score.
- Instead of being starved, batch tasks receive larger execution slices less frequently to maximize L1/L2 CPU cache locality.

```
                    ┌─────────────────────────┐
                    │     Task Execution      │
                    └───────────┬─────────────┘
                                │
                 Is execution burst < 5ms?
                     /                      \
                   YES                       NO
                   /                          \
       [Interactive Priority Boost]    [Batch Cache-Locality Slot]
       (Sub-millisecond scheduling)    (Larger time quanta, lower prio)
```
