## Summary

<!-- What secondary-memory, shell, or subsystem behavior changed and why? -->

## Storage component

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
- [ ] Documentation
- [ ] CI / tooling

## Shell component

- [ ] Shell parser
- [ ] Builtins
- [ ] Pipeline

## Shell component

- [ ] Interactive prompt & signals
- [ ] Command execution & builtins
- [ ] Environment & expansion
- [ ] Resource limits governor
- [ ] Short circuit && / ||

## Shell component

- [ ] Interactive shell
- [ ] Command interpreter

## Shell component

- [ ] Interactive prompt & signals
- [ ] Command execution & builtins
- [ ] Environment & expansion
- [ ] Resource limits governor
- [ ] Short circuit && / ||

## Component

- [ ] Lexer / Parser
- [ ] Expansion engine
- [ ] Builtin commands
- [ ] Redirection / Piping
- [ ] Job control / Signals
- [ ] Interactive REPL
- [ ] Diagnostics / Error formatting
- [ ] Conformance / POSIX

## Recurring task

- [ ] PR validation
- [ ] Daily CI triage
- [ ] Weekly lifecycle review
- [ ] Weekly Linux/BSD study
- [ ] Biweekly storage audit
- [ ] Biweekly signal/wait audit
- [ ] Monthly boot validation
- [ ] Quarterly architecture/security/release review

## Reference study & Upstream study

<!-- Required for lifecycle, ABI, or storage changes -->
<!-- Upstream study: Open-source OS references and Linux/BSD/Redox/xv6 study notes -->
<!-- What SigmaOS adopts: Subsystem architecture and component improvements -->

- Upstream study document:
- Linux/BSD/Redox/xv6 reference:
- What SigmaOS adopts / adapts / rejects:
- Decision: Adopt / Adapt / Reject
- Reason:

## What SigmaOS adopts
- Component adaptation:

## Changes

-

## Reviewer checklist

- [ ] Upstream study and reference checked
- [ ] What SigmaOS adopts is clearly stated
- [ ] Safety and rollback reviewed

## Shell safety review

- [ ] Quote-aware lexer & parser safety
- [ ] Safe environment expansion
- [ ] Built-in command error handling

## Kernel safety review & Reviewer checklist

- [ ] Safety and rollback plan defined
- [ ] No unnecessary `unsafe`

## Shell safety review

- [ ] Command injection prevention verified
- [ ] Environment variable sanitization checked
- [ ] Signal trap safety verified
- [ ] Every `unsafe` block documents invariants
- [ ] Lock ordering documented
- [ ] Interrupt-context safety considered
- [ ] Signal races considered
- [ ] Double-completion prevented
- [ ] Resource limits enforced
- [ ] No unbounded allocation or recursion

## Shell component

- [ ] Interactive REPL
- [ ] Scripting engine
- [ ] Job control
- [ ] Builtins

## Shell safety review

- [ ] Input validation
- [ ] Signal handling
- [ ] Memory safety

## Evidence & Reviewer checklist

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

## Safety and rollback

- [ ] Quoting integrity
- [ ] Redirection safety

## Safety and rollback

- Safety and rollback:
- Risk:
- Safety and rollback:
