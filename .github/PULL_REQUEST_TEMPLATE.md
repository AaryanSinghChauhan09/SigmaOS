## Summary

<!-- What process-management behavior changed and why? -->

## Process component

- [ ] Process table / PID allocation
- [ ] fork / clone
- [ ] exec
- [ ] Scheduler / run queue
- [ ] Context switch
- [ ] Signals
- [ ] Wait / zombie / reaping
- [ ] Process groups / sessions
- [ ] Resource limits
- [ ] Namespaces / isolation
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly scheduler audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Monthly rlimit rehearsal
- [ ] Monthly namespace review
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or scheduler changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Process safety review

- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-reap prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Scheduler benchmark, if performance-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Process documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
