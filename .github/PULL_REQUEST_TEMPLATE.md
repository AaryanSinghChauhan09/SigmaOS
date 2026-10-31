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

## Component impact

- Component/repository:
- User-visible behavior:
- Related issue or plan:

## Recurring task

- [ ] PR-level validation
- [ ] Daily CI triage
- [ ] Weekly subsystem review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly syscall/ABI review
- [ ] Biweekly memory/concurrency audit
- [ ] Monthly boot validation
- [ ] Monthly driver/hardware audit
- [ ] Monthly fuzz triage
- [ ] Monthly diagnostics rehearsal
- [ ] Quarterly release/architecture/security review

## Reference study

<!-- Required for design-changing kernel PRs -->

- Study document:
- Linux/BSD/Redox/xv6 reference:
- Decision: Adopt / Adapt / Reject
- Reason:

## Upstream study and reference behavior

- Reference study:
- Upstream project, document, or behavior reviewed:
- What SigmaOS adopts:
- What remains intentionally different:

## Changes

-
-

## Kernel safety review

- [ ] No unnecessary `unsafe`
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Allocation limits defined
- [ ] OOM behavior defined
- [ ] Error paths tested
- [ ] No silent failure introduced

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

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Component documentation
- [ ] Boot matrix, if applicable
- [ ] Hardware matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
- Risks and rollback:
- Recovery or downgrade procedure:
