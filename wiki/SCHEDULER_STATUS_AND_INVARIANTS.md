# SigmaOS CPU Scheduler Architecture, Invariants, and Development Plan

## Executive Summary & Status Audit

SigmaOS implements a multi-class CPU scheduling subsystem. While individual scheduling algorithms and policy heuristics exist within the codebase, their current evidence level is unit/model testing. To transition from isolated scheduling models to a fully integrated production kernel scheduler, policy decisions must be explicitly decoupled from dispatch mechanism, and the core scheduler must be wired directly to real task lifecycles, timer interrupts, and architecture-specific context switching.

### Current Implementation Inventory

| Scheduler Module | Primary Files | Status | Test / Evidence Level | Next Architectural Milestone |
| :--- | :--- | :--- | :--- | :--- |
| **Scheduler Core** | `src/scheduler/scheduler.rs`<br>`src/kernel/scheduler.rs` | Prototype | Unit tests for run queues & state machine stubs | PR 3/4: Define class interface & task lifecycle integration |
| **CFS (Completely Fair Scheduler)** | `src/kernel/cfs_scheduler.rs`<br>`src/scheduler/distro_schedulers.rs` | Prototype | Unit tests for vruntime & nice weights | PR 7: Implement weighted fair scheduling semantics |
| **BORE (Burst-Oriented Response Enhancer)** | `src/scheduler/bore.rs` | Prototype | Unit tests for burst scoring & EMA decay | PR 9: Integrate as optional interactive heuristic |
| **Real-Time (FIFO & Round-Robin)** | `src/scheduler/distro_schedulers.rs`<br>`src/kernel/roundrobin.rs` | Prototype | Unit tests for FIFO order & RR rotation | PR 10: Enforce RT priority levels & bandwidth guardrails |
| **EEVDF (Earliest Eligible Virtual Deadline First)** | `src/scheduler/eevdf.rs`<br>`src/kernel/sigma_scheduler_eevdf.rs` | Prototype | Unit tests for lag, eligibility & virtual deadlines | PR 8: Implement EEVDF fair-class prototype |
| **SchedExt (Extensible BPF Scheduler)** | `src/scheduler/schedext.rs` | Prototype | Model tests for BPF dispatcher callbacks | PR 18: Document SchedExt interface & policy bounds |

### Core Architectural Gaps

1. **Task Lifecycle Integration**: Runnable tasks must be enqueued and dequeued on real process/thread state changes (spawn, block, wakeup, yield, terminate) without double-running or lost wakeups.
2. **Timer Preemption & Slice Accounting**: Architecture timer interrupts (x86_64 APIC, AArch64 Generic Timer, RISC-V Timer) must drive time-slice consumption, vruntime advancement, and set reschedule flags (`need_resched`).
3. **Context Switch & Idle Loop Wiring**: Transitioning between tasks requires register context saving/restoring, address space switching (`CR3` / `TTBR0`), FPU/SIMD state preservation, and safe idle loop (`hlt` / `wfi`) execution.
4. **Per-Architecture Runtime Validation**: x86_64 QEMU context switching and APIC timer calibration require live verification, while AArch64 and RISC-V boot targets must be validated beyond compile checks.

---

## Design and Reference Frameworks

* **Linux EEVDF Reference**: Linux's EEVDF (Earliest Eligible Virtual Deadline First) scheduler replaces traditional CFS heuristic latency tuning with mathematical virtual runtime/lag tracking and virtual deadlines. Tasks requiring low latency request shorter virtual time slices, allowing them to be scheduled earlier when eligible without exceeding their fair CPU share.
* **FreeBSD ULE & Multi-Core Workload Reference**: FreeBSD's scheduler design emphasizes CPU affinity, per-CPU run queues, interactive vs. non-interactive task classification, hybrid CPU/core topology awareness, and deterministic lock ordering to minimize cache thrashing and migration overhead.

---

## Scheduler Policy and Invariants

Adopt the following mandatory rules before modifying scheduling behavior or adding new policy algorithms:

1. **Separate Policy from Mechanism**: The scheduler core owns run-queue locking, task-state transitions, timer interactions, context-switch hooks, and CPU selection interfaces. Scheduling classes (Idle, Fair, Real-Time, Deadline) own policy decisions. Avoid hybrid policies until class boundaries and precedence rules are explicit.
2. **Define Task States and Transitions**: Document `Runnable`, `Running`, `Blocked`, `Stopped`, `Exiting`, and `Idle` states with valid enqueue, dequeue, wakeup, and preemption transitions. A task must reside on at most one run queue and must never execute on two CPUs simultaneously.
3. **Fair Scheduling Contract**: Specify relative CPU share by weight, wakeup latency targets, starvation bounds, and how sleeping tasks accrue or decay fairness debt (lag).
4. **Real-Time Guardrails**: FIFO/RR tasks must have explicit priority semantics (0..99), time quantum behavior, privilege checking, and bandwidth throttling to prevent system lockup. Real-time claims require measured interrupt, timer, preemption, and worst-case latency paths.
5. **Data-Driven Evaluation**: Record throughput, fairness metrics, wake-to-run latency, context-switch rate, CPU migrations, run-queue imbalance, and idle percentage. Preserve baseline workloads and QEMU configuration for comparisons.
6. **Concurrency Correctness**: Enforce strict run-queue lock ordering, explicit preemption/interrupt disable rules, per-CPU state protection, memory ordering, and context-switch ownership. Preemption creates reentrancy hazards on per-CPU state.
7. **Resource Control Enforcement**: CPU controllers (cgroups/quotas) must actively constrain run-queue allocation and slice distribution before quotas are advertised as enforced.
8. **Separate Architecture Status**: A scheduler is not `Supported` on AArch64 or RISC-V merely because it passes tests on x86_64; validate each target's timer, context switch, idle loop, interrupt return, and CPU topology independently.
9. **Bounded & Stable Diagnostics**: Expose versioned scheduler counters (`schedstats`) through a read-only interface, accounting for counter format evolution over time.

---

## 18-PR Development Roadmap

| PR | Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **PR 1** | `docs(scheduler): audit scheduler status and define invariants` | Audit `cfs.rs`, `bore.rs`, `realtime.rs`, core scheduler, context switch, timers, APIC, task states, CPU controllers, and arch paths. | Every scheduler component is marked Proposed, Prototype, Integrated, or Supported with evidence. |
| **PR 2** | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration. | Invalid state transitions rejected; task/run-queue invariants verified; fairness tests report bounded error under simulated clock. |
| **PR 3** | `refactor(scheduler): define class interface and single policy selection path` | Introduce explicit `SchedClass` interface and policy ordering (Deadline > Real-Time > Fair > Idle). | Each runnable task is owned by exactly one class; precedence hierarchy is verified. |
| **PR 4** | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Kernel tasks enqueued/dequeued exactly once; no lost wakeups or double-running tasks under stress. |
| **PR 5** | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire APIC / Generic Timer interrupts to accounting, slice expiration, reschedule flags, and return-to-user scheduling points. | Booted QEMU test proves two CPU-bound Ring 3 tasks alternate according to policy; context state preserved across timer preemption. |
| **PR 6** | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, scheduler run queues, and idle path (`hlt`). | Booted x86_64 QEMU executes Ring 3 tasks; preemption and block/wakeup function cleanly; FPU/SIMD state preserved. |
| **PR 7** | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping (nice levels -20..19) and vruntime accounting for CFS policy. | Equal-weight tasks converge to equal CPU share within target tolerance; weighted tasks receive proportional CPU shares. |
| **PR 8** | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF virtual deadline, lag, eligibility, requested slice, and sleeper accounting. | Deterministic tests cover eligibility, deadline selection, sleeper debt decay, and weight changes; benchmark compared against CFS. |
| **PR 9** | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define BORE burst scoring influence on fair class; prevent overriding real-time or fairness bounds. | Interactive task latency improves on named workloads without starving CPU-bound tasks; hostile yield patterns bounded. |
| **PR 10** | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate real-time FIFO/RR policy with priority levels (0..99), preemption, quantum rotation, and permissions. | SCHED_FIFO runs until block/yield; SCHED_RR rotates peers at quantum expiration; unprivileged escalation denied. |
| **PR 11** | `feat(scheduler): add CPU affinity and per-CPU run queues` | Implement per-CPU run queues, affinity bitmasks, CPU selection, and migration rules. | Tasks never run outside affinity mask; migration preserves queue accounting invariants; single-CPU mode remains correct. |
| **PR 12** | `feat(scheduler): add SMP load balancing and topology model` | Model NUMA/core/package topology; implement idle-pull and periodic load balancing. | Multi-vCPU QEMU verifies load distribution; lock ordering prevents deadlocks; firmware topology fallback verified. |
| **PR 13** | `feat(scheduler): enforce CPU controller weights and quotas` | Connect CPU cgroup weights and bandwidth limits (`quota` / `period`) to scheduler accounting. | Quotas limit actual CPU execution time in QEMU; child groups inherit constraints; invalid configs rejected. |
| **PR 14** | `feat(scheduler): add versioned scheduler statistics interface` | Expose per-CPU and per-task runtime, wait time, switch count, migration count, and latency metrics. | Counters are monotonic; interface is explicitly versioned; concurrent reads verified. |
| **PR 15** | `test(scheduler): add benchmark and regression suite` | Workloads for CPU-bound, interactive, mixed-weight, thread creation, IPC latency, and oversubscription. | Results capture revision, arch, vCPU count, timer frequency, workload, repetitions, and percentiles. |
| **PR 16** | `test(scheduler): add race, stress, and failure-injection coverage` | Stress wakeups, exit during migration, timer races, priority mutation, and resource exhaustion. | Zero duplicate execution, lost wakeups, double-frees, or run-queue corruption under high-stress fuzzing. |
| **PR 17** | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context switch, idle (`wfi`), and interrupt return paths for AArch64 and RISC-V. | AArch64 QEMU (`virt`) and RISC-V QEMU (`virt`) boot and execute multi-task scheduling tests successfully. |
| **PR 18** | `docs(scheduler): publish policy guide and support matrix` | Document scheduling classes, weights/priorities, tuning parameters, observability, and arch support matrix. | Public policy claims backed by runtime evidence; experimental features clearly labeled. |

> **Recommended Initial Milestone**: Complete **PRs 1–7** (Core infrastructure, deterministic tests, class interface, task lifecycle, timer ticks, x86_64 context switching, and weighted fair semantics) before deciding whether EEVDF or BORE becomes the default fair scheduling policy.

---

## Periodic Maintenance PR Cadence

| Cadence | Task | Required Evidence |
| :--- | :--- | :--- |
| **Every PR** | `ci(scheduler): run scheduler checks for changed paths` | Formatting/build, scheduler unit tests, arch checks, QEMU integration where applicable. |
| **Weekly** | `maint(scheduler): triage scheduler failures and regressions` | Reproducer, affected policy/arch, severity, owner, fix or tracked issue. |
| **Weekly** | `test(scheduler): review fairness and wakeup-latency trends` | Baseline benchmark comparison; flag tail latency, fairness, context switch, or migration regressions. |
| **Monthly** | `audit(scheduler): verify state and run-queue invariants` | Code review of all task state transitions, enqueue/dequeue, migration, timer, and exit paths touched during month. |
| **Monthly** | `test(scheduler): run booted single-CPU scheduling matrix` | CPU-bound, sleep/wakeup, mixed priority, real-time, task exit, and idle wakeup tests on named QEMU target. |
| **Monthly** | `test(scheduler): run SMP and affinity matrix` | Verify CPU pinning, migration, idle pull, fairness, and lock behavior on multi-vCPU topology. |
| **Quarterly** | `maint(scheduler): review policy semantics and starvation bounds` | Review fair-class behavior, BORE heuristic, RT starvation protections, and CPU controllers. |
| **Quarterly** | `perf(scheduler): refresh benchmark baselines` | Re-run fixed workloads with multiple repetitions and latency percentiles; record revision & environment metadata. |
| **Release Candidate** | `release(scheduler): certify architecture support claims` | Booted-task evidence per target architecture, policy test suite pass, known issues documentation. |
| **Post-Incident** | `fix(scheduler): prevent recurrence of issue` | Minimal reproducer, root cause analysis, regression test added to suite. |

---

## Review and Merge Rules

### PR Review Template

```markdown
## Summary
What scheduler behavior, correctness gap, or performance issue does this PR address?

## Scope
Core | Fair class | BORE | RT | SMP | CPU controller | Architecture hook
State what is explicitly out of scope.

## Policy and Invariants
Task states, queue ownership, fairness/priority semantics, lock ordering,
preemption rules, and expected failure behavior.

## Evidence
Unit/model | Booted QEMU single CPU | Booted QEMU SMP | Physical hardware
List exact architecture, vCPU topology, timer configuration, and workload.

## Performance Impact
Baseline and new throughput, median/tail wakeup latency, fairness/share,
context switches, migrations, and run-queue imbalance.

## Tests
State-transition, race/stress, timer/preemption, task exit, fairness,
priority, affinity, resource-controller, and architecture results.

## Status and Documentation
Proposed | Prototype | Integrated | Supported
Link updated feature matrix, scheduler docs, and test results.

## Checklist
- [ ] Run-queue ownership and task transitions are specified
- [ ] Locking/preemption/interrupt assumptions are documented
- [ ] No task can run twice or be lost from a queue
- [ ] Tests cover behavior and failure/race paths
- [ ] Benchmark includes reproducible configuration and tail latency
- [ ] Capability status matches runtime evidence
- [ ] Required reviews and CI checks pass
```

### Reviewer & Merge Gates
* **Isolated Docs / Unit Tests**: Requires **1 maintainer approval**.
* **Scheduler Core / Lock Order / Context Switch / SMP / RT / Cgroup Enforcement**: Requires **2 qualified maintainers**.
* Main branch protected: Required status checks, required reviews, resolved conversations.

---

## Subsystem Completion Gates

* **Prototype**: Policy algorithms and model tests exist, but no real runtime integration is claimed.
* **Integrated**: A booted target schedules actual user tasks through timer preemption, context switching, block/wakeup, and exit paths.
* **Supported**: Repeated automated tests cover fairness, latency, priority classes, failure/race paths, observability, and every advertised architecture/configuration.
