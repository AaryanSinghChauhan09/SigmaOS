# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Overview & Scope Definition

SigmaOS’s repository has a **scheduler** with CFS-inspired, BORE, and real-time FIFO/RR models, plus x86_64 context-switch and APIC code. But the inventory still marks scheduler-to-context-switch integration and APIC timer calibration as next milestones; syscall dispatch is also only partial, and the general boot path is not validated. This plan explicitly defines and audits the **dispatcher** boundary rather than assuming it already exists.

Here, **dispatcher** means the mechanism that hands execution to a selected task, including context save/restore, run-state transitions, interrupt/syscall return paths, and—separately—deferred kernel work dispatch. Keep it distinct from the scheduler, which selects the next runnable task, and the interrupt controller, which identifies and acknowledges hardware interrupts. Linux’s generic IRQ design separates driver API, interrupt-flow handling, and controller-specific operations; Linux workqueues and FreeBSD taskqueues provide useful references for deferred work.

---

## Invariants and Scope

Adopt these rules for dispatcher changes:

- **Single scheduling decision path:** Every context switch must be attributable to a scheduler decision or a defined architecture event; no driver or arbitrary subsystem may switch tasks directly.
- **Context integrity:** Save and restore all architecture-required state—general registers, stack pointer, instruction pointer, flags, address-space state, and enabled FPU/SIMD state—according to the ABI and configuration.
- **Correct state ordering:** Mark the outgoing task’s state and queue membership consistently before switching; mark the incoming task running only when it is actually selected and valid.
- **Interrupt and preemption safety:** Specify where interrupts are enabled, disabled, or masked; avoid holding locks across a context switch unless the lock handoff is explicitly designed and tested. Linux warns that preemptible kernels introduce concurrency and reentrancy hazards.
- **Kernel/user return is a security boundary:** Validate user mode, stack, page-table, and return-state values; a user task must not restore privileged CPU state or execute with kernel mappings/privileges unintentionally.
- **No blocking work in hard-interrupt context:** Keep top-half dispatch bounded, acknowledge/mask the source correctly, and defer sleeping or lengthy work to a task/workqueue. Linux generic IRQ supports distinct flow handlers and threaded handlers; Linux workqueues and FreeBSD taskqueues are models for deferred execution.
- **No lost wakeups or double execution:** A task may be on at most one runnable queue and run on at most one CPU at a time; concurrent wakeup, timeout, signal, and exit must not lose or duplicate transitions.
- **Feature claims follow test tier:** Model tests, unit tests, booted QEMU, SMP QEMU, and physical hardware evidence are distinct. SigmaOS itself specifies that unit tests do not prove runtime integration or bootability.

---

## Proposed PR Sequence

