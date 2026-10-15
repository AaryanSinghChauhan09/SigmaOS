## Summary

<!-- What scheduler or component behavior changed and why? -->

## Scheduler component

- [ ] Scheduler core
- [ ] Run queue / PID allocation
- [ ] fork / clone
- [ ] exec
- [ ] Context switch
- [ ] Preemption & quantum
- [ ] Signals & wait queues
- [ ] Zombie / reaping & orphan adoption
- [ ] Resource limits & CPU affinity
- [ ] Documentation & ADRs
- [ ] CI / tooling

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly scheduler audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or scheduler changes -->

- POSIX section:
- Linux/BSD/Redox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Scheduler safety review

- [ ] Lifecycle transitions are valid (NEW -> READY -> RUNNING -> SLEEPING -> READY, READY -> RUNNING -> ZOMBIE, RUNNING -> STOPPED -> READY, ANY -> ZOMBIE -> REAPED)
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted by PID 1
- [ ] PID reuse is safe
- [ ] Lock ordering and concurrency model documented
- [ ] Interrupt-context safety considered
- [ ] Lost-wakeup prevention verified
- [ ] Double-reap and double-switch prevented
- [ ] Resource limits and run-queue capacity enforced
- [ ] No unbounded allocation or recursion

## Verification

```bash
cargo check --lib
cargo test --lib
./run_sigma_tests.sh
```

## Runtime evidence

- [ ] QEMU boot-to-shell log
- [ ] Scheduler test / benchmark report
- [ ] Artifact hash

## Safety and rollback

- [ ] No broad credentials or host access introduced
- [ ] Untrusted input validated
- [ ] Rollback/recovery behavior defined
- [ ] Security review completed, if applicable
