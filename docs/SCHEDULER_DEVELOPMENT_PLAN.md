# SigmaOS CPU Scheduler Development Plan & Maintenance Specification

## Overview

This document specifies the official 18-PR sequence roadmap, maintenance cadences, PR review rules, and completion gates for the SigmaOS CPU Scheduler subsystem. It serves as the authoritative blueprint for transitioning scheduling components from unit-tested models to integrated, supported kernel subsystems.

---

## 18-PR Sequence Roadmap

| PR | Proposed Title | Scope | Acceptance Criteria |
|---|---|---|---|
| **PR 1** | `docs(scheduler): audit scheduler status and define invariants` | Audit `cfs.rs`, `bore.rs`, `realtime.rs`, core scheduler, context switch, timers, APIC, task states, CPU controllers, and per-architecture paths. Document policy precedence, states, fairness goals, and test levels. | Every scheduler component is marked Proposed, Prototype, Integrated, or Supported with evidence; unit/model results are not described as runtime scheduling. |
| **PR 2** | `test(scheduler): add deterministic state-transition and fairness tests` | Expand unit tests for enqueue/dequeue, wakeups, blocking, exit, duplicate enqueue, priority changes, and task migration. Add deterministic simulated-clock tests. | Invalid transitions are rejected; task/run-queue invariants are checked; fairness tests report measurable shares and bounded error under defined workloads. |
| **PR 3** | `refactor(scheduler): define class interface and single policy selection path` | Introduce an explicit scheduler-class interface and policy ordering for idle, fair, and real-time classes. Make selection and lifecycle callbacks consistent. | Each runnable task is owned by one class; class precedence is tested; adding a class does not duplicate task lifecycle logic. |
| **PR 4** | `feat(scheduler): integrate runnable queues with task lifecycle` | Connect process/thread creation, wakeup, block, yield, exit, and wait paths to run-queue operations. | Real kernel tasks are enqueued/dequeued exactly once; no lost wakeups, stale queue entries, or double-running tasks in stress tests. |
| **PR 5** | `feat(scheduler): connect timer ticks and preemption to scheduler core` | Wire architecture timer interrupts to accounting, time-slice expiration, reschedule requests, and safe return-to-user scheduling points. | QEMU test proves two CPU-bound user tasks alternate according to policy; timer interrupts do not corrupt context; idle CPUs wake correctly. |
| **PR 6** | `feat(scheduler): integrate x86_64 context switching and idle loop` | Connect `src/arch/x86_64/context.rs`, APIC timer, scheduler run queues, and idle path. Validate FPU/SIMD state and stack/address-space switching. | Booted x86_64 QEMU runs multiple Ring 3 tasks; preemption and block/wakeup work; context state is preserved; idle does not miss a reschedule interrupt. |
| **PR 7** | `feat(scheduler): implement weighted fair scheduling semantics` | Define weight mapping and accounting for the existing CFS-inspired implementation; document fairness and nice-like behavior. | Long-running equal-weight tasks converge to equal CPU share within a stated tolerance; weighted tasks receive defined proportional shares; runnable tasks do not starve. |
| **PR 8** | `feat(scheduler): implement EEVDF fair-class prototype` | Implement EEVDF-style eligibility, lag, virtual deadlines, requested slices, and sleeper accounting as a separate class or behind a development feature flag. | Deterministic tests cover eligibility, earliest deadline selection, sleeper abuse, wakeup preemption, and weight changes; benchmarks compare to the existing fair policy before any default change. |
| **PR 9** | `feat(scheduler): integrate BORE as optional interactive heuristic` | Define exactly how BORE burst scoring influences the fair class; prevent it from overriding fairness bounds or real-time precedence. | Interactive latency improves on named workloads without violating starvation/fairness thresholds; hostile yield/sleep patterns cannot gain unbounded CPU advantage; policy can be disabled for comparison. |
| **PR 10** | `feat(scheduler): enforce FIFO and round-robin policy semantics` | Integrate existing real-time FIFO/RR policy with task priorities, preemption, time quanta, and permissions. | FIFO runs until block/yield/preemption by higher-priority work; RR rotates peers at the configured quantum; unprivileged priority escalation is denied; starvation and recovery behavior are tested. |
| **PR 11** | `feat(scheduler): add CPU affinity and per-CPU run queues` | Add affinity masks, CPU selection, per-CPU queues, and migration rules after SMP context switching is stable. | Tasks never run outside their affinity; migration preserves queue/accounting invariants; idle CPU balancing is tested; single-CPU mode remains correct. |
| **PR 12** | `feat(scheduler): add SMP load balancing and topology model` | Represent CPU/core/package topology; implement idle-pull and/or periodic balancing; account for cache affinity and migration cost. | Multi-vCPU QEMU tests verify progress, balance, and bounded migration; no run-queue lock inversion; topology fallback works when firmware data is absent. |
| **PR 13** | `feat(scheduler): enforce CPU controller weights and quotas` | Connect cgroup-like CPU weights/limits from process creation through scheduler accounting and throttling. | Quotas limit actual CPU time in QEMU tests; child groups inherit constraints; configuration changes are synchronized; unimplemented controller states are rejected rather than silently accepted. |
| **PR 14** | `feat(scheduler): add versioned scheduler statistics interface` | Add counters for runtime, run-queue wait, wake-to-run latency, switches, migrations, balance attempts, and policy class. | Counters are monotonic or have explicitly defined reset semantics; interface is versioned; tests cover overflow and concurrent reads. |
| **PR 15** | `test(scheduler): add benchmark and regression suite` | Add CPU-bound, interactive, mixed-weight, process/thread creation, IPC, wakeup-latency, and oversubscription workloads. Capture baselines and environment metadata. | Each result records kernel revision, architecture, vCPU count, timer frequency, configuration, workload, repetitions, and percentiles; performance changes are reviewed against both latency and fairness. |
| **PR 16** | `test(scheduler): add race, stress, and failure-injection coverage` | Exercise concurrent wakeups, task exit during migration, timer races, priority changes, CPU offline/online if supported, and resource exhaustion. | No duplicate execution, lost wakeup, use-after-free, deadlock, or corrupted run queue; stress tests are repeatable and time bounded. |
| **PR 17** | `feat(scheduler): bring up AArch64 and RISC-V scheduling paths` | Integrate timer, context-switch, idle, and interrupt-return paths per architecture in separate PRs. | Each target boots to multiple user tasks and passes core scheduling tests; per-architecture status is independently reported. |
| **PR 18** | `docs(scheduler): publish policy guide and support matrix` | Document scheduler classes, weights/priorities, limits, tuning knobs, observability, architecture support, and known limitations. | Every public policy claim links to implementation and runtime tests; experimental policies are labeled; no Linux/BSD parity is implied. |

---

## Recommended Initial Milestone

**Milestone Goal**: Complete **PRs 1–7** before deciding whether EEVDF or BORE becomes the default fair policy in SigmaOS.

> **Crucial Invariant**: A policy implementation that passes unit tests but is not wired to task lifecycle, timer preemption, and context switching is still a model, not an integrated scheduler.

---

## Periodic Maintenance Cadence

| Cadence | Recurring Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(scheduler): run scheduler checks for changed paths` | Formatting/build, scheduler unit tests, architecture checks, and QEMU integration where applicable; skipped checks explained. |
| **Weekly** | `maint(scheduler): triage scheduler failures and regressions` | Reproducer, affected policy/architecture, severity, owner, and fix or tracked issue; test failures are not ignored as "flaky" without evidence. |
| **Weekly** | `test(scheduler): review fairness and wakeup-latency trends` | Compare baseline workload results; flag regressions in tail latency, fairness, context switches, or CPU balance. |
| **Monthly** | `audit(scheduler): verify state and run-queue invariants` | Review all task state transitions, enqueue/dequeue, migration, timer, and exit paths touched during the month. |
| **Monthly** | `test(scheduler): run booted single-CPU scheduling matrix` | CPU-bound, sleep/wakeup, mixed priority, real-time, task exit, and idle wakeup tests on named QEMU target. |
| **Monthly** | `test(scheduler): run SMP and affinity matrix` | Run only once SMP is integrated; check CPU pinning, migration, idle pull, fairness, and lock behavior on named vCPU topology. |
| **Quarterly** | `maint(scheduler): review policy semantics and starvation bounds` | Review fair-class behavior, BORE heuristic, RT starvation protections, CPU controllers, and documented priorities. |
| **Quarterly** | `perf(scheduler): refresh benchmark baselines` | Re-run fixed workloads with multiple repetitions and latency percentiles; include environment and source revision. |
| **Each Release Candidate** | `release(scheduler): certify architecture support claims` | Booted-task evidence per claimed architecture, policy tests, known issues, scheduler stats, and recovery behavior. |
| **After Each Incident** | `fix(scheduler): prevent recurrence of <race or starvation issue>` | Minimal reproducer, root cause, regression test, affected policies/architectures, and release/backport decision. |

---

## Review and Merge Rules

### PR Review Template

All scheduler Pull Requests must include the following standard review template in their PR description:

```markdown
## Summary
What scheduler behavior, correctness gap, or performance issue does this PR address?

## Scope
Core | fair class | BORE | RT | SMP | CPU controller | architecture hook
State what is explicitly out of scope.

## Policy and invariants
Task states, queue ownership, fairness/priority semantics, lock ordering,
preemption rules, and expected failure behavior.

## Evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical hardware
List exact architecture, vCPU topology, timer configuration, and workload.

## Performance impact
Baseline and new throughput, median/tail wakeup latency, fairness/share,
context switches, migrations, and run-queue imbalance.

## Tests
State-transition, race/stress, timer/preemption, task exit, fairness,
priority, affinity, resource-controller, and architecture results.

## Status and documentation
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

### Reviewer Thresholds
- **Isolated Documentation / Test Changes**: Requires **1 maintainer approval**.
- **Scheduler Core Locking, Context Switching, SMP Migration, Real-Time Policy, or CPU Controllers**: Requires **2 qualified maintainer approvals**.

---

## Completion Gates

1. **Prototype**: Policy algorithms and model tests exist, but no real runtime integration is claimed.
2. **Integrated**: A booted target schedules actual user tasks through timer preemption, context switching, block/wakeup, and exit.
3. **Supported**: Repeated tests cover fairness, latency, priority classes, failure/race paths, observability, and every advertised architecture/configuration.
