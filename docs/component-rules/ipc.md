# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC primitives (pipes, signal, unix sockets)
- FreeBSD Capsicum / IPC mechanisms
- xv6 pipe & signal handling

## Purpose
Define ownership, capability validation, and inter-process lifecycle
for SigmaOS IPC components.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.

## Prohibited

- Unbounded IPC queue allocations
- Unsandboxed or unverified cross-process communication
- Bypassing capability checks on IPC endpoints
- Status promotion without cross-process integration evidence

## Required evidence

- Unit test report
- Cross-process interop test report
- Stress test log for queue limits
- Regression test for every IPC bug fix

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Runtime path plus integration tests |
| Hardened | Fuzzing, negative tests, security review |
| Supported | Reproducible boot/install/update evidence |
