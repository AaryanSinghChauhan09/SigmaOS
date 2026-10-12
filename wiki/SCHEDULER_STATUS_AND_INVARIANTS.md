# SigmaOS CPU Scheduler Architecture, Status, and Development Plan

## 1. Executive Summary & Architectural Audit

This document establishes the master architecture, status assessment, operational invariants, and 18-PR roadmap for the **SigmaOS CPU Scheduler Subsystem**.

SigmaOS currently contains unit-tested implementation models for several CPU scheduling policies across `src/scheduler/` and `src/kernel/`. However, in accordance with SigmaOS repository evidence rules, **a policy implementation that passes unit tests but is not connected to task lifecycle, timer preemption, and context switching remains a Prototype model, not an Integrated kernel scheduler.**

### Component Audit Matrix

| Component | Primary Files | Current Status | Current Evidence & Next Milestone |
|---|---|---|---|
| **Scheduler Core & Dispatcher** | `src/scheduler/scheduler.rs`, `src/scheduler/core.rs`, `src/kernel/scheduler.rs` | **Prototype** | Unit tests for run queues and state machine stubs. *Next milestone: Integrate canonical Thread objects into CPU run queue and scheduler dispatch path.* |
| **Completely Fair Scheduler (CFS)** | `src/kernel/cfs_scheduler.rs`, `src/scheduler/distro_schedulers.rs` | **Prototype** | Unit tests for `vruntime` tracking and weight calculation. *Next milestone: Autogroup scheduling and timer tick integration.* |
| **Burst-Oriented Response Enhancer (BORE)** | `src/scheduler/bore.rs` | **Prototype** | Standalone unit tests for burst scoring, exponential moving average (EMA) decay, and priority boosting. *Next milestone: Integration with Fair class preemption logic.* |
| **EEVDF Fair Scheduler** | `src/scheduler/eevdf.rs`, `src/kernel/sigma_scheduler_eevdf.rs` | **Prototype** | Standalone unit tests for virtual deadline ordering, lag calculation, and eligibility tracking. *Next milestone: Evaluation as primary Fair scheduling class.* |
| **Real-Time Scheduler (FIFO/RR)** | `src/scheduler/distro_schedulers.rs`, `src/kernel/roundrobin.rs` | **Prototype** | Unit tests for bitmap $O(1)$ priority queue selection, SCHED_FIFO ordering, and SCHED_RR quantum rotation. *Next milestone: RT bandwidth throttling and capability checks.* |
| **x86_64 Context Switch & APIC Timer** | `src/arch/x86_64/context.rs`, `src/kernel/sigma_timer.rs` | **Prototype** | Isolated assembly frame save/restore and APIC calibration stubs. *Next milestone: Scheduler run-queue integration and preemption context switch test harness.* |
| **AArch64 & RISC-V Port Status** | `src/arch/aarch64/`, `src/arch/riscv/` | **Proposed** | Early boot compilation paths. *Next milestone: Validate timer interrupts, context switching, and idle loop in QEMU.* |
| **CPU Controllers & Cgroups** | `src/kernel/sigma_cgroup_v2.rs` | **Prototype** | Hierarchical model and weight/quota configuration logic. *Next milestone: Integration with scheduler and VFS for process resource enforcement.* |

---

## 2. Scheduler Policy, Design Rules, and System Invariants

Before altering any scheduling behavior or default policies, the following nine rules and invariants must be strictly enforced:

### Rule 1: Separate Policy from Mechanism
- **Mechanism:** The scheduler core (`src/scheduler/core.rs`) owns per-CPU run-queue locking, task-state transitions, timer interrupt interactions, context-switch dispatch hooks, and CPU selection interfaces.
- **Policy:** Scheduling classes (`Stop/Deadline`, `RealTime`, `Fair`, `Idle`) own policy-specific logic (e.g., picking the next candidate task, updating virtual runtime, adjusting deadlines or burst scores).
- **Precedence Order:** Higher-priority classes strictly preempt lower-priority classes:
  1. `Stop` / `Deadline` (Emergency kernel tasks and hard deadlines)
  2. `RealTime` (`SCHED_FIFO` and `SCHED_RR` priorities 1..99)
  3. `Fair` (`EEVDF`, `CFS`, or `BORE`-enhanced fair class)
  4. `Idle` (Per-CPU background idle loop)
- Hybrid policies (e.g., mixing CFS and BORE or EEVDF) are prohibited until class boundaries and precedence rules are explicitly defined in code.

### Rule 2: Strict Task States and Single-Queue Invariants
- Tasks move through explicitly defined states: `Runnable`, `Running`, `Blocked`, `Stopped`, `Exiting`, and `Idle`.
- **Single Run-Queue Invariant:** A task must reside on at most **one** run queue at any point in time. Duplicate enqueue calls or enqueueing an already running task must trigger an immediate assertion panic.
- **Exclusive Execution Invariant:** A task must **never** be selected for execution or run on two CPUs simultaneously. SMP migrations must transition the task through `Runnable` state under cross-CPU run-queue locking protocols.

### Rule 3: Measurable Fair-Scheduling Contract
- Relative CPU shares are defined via nice levels / weight tables (nice -20 to +19 mapped to relative weights).
- **Virtual Time / Virtual Deadline:** Sleeper tasks accrued lag must be bounded upon wakeup (normalized against minimum `vruntime` or eligible lag floor) to prevent sleeper abuse.
- **Starvation Bound:** Every runnable task in the `Fair` class must receive CPU time within a bounded scheduling period $T_{period} = \max(\text{sysctl\_sched\_latency}, N \times \text{sysctl\_sched\_min\_granularity})$.

### Rule 4: Real-Time Policy Guardrails
- `SCHED_FIFO` tasks run until they explicitly yield, block, or are preempted by a higher-priority real-time task.
- `SCHED_RR` tasks run for a fixed time quantum (e.g., 10 ms or 100 ms depending on priority) before rotating to the tail of their priority queue.
- **Privilege Enforcement:** Elevating a process to `SCHED_FIFO` or `SCHED_RR` requires `CAP_SYS_NICE`.
- **Bandwidth Throttling:** RT tasks are constrained by default (e.g., max 95% CPU time per second) to prevent kernel denial-of-service or lockup of non-RT system threads.

