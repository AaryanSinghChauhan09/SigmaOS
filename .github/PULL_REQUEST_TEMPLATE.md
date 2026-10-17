## Summary

<!-- What kernel or component behavior changed and why? -->

## Kernel component

- [ ] Boot / arch init

## Shell component

- [ ] Interactive shell
- [ ] Shell component
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

## Shell component

- [ ] Shell parser / lexer
- [ ] Builtins / environment
- [ ] Redirection / pipes
- [ ] Job control / signals
- [ ] Interactive shell / prompt

## Component

- [ ] Shell component
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

## Reference study / Upstream study

## Shell component

- [ ] Shell component

<!-- Upstream study required for design-changing kernel PRs -->

- Study document:
- Upstream study / Linux/BSD/Redox/xv6 reference:
- What SigmaOS adopts:
- Decision: Adopt / Adapt / Reject
- Reason:

## Changes

-

## Reviewer checklist

- [ ] Upstream study and reference checked
- [ ] What SigmaOS adopts is clearly stated
- [ ] Safety and rollback reviewed

## Shell safety review

- [ ] No unbounded inputs
- [ ] Safe quoting & expansion

## Shell safety review

- [ ] Deterministic script execution
- [ ] Explicit exit status codes
- [ ] Safe expansion order
- [ ] Quoting integrity

## Shell safety review

- [ ] Deterministic script execution verified
- [ ] Explicit exit status codes
- [ ] Safe expansion order
- [ ] Quoting integrity & redirection safety
- [ ] No unbounded input or partial execution on parse error

## Reviewer checklist & Safety and rollback

- [ ] Upstream study notes provided
- [ ] Reviewer checklist completed
- [ ] Safety and rollback plan defined

- [ ] No unnecessary `unsafe`

## Shell safety review

- [ ] Shell safety review
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Error paths tested
- [ ] No silent failure introduced

## Shell safety review

- [ ] Quoting integrity verified
- [ ] Safe expansion order
- [ ] Redirection safety checked
- [ ] Signal safety checked
- [ ] Exit status explicit

## Reviewer checklist

- [ ] Upstream study verified
- [ ] Safety and rollback plan verified
- [ ] Tests and evidence verified

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

## Status updates

- [ ] `FEATURE_STATUS.toml`
- [ ] `CAPABILITY_MATRIX.toml`
- [ ] Component documentation
- [ ] Boot matrix, if applicable
- [ ] Hardware matrix, if applicable

## Safety and rollback

## Shell safety review

- [ ] Shell safety review

- Risk:
- Safety and rollback:
