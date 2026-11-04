## Summary

<!-- What subsystem behavior changed and why? -->

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
- [ ] Weekly feature/bug triage
- [ ] Weekly POSIX/sh study
- [ ] Biweekly parser/expansion audit
- [ ] Biweekly built-in review
- [ ] Monthly boot-to-shell validation
- [ ] Monthly compatibility suite
- [ ] Monthly fuzz triage
- [ ] Monthly security review
- [ ] Quarterly release/UX/performance review

## Upstream study and reference behavior

- Reference study:
- Upstream project, document, or behavior reviewed:
- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- What SigmaOS adopts:
- What remains intentionally different:
- Compatibility impact:

<!-- Detailed motivation behind the component rule or capability enhancement -->

## Implementation notes

<!-- Architecture decisions, safety invariants, and data structures -->

## Tests

<!-- Description of unit, integration, and fuzz testing added -->

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly feature/bug triage
- [ ] Weekly POSIX/sh study
- [ ] Biweekly parser/expansion audit
- [ ] Biweekly built-in review
- [ ] Monthly boot-to-shell validation
- [ ] Monthly compatibility suite
- [ ] Monthly fuzz triage
- [ ] Monthly security review
- [ ] Quarterly release/UX/performance review

## Reference behavior

<!-- Required for compatibility or grammar changes -->

- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Subsystem safety reviews

### Kernel safety review
- [ ] Preserves boot contract
- [ ] `unsafe` justified
- [ ] Classified panic diagnostics

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

- Unit tests
- Integration tests

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

## Security review

- [ ] Capability/namespace checks enforced

## Reviewer checklist

- [ ] Scope is narrow
- [ ] `unsafe` justified

## Safety and rollback

- Double-completion prevented
- Resource limits enforced

## File safety review

- [ ] Expansion order is correct
- [ ] Quoting behavior is preserved
- [ ] Redirection failures are handled
- [ ] Exit codes are explicit
- [ ] Signal behavior is defined
- [ ] No unbounded recursion or allocation
- [ ] Interactive terminal state is restored
- [ ] Script mode does not execute invalid partial input
- Capability/namespace checks enforced:
- Shared-memory or descriptor implications:
- Input validation and failure behavior:
- Rollback or downgrade path:

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

## Capability status

## Integration impact

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Shell documentation
- [ ] Boot matrix, if applicable
- [ ] Integration impact documented

## Reviewer checklist

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
