# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Subsystem Definition & Boundary Mapping

In SigmaOS, execution handoff and task dispatch are critical kernel operations that connect scheduling policy decisions with architecture-specific execution state transitions. To maintain modularity, safety, and clear system boundaries, the **dispatcher** is explicitly defined as the mechanism that hands execution to a selected task, including context save/restore, run-state transitions, interrupt/syscall return paths, and deferred kernel work dispatch.

The dispatcher is strictly decoupled from adjacent kernel subsystems:
- **Scheduler (`src/scheduler/` & `src/kernel/scheduler.rs`):** Responsible for selecting the next runnable task based on policy (CFS, BORE, EEVDF, Real-Time FIFO/RR, SchedExt). The scheduler makes policy decisions but does not perform CPU register manipulation or stack manipulation directly.
- **Dispatcher (`docs/DISPATCHER_PLAN.md` & `src/kernel/dispatcher.rs`):** Executes task context switching, manages execution state transitions (`Ready` -> `Running` -> `Blocked`/`Zombie`), coordinates preemption hooks, processes syscall/interrupt return paths, and executes/routes deferred work queues.
- **Interrupt Controller (`src/kernel/interrupt.rs` & `src/arch/x86_64/apic.rs`):** Identifies, acknowledges (EOI), and masks hardware interrupts.
- **Deferred Work Engine (`src/kernel/workqueue.rs`):** Manages tasklet and workqueue execution outside hard-interrupt context, modeled on Linux workqueues and FreeBSD taskqueues.

---

## Operational Invariants & Safety Directives

All dispatcher implementations, context switches, interrupt flow handlers, and deferred work queues in SigmaOS must adhere to the following 8 invariants:

1. **Single scheduling decision path:** Every context switch must be attributable to an explicit scheduler decision or a defined architecture event; no driver or arbitrary subsystem may switch tasks directly.
2. **Context integrity:** Save and restore all architecture-required state—general registers, stack pointer, instruction pointer, flags, address-space state (CR3 / page tables), and enabled FPU/SIMD state—according to the ABI and target CPU configuration.
3. **Correct state ordering:** Mark the outgoing task’s state and queue membership consistently before switching; mark the incoming task running only when it is actually selected and valid.
4. **Interrupt and preemption safety:** Specify where interrupts are enabled, disabled, or masked; avoid holding locks across a context switch unless the lock handoff is explicitly designed and tested to prevent deadlocks and priority inversions.
5. **Kernel/user return is a security boundary:** Validate user mode, stack pointer, page tables, and return-state values; a user task must not restore privileged CPU state or execute with kernel mappings/privileges unintentionally.
6. **No blocking work in hard-interrupt context:** Keep top-half interrupt dispatch strictly bounded, acknowledge/mask the source correctly, and defer sleeping or lengthy work to a task/workqueue.
7. **No lost wakeups or double execution:** A task may be on at most one runnable queue and run on at most one CPU at a time; concurrent wakeup, timeout, signal, and exit must not lose or duplicate state transitions.
8. **Feature claims follow test tier:** Model tests, unit tests, booted QEMU single-CPU, booted QEMU SMP, and physical hardware evidence are distinct. Unit tests do not prove runtime integration or bootability.

---

## Proposed 16-PR Sequence Roadmap

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
| 10 | `feat(dispatch): implement interrupt controller flow interface` | Define common IRQ descriptor and flow semantics for edge, level, per-CPU, and spurious interrupts; keep APIC/GIC/PLIC-specific operations in controller backends. | Mock tests cover mask/ack/EOI ordering; spurious IRQs are safe; each architecture backend documents supported flow types. |
| 11 | `feat(dispatch): add bounded deferred-work executor` | Implement a minimal queue for work that must run in process/thread context, with deduplication, cancellation, flush/drain, and bounded concurrency. | IRQ handlers can enqueue work without blocking; work executes outside hard-IRQ context; shutdown drains or cancels work deterministically; queue saturation is handled. |
| 12 | `feat(dispatch): define deferred-work priority and forward-progress rules` | Add queue classes, ordered execution where required, work limits, CPU affinity policy, and a reserve path for work needed during memory pressure. | Tests demonstrate no deadlock on reclaim-related work and no unbounded worker creation; ordering is guaranteed only for explicitly ordered queues. |
| 13 | `feat(dispatch): add dispatcher tracing and counters` | Add bounded counters for switches, interrupt dispatch, reschedule requests, deferred work queue/run/latency, queue depth, and failures. | Read-only versioned stats can be queried; counters are per-CPU or safely synchronized; trace data excludes secrets and does not allocate unbounded memory in interrupt context. |
| 14 | `test(dispatch): add timer, interrupt, and workqueue fault injection` | Exercise spurious interrupts, queue saturation, handler removal during dispatch, cancellation races, timer reprogramming, and task exit during handoff. | Reproducible stress tests show safe cleanup and no hangs/UAF/double execution; all failures produce bounded diagnostics. |
| 15 | `test(dispatch): add QEMU end-to-end and architecture matrix` | Add booted x86_64 tests first, then separate AArch64 and RISC-V bring-up PRs for their timer, interrupt, and context-switch paths. | Each architecture has independent runtime evidence; tests cover initial task, preemption, block/wake, interrupt return, and idle. |
| 16 | `docs(dispatch): publish support matrix and recovery guide` | Document dispatch semantics, architecture limitations, interrupt/deferred-work contracts, debugging counters, and known failure recovery. | Every Integrated/Supported claim links to a reproducible test and exact target; unsupported SMP, affinity, or interrupt types are explicit. |

