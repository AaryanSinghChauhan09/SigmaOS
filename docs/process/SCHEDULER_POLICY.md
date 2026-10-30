# SigmaOS Scheduler Subsystem Policy

## Authoritative Lifecycle State Machine

Tasks transition through New, Ready, Running, Sleeping, Stopped, Zombie, and
Reaped states. A task enters Ready only after admission and queue assignment,
enters Running only through scheduler selection, and reaches Reaped only after
exit cleanup and ownership release.

The scheduler must preserve a single authoritative state transition, reject
invalid transitions, and make CPU ownership explicit. State changes, queue
operations, wakeups, and teardown are observable in deterministic tests.

## Policy

Use bounded run-queue operations, explicit preemption points, synchronized SMP
ownership, and evidence-backed latency/fairness measurements. No policy may
silently drop a task or report support without the corresponding test tier.

