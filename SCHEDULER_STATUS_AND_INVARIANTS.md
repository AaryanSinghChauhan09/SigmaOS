# SigmaOS CPU Scheduler: Architecture Status & Core Invariants

## Executive Summary
This document defines the architectural status, core rules, task state machine transitions, policy precedence, and concurrency invariants for the SigmaOS CPU Scheduler framework.

The SigmaOS scheduler consists of multiple policy engines (`cfs.rs`, `bore.rs`, `eevdf.rs`, `realtime.rs`) alongside core dispatcher interfaces and CPU run queues. While unit-level algorithms (fairness, burst scoring, FIFO ordering, and RR rotation) pass isolated unit tests, full OS integration requires connecting policy selection to runnable task lifecycles, timer interrupts, context switching, APIC timer calibration, and per-architecture hardware contexts.

---

## Architecture Status Matrix

| Subsystem Component | Scope | Current Status | Supporting Files | Evidence & Notes |
| :--- | :--- | :--- | :--- | :--- |
| **Scheduler Core** | Core run queues, task states, class selection | `Prototype` | `src/scheduler/scheduler.rs`, `src/kernel/scheduler.rs` | Unit-level state machine stubs; run-queue enqueue/dequeue tested in isolation. |
| **CFS Fair Scheduler** | Completely Fair Scheduler, vruntime, nice weights | `Prototype` | `src/kernel/cfs_scheduler.rs`, `src/scheduler/distro_schedulers.rs` | Math and tree placement verified; requires lifecycle & timer preemption integration. |
| **BORE Heuristic** | Burst-Oriented Response Enhancer | `Prototype` | `src/scheduler/bore.rs` | Burst calculation and EMA decay unit-tested; acts as interactive heuristic wrapper. |
| **EEVDF Fair Scheduler** | Earliest Eligible Virtual Deadline First | `Prototype` | `src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs` | Lag calculation, eligibility, and deadline selection unit-tested. |
| **Real-time (FIFO/RR)** | POSIX SCHED_FIFO and SCHED_RR | `Prototype` | `src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs` | Priority array (100 levels) and quantum rotation tested; needs timer interrupt preemption. |
| **x86_64 Architecture** | APIC timer, context switch, Ring 3 entry | `Prototype` | `src/arch/x86_64/context.rs`, `src/kernel/interrupt.rs` | Register save/restore stubs exist; needs QEMU timer calibration and preemption integration. |
| **AArch64 Target** | Generic timer, exception vector context | `Prototype` | `src/arch/aarch64/` | Boot paths compile-only; requires QEMU virt timer & switch validation. |
| **RISC-V Target** | SBI timer, trap handler context | `Prototype` | `src/arch/riscv64/` | Boot paths compile-only; requires QEMU virt SBI timer & switch validation. |

---

## Task States and Valid Transitions

The SigmaOS task lifecycle models explicit task states to prevent race conditions, lost wakeups, or duplicate execution across CPUs:

### 1. Task States
- **`Runnable` (`Ready`)**: Task is enqueued in a scheduler run queue and available for execution on an eligible CPU.
- **`Running`**: Task is currently executing on a CPU core. Owned by `cpu.current_task`.
- **`Blocked` (`Sleeping`)**: Task is waiting on an I/O event, semaphore, mutex, or wait queue. Not in any scheduler run queue.
- **`Stopped`**: Task is suspended by job control or a debugger (e.g., `SIGSTOP`).
- **`Exiting` (`Zombie` / `Terminated`)**: Task has terminated and is awaiting child reaping by its parent or init.

### 2. Transition Rules
- **Enqueue (`Runnable` $\to$ `Running`)**: Triggered by `pick_next_task()`. Task is dequeued from the run queue and assigned to a CPU.
- **Preempt / Yield (`Running` $\to$ `Runnable`)**: Triggered by time-slice expiration or higher-priority task enqueue. Task re-enters run queue.
- **Block (`Running` $\to$ `Blocked`)**: Triggered when a resource is unavailable. Task releases CPU without re-entering run queue.
- **Wakeup (`Blocked` $\to$ `Runnable`)**: Triggered by an interrupt handler or kernel event. Task is placed onto an eligible per-CPU run queue.
- **Exit (`Running` $\to$ `Exiting`)**: Task releases resources, transitions to `Zombie`, and wakes parent.

---

## 9 Core Policy and Concurrency Invariants

1. **Policy Separation**: The scheduler core (`src/kernel/scheduler.rs`) owns run-queue locking, state transitions, timer preemption hooks, and context switching. Policy classes (`CFS`, `EEVDF`, `BORE`, `Realtime`) own task pick ordering and virtual runtime accounting.
2. **Single Queue Ownership**: A task entity must reside on at most one run queue at any moment. Re-enqueuing an already runnable task is a strict kernel panic / bug.
3. **Single CPU Execution**: A task must never execute concurrently on two or more CPU cores.
4. **Strict Policy Precedence Hierarchy**:
   - `SCHED_FIFO` / `SCHED_RR` (Real-time, priorities 0–99) strictly preempt Fair and Idle classes.
   - `SCHED_OTHER` / `SCHED_FAIR` (EEVDF / CFS / BORE) execute when no RT tasks are runnable.
   - `SCHED_IDLE` executes only when no RT or Fair tasks are runnable.
5. **Measurable Fairness Contract**: Under Fair scheduling, relative CPU time share strictly corresponds to nice weights ($w_i$). Sleeper debt accretion/decay is bounded to prevent unbounded CPU hogging upon wakeup.
6. **Real-time Guardrails**: RT tasks operate with explicit priority semantics. Bandwidth throttling caps total RT execution (e.g., maximum 95% CPU time) to prevent kernel starvation.
7. **Concurrency and Lock Ordering**: Per-CPU run-queue locks must be acquired with interrupts disabled (`irqsave`). Under SMP migration, run-queue locks must be acquired in ascending CPU ID order (`rq_lock(min(cpuA, cpuB))` before `rq_lock(max(cpuA, cpuB))`) to prevent deadlocks.
8. **Enforced Resource Limits**: CPU controllers (`cgroup` weights and quotas) must be explicitly accounted for during time-slice updates. Unenforced quotas must not be reported as active.
9. **Independent Architecture Validation**: A policy is marked `Supported` only after booted QEMU/hardware verification passes on that specific target architecture (x86_64, AArch64, RISC-V).
