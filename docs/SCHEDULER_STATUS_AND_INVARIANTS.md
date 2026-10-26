# SigmaOS CPU Scheduler Status, Architecture & Policy Invariants

## Executive Overview & Architectural Inventory

SigmaOS's published `FEATURE_STATUS.toml` identifies three primary scheduling implementations in `src/scheduler/` and `src/kernel/`:
* **Completely Fair Scheduler (CFS)** (`src/kernel/cfs_scheduler.rs`, `src/scheduler/cfs.rs`, `src/scheduler/distro_schedulers.rs`)
* **Burst-Oriented Response Enhancer (BORE)** (`src/scheduler/bore.rs`, `src/scheduler/bore_engine.rs`)
* **Real-Time FIFO & Round-Robin (SCHED_FIFO / SCHED_RR)** (`src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs`, `src/scheduler/preempt_rt.rs`)
* **Earliest Eligible Virtual Deadline First (EEVDF)** (`src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs`)

Current evidence for these scheduling modules is at unit-level—verifying fairness metrics, burst scoring calculations, FIFO ordering, and RR quantum rotations. Real kernel execution requires connecting scheduling policy to actual runnable tasks, timer preemption interrupts, and x86_64/AArch64/RISC-V context switching before extending the policy set.

## Core Scheduling Rules & Invariants

Adopt the following architectural invariants and safety rules across all SigmaOS CPU scheduler components:

### Policy/Mechanism Separation
The scheduler core owns run-queue locking, task-state transitions, timer interactions, context-switch hooks, and CPU selection interfaces. Scheduling classes (Idle, Fair, Real-Time, Deadline) own policy decisions. A hybrid CFS/BORE/EEVDF policy must not be used until class boundaries and precedence rules are explicitly defined.

### Explicit Task State Machine
Document and enforce all valid task lifecycle states and transitions:
* **States**: `New`, `Runnable`, `Running`, `Blocked` (Sleeping), `Stopped`, `Exiting` (Zombie/Reaped), `Idle`.
* **Transitions**: Valid enqueue, dequeue, select_to_run, block, yield, wakeup, and exit/reap paths.
* **Invariants**:
  1. A task must be present on at most **one** run queue at any given instant.
  2. A task must **never** run on two CPUs simultaneously.
  3. Transitions directly from `Blocked` or `Exiting` to `Running` without passing through `Runnable` enqueue are prohibited.

### Measurable Fair Scheduling Contract
Fair scheduling policy must specify relative CPU share by weight, wakeup latency targets, starvation bounds, and how sleeping tasks accrue or decay fairness debt (lag). Linux EEVDF serves as a reference for combining virtual runtime fairness with virtual deadlines (`vruntime` / `vdeadline`) and handling sleeper behavior without allowing sleeper-burst exploitation.

### Real-Time Policy Guardrails
Real-time policies (`SCHED_FIFO`, `SCHED_RR`) must have explicit priority semantics (100 priority levels), quantum behavior, privilege checking, and starvation controls (e.g., RT bandwidth throttling). A policy cannot be labeled "real-time" until interrupt, timer, preemption, and worst-case scheduling latency paths are measured on booted targets.

### Data-Driven Empirical Benchmarking
Scheduler changes require hard telemetry, not anecdotes. Every benchmark or comparison must record:
* Throughput, fairness index (Jain's fairness index), wake-to-run latency (median, p95, p99), context-switch rate, CPU migrations, run-queue imbalance, and idle time.
* Baseline workloads and hardware/QEMU configurations for side-by-side comparisons.

### Concurrency and Correctness
Explicitly define:
* Run-queue lock ordering (e.g., per-CPU run-queue lock before task lock; double-lock acquired in ascending CPU ID order).
* Preemption and interrupt assumptions (preemption disabled during critical scheduling paths).
* Per-CPU state protection and memory ordering (atomic memory barriers on context switch).
* Task ownership during context switching. Preemption creates concurrency/reentrancy hazards, including for per-CPU state.

### Resource Control Integration
CPU controllers (cgroup-like quotas and weights) must affect real scheduling paths. CPU quotas, limits, and weights must be accounted for in scheduler run-queue accounting and throttling logic rather than existing solely as an unintegrated model.

### Independent Per-Architecture Validation
A scheduler is not Supported on AArch64 or RISC-V merely because it passes on x86_64. Validate each architecture's:
* Timer setup and APIC/GIC/PLIC calibration.
* Context switch save/restore (registers, stack, address space, FPU/SIMD state).
* Idle loop and interrupt return paths.
* CPU topology and SMP messaging.

### Versioned Bounded Diagnostics
Expose versioned, read-only scheduler counters (`/proc/schedstat` style). Counter formats evolve over time, so interfaces must include explicit versioning (`schedstat_version: 1`) to guarantee diagnostic tools remain stable.

## Subsystem Capability and Status Matrix

| Subsystem Component | Module Location | Status | Current Evidence | Next Milestone |
| :--- | :--- | :--- | :--- | :--- |
| **Scheduler Core** | `src/scheduler/core.rs`, `src/kernel/scheduler.rs` | Prototype | Core run-queue operations & state transitions | PR 2: State-transition & fairness test expansion |
| **CFS Fair Class** | `src/kernel/cfs_scheduler.rs`, `src/scheduler/cfs.rs` | Prototype | Unit tests for vruntime & weight mapping | PR 7: Weighted fair scheduling semantics |
| **BORE Heuristic** | `src/scheduler/bore.rs`, `src/scheduler/bore_engine.rs` | Prototype | Burst score decay & priority boost calculations | PR 9: Optional interactive heuristic for Fair class |
| **Real-Time (FIFO/RR)** | `src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs` | Prototype | FIFO ordering & RR quantum rotation tests | PR 10: RT priority semantics & bandwidth limits |
| **EEVDF Prototype** | `src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs` | Prototype | Eligibility, lag, & virtual deadline calculations | PR 8: EEVDF fair-class prototype & evaluation |
| **x86_64 Context Switch** | `src/arch/x86_64/context.rs` | Prototype | Assembly register save/restore stubs | PR 6: x86_64 context switch & idle loop integration |
| **CPU Controllers** | `src/kernel/cgroup.rs` | Proposed | Model parameters for CPU weights/quotas | PR 13: Scheduler quota enforcement & throttling |
