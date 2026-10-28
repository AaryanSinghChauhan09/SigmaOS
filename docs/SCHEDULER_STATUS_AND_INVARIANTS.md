# SigmaOS CPU Scheduler Status, Architecture & Policy Invariants

## Executive Summary & Subsystem Inventory Audit

SigmaOS provides a multi-policy CPU scheduling architecture designed to support general-purpose workstation, server, real-time, interactive, and custom kernel workloads. The scheduler infrastructure comprises five primary scheduling policies alongside core run-queue management, architecture context-switching hooks, interrupt timer integration, and CPU resource control models.

### Subsystem Architecture Inventory Audit

| Subsystem / Policy | Source Files | Evidence Tier | Architectural Status & Gaps |
|---|---|---|---|
| **Core Scheduler & Run-Queue** | `src/scheduler/scheduler.rs`<br>`src/kernel/scheduler.rs`<br>`src/scheduler/core.rs` | **Prototype** | Provides per-CPU run queues, task state transition logic, and basic lock management. Currently tested at unit level; full integration with real kernel tasks and context switching is pending. |
| **Completely Fair Scheduler (CFS)** | `src/kernel/cfs_scheduler.rs`<br>`src/scheduler/distro_schedulers.rs` | **Prototype** | Implements virtual runtime (`vruntime`) tracking, red-black tree/queue ordering, nice-to-weight conversions, and basic latency target calculations. Lacks group-scheduling accounting and timer interrupt preemption integration. |
| **Burst-Oriented Response Encoder (BORE)** | `src/scheduler/bore.rs`<br>`src/scheduler/bore_engine.rs` | **Prototype** | Implements burst-score tracking with exponential moving average (EMA) decay for boosting interactive tasks. Currently operates as a unit-tested heuristic model; requires explicit policy guardrails to prevent starvation of non-burst workloads. |
| **Real-Time FIFO & Round-Robin (SCHED_FIFO / SCHED_RR)** | `src/scheduler/distro_schedulers.rs`<br>`src/kernel/roundrobin.rs` | **Prototype** | Supports 100 real-time priority levels with bitmap-accelerated $O(1)$ run-queue selection and quantum rotation. Guardrails, priority inheritance/ceiling, and bandwidth limits require measurement under interrupt load. |
| **Earliest Eligible Virtual Deadline First (EEVDF)** | `src/scheduler/eevdf.rs`<br>`src/kernel/sigma_scheduler_eevdf.rs` | **Prototype** | Implements virtual deadline calculation, lag tracking, and eligibility criteria based on Linux EEVDF principles. Operates standalone; requires integration into the primary fair class selection loop. |
| **Extensible Scheduler Framework (SchedExt / BPF)** | `src/scheduler/schedext.rs`<br>`src/scheduler/ebpf_scheduler.rs` | **Prototype** | Exposes dynamic scheduling callbacks for eBPF/custom governors (e.g. gaming, server tuning). Lacks safe fallback handlers and execution boundary protections. |
| **x86_64 Context Switch & APIC Timer** | `src/arch/x86_64/context.rs`<br>`src/kernel/apic.rs` | **Prototype** | Register save/restore and APIC timer tick handlers exist in isolation. APIC calibration in QEMU and scheduler reschedule interrupt returns are outstanding. |
| **AArch64 & RISC-V Architecture Paths** | `src/arch/aarch64/`<br>`src/arch/riscv/` | **Proposed / Prototype** | Context-switch frames, timer drivers, and idle paths exist as compile-checked stubs; unvalidated on live vCPU targets. |
| **CPU Controllers & Resource Controls** | `src/kernel/cgroup.rs` | **Prototype** | Defines CPU weight and quota models; needs full accounting integration into run-queue time-slice distribution. |

---

## Core Scheduling Rules & Invariants

To guarantee deterministic, safe, and starvation-free execution across all supported CPU architectures, all scheduler modifications must satisfy nine strict rules and invariants:

### Rule 1: Policy/Mechanism Separation
- **Invariant:** The scheduler core owns mechanism (run-queue lock ordering, task-state machine, timer accounting, context-switch hooks, and CPU selection interfaces). Scheduling classes own policy decisions (task selection, virtual runtime/deadline updates, priority evaluation).
- **Rule:** A hybrid scheduling policy (e.g., combining CFS, BORE, or EEVDF) must never blur class boundaries. Scheduling decisions must follow strict policy precedence:
  1. **Real-Time Class:** `SCHED_FIFO` / `SCHED_RR` (highest precedence, strictly prioritized over fair work).
  2. **Fair Class:** `SCHED_NORMAL` / `SCHED_BATCH` (EEVDF or weighted fair policy with optional BORE interactive heuristics).
  3. **Idle Class:** `SCHED_IDLE` (lowest precedence, scheduled only when no runnable RT or Fair tasks exist).

