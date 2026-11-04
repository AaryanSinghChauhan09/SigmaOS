## Summary

<!-- What file-management or system behavior changed and why? -->

## Motivation

## Implementation notes

## Component

- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
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
- [ ] I/O component
- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] IPC handler
- [ ] IPC semantics

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Shell component

- [ ] Shell parser
- [ ] Shell builtins

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
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Changes

-
-

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

- [ ] Interrupt safety verified
- [ ] DMA boundary checked

## Storage safety review

- [ ] Double-completion prevented
- [ ] Resource limits enforced

## Shell safety review

- [ ] Exit codes checked

## Security review

- [ ] Capability/namespace checks enforced

## Tests

- [ ] Unit tests added/updated

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

## Capability status

## Integration impact

## Reviewer checklist

- [ ] Scope is narrow
- [ ] `unsafe` justified

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] File documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:

## Component
- Core subsystem component

## Study document
- Periodic research study document

## Safety and rollback
- Safety and rollback verification

## Kernel safety review
- Kernel safety verification

## Shell component
- Shell component review

## Shell safety review
- Shell safety review

## I/O component
- [ ] Interrupt manager
- [ ] DMA manager / DMA queue
- [ ] Buffer manager
- [ ] I/O scheduler

## I/O policy
- [ ] I/O policy contract verified

## I/O safety review
- [ ] Interrupt integrity and zero leak verified

## Storage component
- [ ] Block manager
- [ ] Cache manager
- [ ] Filesystem core
- [ ] Mount manager

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- [ ] Double-completion prevented
- [ ] Resource limits enforced

## IPC handler
- [ ] IPC semantics
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Security review
- [ ] Capability/namespace checks enforced

## Verification
- [ ] ./run_sigma_tests.sh
- Capability status
- Integration impact
