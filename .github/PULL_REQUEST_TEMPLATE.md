## Summary

<!-- What kernel behavior changed and why? -->

## Kernel component

- [ ] Boot / arch init
- [ ] Interrupts / traps
- [ ] Memory / paging / swap
- [ ] Scheduler / processes
- [ ] VFS / filesystems
- [ ] Syscalls / ELF
- [ ] Drivers
- [ ] Networking / sockets
- [ ] IPC / signals / timers
- [ ] Security / capabilities
- [ ] Diagnostics
- [ ] Build / CI

## Summary

<!-- What secondary-memory behavior changed and why? -->

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

- Reference study:
- Upstream project, document, or behavior reviewed:
- POSIX section:
- Bash/dash/Zsh/BusyBox reference:
- SigmaOS current behavior:
- What SigmaOS adopts:
- What remains intentionally different:
- Compatibility impact:

## Reference behavior

<!-- Required for lifecycle, ABI, or storage changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
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
- Capability/namespace checks enforced:

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] Fuzz target or reason for exemption
- [ ] QEMU boot evidence, if boot-affecting
- [ ] Driver/hardware evidence, if driver-affecting
- [ ] Security review requested, if applicable

## IPC handler

- [ ] This change affects IPC handler behavior, or I confirm it does not.
- IPC semantics:
- Blocking behavior:
- Non-blocking behavior:
- EOF / closure behavior:

## Verification

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] Focused unit tests pass.
- [ ] New/updated shell tests
- [ ] POSIX/conformance test evidence
- [ ] Fuzz target or documented exemption
- [ ] Boot-to-shell evidence, if boot-affecting
- [ ] Security review requested, if applicable
- [ ] Documentation and wiki mirrors updated where applicable.
- Test/evidence commands:
- Results:

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

## Risks and rollback

- Risk:
- Rollback:
- Recovery or downgrade procedure:

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:
