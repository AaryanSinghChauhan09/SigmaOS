# SigmaOS CPU Scheduler Development Plan & 18-PR Sequence Roadmap

## Executive Summary & Objectives

The SigmaOS CPU Scheduler Development Plan defines the 18-PR sequence, periodic maintenance cadences, review and merge gates, and completion gates required to evolve the SigmaOS scheduler from unit-tested model implementations into a fully integrated, multi-policy, SMP-capable kernel subsystem.

Key objectives:
1. Establish clean separation between core scheduler mechanics and policy classes (Fair, Real-Time, BORE, Idle).
2. Wire real task lifecycles, timer interrupts, context switches, and APIC timer calibration on x86_64 QEMU.
3. Advance fair scheduling using EEVDF lag and virtual deadline principles, with BORE interactive heuristics as an optional feature.
4. Support SMP load balancing, CPU affinity, CPU controller resource quotas, and versioned observability counters.
5. Expand architecture support to AArch64 and RISC-V through verified boot and preemption paths.

---

## 18-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| 1 | `docs(scheduler): audit scheduler status and define invariants` | Audit `cfs.rs`, `bore.rs`, `realtime.rs`, `eevdf.rs`, scheduler core, context switch, timers, APIC, task states, CPU controllers, and per-architecture paths. Document policy precedence, states, fairness goals, and test levels. | Every scheduler component is marked Proposed, Prototype, Integrated, or Supported with evidence; unit/model results are not described as runtime scheduling. |
| 2 | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration. Add deterministic simulated-clock tests. | Invalid transitions are rejected; task/run-queue invariants are checked; fairness tests report measurable shares and bounded error under defined workloads. |
| 3 | `refactor(scheduler): define class interface and single policy selection path` | Introduce an explicit scheduler-class interface and policy ordering for idle, fair, and real-time classes. Make selection and lifecycle callbacks consistent. | Each runnable task is owned by one class; class precedence is tested; adding a class does not duplicate task lifecycle logic. |
| 4 | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Real kernel tasks are enqueued/dequeued exactly once; no lost wakeups, stale queue entries, or double-running tasks in stress tests. |
| 5 | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire architecture timer interrupts to accounting, time-slice expiration, reschedule requests, and safe return-to-user scheduling points. | QEMU test proves two CPU-bound user tasks alternate according to policy; timer interrupts do not corrupt context; idle CPUs wake correctly. |
| 6 | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, scheduler run queues, and idle path. Validate FPU/SIMD state and stack/address-space switching. | Booted x86_64 QEMU runs multiple Ring 3 tasks; preemption and block/wakeup work; context state is preserved; idle does not miss a reschedule interrupt. |
| 7 | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping and accounting for the existing CFS-inspired implementation; document fairness and nice-like behavior. | Long-running equal-weight tasks converge to equal CPU share within a stated tolerance; weighted tasks receive defined proportional shares; runnable tasks do not starve. |
| 8 | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF-style eligibility, lag, virtual deadlines, requested slices, and sleeper accounting as a separate class or behind a development feature flag. | Deterministic tests cover eligibility, earliest deadline selection, sleeper abuse, wakeup preemption, and weight changes; benchmarks compare to the existing fair policy before any default change. |
| 9 | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define exactly how BORE burst scoring influences the fair class; prevent it from overriding fairness bounds or real-time precedence. | Interactive latency improves on named workloads without violating starvation/fairness thresholds; hostile yield/sleep patterns cannot gain unbounded CPU advantage; policy can be disabled for comparison. |
| 10 | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate existing real-time FIFO/RR policy with task priorities, preemption, time quanta, and permissions. | FIFO runs until block/yield/preemption by higher-priority work; RR rotates peers at the configured quantum; unprivileged priority escalation is denied; starvation and recovery behavior are tested. |
| 11 | `feat(scheduler): add CPU affinity and per-CPU run queues` | Add affinity masks, CPU selection, per-CPU queues, and migration rules after SMP context switching is stable. | Tasks never run outside their affinity; migration preserves queue/accounting invariants; idle CPU balancing is tested; single-CPU mode remains correct. |
| 12 | `feat(scheduler): add SMP load balancing and topology model` | Represent CPU/core/package topology; implement idle-pull and/or periodic balancing; account for cache affinity and migration cost. | Multi-vCPU QEMU tests verify progress, balance, and bounded migration; no run-queue lock inversion; topology fallback works when firmware data is absent. |
| 13 | `feat(scheduler): enforce CPU controller weights and quotas` | Connect cgroup-like CPU weights/limits from process creation through scheduler accounting and throttling. | Quotas limit actual CPU time in QEMU tests; child groups inherit constraints; configuration changes are synchronized; unimplemented controller states are rejected rather than silently accepted. |
| 14 | `feat(scheduler): add versioned scheduler statistics interface` | Add counters for runtime, run-queue wait, wake-to-run latency, switches, migrations, balance attempts, and policy class. | Counters are monotonic or have explicitly defined reset semantics; interface is versioned; tests cover overflow and concurrent reads. |
| 15 | `test(scheduler): add benchmark and regression suite` | Add CPU-bound, interactive, mixed-weight, process/thread creation, IPC, wakeup-latency, and oversubscription workloads. Capture baselines and environment metadata. | Each result records kernel revision, architecture, vCPU count, timer frequency, configuration, workload, repetitions, and percentiles; performance changes are reviewed against both latency and fairness. |
| 16 | `test(scheduler): add race, stress, and failure-injection coverage` | Exercise concurrent wakeups, task exit during migration, timer races, priority changes, CPU offline/online if supported, and resource exhaustion. | No duplicate execution, lost wakeup, use-after-free, deadlock, or corrupted run queue; stress tests are repeatable and time bounded. |
| 17 | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context-switch, idle, and interrupt-return paths per architecture in separate PRs. | Each target boots to multiple user tasks and passes core scheduling tests; per-architecture status is independently reported. |
| 18 | `docs(scheduler): publish policy guide and support matrix` | Document scheduler classes, weights/priorities, limits, tuning knobs, observability, architecture support, and known limitations. | Every public policy claim links to implementation and runtime tests; experimental policies are labeled; no Linux/BSD parity is implied. |

> **Recommended Milestone Focus**: Complete PRs **1–7** before deciding whether EEVDF or BORE becomes the default fair policy on x86_64.

---

## Periodic Maintenance Cadences

| Cadence | Task | Required Evidence |
|---|---|---|
| Every PR | `ci(scheduler)` check changed paths | Formatting, compilation, unit tests, and QEMU integration pass cleanly. |
| Weekly | Triage failures & regressions | Reproducer log, severity, affected policy, and tracked issue. |
| Weekly | Review fairness & latency trends | Compare baseline workload metrics; flag regressions in tail latency or CPU share. |
| Monthly | Audit state & queue invariants | Review task state machine transitions, lock order, enqueue/dequeue, and exit paths. |
| Monthly | Run booted single-CPU matrix | CPU-bound, sleep/wakeup, mixed priority, RT, task exit, and idle wakeup tests on x86_64 QEMU. |
| Monthly | Run SMP & affinity matrix | Test vCPU pinning, load balancing, idle pull, and run-queue lock ordering on multi-vCPU QEMU. |
| Quarterly | Review policy semantics | Audit RT starvation protections, BORE interactive boost boundaries, and CPU controller limits. |
| Quarterly | Refresh benchmark baselines | Run standardized benchmarks across workloads with percentiles and environment metadata. |
| Release Candidate | Certify architecture matrix | Booted evidence for each claimed architecture (`x86_64`, `aarch64`, `riscv64`). |
| Incident | Prevent recurrence | Add minimal reproducer, root-cause analysis, and regression test suite entry. |

---

## Review and Merge Rules

PRs touching scheduler code must supply the following Pull Request template details:
- **Summary**: Problem, correctness gap, or performance goal addressed.
- **Scope**: Core / Fair / BORE / RT / SMP / CPU controller / Architecture hook.
- **Policy and Invariants**: Explicit task states, queue ownership, lock order, preemption assumptions.
- **Evidence**: Unit model vs. booted QEMU evidence.
- **Performance Impact**: Throughput, latency, context-switch rate, migration count.
- **Tests**: State transition, race/stress, timer, preemption, fairness, affinity tests.
- **Status & Documentation**: Proposed / Prototype / Integrated / Supported.

Requires **one maintainer approval** for isolated documentation or test changes and **two qualified reviewers** for core locking, context switching, SMP migration, RT policy, or CPU controller enforcement.

---

## Completion Gates

- **Prototype**: Policy algorithms and model unit tests exist; no live runtime integration claimed.
- **Integrated**: Booted target schedules real kernel/user tasks with timer preemption, context switching, block/wakeup, and exit paths.
- **Supported**: Continuous multi-workload tests verify fairness, latency, priority rules, failure/race paths, and versioned observability across advertised architectures.
