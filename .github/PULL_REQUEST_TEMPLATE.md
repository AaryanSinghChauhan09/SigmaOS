## Summary
What dispatch behavior or integration gap does this PR address?

## Component
Context switch | interrupt flow | return-to-user | deferred work | SMP
State what is not included.

## Reference study / What SigmaOS adopts
Task/run-queue ownership, context state, interrupt/preemption assumptions,
locking, queue semantics, and forward-progress guarantees.

## Runtime evidence
Unit/model | booted QEMU single CPU | booted QEMU SMP | physical target
List architecture, machine, timer/controller configuration, and workload.

## Safety and rollback / Risks and rollback
Malformed frame, spurious IRQ, queue saturation, task exit, cancellation,
cleanup, privilege return, and resource-limit behavior.

## Tests and metrics
State-transition, race/stress, architecture tests, switch/IRQ latency,
queue depth, and anything not tested.

## Status and docs
Proposed | Prototype | Integrated | Supported
Link feature inventory, design docs, and test evidence.

## Kernel safety review
- [ ] Scheduler selection and dispatcher mechanics remain separate
- [ ] Context frame and privilege return are validated
- [ ] IRQ handlers are bounded; deferred work runs in valid context
- [ ] No lost wakeup, duplicate queue membership, or double execution
- [ ] Shutdown/removal synchronizes in-flight dispatch/work
- [ ] Required CI passes; review conversations resolved
