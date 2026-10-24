# SigmaOS CPU Scheduler Status & Invariants Specification

## Overview

This document provides a comprehensive audit of the SigmaOS CPU Scheduler implementation status across all scheduler modules, core run-queue infrastructure, architecture context-switching, timer calibration, task states, and resource controllers. It defines the core policy invariants, state transition rules, and classification gates governing all scheduler changes in SigmaOS.

---

## Subsystem Audit and Capability Status Matrix

Each scheduler component in SigmaOS is evaluated against strict capability levels (**Proposed**, **Prototype**, **Integrated**, **Supported**). Unit-level or model-level evidence is explicitly distinguished from real runtime execution.

| Component | File Path(s) | Current Status | Key Evidence / Gaps | Next Milestone |
|---|---|---|---|---|
| **Scheduler Core** | `src/scheduler/scheduler.rs`, `src/kernel/scheduler.rs` | **Prototype** | Unit tests pass for run-queue operations, task state machine stubs, and policy dispatch. Gaps: Runnable queues not fully connected to task lifecycle hooks across process creation/exit. | PR 3: SchedClass interface refactor & PR 4: Task lifecycle run-queue integration |
| **CFS (Completely Fair Scheduler)** | `src/kernel/cfs_scheduler.rs`, `src/scheduler/distro_schedulers.rs` | **Prototype** | Unit tests for vruntime tracking, nice-to-weight mapping, and tree insertion order. Gaps: Needs weighted fair scheduling semantics enforcement and group scheduling integration. | PR 7: Implement weighted fair scheduling semantics |
| **BORE (Burst-Oriented Response Enhancer)** | `src/scheduler/bore.rs` | **Prototype** | Unit tests for burst score calculation, exponential moving average decay, and interactive boosting. Gaps: Needs integration as an optional heuristic within the Fair class without violating fairness bounds. | PR 9: Integrate BORE as optional interactive heuristic for Fair class |
| **EEVDF (Earliest Eligible Virtual Deadline First)** | `src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs` | **Prototype** | Phase 1 implementation complete with unit tests for virtual deadline calculation, lag accounting, and eligibility tracking. Gaps: Needs runtime evaluation against CFS baseline. | PR 8: Implement EEVDF fair-class prototype & evaluation against CFS baseline |
| **Real-Time (SCHED_FIFO / SCHED_RR)** | `src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs` | **Prototype** | Unit tests for priority bitmap pick and round-robin quantum rotation. Gaps: Needs priority privilege enforcement, interrupt/preemption worst-case latency measurements, and RT starvation guardrails. | PR 10: Enforce FIFO and round-robin policy semantics & RT bandwidth throttling |
| **Context Switching (x86_64)** | `src/arch/x86_64/context.rs`, `src/arch/x86_64/gdt.rs` | **Prototype** | Unit tests and assembly context frame save/restore routines exist. Gaps: Needs integration with scheduler run queues, APIC timer preemption, userland Ring 3 switch, and FPU/SIMD state. | PR 6: Integrate x86_64 context switching and idle loop |
| **Timers & APIC Calibration** | `src/arch/x86_64/apic.rs`, `src/kernel/time.rs` | **Prototype** | Local APIC timer tick stubs and interrupt vectors implemented. Gaps: Needs QEMU APIC timer calibration and preemption tick dispatch into scheduler core accounting. | PR 5: Connect timer ticks and preemption to scheduler core |
| **CPU Controllers & Quotas** | `src/process/cgroups.rs` | **Proposed** | Conceptual model for CPU share/quota structs. Gaps: Scheduler accounting paths do not currently enforce cgroup weights/quotas. | PR 13: Enforce CPU controller weights and quotas |
| **AArch64 / RISC-V Targets** | `src/arch/aarch64/`, `src/arch/riscv64/` | **Prototype** | Architecture compilation targets and basic context context definitions exist. Gaps: Unvalidated timer, idle path, and interrupt return paths on QEMU. | PR 17: Bring up AArch64 and RISC-V scheduling paths |

---

## Core Policy Rules and Invariants

To ensure correctness, predictability, and safety across all scheduling policies, SigmaOS enforces nine core rules and invariants:

