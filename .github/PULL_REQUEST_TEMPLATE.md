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

## Related issue / study

Closes #<issue-id>
- Upstream study:
- Reference study / ADR:

## Component

- [ ] Boot / platform
- [ ] Kernel
- [ ] Memory
- [ ] Scheduler
- [ ] VFS / filesystems
- [ ] Syscalls
- [ ] Drivers
- [ ] Networking
- [ ] IPC / IPC handler
- [ ] Security
- [ ] Userspace
- [ ] Desktop
- [ ] Packaging / installer
- [ ] Documentation
- [ ] CI / tooling

## Shell component

- [ ] Interactive shell
- [ ] Builtins
- [ ] Lexer / Parser
- [ ] Environment / Variables

## What SigmaOS adopts / changes

<!-- State design principles adopted, deviations, or IPC semantics / shell behavior -->
- IPC semantics:
  - Blocking behavior:
  - Non-blocking behavior:
  - EOF / closure behavior:

## Shell component

- Component affected:

## Shell safety review

- Security review status:

## Changes

-
-

## Safety review

### Kernel safety review

- [ ] No unnecessary `unsafe`
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Allocation limits defined
- [ ] OOM behavior defined
- [ ] Error paths tested
- [ ] No silent failure introduced

### Shell safety review

- [ ] Quote integrity preserved
- [ ] Safe expansion order enforced
- [ ] Redirection and signal safety verified

### Security review

- [ ] Capability/namespace checks enforced
- [ ] Untrusted input validated

## Status updates

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] New/updated unit tests
- [ ] Integration tests
- [ ] QEMU / runtime test
- [ ] Documentation
- [ ] Status updates (`FEATURE_STATUS.toml` / `CAPABILITY_MATRIX.toml`)

## Shell safety review

- [ ] `cargo check --lib`
- [ ] `cargo fmt --check`
- [ ] `cargo test --lib`
- [ ] `./run_sigma_tests.sh`
- [ ] QEMU boot evidence, if applicable
- [ ] Installer/recovery evidence, if applicable
- [ ] Hardware evidence, if applicable

## Shell safety review

- [ ] Redirection safety verified
- [ ] Signal handling verified
- [ ] No unbounded input buffers

## Safety and rollback

- [ ] Shell safety review completed
- [ ] Security review requested / completed
- [ ] Capability/namespace checks enforced
- [ ] Untrusted input validated
- [ ] Risk:
- [ ] Rollback:

## Reviewer checklist

- [ ] Scope is narrow and well-defined
- [ ] Tests cover positive and failure paths
- [ ] Runtime evidence is linked
- [ ] Documentation reflects actual implementation
