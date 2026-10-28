# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (pipes, signals, domain sockets)
- FreeBSD Capsicum IPC
- xv6 pipe implementation

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS Inter-Process Communication components.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.
4. Resource cleanup must occur automatically when process endpoints terminate.

## Prohibited

- Unchecked IPC message buffers or unbounded queueing
- Unauthenticated IPC channels bypassing capability checks
- Orphaned IPC channels after process exit
- Status promotion without multi-process integration tests

## Required evidence

- IPC unit test report
- Cross-process end-to-end test execution log
- Resource leakage audit log on process termination
- Regression test for every IPC bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
