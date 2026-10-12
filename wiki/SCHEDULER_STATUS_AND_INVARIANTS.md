# SigmaOS CPU Scheduler Architecture, Audit Status, and System Invariants

## Executive Summary & Scope

This document establishes the official architecture, component audit status, design invariants, policy rules, and multi-architecture roadmap for the SigmaOS CPU Scheduler subsystem.

SigmaOS scheduling algorithms—including Completely Fair Scheduler (CFS), Burst-Oriented Response Enhancer (BORE), Earliest Eligible Virtual Deadline First (EEVDF), and Real-Time (SCHED_FIFO / SCHED_RR)—are audited and categorized according to strict runtime evidence completion gates: **Proposed**, **Prototype**, **Integrated**, or **Supported**.

> **Crucial Rule:** Standalone unit tests and simulated clock models prove algorithm correctness in isolation, but do **not** constitute proof of runtime scheduling until connected to real kernel task lifecycles, timer preemption interrupts, context switching, and APIC/GIC/CLINT timers.

---

## 1. Scheduler Component Audit & Completion Status Matrix

| Subsystem Component | Primary Implementation File(s) | Status | Evidence & Test Coverage | Notes & Architectural Gaps |
| :--- | :--- | :--- | :--- | :--- |
| **CFS Fair Class** | `src/kernel/cfs_scheduler.rs`, `src/scheduler/distro_schedulers.rs` | **Prototype** | Unit tests for `vruntime`, weight table, `sched_entity` | Algorithm model only; lacks task lifecycle and timer tick preemption integration. |
| **BORE Heuristic** | `src/scheduler/bore.rs` | **Prototype** | Unit tests for burst score EMA decay, penalty score, and interactivity boost | Operates on simulated time; needs integration with Fair class preemption logic. |
| **Real-Time Policy** | `src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs` | **Prototype** | Unit tests for priority ordering (0-99), bitmap pick, and RR quantum rotation | Algorithm model; needs privilege checks, bandwidth throttling, and interrupt return preemption. |
| **EEVDF Fair Class** | `src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs` | **Prototype** | Unit tests for virtual time, lag calculation, eligibility, and deadline selection | Standalone prototype; scheduled for Fair class default evaluation in PR 8. |
| **SchedExt Framework** | `src/scheduler/schedext.rs` | **Prototype** | Unit tests for eBPF extension callbacks and dispatch queues | Model framework; needs kernel eBPF runtime integration. |
| **Scheduler Core** | `src/scheduler/scheduler.rs`, `src/kernel/scheduler.rs` | **Prototype** | Basic run-queue structures and process state tracking | Core run-queue lock and class dispatch interface definition required (PR 3/4). |
| **Context Switch** | `src/arch/x86_64/context.rs` | **Prototype** | Assembly register switch routines (`__switch_to`) | x86_64 context switch routines exist but require full integration with scheduler run queues and APIC timer ticks. |
| **Timer Preemption & APIC** | `src/timer/timer.rs`, `src/interrupt/apic_driver.rs` | **Prototype** | APIC timer configuration and tick unit tests | APIC calibration in QEMU and connection to reschedule requests need wiring (PR 5/6). |
| **Task Lifecycle & States** | `src/scheduler/process.rs`, `src/kernel/process.rs` | **Prototype** | Task state enumeration and state machine stubs | Needs explicit enqueue/dequeue hooks on task fork, block, yield, wakeup, and exit (PR 4). |
| **CPU Controllers & Quotas** | `src/process/sovereign_scheduler_governor.rs` | **Prototype** | Model stubs for group CPU weights and bandwidth limits | Quotas are not yet enforced in scheduler tick accounting. |
| **x86_64 Architecture Path** | `src/arch/x86_64/` | **Prototype** | Boot & context switch assembly | Active target for initial runtime integration (PR 6). |
| **AArch64 Architecture Path**| `src/arch/aarch64/` | **Proposed** | Early compile-only stubs | Unvalidated in QEMU runtime; requires GIC timer and context switch bringup (PR 17). |
| **RISC-V Architecture Path**  | `src/arch/riscv/` | **Proposed** | Early compile-only stubs | Unvalidated in QEMU runtime; requires CLINT/PLIC timer and context switch bringup (PR 17). |

---

## 2. Policy Precedence and Separation Rules

To preserve architectural clarity and maintainability, SigmaOS strictly enforces the **separation of policy from mechanism**:

1. **Mechanism Ownership:** The scheduler core (`src/scheduler/scheduler.rs`) owns:
   - Run-queue locking and memory ordering.
   - Task state machine transitions (`Runnable` <-> `Running` <-> `Blocked`).
   - Per-CPU state protection and context-switch dispatch.
   - Timer tick routing and reschedule flags (`need_resched`).

2. **Policy Ownership:** Scheduling classes implement specific policy logic via a standardized class interface (`SchedulingClass`):
   - **Class Precedence (Highest to Lowest):**
     1. **Real-Time Class (`SCHED_FIFO` / `SCHED_RR`):** Priority levels 0..99. Real-time tasks strictly preempt any non-real-time task.
     2. **Fair Class (`SCHED_OTHER` / EEVDF / CFS / BORE):** Shares CPU proportionally based on weight, virtual deadlines, and interactivity boost.
     3. **Idle Class (`SCHED_IDLE`):** Executes only when no Real-Time or Fair tasks are runnable.

3. **Precedence Invariant:** A lower-precedence class is evaluated **only** when higher-precedence run queues are completely empty. Hybrid policy hooks (such as BORE burst scoring) operate strictly **within** the Fair class and must never override Real-Time precedence or violate Fair class starvation bounds.

---

## 3. Task States and Transition Invariants

### 3.1 Task State Machine
The core scheduler tracks tasks across six canonical states:
- `Runnable`: Task is queued in a run-queue and eligible for CPU execution.
- `Running`: Task is actively executing on a CPU core.
- `Blocked`: Task is waiting for an I/O event, timer, lock, or IPC signal.
- `Stopped`: Task is suspended by a signal (`SIGSTOP`/`SIGTSTP`) or debugger.
- `Exiting`: Task is terminating and freeing resources; zombie state until reaped.
- `Idle`: Per-CPU idle loop state when no runnable tasks exist on the CPU.

### 3.2 Core State Invariants
1. **Single Queue Ownership:** A task must reside on at most **one** run-queue at any given instant. Duplicate queueing is a fatal kernel panic condition.
2. **Single CPU Execution:** A task must **never** execute on two CPUs simultaneously. CPU migration requires removing the task from the old CPU's run-queue, waiting for context-switch completion, updating `task.cpu`, and enqueuing onto the new CPU's run-queue under appropriate lock protection.
3. **Atomic State Transitions:** All state transitions (`enqueue`, `dequeue`, `block`, `wakeup`, `yield`, `exit`) must be performed under the protection of the run-queue spinlock with local interrupts disabled (`spin_lock_irqsave`).

---

## 4. Fair Scheduling Contract

The Fair scheduling policy (CFS / EEVDF reference) guarantees predictable CPU share allocation and bounded wakeup latency:

### 4.1 Weight & Virtual Time Mapping
- `nice` values range from `-20` (highest share, weight `88761`) to `+19` (lowest share, weight `15`). Nice `0` corresponds to standard weight `1024`.
- Virtual runtime (`vruntime`) advances according to allocated weight:
  $$\Delta \text{vruntime} = \Delta \text{exec\_time} \times \frac{\text{NICE\_0\_LOAD}}{\text{task\_weight}}$$

### 4.2 Virtual Deadlines & Lag (EEVDF)
- Tasks request a time slice $q$ (default 6ms - 10ms) adjusted by latency sensitivity (`latency_nice`).
- Virtual deadline $V$ is computed as:
  $$V = S + \frac{q}{w}$$
  where $S$ is virtual start time and $w$ is weight.
- Task eligibility requires virtual lag $L \ge 0$, where $L = V_{\text{avg}} - vruntime$.

### 4.3 Sleeper Debt & Decay Controls
- Sleeping tasks accruing debt upon sleep must not gain an unbounded CPU advantage upon wakeup.
- When a task wakes up after sleeping, its `vruntime` is normalized to:
  $$\text{vruntime\_wakeup} = \max(\text{vruntime\_old}, \text{min\_vruntime} - \Delta \text{max\_lag})$$
  where $\Delta \text{max\_lag}$ is capped (e.g., 10ms) to prevent sleeper abuse.

---

## 5. Real-Time Policy Guardrails

1. **Priority Semantics:** Real-Time priorities range from 0 to 99 (where 0 is highest RT priority). Higher priority RT tasks unconditionally preempt lower priority RT tasks.
2. **Policy Behavior:**
   - `SCHED_FIFO`: Executes until it blocks, yields, or is preempted by a higher priority RT task.
   - `SCHED_RR`: Executes until quantum expiration (default 10ms), after which it moves to the tail of its priority queue.
3. **Privilege Enforcement:** Unprivileged processes cannot change scheduling policy to `SCHED_FIFO` or `SCHED_RR` or increase RT priority without `CAP_SYS_NICE`.
4. **RT Bandwidth Throttling:** To prevent CPU starvation of critical system services during runaway RT loops, Real-Time tasks are throttled to a maximum of **95% CPU time** over a 1-second period, reserving at least 5% CPU time for Fair/system tasks.

---

## 6. Concurrency, Locking & Preemption Invariants

1. **Lock Hierarchy & Ordering:**
   - When acquiring multiple locks (e.g., during load balancing or migration):
     $$\text{Lock Address Ordering: } \text{Lock}(\text{CPU}_A) < \text{Lock}(\text{CPU}_B) \text{ where } A < B$$
   - Prevents deadlock during concurrent cross-CPU migrations.
2. **Interrupt & Preemption Context Rules:**
   - Run-queue locks must be acquired with interrupts disabled (`spin_lock_irqsave`) to prevent deadlock from timer interrupt reentrancy.
   - Preemption is disabled (`preempt_disable()`) during context switching, per-CPU variable access, and critical kernel sections.
3. **Reschedule Execution Points:**
   - Timer ticks evaluate reschedule conditions and set the per-thread `need_resched` flag.
   - Preemption occurs at explicit reschedule points: returning from interrupts/syscalls to user mode (`ret_to_user`), kernel preemption points (`preempt_enable()`), or explicit `yield()` calls.

---

## 7. Resource Controls & CPU Controllers

1. **Cgroup / Governor Integration:**
   - CPU shares (`cpu.weight`) modify the task weight in Fair scheduling calculations.
   - CPU quotas (`cpu.max = $quota $period`) enforce hard time limits on process groups.
2. **Enforcement Rule:** CPU controllers and quotas must **not** be advertised as Supported until actual accounting logic in the scheduler tick path throttles non-compliant task groups.

---

## 8. Multi-Architecture Support Matrix

| Architecture | Timer Source | Context Switch Path | Idle Loop | Target Status |
| :--- | :--- | :--- | :--- | :--- |
| **x86_64** | Local APIC Timer | `src/arch/x86_64/context.rs` | `hlt` with interrupt enable | **Prototype** (Path to Integrated in PR 6) |
| **AArch64** | ARM Generic Timer | `src/arch/aarch64/context.rs` | `wfi` | **Proposed** (Bringup in PR 17) |
| **RISC-V** | RISC-V CLINT/Sstc | `src/arch/riscv/context.rs` | `wfi` | **Proposed** (Bringup in PR 17) |

---

## 9. Diagnostics & Versioned Scheduler Statistics (`schedstats`)

Scheduler statistics are exposed through a versioned interface (`/proc/schedstat` schema v1) to ensure telemetry tools maintain backward compatibility:

