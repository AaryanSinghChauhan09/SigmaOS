# SigmaOS: CachyOS-Style BORE (Burst-Oriented Response Enhancer) Scheduler

This document specifies the algorithm, design constraints, and implementation mechanics of SigmaOS's predictive priority scheduler based on CachyOS's BORE scheduler.

---

## 🚀 Theoretical Foundations: CFS & EEVDF

While standard Linux systems historically used the Completely Fair Scheduler (CFS) and modern Linux uses the Earliest Eligible Virtual Deadline First (EEVDF) scheduler, they often suffer under mixed workloads. Heavy CPU-bound background processes (batch tasks) can temporarily starve highly interactive user-facing processes (such as GUI desktop components).

SigmaOS integrates a **BORE (Burst-Oriented Response Enhancer)** scheduler to mathematically isolate and reward interactive tasks.

---

## 🌀 BORE Scheduler Design

The BORE scheduler tracks process execution patterns in terms of "burstiness".

```
                                  +-----------------------+
                                  |     PROCESS SPAWNED   |
                                  +-----------------------+
                                              |
                                              v
                                  +-----------------------+
                                  | Initial Burst Score: 0|
                                  +-----------------------+
                                              |
                                     Task Executed / Blocked
                                              |
                     +------------------------+------------------------+
                     |                                                 |
                     v                                                 v
        [Task Runs Long Unblocked]                         [Task Blocks on I/O / Input]
                     |                                                 |
                     v                                                 v
        +-------------------------+                       +-------------------------+
        | Increase Burst Score    |                       | Decay Burst Score       |
        | (Penalize Batch Task)   |                       | (Reward Interactive)    |
        +-------------------------+                       +-------------------------+
                     |                                                 |
                     +------------------------+------------------------+
                                              |
                                              v
                                  +-----------------------+
                                  | Calculate Virtual     |
                                  | Deadline (Weighted    |
                                  | by Burst Score)       |
                                  +-----------------------+
```

### 1. Burst Score Tracking
* **Burst Score:** An integer indicating the "burstiness" of a task (typically ranging from 0 to 15).
* **Burst Accumulation:** When a process runs continuously without blocking, its burst score increases.
* **Burst Decay:** When a process blocks on I/O, user input, or timer wakeups, its burst score decays toward 0.

### 2. Virtual Deadline Adaptation
The scheduler uses the EEVDF algorithm to select the next runnable process. However, the task's virtual deadline is adjusted by its burst score:

$$\text{vruntime\_increment} = \text{run\_slice} \times \text{burst\_weight}$$

* **Batch Tasks (High Burst Score):** Have high burst weights, causing their virtual runtimes to advance rapidly. They receive delayed execution opportunities.
* **Interactive Tasks (Low Burst Score):** Have minimal burst weights. Their virtual runtimes advance slowly, ensuring they receive immediate execution priority upon wakeup.

---

## 🛡️ Priority Inversion & Starvation Protections

To prevent CPU-bound tasks from starving entirely under highly interactive workloads, the scheduler implements:
* **Realtime Bound Limits:** Caps the maximum penalty applied to batch tasks to guarantee minimum slice execution.
* **Fair-Share Decaying:** Periodic global decay cycles that reset task burst scores over time, balancing absolute throughput and fluid interactive responsiveness.