**Recommended First Milestone:** Focus strictly on **PRs 1–8** to deliver the first complete vertical slice: boot -> idle -> timer interrupt -> scheduler selection -> context switch -> block/wake -> user return.

---

## Periodic Maintenance Cadences

| Cadence | Recurring Task | Required Evidence |
|---|---|---|
| **Every PR** | `ci(dispatch): run architecture and state-invariant checks` | Build, context-frame/layout tests, scheduler-state tests, and relevant QEMU checks. |
| **Weekly** | `maint(dispatch): triage switch, interrupt, and workqueue regressions` | Reproducer, architecture/configuration, lifecycle stage, owner, and tracked issue/fix. |
| **Weekly** | `test(dispatch): review latency and queue saturation trends` | Context-switch count, interrupt-to-dispatch latency, wakeup-to-run latency, and queue depth. |
| **Monthly** | `audit(dispatch): review context and return-path correctness` | Review register/FPU/address-space save/restore, privilege transitions, and syscall/exception return hooks. |
| **Monthly** | `test(dispatch): run single-CPU boot and preemption suite` | Fresh QEMU boot, idle, timer preemption, yield, block/wake, task exit, and interrupt return. |
| **Monthly** | `test(dispatch): run IRQ lifecycle and deferred-work suite` | Registration/removal, mask/ack/EOI, spurious IRQ, queued work, cancellation, drain, and saturation tests. |
| **Quarterly** | `audit(dispatch): review SMP locks and migration safety` | Audit lock order, CPU migration, IPI reschedule, queue ownership, and memory ordering. |
| **Quarterly** | `perf(dispatch): refresh latency and throughput baselines` | Repeat benchmark runs with fixed QEMU/hardware configuration; report median/tail latency and variance. |
| **Release Candidate** | `release(dispatch): certify supported architecture claims` | Booted test results by architecture, dispatch/IRQ/workqueue tests, known limitations, and recovery path. |
| **Post-Incident** | `fix(dispatch): prevent recurrence of handoff or dispatch bug` | Minimal reproducer, root cause, regression test, affected architecture, and release/backport decision. |

---

## Component-Specific Rules

1. **Scheduler Interface:** Scheduler returns a valid runnable task and its policy metadata; dispatcher performs the actual context switch and state transition.
2. **Context Switch:** Explicitly declare caller-saved vs callee-saved register state; enforce stack ownership, alignment, FPU/SIMD state saving (XSAVE/FXSAVE), and page-table (CR3) switching rules.
3. **Interrupt Entry/Exit:** Keep interrupt controller flow (mask, acknowledge, EOI) separated from top-half/bottom-half dispatch policy.
4. **Deferred Work:** Do not execute sleeping or unbounded work in hard-interrupt context. Enforce queue depth limits, cancellation safety, and drain protocols.
5. **Task Lifecycle:** Context switches must not race with process termination, signal delivery, or resource teardown.
6. **CPU Topology & SMP:** Enable per-CPU dispatch structures and CPU migration locks only after single-CPU APIC timer and IPI handling are fully verified.
7. **Observability:** Maintain bounded per-CPU event counters for switches, preemptions, and IRQ latency. Hard-IRQ paths must never allocate or print dynamically.
8. **Feature Inventory Tracking:** Update `FEATURE_STATUS.toml`, architecture status matrices, and QEMU test logs in tandem with code changes.

---

## PR Review Template & Merge Gates

```text
## Summary
What dispatch behavior or integration gap does this PR address?

## Scope
Context switch | interrupt flow | return-to-user | deferred work | SMP
State what is not included.

## Contract and Invariants
Task/run-queue ownership, context state, interrupt/preemption assumptions,
locking, queue semantics, and forward-progress guarantees.

## Runtime Evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical target
List architecture, machine, timer/controller configuration, and workload.

## Failure and Security Behavior
Malformed frame, spurious IRQ, queue saturation, task exit, cancellation,
cleanup, privilege return, and resource-limit behavior.

## Tests and Metrics
State-transition, race/stress, architecture tests, switch/IRQ latency,
queue depth, and anything not tested.

## Status and Docs
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

## Subsystem Completion Criteria

- **Prototype:** Dispatcher logic is modeled or unit-tested but not yet exercised by real task switches on booted hardware/QEMU.
- **Integrated:** A booted QEMU target successfully executes real tasks through timer/interrupt dispatch, context switching, block/wake, and return to user mode.
- **Supported:** Repeatable automated tests cover each target architecture, interrupt flow type, SMP locking, deferred work queue, fault injection, and published diagnostic counters.
