# SigmaOS Dispatcher Master Architecture & Support Matrix Plan

## Invariants and Scope

The dispatcher owns task-state transitions, runnable-queue admission, context
switch selection, and deferred work hand-off. Architecture-specific code may
save or restore a context, but it must not create a second scheduling policy.

- Single scheduling decision path: every runnable task enters through the same
  dispatcher policy and queue invariants.
- Context integrity: a switch preserves the task's stack pointer, instruction
  pointer, flags, and address-space identity as one validated frame.
- No blocking work in hard-interrupt context: interrupt top halves only record
  work or enqueue bounded deferred work; draining occurs in thread context.
- A task is enqueued at most once, and only a runnable task can be dispatched.
- Saturated deferred queues fail explicitly and expose a recovery path.

## Proposed PR Sequence Roadmap

1. Document and test the task-state and queue invariants.
2. Introduce architecture-neutral dispatcher interfaces.
3. Add per-architecture context-frame adapters with focused tests.
4. Add bounded deferred-work accounting and saturation telemetry.
5. Add boot, stress, and regression evidence before promoting support status.

## Periodic Maintenance PR Cadence

- Daily: run focused dispatcher and deferred-work tests.
- Weekly: review queue saturation, context-switch failures, and regressions.
- Monthly: refresh the architecture support matrix and update the rollback plan.

## Component-Specific Rules

- Scheduler code must not perform filesystem, network, or allocation-heavy work
  in hard-interrupt context.
- Architecture adapters must reject malformed or incomplete context frames.
- Queue operations must make ownership and failure behavior observable.
- New policy changes require a focused test and a documented migration path.

## PR Template and Merge Gates

Every dispatcher PR must identify the affected architecture/component, cite the
reference study or behavior, describe risks and rollback, and include focused
test output. Merge requires the full native suite plus a review of hard-IRQ
blocking behavior and context integrity.

## Completion Criteria

- All supported architectures use one documented scheduling decision path.
- Context-frame and queue invariants have focused automated coverage.
- Hard-interrupt paths have explicit non-blocking evidence.
- Saturation, rollback, and unsupported-architecture behavior are documented.
- The docs copy and wiki copy remain byte-for-byte identical.