### Versioned Counter Schema (v1)
- `version`: Protocol version integer (`1`).
- `timestamp`: System uptime in nanoseconds.
- **Per-CPU Statistics:**
  - `cpu_id`: Core identifier.
  - `yield_count`: Total voluntary yield operations.
  - `sched_count`: Total schedule calls.
  - `sched_goidle`: Total transitions to idle state.
  - `ttwu_count`: Total try-to-wake-up calls.
  - `ttwu_local`: Try-to-wake-up calls on local CPU.
- **Per-Task Statistics:**
  - `exec_time_ns`: Cumulative CPU execution time in nanoseconds.
  - `wait_time_ns`: Time spent waiting in runnable state.
  - `involuntary_switches`: Preemption switch count.
  - `voluntary_switches`: Block/yield switch count.

---

## 10. Proposed PR Sequence & Completion Gates

### 10.1 18-PR Sequence

| PR | Proposed Title | Scope Summary | Acceptance Criteria |
| :---: | :--- | :--- | :--- |
| **1** | `docs(scheduler): audit scheduler status and define invariants` | Audit cfs.rs, bore.rs, realtime.rs, scheduler core, context switch, timers, APIC, task states, CPU controllers, and per-architecture paths. Document policy precedence, states, fairness goals, and test levels. | Every scheduler component is marked Proposed, Prototype, Integrated, or Supported with evidence; unit/model results are not described as runtime scheduling. |
| **2** | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration. Add deterministic simulated-clock tests. | Invalid transitions are rejected; task/run-queue invariants are checked; fairness tests report measurable shares and bounded error under defined workloads. |
| **3** | `refactor(scheduler): define class interface and single policy selection path` | Introduce an explicit scheduler-class interface and policy ordering for idle, fair, and real-time classes. Make selection and lifecycle callbacks consistent. | Each runnable task is owned by one class; class precedence is tested; adding a class does not duplicate task lifecycle logic. |
| **4** | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Real kernel tasks are enqueued/dequeued exactly once; no lost wakeups, stale queue entries, or double-running tasks in stress tests. |
| **5** | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire architecture timer interrupts to accounting, time-slice expiration, reschedule requests, and safe return-to-user scheduling points. | QEMU test proves two CPU-bound user tasks alternate according to policy; timer interrupts do not corrupt context; idle CPUs wake correctly. |
| **6** | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, scheduler run queues, and idle path. Validate FPU/SIMD state and stack/address-space switching. | Booted x86_64 QEMU runs multiple Ring 3 tasks; preemption and block/wakeup work; context state is preserved; idle does not miss a reschedule interrupt. |
| **7** | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping and accounting for the existing CFS-inspired implementation; document fairness and nice-like behavior. | Long-running equal-weight tasks converge to equal CPU share within a stated tolerance; weighted tasks receive defined proportional shares; runnable tasks do not starve. |
| **8** | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF-style eligibility, lag, virtual deadlines, requested slices, and sleeper accounting as a separate class or behind a development feature flag. | Deterministic tests cover eligibility, earliest deadline selection, sleeper abuse, wakeup preemption, and weight changes; benchmarks compare to existing fair policy. |
| **9** | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define exactly how BORE burst scoring influences the fair class; prevent it from overriding fairness bounds or real-time precedence. | Interactive latency improves on named workloads without violating starvation/fairness thresholds; hostile yield/sleep patterns cannot gain unbounded CPU advantage. |
| **10** | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate existing real-time FIFO/RR policy with task priorities, preemption, time quanta, and permissions. | FIFO runs until block/yield/preemption by higher-priority work; RR rotates peers at the configured quantum; unprivileged priority escalation is denied. |
| **11** | `feat(scheduler): add CPU affinity and per-CPU run queues` | Add affinity masks, CPU selection, per-CPU queues, and migration rules after SMP context switching is stable. | Tasks never run outside their affinity; migration preserves queue/accounting invariants; idle CPU balancing is tested; single-CPU mode remains correct. |
| **12** | `feat(scheduler): add SMP load balancing and topology model` | Represent CPU/core/package topology; implement idle-pull and/or periodic balancing; account for cache affinity and migration cost. | Multi-vCPU QEMU tests verify progress, balance, and bounded migration; no run-queue lock inversion; topology fallback works when firmware data is absent. |
| **13** | `feat(scheduler): enforce CPU controller weights and quotas` | Connect cgroup-like CPU weights/limits from process creation through scheduler accounting and throttling. | Quotas limit actual CPU time in QEMU tests; child groups inherit constraints; configuration changes are synchronized; unimplemented controller states are rejected. |
| **14** | `feat(scheduler): add versioned scheduler statistics interface` | Add counters for runtime, run-queue wait, wake-to-run latency, switches, migrations, balance attempts, and policy class. | Counters are monotonic or have explicitly defined reset semantics; interface is versioned; tests cover overflow and concurrent reads. |
| **15** | `test(scheduler): add benchmark and regression suite` | Add CPU-bound, interactive, mixed-weight, process/thread creation, IPC, wakeup-latency, and oversubscription workloads. | Each result records kernel revision, architecture, vCPU count, timer frequency, configuration, workload, repetitions, and percentiles. |
| **16** | `test(scheduler): add race, stress, and failure-injection coverage` | Exercise concurrent wakeups, task exit during migration, timer races, priority changes, CPU offline/online, and resource exhaustion. | No duplicate execution, lost wakeup, use-after-free, deadlock, or corrupted run queue; stress tests are repeatable and time bounded. |
| **17** | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context-switch, idle, and interrupt-return paths per architecture in separate PRs. | Each target boots to multiple user tasks and passes core scheduling tests; per-architecture status is independently reported. |
| **18** | `docs(scheduler): publish policy guide and support matrix` | Document scheduler classes, weights/priorities, limits, tuning knobs, observability, architecture support, and known limitations. | Every public policy claim links to implementation and runtime tests; experimental policies are labeled; no Linux/BSD parity is implied. |

---

### 10.2 Completion Gate Definitions

1. **Proposed:** Specification, interface definitions, and design documentation complete. No functional implementation claimed.
2. **Prototype:** Policy algorithms and standalone unit models exist and pass offline test suites. Not wired to runtime kernel task dispatch, context switching, or hardware timer preemption.
3. **Integrated:** Connected to live kernel tasks on a booted target. Timer interrupts trigger preemption, context switching executes safely between processes/threads, and block/wakeup state transitions operate on live hardware/QEMU.
4. **Supported:** Passes comprehensive regression, stress, SMP, multi-architecture, and workload fairness test suites under continuous integration. Documented with stable production interfaces.

---

### 10.3 Periodic Maintenance Cadence

- **Every PR:** `ci(scheduler)` runs unit tests, architecture checks, and formatting verification.
- **Weekly:** `maint(scheduler)` triages failures and regressions; reviews fairness and wakeup latency trends.
- **Monthly:** `audit(scheduler)` verifies task state and run-queue invariants across touched paths; executes booted single-CPU and SMP scheduling test matrix.
- **Quarterly:** `maint(scheduler)` reviews policy semantics, starvation bounds, and refreshes performance benchmark baselines.
- **Release Candidate:** `release(scheduler)` certifies architecture support claims with booted-task evidence.
- **Incident Response:** `fix(scheduler)` delivers minimal reproducers, root cause analyses, and regression tests for concurrency or starvation issues.

---

### 10.4 Standard Pull Request Template

```markdown
## Summary
[Summary of scheduler behavior, correctness gap, or performance issue addressed]

## Scope
Core | Fair class | BORE | RT | SMP | CPU controller | Architecture hook
[Explicitly state out-of-scope items]

## Policy and Invariants
[Task states, queue ownership, fairness/priority semantics, lock ordering, preemption rules]

## Evidence
Unit/model | Booted QEMU single CPU | Booted QEMU SMP | Physical hardware
[Exact architecture, vCPU topology, timer configuration, and workload]

## Performance Impact
[Baseline vs new throughput, median/tail wakeup latency, fairness/share, context switches, migrations]

## Status and Documentation
Proposed | Prototype | Integrated | Supported
[Link updated feature matrix, scheduler docs, and test results]
```
