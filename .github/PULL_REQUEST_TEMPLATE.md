## Summary

<!-- What file-management behavior changed and why? -->

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

## Component
- [ ] Thread manager
- [ ] Scheduler
- [ ] Process management
- [ ] I/O component
- [ ] Memory component
- [ ] File component
- [ ] IPC handler

## Motivation
- Upstream study and architectural parity

## Implementation notes
- Native Rust implementations across subsystems

## Tests
- Comprehensive unit and integration test coverage

## Verification
- `cargo check --lib`
- `cargo test --lib`
- `./run_sigma_tests.sh`

## Capability status
- Verified and validated

## Integration impact
- Non-breaking additive extensions

## Reviewer checklist
- [ ] Scope is narrow
- [ ] `unsafe` justified

## Shell component
- [ ] Shell terminal
- [ ] Desktop compositor

## Shell safety review
- [ ] Input sanitization verified

## I/O component
- [ ] Interrupt manager
- [ ] DMA manager
- [ ] Buffer manager
- [ ] I/O scheduler

## I/O policy
- [ ] I/O completeness
- [ ] Interrupt integrity
- [ ] Buffer semantics

## I/O safety review
- [ ] No buffer cache leaks
- [ ] Double DMA prevented
- [ ] Spurious interrupt handling verified

## IPC handler
- [ ] IPC semantics
- Blocking behavior: non-blocking or bounded
- Non-blocking behavior: returns WouldBlock
- EOF / closure behavior: explicit clean disconnection
- Security review: Capability/namespace checks enforced