### 1. Separate Policy from Mechanism
- The **Scheduler Core** strictly owns run-queue locking, task-state transitions, timer interrupt handling, context-switch assembly hooks, CPU selection interfaces, and per-CPU run-queue management.
- **Scheduling Classes** (Idle, Fair/EEVDF/CFS, Real-Time FIFO/RR, Deadline) strictly own policy decisions, virtual runtime/deadline math, priority ordering, and slice selection logic.
- Hybrid or compound policies (e.g., CFS + BORE) are forbidden until class boundaries and precedence rules are explicit and modularly isolated.

### 2. Define Task States and Transitions
- Task states are explicitly defined as: `Runnable`, `Running`, `Blocked`, `Stopped`, `Exiting`, and `Idle`.
- Valid transitions:
  - `Runnable` -> `Running` (selected by scheduler core on CPU)
  - `Running` -> `Runnable` (preempted or time-slice yield)
  - `Running` -> `Blocked` (waiting on I/O, wait queue, futex, or signal)
  - `Blocked` -> `Runnable` (wakeup event triggered)
  - `Running` -> `Exiting` (task exit or fatal signal)
- **Queue Invariants**:
  - A task must reside on **at most one** run queue at any given instant.
  - A task must **never** run on two CPUs simultaneously.

### 3. Fair Scheduling Needs a Measurable Contract
- Fair scheduling classes must define relative CPU shares derived from task weight / nice values.
- Wakeup latency targets and maximum starvation bounds must be explicitly defined.
- Sleeping tasks accruing or decaying fairness debt (lag) must follow bounded algorithms (e.g., EEVDF lag bounds) to prevent sleeper abuse or long-idle task preemption storms upon wakeup.

### 4. Real-Time Policies Need Guardrails
- `SCHED_FIFO` and `SCHED_RR` tasks must have explicit priority semantics (0–99), quantum behavior, privilege policy (preventing unprivileged user processes from setting RT class), and starvation controls (RT bandwidth throttling, e.g., capping RT tasks to 95% CPU time).
- No policy may be designated as real-time until interrupt disable times, preemption paths, and worst-case context-switch latencies are measured.

### 5. Scheduler Changes Require Data, Not Anecdotes
- Every scheduler modification must be validated against quantitative metrics:
  - System throughput
  - Fairness index (Jain's fairness index or share ratio)
  - Wake-to-run latency (median, 95th, 99th percentile)
  - Context-switch frequency
  - CPU migration count and run-queue imbalance
  - Idle CPU time percentage
- Baseline workloads and fixed QEMU / hardware test configurations must be preserved for comparisons.

### 6. Concurrency is Part of Correctness
- Run-queue lock ordering must be strictly documented (e.g., lower CPU ID run-queue lock acquired first during load balancing to prevent deadlocks).
- Preemption and interrupt disable assumptions must be explicitly documented per critical section.
- Per-CPU state protection and memory ordering (barriers / atomic synchronization) must be maintained across CPU context switching and migration paths.

### 7. Resource Controls Must Affect Real Scheduling
- CPU controllers (shares, quotas, periods) exist as models in userspace/kernel structures but must not be advertised as supported until scheduler dispatch and tick accounting paths actually account for and throttle tasks based on quota limits.

### 8. Keep Architecture Status Separate
- A policy or scheduler feature validated on x86_64 QEMU cannot be claimed as Supported on AArch64 or RISC-V.
- Each architecture must independently validate its timer calibration, context switch assembly, idle loop, interrupt return path, and CPU topology.

### 9. Keep Diagnostics Bounded and Stable
- Expose versioned, read-only scheduler statistics through a structured interface (e.g., `schedstats`).
- Counters must be monotonic or have explicitly defined reset semantics, and diagnostic overhead must be bounded so observability does not distort scheduling metrics.

---

## Precedence Rules

When picking the next task to run on a CPU, scheduling classes are evaluated in strict priority order:

1. **Stop Class** (Kernel migration / shutdown tasks)
2. **Deadline Class** (`SCHED_DEADLINE` hard real-time tasks)
3. **Real-Time Class** (`SCHED_FIFO` / `SCHED_RR` tasks by priority level 99 down to 0)
4. **Fair Class** (EEVDF / CFS / BORE weighted fair scheduling)
5. **Idle Class** (Per-CPU idle thread)
