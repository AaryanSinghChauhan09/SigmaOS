# SigmaOS CPU Scheduler Status, Architecture & Policy Invariants

## Overview & Architecture Audit

SigmaOS CPU scheduling coordinates CPU resource allocation across runnable tasks, balancing throughput, low wakeup latency, real-time determinism, and fairness.

The scheduler architecture consists of two primary layers:
1. **Scheduler Core (`src/scheduler/core.rs`, `src/scheduler/run_queue.rs`, `src/kernel/scheduler.rs`)**: Owns run-queue synchronization, CPU run queues, per-CPU state, task state machine transitions, timer preemption hooks, context-switch interfaces, and policy dispatch.
2. **Scheduling Policy Classes (`src/scheduler/distro_schedulers.rs`, `src/scheduler/bore.rs`, `src/scheduler/eevdf.rs`, `src/kernel/cfs_scheduler.rs`)**: Implements policy selection logic for Real-Time (FIFO/RR), Fair (CFS/EEVDF), BORE interactive heuristics, and Idle classes.

### Current Implementation Status Audit

| Subsystem / Module | Scope | Current Status | Test Evidence | Outstanding Gaps / Next Steps |
|---|---|---|---|---|
| `src/scheduler/core.rs` | Scheduler core & run queue | Prototype | Unit tests for queue bounds & state machine | Connect process/thread creation, wakeup, block, yield, and exit to run queue |
| `src/kernel/cfs_scheduler.rs` | Completely Fair Scheduler | Prototype | Unit tests for vruntime & nice weights | Connect weighted vruntime to timer ticks and task execution |
| `src/scheduler/eevdf.rs` | EEVDF scheduler algorithm | Prototype | Unit tests for lag, eligibility, & deadlines | Fair class integration & comparison against CFS baseline |
| `src/scheduler/bore.rs` | BORE burst scoring heuristic | Prototype | Unit tests for burst score & EMA decay | Integrate as optional interactive heuristic for Fair class |
| `src/scheduler/distro_schedulers.rs` | Real-Time (FIFO/RR) policies | Prototype | Unit tests for priority queues & RR quanta | Connect priority preemption, timer ticks, and RT bandwidth limits |
| x86_64 Context Switch & APIC | Architecture execution | Prototype | Unit tests for context frame layout | Wire APIC timer interrupt, context switch, and Ring 3 user tasks |
| AArch64 & RISC-V Targets | Cross-architecture boot | Proposed / Prototype | Compilation checks | Bring up timer, context switch, idle loop, and interrupt return paths |

---

## Core Scheduling Rules & Invariants

To guarantee correctness, safety, and predictability, all scheduler developments in SigmaOS must adhere to the following 9 core invariants:

1. **Policy/Mechanism Separation**: The core scheduler owns run queues, locking, task state transitions, timer ticks, context-switch triggers, and CPU selection. Scheduling policy classes implement ordering and slice allocation decisions.
2. **Explicit Task State Machine**: Every task must be in exactly one state (`Runnable`, `Running`, `Blocked`, `Stopped`, `Exiting`, `Idle`). Task transitions (`enqueue`, `dequeue`, `wakeup`, `block`, `preempt`) must be explicit and valid.
3. **Single Run-Queue Ownership**: A task must exist on at most one run queue at any given time and must never run on two CPUs simultaneously.
4. **Strict Policy Precedence**: Execution order strictly obeys class hierarchy: Real-Time (`SCHED_FIFO` / `SCHED_RR`) > Fair (`SCHED_OTHER` / `CFS` / `EEVDF`) > Idle (`SCHED_IDLE`).
5. **Measurable Fairness Contract**: Fair scheduling policies must distribute CPU time in proportion to task weight/nice levels, bounded by lag and virtual deadline constraints without starvation.
6. **Real-Time Guardrails & Bandwidth Control**: Real-time tasks must have explicit priorities (1–99), quantum bounds for round-robin rotation, and bandwidth limits to prevent kernel or system starvation.
7. **Concurrency & Lock Ordering**: Per-CPU run queues must enforce strict lock ordering (e.g. acquire lower CPU ID lock before higher CPU ID lock during migration) and protect against reentrancy from preemptive timer interrupts.
8. **Enforced Resource Controls**: CPU controller weights and quotas must be explicitly tracked and enforced in scheduler accounting paths before being advertised as working.
9. **Evidence-Driven Capability Status**: A scheduler capability cannot be marked `Supported` or `Integrated` based solely on unit or model tests. Real hardware or booted QEMU integration evidence is mandatory.

---

## Task State Machine & Transitions

```
                 +--------------+
                 |     New      |
                 +--------------+
                        |
                        v (enqueue)
+------------+  wakeup   +--------------+  select_next   +--------------+
|  Blocked   | --------> |   Runnable   | -------------> |   Running    |
+------------+           +--------------+                +--------------+
      ^                         ^                               |
      |          preempt / yield|                               |
      +-------------------------+-------------------------------+
                block                       exit
                                              |
                                              v
                                       +--------------+
                                       |   Exiting    |
                                       +--------------+
```

---

## Arch Support Matrix & Test Levels

| Architecture / Platform | Target Configuration | Context Switch | Timer Preemption | Status |
|---|---|---|---|---|
| `x86_64` | QEMU / Bare Metal | Integrated (Ring 3/0) | APIC Timer Interrupt | Prototype |
| `aarch64` | QEMU `virt` | Prototype | Generic Timer | Proposed |
| `riscv64` | QEMU `virt` | Prototype | SBI Timer | Proposed |

---

## Operational Guidelines

1. **Deterministic Testing**: Every state transition, run-queue manipulation, and fairness calculation must have deterministic unit tests with simulated clock ticks.
2. **Lock Order**: Never acquire multiple per-CPU run queue locks without sorting CPU IDs. Disable local interrupts during run-queue lock acquisition in kernel context.
3. **Traceability**: All state mutations must increment per-CPU versioned scheduler statistics counters.
