## Summary

<!-- What file-management behavior changed and why? -->

## I/O component

- [ ] I/O policy
- [ ] Interrupt manager
- [ ] DMA manager / DMA queue
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O safety review

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review

- [ ] Double-completion prevented
- [ ] Resource limits enforced

## File component

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

## Evidence

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

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] File documentation
- [ ] Boot matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:

## I/O component

- [ ] Interrupt manager
- [ ] DMA manager / DMA queue
- [ ] Buffer manager
- [ ] I/O scheduler
- [ ] I/O policy
- [ ] I/O safety review

## Storage component
- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager

## Storage safety review
- [ ] Double-completion prevented
- [ ] Resource limits enforced

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Security review

- Kernel safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## Verification

- [ ] Focused unit tests pass.
- [ ] ./run_sigma_tests.sh
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:

## Shell component

- [ ] Interactive shell
- [ ] Builtins
- [ ] Parser
- [ ] Diagnostics

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

## Shell safety review

- [ ] Shell safety review complete
- [ ] Safety and rollback