| PR | Proposed title | Scope | Acceptance criteria |
|---|---|---|---|
| 1 | `docs(dispatch): define scheduler-dispatcher-IRQ boundaries` | Audit scheduler, context-switch code, APIC/timers, interrupt entry/return, syscall entry/return, task states, work queues, and architecture directories. Document ownership and status. | The component map identifies who selects, switches, acknowledges, and defers work; every current path is marked by actual evidence; no unverified “dispatcher” capability is claimed. |
| 2 | `test(dispatch): add context-frame layout and invariant tests` | Add tests for initial task frames, saved-register layouts, stack alignment, flags masks, and task/run-queue invariants. | Layout checks match architecture code; invalid task states are rejected; tests identify themselves as unit/model rather than runtime evidence. |
| 3 | `refactor(dispatch): define architecture context-switch interface` | Define architecture-neutral switch contract and per-architecture implementation boundary; specify caller-saved/callee-saved state, FPU/SIMD handling, stack, address-space, and return semantics. | x86_64 uses the interface without behavior change; missing AArch64/RISC-V implementations return unsupported rather than silently succeeding; contract is documented. |
| 4 | `feat(dispatch): connect scheduler selection to x86_64 context switch` | Integrate `src/arch/x86_64/context.rs` with scheduler run-queue selection and task lifecycle. | QEMU runs at least two real user tasks; switching preserves registers, stack, address space, and FPU state; blocked task wakeup returns to the correct continuation. The current inventory explicitly lists EEVDF scheduler integration as the context-switch next milestone. |
| 5 | `feat(dispatch): add safe first-task and idle-task handoff` | Implement initial task entry, per-CPU idle task, no-runnable-task behavior, and first switch from boot code. | Fresh QEMU boot reaches idle then launches a test user task; idle does not spin at full CPU when a halt/wait instruction is available; wakeup reliably resumes scheduling. |
| 6 | `feat(dispatch): connect timer interrupt return to reschedule path` | Wire APIC timer ticks to accounting and reschedule requests; perform scheduling only at a safe interrupt-return point. | QEMU preempts CPU-bound tasks; timer EOI/ack occurs exactly once; interrupt frame is restored correctly; timing and lost-tick behavior are documented. APIC timer calibration is currently a stated gap. |
| 7 | `feat(dispatch): connect syscall and exception return-to-user hooks` | Centralize return-to-user checks for pending reschedule, signals, and deferred events; preserve correct exception/syscall frames. | Syscall/exception return cannot bypass required scheduling or signal checks; nested interrupt and fault paths are tested; invalid user return state is rejected. |
| 8 | `feat(dispatch): add wakeup and yield handoff semantics` | Integrate yield, wakeup, block, timeout, and exit paths with dispatcher entry points; define preemption policy. | Tests cover yield to another task, wakeup of higher-priority task, timeout races, and exit; no task remains running after it is blocked or exited. |
| 9 | `feat(dispatch): add run-queue locking and SMP switch protocol` | Add per-CPU dispatch state, run-queue lock rules, migration synchronization, and reschedule IPI handling. Defer until x86_64 single-CPU path is stable. | SMP QEMU stress proves no double execution, stale queue membership, deadlock, or missed reschedule; lock order and memory ordering are documented. |
| 10 | `feat(dispatch): implement interrupt controller flow interface` | Define common IRQ descriptor and flow semantics for edge, level, per-CPU, and spurious interrupts; keep APIC/GIC/PLIC-specific operations in controller backends. | Mock tests cover mask/ack/EOI ordering; spurious IRQs are safe; each architecture backend documents supported flow types. Linux’s generic IRQ layer separates flow from chip details. |
| 11 | `feat(dispatch): add bounded deferred-work executor` | Implement a minimal queue for work that must run in process/thread context, with deduplication, cancellation, flush/drain, and bounded concurrency. | IRQ handlers can enqueue work without blocking; work executes outside hard-IRQ context; shutdown drains or cancels work deterministically; queue saturation is handled. Linux workqueues and FreeBSD taskqueues are references, not APIs to clone. |
| 12 | `feat(dispatch): define deferred-work priority and forward-progress rules` | Add queue classes, ordered execution where required, work limits, CPU affinity policy, and a reserve path for work needed during memory pressure—only when the allocator and worker-thread infrastructure support it. | Tests demonstrate no deadlock on reclaim-related work and no unbounded worker creation; ordering is guaranteed only for explicitly ordered queues. Linux workqueue docs stress concurrency management and forward progress. |
| 13 | `feat(dispatch): add dispatcher tracing and counters` | Add bounded counters for switches, interrupt dispatch, reschedule requests, deferred work queue/run/latency, queue depth, and failures. | Read-only versioned stats can be queried; counters are per-CPU or safely synchronized; trace data excludes secrets and does not allocate unbounded memory in interrupt context. |
| 14 | `test(dispatch): add timer, interrupt, and workqueue fault injection` | Exercise spurious interrupts, queue saturation, handler removal during dispatch, cancellation races, timer reprogramming, and task exit during handoff. | Reproducible stress tests show safe cleanup and no hangs/UAF/double execution; all failures produce bounded diagnostics. |
| 15 | `test(dispatch): add QEMU end-to-end and architecture matrix` | Add booted x86_64 tests first, then separate AArch64 and RISC-V bring-up PRs for their timer, interrupt, and context-switch paths. | Each architecture has independent runtime evidence; tests cover initial task, preemption, block/wake, interrupt return, and idle. Current inventory marks AArch64/RISC-V boot paths compile-only or unvalidated. |
| 16 | `docs(dispatch): publish support matrix and recovery guide` | Document dispatch semantics, architecture limitations, interrupt/deferred-work contracts, debugging counters, and known failure recovery. | Every Integrated/Supported claim links to a reproducible test and exact target; unsupported SMP, affinity, or interrupt types are explicit. |

*Note: PRs 1–8 are prioritized first to establish the initial vertical slice (boot → idle → timer → scheduler selection → context switch → block/wake/exit).*

---

## Periodic Maintenance PR Cadence

| Cadence | Proposed recurring PR/task | Required evidence |
|---|---|---|
| Every PR | `ci(dispatch): run architecture and state-invariant checks` | Build, context-frame/layout tests, scheduler-state tests, and relevant QEMU checks; skipped checks explained. |
| Weekly | `maint(dispatch): triage switch, interrupt, and workqueue regressions` | Reproducer, architecture/configuration, lifecycle stage, owner, and fix or tracked issue. |
| Weekly | `test(dispatch): review latency and queue saturation trends` | Context-switch count, interrupt-to-dispatch latency, wakeup-to-run latency, deferred-work delay, and queue-depth trends from fixed workloads. |
| Monthly | `audit(dispatch): review context and return-path correctness` | Review register/FPU/address-space save/restore, privilege transitions, interrupt masking, and syscall/exception return hooks. |
| Monthly | `test(dispatch): run single-CPU boot and preemption suite` | Fresh QEMU boot, idle, timer preemption, yield, block/wake, task exit, and interrupt-return checks. |
| Monthly | `test(dispatch): run IRQ lifecycle and deferred-work suite` | Registration/removal, mask/ack/EOI, spurious IRQ, queued work, cancellation, drain, and saturation tests. |
| Quarterly | `audit(dispatch): review SMP locks and migration safety` | Run only after SMP path exists; audit lock order, CPU migration, IPI reschedule, queue ownership, and memory ordering. |
| Quarterly | `perf(dispatch): refresh latency and throughput baselines` | Repeat benchmark runs with fixed QEMU/hardware configuration and report median/tail latency, throughput, queue depth, and variance. |
| Each release candidate | `release(dispatch): certify supported architecture claims` | Booted test results by architecture, dispatch/IRQ/workqueue tests, known limitations, and recovery path. |
| After an incident | `fix(dispatch): prevent recurrence of <handoff or dispatch bug>` | Minimal reproducer, root cause, regression test, affected architecture, and release/backport decision. |

---

## Component-Specific Rules

- **Scheduler interface:** Scheduler returns a valid runnable task and its policy metadata; dispatcher performs the actual handoff. Keep these responsibilities separate so scheduler tests do not pretend to validate assembly context switching.
- **Context switch:** Specify which registers are saved by the caller versus switch routine; define stack ownership, FPU/SIMD state policy, address-space switch timing, and task canaries/debug assertions. Architecture implementations must pass the same contract tests.
- **Interrupt entry/exit:** Keep controller operations (mask, acknowledge, EOI) distinct from generic flow policy. Handle edge/level semantics, shared interrupts, spurious interrupts, nesting, and per-CPU interrupts deliberately. Linux documents these as separate flow/controller layers.
- **Deferred work:** Do not run sleeping or unbounded work in hard-interrupt context. Define whether a work item can be queued twice, run concurrently, canceled, flushed, or requeued; bound concurrency and queue depth. Linux workqueues support thread-context execution, ordering, affinity, and concurrency limits; FreeBSD taskqueues provide another model for asynchronous execution.
- **Task lifecycle:** A switch must not race with exit/reap, signal delivery, or service shutdown. Keep process-manager and scheduler changes in coordinated but independently testable PRs.
- **CPU topology/SMP:** Add CPU affinity and per-CPU dispatch only after APIC/GIC/PLIC timer and IPI paths work; test migration and CPU online/offline behavior separately.
- **Observability:** Record switch/dispatch reasons and latency with bounded, privacy-safe tracing. Do not print from hard-IRQ paths or make diagnostics change timing materially.
- **Feature inventory:** Update `FEATURE_STATUS.toml`, component docs, and QEMU matrix in the same PR when evidence changes. Do not promote “working” model status to Integrated without a booted path.

---

## PR Template and Merge Gates

```text
## Summary
What dispatch behavior or integration gap does this PR address?

## Scope
Context switch | interrupt flow | return-to-user | deferred work | SMP
State what is not included.

## Contract and invariants
Task/run-queue ownership, context state, interrupt/preemption assumptions,
locking, queue semantics, and forward-progress guarantees.

## Runtime evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical target
List architecture, machine, timer/controller configuration, and workload.

## Failure and security behavior
Malformed frame, spurious IRQ, queue saturation, task exit, cancellation,
cleanup, privilege return, and resource-limit behavior.

## Tests and metrics
State-transition, race/stress, architecture tests, switch/IRQ latency,
queue depth, and anything not tested.

## Status and docs
Proposed | Prototype | Integrated | Supported
Link feature inventory, design docs, and test evidence.

## Checklist
- [ ] Scheduler selection and dispatcher mechanics remain separate
- [ ] Context frame and privilege return are validated
- [ ] IRQ handlers are bounded; deferred work runs in valid context
- [ ] No lost wakeup, duplicate queue membership, or double execution
- [ ] Shutdown/removal synchronizes in-flight dispatch/work
- [ ] Required CI passes; review conversations resolved
```

---

## Completion Criteria

- **Prototype:** Dispatcher logic is modeled or unit-tested but not exercised by real task switches.
- **Integrated:** A booted QEMU target runs real tasks through timer/interrupt dispatch, context switching, block/wake, and return to user mode.
- **Supported:** Repeatable tests cover each advertised architecture, interrupt flow, SMP claims, deferred work, race/error paths, and published diagnostics.

For SigmaOS, the immediate completion gate is x86_64 QEMU integration of scheduler selection, APIC timer preemption, and `context.rs`.