### Rule 2: Explicit Task State Machine
- **Invariant:** Every task in the system resides in exactly one explicit state: `New`, `Runnable`, `Running`, `Blocked`, `Stopped`, `Exiting`, or `Idle`.
- **Transitions:**
  - `New` $\rightarrow$ `Runnable`: Task initialization complete; enqueued into CPU run queue.
  - `Runnable` $\rightarrow$ `Running`: Selected by scheduler core for CPU execution.
  - `Running` $\rightarrow$ `Runnable`: Preempted by timer interrupt or voluntary yield.
  - `Running` $\rightarrow$ `Blocked`: Task waits on I/O, IPC, futex, or sleep timer.
  - `Blocked` $\rightarrow$ `Runnable`: Wakeup event occurs; task re-enqueued.
  - `Running` $\rightarrow$ `Stopped`: Received stop signal (`SIGSTOP`/`SIGTSTP`).
  - `Stopped` $\rightarrow$ `Runnable`: Received continue signal (`SIGCONT`).
  - `Running` $\rightarrow$ `Exiting`: Task invokes exit or receives fatal signal; resources pending cleanup.
- **Queue Ownership:** A task must exist on at most one CPU run queue at any moment and must never be assigned to run on two CPUs simultaneously.

### Rule 3: Policy/Mechanism Separation & Measurable Fair Scheduling Contract
- **Invariant:** The fair scheduling policy must enforce proportional CPU allocation based on task weight (derived from nice values or cgroup weights).
- **Contract:** Relative CPU share for task $i$ with weight $w_i$ among runnable tasks in set $R$ is given by:
  $$\text{Share}_i = \frac{w_i}{\sum_{j \in R} w_j}$$
- **Sleeper Debt & Lag Bounding:** Sleeping tasks accruing fairness debt (lag) must be bounded upon wakeup to prevent CPU starvation of long-running tasks. EEVDF virtual deadlines and eligibility rules serve as the target reference model.

### Rule 4: Real-Time Policy Guardrails
- **Invariant:** `SCHED_FIFO` tasks run until they yield, block, or are preempted by a higher-priority RT task. `SCHED_RR` tasks rotate within equal priority levels upon quantum expiration.
- **Guardrails:**
  - Real-time priority escalation requires explicit administrative capabilities (`CAP_SYS_NICE`).
  - Real-time bandwidth throttling must reserve a configurable slice (default 5%) for non-RT administrative tasks to prevent unrecoverable kernel lockups.
  - Worst-case interrupt-to-run latency must be bounded and measured under load.

### Rule 5: Data-Driven Scheduler Decisions
- **Invariant:** All scheduler optimizations, tuning changes, or heuristic additions must be backed by quantitative benchmark data across baseline workloads.
- **Tracked Metrics:** Throughput, fairness index (Jain's Fairness Index), wake-to-run latency (median and 99th percentile), context-switch frequency, per-CPU migration rate, run-queue imbalance ratio, and CPU idle time.

### Rule 6: Concurrency and Reentrancy Correctness
- **Invariant:** Run-queue locking must follow strict hierarchical lock ordering to prevent deadlocks during task migration or cross-CPU wakeups.
- **Preemption & Interrupt Rules:** Context switches must never occur while holding spinlocks or inside hard-interrupt handlers unless explicitly executing on a dedicated interrupt-return boundary. Per-CPU scheduler state must be protected against reentrant interrupt execution.

### Rule 7: Enforced Resource Controls
- **Invariant:** CPU controllers (`cgroups`) must enforce configured weights and hard quota limits during runtime scheduling accounting.
- **Rule:** Unenforced or stubbed resource limits must be marked as `Prototype` or `Proposed` in system telemetry and must not be reported as `Supported`.

### Rule 8: Per-Architecture Status Isolation
- **Invariant:** Feature status for CPU scheduling is evaluated independently per CPU architecture (`x86_64`, `AArch64`, `RISC-V`).
- **Rule:** Validating scheduler behavior on `x86_64` QEMU does not grant `Supported` status to `AArch64` or `RISC-V`. Each architecture must independently pass timer preemption, context-switching, idle loop, and interrupt return verification.

### Rule 9: Bounded & Stable Diagnostics
- **Invariant:** Scheduler counters and statistics (e.g. `/proc/schedstat` or versioned kernel IPC interfaces) must be read-only, non-allocating, and versioned.
- **Rule:** Counter updates must use atomic or per-CPU lockless operations to avoid adding contention to primary scheduling paths.

---

## Evidence Tiers & Capability Matrix

| Feature / Subsystem | Capability Status | Current Evidence Level | Target Verification Gate |
|---|---|---|---|
| Task State Machine | Integrated | Unit Tests (`tests/scheduler/core.rs`) | Booted QEMU multi-task execution |
| CFS Fair Class | Prototype | Unit Tests (`tests/scheduler/fairness/`) | Bounded CPU share under load |
| BORE Interactive Boosting | Prototype | Model Tests (`src/scheduler/bore.rs`) | Latency benchmark comparison |
| Real-Time FIFO / RR | Prototype | Unit Tests (`src/scheduler/distro_schedulers.rs`) | Hard-IRQ preemption measurement |
| EEVDF Deadline Class | Prototype | Algorithm Tests (`src/scheduler/eevdf.rs`) | Single-CPU QEMU fair selection |
| x86_64 Preemptive Switch | Prototype | Unit Tests (`tests/scheduler/context_switch/`) | Booted Ring 3 multi-task execution |
| SMP Load Balancing | Prototype | Unit Tests (`tests/scheduler/affinity/`) | Multi-vCPU QEMU load distribution |
| CPU Controller Limits | Proposed | Model Definition (`src/kernel/cgroup.rs`) | Quota throttling integration |
