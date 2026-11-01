## Summary

<!-- What changed and why? -->

## Task type

- [ ] Daily CI maintenance
- [ ] Weekly dependency/security review
- [ ] Weekly open-source study
- [ ] Biweekly component gap review
- [ ] Monthly boot validation
- [ ] Monthly installer/recovery rehearsal
- [ ] Monthly package/update rehearsal
- [ ] Quarterly release review
- [ ] Quarterly architecture review
- [ ] Quarterly hardware audit

## Component

- [ ] Boot / platform
- [ ] Kernel
- [ ] Memory
- [ ] Scheduler
- [ ] VFS / filesystems
- [ ] Syscalls
- [ ] Drivers
- [ ] Networking
- [ ] IPC
- [ ] Security
- [ ] Userspace
- [ ] Desktop
- [ ] Packaging / installer
- [ ] Documentation
- [ ] CI / tooling

## Summary

<!-- What behavior changed and why? -->

## Summary

<!-- What I/O-management behavior changed and why? -->

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

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

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly I/O audit
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

## Changes

-
-

## Evidence

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New or updated tests
- [ ] QEMU boot evidence, if applicable
- [ ] Installer/recovery evidence, if applicable
- [ ] Hardware evidence, if applicable
- [ ] Security review requested, if applicable

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Component documentation
- [ ] Roadmap or release checklist

## Reference behavior

<!-- Required for lifecycle, ABI, or I/O changes -->

- POSIX section:
- Linux/BSD/Redox/xv6 reference:
- SigmaOS current behavior:
- New behavior:
- Compatibility impact:

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

## Security review

- Kernel safety review:
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
- Recovery or downgrade procedure:

## Capability status

- Current status:
- Evidence supporting the status:
- Claims intentionally left partial:

## Integration impact

- APIs, ABI, protocols, or file formats affected:
- Cross-repository synchronization required:
- Release or migration notes:
