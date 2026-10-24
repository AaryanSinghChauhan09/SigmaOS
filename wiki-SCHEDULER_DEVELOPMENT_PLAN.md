# SigmaOS CPU Scheduler: Development & Maintenance Plan

## Overview & Scope
This document details the 18-PR development roadmap, recommended initial milestone, periodic maintenance cadences, PR review templates, and merge gates for the SigmaOS CPU Scheduler subsystem.

---

## 18-PR Development Roadmap

| PR | Title | Scope | Acceptance Criteria |
| :--- | :--- | :--- | :--- |
| **1** | `docs(scheduler): audit scheduler status and define invariants` | Audit `cfs.rs`, `bore.rs`, `realtime.rs`, core scheduler, context switch, timers, APIC, task states, and CPU controllers. | All scheduler components marked `Proposed`, `Prototype`, `Integrated`, or `Supported` with evidence. |
| **2** | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration with simulated clocks. | Invalid transitions rejected; queue invariants verified; fairness tests report measurable shares and bounded error. |
| **3** | `refactor(scheduler): define class interface and single policy selection path` | Introduce explicit `SchedClass` interface and policy ordering for Idle, Fair, and Realtime classes. | Each runnable task owned by one class; class precedence verified; class addition does not duplicate lifecycle logic. |
| **4** | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Real kernel tasks enqueued/dequeued exactly once; no lost wakeups, stale queue entries, or double-running tasks. |
| **5** | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire timer interrupts to accounting, time-slice expiration, reschedule requests, and safe return-to-user scheduling points. | QEMU test proves two CPU-bound tasks alternate according to policy; timer interrupts preserve context; idle CPUs wake correctly. |
| **6** | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, run queues, and idle path. Validate FPU/SIMD state and page table switching. | Booted x86_64 QEMU runs multiple Ring 3 tasks; preemption/block/wakeup work; context state preserved; idle does not miss resched IRQ. |
| **7** | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping and accounting for fair implementation; document fairness and nice-like behavior. | Long-running equal-weight tasks converge to equal CPU share; weighted tasks receive proportional shares; no starvation. |
| **8** | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF eligibility, lag, virtual deadlines, requested slices, and sleeper accounting behind a feature flag or separate class. | Deterministic tests cover eligibility, deadline selection, sleeper abuse, and weight changes; benchmarks compare to CFS baseline. |
| **9** | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define exactly how BORE burst scoring influences the fair class; prevent overriding fairness bounds or RT precedence. | Interactive latency improves on named workloads without violating starvation/fairness thresholds; policy can be toggled off. |
| **10** | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate RT FIFO/RR policy with task priorities, preemption, quanta, permissions, and RT bandwidth throttling. | FIFO runs until block/yield/preempt by higher RT; RR rotates peers at quantum; unprivileged escalation denied; starvation guarded. |
| **11** | `feat(scheduler): add CPU affinity and per-CPU run queues` | Add affinity masks, CPU selection, per-CPU queues, and migration rules after SMP context switching is stable. | Tasks execute strictly within affinity mask; migration preserves invariants; idle CPU balancing tested; single-CPU mode remains correct. |
| **12** | `feat(scheduler): add SMP load balancing and topology model` | Represent CPU/core/package topology; implement idle-pull and periodic balancing; account for cache affinity and migration cost. | Multi-vCPU QEMU tests verify progress, balance, and bounded migration; no run-queue lock inversion; topology fallback verified. |
| **13** | `feat(scheduler): enforce CPU controller weights and quotas` | Connect cgroup-like CPU weights and limits from process creation through scheduler accounting and throttling. | Quotas limit CPU time in QEMU tests; child groups inherit constraints; configuration changes synchronized; invalid states rejected. |
| **14** | `feat(scheduler): add versioned scheduler statistics interface` | Add versioned counters for runtime, run-queue wait, wake-to-run latency, context switches, migrations, and balance attempts. | Counters are monotonic; interface is versioned; overflow and concurrent reads verified in tests. |
| **15** | `test(scheduler): add benchmark and regression suite` | Add CPU-bound, interactive, mixed-weight, process creation, IPC, wakeup latency, and oversubscription workloads. | Results record revision, arch, vCPU count, timer frequency, repetition count, and percentiles; performance reviewed against latency/fairness. |
| **16** | `test(scheduler): add race, stress, and failure-injection coverage` | Exercise concurrent wakeups, task exit during migration, timer races, priority changes, and resource exhaustion. | No duplicate execution, lost wakeup, use-after-free, deadlock, or corrupted run queue under high stress. |
| **17** | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context switch, idle, and interrupt return paths per architecture in separate PRs. | Each target boots multiple user tasks and passes core scheduling tests; per-architecture status reported independently. |
| **18** | `docs(scheduler): publish policy guide and support matrix` | Document scheduler classes, weights/priorities, limits, tuning knobs, observability, arch support, and known limitations. | Every public policy claim links to implementation and runtime test evidence. |

