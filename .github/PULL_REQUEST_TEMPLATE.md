## Summary

<!-- What subsystem behavior changed and why? -->

## Motivation

<!-- Why was this change made? -->

## Implementation notes

<!-- Key implementation details -->

## Component

- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] IPC handler
- [ ] File descriptor manager
- [ ] Inode and directory manager
- [ ] Permission manager
- [ ] Lock manager
- [ ] Mount / unmount
- [ ] Cache
- [ ] Filesystem
- [ ] Recovery
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Component impact

- [ ] Shell component
- [ ] File component
- [ ] Storage component
- [ ] I/O component

## I/O component

- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## IPC semantics

- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly memory audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or file changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Changes

-
-

## Tests

- Unit tests
- Integration tests

## Verification

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-boot evidence, if boot-affecting
- [ ] File benchmark, if performance-affecting
- [ ] Security review requested, if applicable

## Security review

- [ ] Capability/namespace checks enforced

## Reviewer checklist

- [ ] Scope is narrow
- [ ] `unsafe` justified

## Safety and rollback

- Double-completion prevented
- Resource limits enforced

## File safety review

- [ ] Lifecycle transitions are valid
- [ ] Zombies are eventually reaped
- [ ] Orphans are adopted
- [ ] PID reuse is safe
- [ ] Lock ordering is documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-unlink prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## I/O safety review

- [ ] Double-completion prevented
- [ ] Resource limits enforced

## Storage safety review

- [ ] Double-completion prevented
- [ ] Resource limits enforced

## Capability status

- [ ] Updated in matrix

## Integration impact

- [ ] Subsystem integration verified

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] File documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