### Rule 5: Data-Driven Performance Evaluation
- All policy claims must be verified using quantitative metrics:
  - Throughput (operations / tasks completed per second)
  - Fairness score (Jain's Fairness Index across equal-weight tasks)
  - Wake-to-run latency (median, p95, p99, and max tail latency)
  - Context-switch rate (voluntary vs involuntary switches / sec)
  - CPU migration count and run-queue imbalance metrics
- Comparisons must preserve baseline workloads and hardware/QEMU topologies across runs.

### Rule 6: Concurrency and Preemption Safety
- **Lock Ordering:** Always acquire `runqueue_lock` for CPU $A$ before CPU $B$ where $\text{CPU\_ID}(A) < \text{CPU\_ID}(B)$ during load balancing or task migration to prevent deadlocks.
- **Interrupt Protection:** Spinlocks securing per-CPU run queues must disable local interrupts (`spin_lock_irqsave`).
- **Preemptible Kernel Invariants:** Critical regions modifying per-CPU task state or run queues must disable preemption (`preempt_disable()`) to prevent reentrancy corruption during nested timer interrupts.

### Rule 7: Real Resource Controller Enforcement
- CPU controller weights (`cpu.weight`) and bandwidth quotas (`cpu.max` e.g., $quota / period$) must be directly accounted for during scheduler pick and tick processing.
- Quota depletion forces immediate task throttling (`dequeue_task` / transition to `Throttled` queue) until the next period timer refresh.

### Rule 8: Architecture Status Isolation
- Architectural qualification is independent per architecture (`x86_64`, `aarch64`, `riscv64`).
- A scheduler verified on x86_64 QEMU cannot be marked `Supported` on AArch64 or RISC-V until timer interrupts, context switches, stack frame alignment, FPU/SIMD state preservation, and idle paths pass native QEMU test suites on those architectures.

### Rule 9: Stable and Versioned Scheduler Statistics
- Expose scheduler counters via a versioned, read-only stats structure (`/proc/schedstat` parity).
- Counters (e.g., `cpu_switches`, `pcount`, `sched_goidle`) must be monotonic or feature explicit reset semantics. ABI format modifications require incrementing the interface version number.

---

## 3. Proposed 18-PR Sequence Roadmap

| PR | Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **1** | `docs(scheduler): audit scheduler status and define invariants` | Audit `cfs.rs`, `bore.rs`, `realtime.rs`, scheduler core, context switch, timers, APIC, task states, CPU controllers, and per-architecture paths. Document policy precedence, states, fairness goals, and test levels. | Every scheduler component is marked Proposed, Prototype, Integrated, or Supported with evidence; unit/model results are not described as runtime scheduling. |
| **2** | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration. Add deterministic simulated-clock tests. | Invalid transitions are rejected; task/run-queue invariants are checked; fairness tests report measurable shares and bounded error under defined workloads. |
| **3** | `refactor(scheduler): define class interface and single policy selection path` | Introduce an explicit scheduler-class interface and policy ordering for idle, fair, and real-time classes. Make selection and lifecycle callbacks consistent. | Each runnable task is owned by one class; class precedence is tested; adding a class does not duplicate task lifecycle logic. |
| **4** | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Real kernel tasks are enqueued/dequeued exactly once; no lost wakeups, stale queue entries, or double-running tasks in stress tests. |
| **5** | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire architecture timer interrupts to accounting, time-slice expiration, reschedule requests, and safe return-to-user scheduling points. | QEMU test proves two CPU-bound user tasks alternate according to policy; timer interrupts do not corrupt context; idle CPUs wake correctly. |
| **6** | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, scheduler run queues, and idle path. Validate FPU/SIMD state and stack/address-space switching. | Booted x86_64 QEMU runs multiple Ring 3 tasks; preemption and block/wakeup work; context state is preserved; idle does not miss a reschedule interrupt. |
| **7** | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping and accounting for the existing CFS-inspired implementation; document fairness and nice-like behavior. | Long-running equal-weight tasks converge to equal CPU share within a stated tolerance; weighted tasks receive defined proportional shares; runnable tasks do not starve. |
| **8** | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF-style eligibility, lag, virtual deadlines, requested slices, and sleeper accounting as a separate class or behind a development feature flag. | Deterministic tests cover eligibility, earliest deadline selection, sleeper abuse, wakeup preemption, and weight changes; benchmarks compare to existing fair policy before default change. |
| **9** | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define exactly how BORE burst scoring influences the fair class; prevent it from overriding fairness bounds or real-time precedence. | Interactive latency improves on named workloads without violating starvation/fairness thresholds; hostile yield/sleep patterns cannot gain unbounded CPU advantage. |
| **10** | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate existing real-time FIFO/RR policy with task priorities, preemption, time quanta, and permissions. | FIFO runs until block/yield/preemption by higher-priority work; RR rotates peers at configured quantum; unprivileged priority escalation is denied. |
| **11** | `feat(scheduler): add CPU affinity and per-CPU run queues` | Add affinity masks, CPU selection, per-CPU queues, and migration rules after SMP context switching is stable. | Tasks never run outside their affinity; migration preserves queue/accounting invariants; idle CPU balancing is tested; single-CPU mode remains correct. |
| **12** | `feat(scheduler): add SMP load balancing and topology model` | Represent CPU/core/package topology; implement idle-pull and periodic balancing; account for cache affinity and migration cost. | Multi-vCPU QEMU tests verify progress, balance, and bounded migration; no run-queue lock inversion; topology fallback works when firmware data is absent. |
| **13** | `feat(scheduler): enforce CPU controller weights and quotas` | Connect cgroup-like CPU weights/limits from process creation through scheduler accounting and throttling. | Quotas limit actual CPU time in QEMU tests; child groups inherit constraints; configuration changes are synchronized; unimplemented states are rejected. |
| **14** | `feat(scheduler): add versioned scheduler statistics interface` | Add counters for runtime, run-queue wait, wake-to-run latency, switches, migrations, balance attempts, and policy class. | Counters are monotonic or have explicitly defined reset semantics; interface is versioned; tests cover overflow and concurrent reads. |
| **15** | `test(scheduler): add benchmark and regression suite` | Add CPU-bound, interactive, mixed-weight, process/thread creation, IPC, wakeup-latency, and oversubscription workloads. Capture baselines and metadata. | Each result records kernel revision, architecture, vCPU count, timer frequency, configuration, workload, repetitions, and percentiles. |
| **16** | `test(scheduler): add race, stress, and failure-injection coverage` | Exercise concurrent wakeups, task exit during migration, timer races, priority changes, CPU offline/online, and resource exhaustion. | No duplicate execution, lost wakeup, use-after-free, deadlock, or corrupted run queue; stress tests are repeatable and time bounded. |
| **17** | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context-switch, idle, and interrupt-return paths per architecture in separate PRs. | Each target boots to multiple user tasks and passes core scheduling tests; per-architecture status is independently reported. |
| **18** | `docs(scheduler): publish policy guide and support matrix` | Document scheduler classes, weights/priorities, limits, tuning knobs, observability, architecture support, and known limitations. | Every public policy claim links to implementation and runtime tests; experimental policies are labeled; no Linux/BSD parity is implied without evidence. |

### Recommended Initial Focus: PRs 1–7
**PRs 1 through 7 must be completed and verified before deciding whether EEVDF or BORE becomes the default fair scheduling policy.** Establishing real kernel task lifecycle hooks, timer preemption interrupts, and x86_64 context switching transforms the scheduling policy from a unit-tested theoretical model into a functional, live CPU scheduler.

---

## 4. Periodic Maintenance Cadence Plan

| Cadence | Task | Required Evidence & Outputs |
|---|---|---|
| **Every PR** | Run scheduler checks for changed paths | Code formatting, build checks, scheduler unit tests (`rustc --test`), and QEMU integration smoke tests. |
| **Weekly** | Triage scheduler failures and regressions | Log issue reporst with minimal reproducer, affected policy/arch, severity, and root cause analysis. |
| **Weekly** | Review fairness and wakeup-latency trends | Benchmark tracking output comparing baseline workload results; flag regressions in tail latency, fairness, or context switches. |
| **Monthly** | Verify state and run-queue invariants | Formal audit of state machine logic, enqueue/dequeue routines, migration safety, and task exit paths touched during the month. |
| **Monthly** | Run booted single-CPU scheduling matrix | Execute CPU-bound, sleep/wakeup, mixed priority, real-time, and idle wakeup test suites on x86_64 QEMU target. |
| **Monthly** | Run SMP and affinity matrix | Execute multi-vCPU QEMU test matrix checking CPU pinning, migration, idle pull, and run-queue spinlock safety. |
| **Quarterly** | Review policy semantics and starvation bounds | Review fair class behavior, BORE heuristic limits, RT starvation protections, and CPU controller accounting. |
| **Quarterly** | Refresh benchmark baselines | Run standardized benchmarks with multiple iterations; record median, p95, p99 latencies, vCPU count, and git commit SHA. |
| **Release Candidate** | Certify architecture support claims | Compile and attach booted-task test evidence per claimed architecture (`x86_64`, `aarch64`, `riscv64`). |
| **After Incident** | Regression prevention fix | Minimal reproducer, root-cause analysis document, and dedicated regression test suite. |

---

## 5. Review and Merge Gates

### Standard Scheduler PR Template
Every pull request altering scheduler functionality must utilize the following description format:

```markdown
## Summary
What scheduler behavior, correctness gap, or performance issue does this PR address?

## Scope
Core | fair class | BORE | RT | SMP | CPU controller | architecture hook
State what is explicitly out of scope.

## Policy and Invariants
Task states, queue ownership, fairness/priority semantics, lock ordering,
preemption rules, and expected failure behavior.

## Evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical hardware
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

### Approval & Review Rules
- **Isolated Documentation / Unit Test Changes:** Requires approval from at least **one** maintainer.
- **Scheduler Core, Lock Ordering, Context Switching, SMP Migration, RT Policy, or Cgroups:** Requires explicit review and approval from at least **two** qualified maintainers (`needs-second-reviewer`).

---

## 6. Subsystem Completion Gates

SigmaOS explicitly distinguishes three capability states for scheduler components:

1. **Prototype:** Policy algorithms, data structures, and unit-level model tests exist (`rustc --test`). Real runtime scheduling is not claimed.
2. **Integrated:** A booted QEMU target schedules actual runnable user/kernel tasks using timer interrupt preemption, register context switching, address-space switching, blocking/wakeups, and task exits.
3. **Supported:** Repeated automated test suites verify fairness, latency, priority classes, race/stress conditions, observability counters, and CPU controllers across every advertised architecture and hardware/QEMU topology.