### Recommended Initial Milestone
Complete **PRs 1–7** (Documentation, Tests, SchedClass Refactor, Task Lifecycle Integration, Timer Preemption, x86_64 Context Switch, and Weighted Fair Scheduling) before deciding whether EEVDF or BORE becomes the default fair policy.

---

## Periodic Maintenance Cadences

| Cadence | Recurring Task | Required Evidence |
| :--- | :--- | :--- |
| **Every PR** | `ci(scheduler): run scheduler checks for changed paths` | Formatting (`cargo fmt`), build (`cargo check`), scheduler unit tests, and QEMU integration where applicable. |
| **Weekly** | `maint(scheduler): triage scheduler failures and regressions` | Reproducer, affected policy/arch, severity, owner, and tracking issue or fix. |
| **Weekly** | `test(scheduler): review fairness and wakeup-latency trends` | Benchmark comparison against baseline workload results; flag regressions in tail latency, context switches, or CPU balance. |
| **Monthly** | `audit(scheduler): verify state and run-queue invariants` | Review task state transitions, enqueue/dequeue, migration, timer, and exit paths touched during the month. |
| **Monthly** | `test(scheduler): run booted single-CPU scheduling matrix` | CPU-bound, sleep/wakeup, mixed priority, RT, task exit, and idle wakeup tests on named QEMU target. |
| **Monthly** | `test(scheduler): run SMP and affinity matrix` | Run once SMP is integrated; check CPU pinning, migration, idle pull, fairness, and lock behavior on vCPU topology. |
| **Quarterly** | `maint(scheduler): review policy semantics and starvation bounds` | Review fair-class behavior, BORE heuristic, RT starvation protections, CPU controllers, and documented priorities. |
| **Quarterly** | `perf(scheduler): refresh benchmark baselines` | Re-run fixed workloads with multiple repetitions and latency percentiles; record environment and git commit revision. |
| **Release Candidate** | `release(scheduler): certify architecture support claims` | Booted-task evidence per claimed architecture, policy tests, known issues, scheduler stats, and recovery behavior. |
| **Post-Incident** | `fix(scheduler): prevent recurrence of race/starvation` | Minimal reproducer, root cause analysis, regression test, affected policies/archs, and release/backport decision. |

---

## Review and Merge Rules & PR Template

### PR Template
```markdown
## Summary
What scheduler behavior, correctness gap, or performance issue does this PR address?

## Scope
Core | fair class | BORE | RT | SMP | CPU controller | architecture hook
State what is explicitly out of scope.

## Policy and Invariants
Task states, queue ownership, fairness/priority semantics, lock ordering, preemption rules, and expected failure behavior.

## Evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical hardware
List exact architecture, vCPU topology, timer configuration, and workload.

## Performance Impact
Baseline and new throughput, median/tail wakeup latency, fairness/share, context switches, migrations, and run-queue imbalance.

## Tests
State-transition, race/stress, timer/preemption, task exit, fairness, priority, affinity, resource-controller, and architecture results.

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

### Review and Merge Gates
- **1 Maintainer Approval**: Required for isolated documentation or test updates.
- **2 Qualified Reviewers**: Required for scheduler-core locking, context switching, SMP migration, RT policies, or CPU-controller enforcement.
- **Protected Main Branch**: Enforce PR-only changes, required approvals, passing CI checks, and resolved review conversations.

---

## Completion Gates

1. **Prototype**: Policy algorithms and model tests exist, but no real kernel runtime integration is claimed.
2. **Integrated**: A booted target schedules actual user tasks through timer preemption, context switching, block/wakeup, and task exit.
3. **Supported**: Repeated tests cover fairness, latency, priority classes, failure/race paths, observability, and every advertised target architecture/configuration.
