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

- [ ] Interactive shell
- [ ] Command interpreter

## Component

- [ ] Shell component
- [ ] PR-level validation

## Shell component
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

## Reference study (Upstream study)

## Shell component

- Upstream study:
- Linux/BSD/Redox/xv6 reference:
- Decision: Adopt / Adapt / Reject (What SigmaOS adopts)
- Reason:

## Reviewer checklist

- [ ] Reviewer checklist completed

## Changes

-

## Shell safety review

- [ ] Quote-aware lexer & parser safety
- [ ] Safe environment expansion
- [ ] Built-in command error handling

## Kernel safety review

- [ ] No unnecessary `unsafe`

## Shell safety review

- [ ] Shell safety review
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Error paths tested
- [ ] No silent failure introduced

## Shell safety review

- [ ] Deterministic script execution
- [ ] Explicit exit status on error
- [ ] Quoting and globbing expansion verified
- [ ] No partial execution on parse error
- [ ] Signal handling and redirection safe

## Reviewer checklist

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

- Risk:
- Rollback:

## Reviewer checklist

- [ ] Verification complete
