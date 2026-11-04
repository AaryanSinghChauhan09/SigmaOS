# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux Unix domain sockets, pipes, and eventfd
- FreeBSD POSIX IPC and kqueue
- Plan 9 channel and plumber IPC models

## Purpose
Define required behavior, safety boundaries, lifetime management, and validation
for SigmaOS Inter-Process Communication (pipes, signals, shared memory, and queues).

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.
4. Channel buffers must define backpressure and overflow boundaries.
5. Signal delivery must prevent unbounded queueing and stack corruption.
6. File descriptor tables must atomically register and release IPC endpoints.
7. Disconnected endpoints must return standard broken pipe / EOF indicators.

## Prohibited

- Unchecked buffer overruns in shared memory regions
- Unauthenticated or capability-bypassing message injection
- Deadlocking IPC ringbuffers
- Status promotion without cross-process integration tests

## Required evidence

- Unit test report
- Integration test report
- Cross-process interoperability tests
- Regression test for every bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
