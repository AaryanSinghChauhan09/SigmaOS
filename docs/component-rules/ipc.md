# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (pipes, UNIX domain sockets, futex)
- FreeBSD IPC
- Redox schemes IPC

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Inter-Process Communication components.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.

## Prohibited

- Unbounded queue allocations or buffer overflows
- IPC endpoints bypassing process capability checks
- Leaking file descriptors or endpoints across fork/exec boundaries
- Dangling shared memory mappings

## Required evidence

- Cross-process interoperability unit and integration tests
- Capability rejection / denial tests
- Non-blocking and blocking semantics test reports
- Pipe EOF and teardown regression tests

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Cross-process test suite and runtime path integration |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
