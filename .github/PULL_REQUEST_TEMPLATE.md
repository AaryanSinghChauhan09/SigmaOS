# SigmaOS pull request

## Summary

<!-- What secondary-memory, kernel, IPC, shell, or system behavior changed and why? -->

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

## Storage component

- [ ] Block device manager
- [ ] virtio / NVMe / virtio
- [ ] Block cache manager
- [ ] Filesystem manager
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
- [ ] Biweekly storage audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Upstream study and reference behavior

<!-- Required for lifecycle, ABI, or storage changes -->

- Upstream study:
- Reference study:
- Upstream project, document, or behavior reviewed:
- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- What SigmaOS adopts:
- What remains intentionally different:
- Compatibility impact:

## Changes

-
-

## Storage safety review

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

## Security review

- Kernel safety review:
- Safety and rollback:
- Reviewer checklist:

## Shell safety review

- Shell safety review:
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

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

## Verification

- [ ] Focused unit tests pass.
- [ ] ./run_sigma_tests.sh
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Storage documentation
- [ ] Boot matrix, if applicable

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:

## Risks and rollback

- Risks and rollback:
- Risk:
- Rollback:
- Recovery or downgrade procedure:
