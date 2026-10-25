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

## Shell component

- [ ] Lexer / parser / AST
- [ ] Variables / expansion
- [ ] Redirection / pipelines
- [ ] Built-ins / environment
- [ ] Job control / signals
- [ ] Terminal / interactive
- [ ] Security / resource limits

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

- [ ] Input/resource limits enforced
- [ ] Quote-aware tokenization and argument boundary preservation
- [ ] Clean process and file descriptor handling on success/failure
- [ ] No untrusted code execution from expansion strings
- [ ] Error paths and non-zero exit status handling verified

## Reference study

- Study document:
- Upstream study:
- Reference:
- What SigmaOS adopts:
- Decision: Adopt / Adapt / Reject
- Reason:

## Safety and rollback

- Risk:
- Safety and rollback:

## Reviewer checklist

- [ ] Code review completed
- [ ] All safety assertions verified
