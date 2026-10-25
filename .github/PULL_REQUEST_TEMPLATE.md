## Summary

<!-- What I/O-management behavior changed and why? -->

## I/O component

- [ ] I/O policy
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] Submit / complete
- [ ] Callback
- [ ] Interrupt
- [ ] DMA
- [ ] Buffer
- [ ] Recovery
- [ ] procfs / diagnostics
- [ ] Security / capabilities
- [ ] Documentation
- [ ] CI / tooling

## Component

- [ ] Kernel component
- [ ] Shell component

## Kernel component

- [ ] I/O Subsystem
- [ ] VFS / Block
- [ ] Memory / DMA

## Shell component

- [ ] Command interpreter
- [ ] Interactive shell

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly I/O audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference behavior

<!-- Required for lifecycle, ABI, or I/O changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

## Upstream study

- Study document:
- Upstream reference:

## What SigmaOS adopts

- Adopted features:

## Changes

-
-

## I/O safety review

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

## Kernel safety review

- [ ] Memory safety verified
- [ ] Deadlock prevention verified

## Shell safety review

- [ ] Signal safety verified
- [ ] Builtin execution contract verified

## Safety and rollback

- Risk:
- Rollback plan:

## Reviewer checklist

- [ ] Tests pass locally and in CI
- [ ] Safety and evidence requirements met

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-boot evidence, if boot-affecting
- [ ] I/O benchmark, if performance-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] I/O documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
