## Summary

<!-- What changed and why? -->

## Subsystem / Component

### Kernel component

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

### Shell component

- [ ] Lexer / Parser
- [ ] Expansion & Globbing
- [ ] Pipelines & Redirection
- [ ] Built-ins & Exit Codes
- [ ] Interactive & Prompt
- [ ] Diagnostics & Errors

### IPC handler & IPC semantics

- [ ] Message Queues / Buffers
- [ ] Blocking behavior: timeout / wait
- [ ] Non-blocking behavior: EAGAIN / EWOULDBLOCK
- [ ] EOF / closure behavior: single close / teardown

## Recurring task

## Shell component

<!-- Specify if this affects shell or userland components. -->

## Shell safety review

<!-- Specify shell safety review criteria. -->

## Component impact

## Reference study

<!-- Required for design-changing PRs -->

- Study document:
- Linux/BSD/Redox/xv6 reference:
- Decision: Adopt / Adapt / Reject
- Reason:

## Shell component

- Shell component:
- Grammar/conformance impact:

## Shell safety review

- [ ] Shell safety principles verified

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
- [ ] Fuzz target or reason for exemption
- [ ] QEMU boot evidence, if boot-affecting
- [ ] Driver/hardware evidence, if driver-affecting
- [ ] Security review requested, if applicable

## Status updates

- [ ] Capability status updated in `FEATURE_STATUS.toml` / `CAPABILITY_MATRIX.toml`
- [ ] Integration impact and component documentation updated
- [ ] Boot matrix, if applicable
- [ ] Hardware matrix, if applicable

## Risks and rollback

- Risk:
- Rollback:
