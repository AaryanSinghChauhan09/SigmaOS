## Summary

<!-- What behavior changed and why? -->

## Component

- [ ] Shell component
- [ ] Memory policy
- [ ] Physical memory manager
- [ ] Buddy allocator
- [ ] Slab allocator
- [ ] Paging
- [ ] Page tables
- [ ] Virtual memory
- [ ] Protection
- [ ] Swap
- [ ] Reclaim
- [ ] OOM
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly storage audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference study (Upstream study)

<!-- Required for lifecycle, ABI, or storage changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes (What SigmaOS adopts)

-
-

## Shell safety review

- [ ] Shell safety review
- [ ] Kernel safety review
- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
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
- [ ] Storage benchmark, if performance-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Storage documentation
- [ ] Boot matrix, if applicable

## Risks and rollback (Safety and rollback)

- Risk:
- Rollback:
