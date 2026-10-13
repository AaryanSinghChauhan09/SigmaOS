# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (Pipes, Unix Domain Sockets, System V / POSIX IPC)
- FreeBSD Capsicum & IPC
- Mach / Plan 9 Message Passing

## Purpose
Define safety boundaries, lifecycle management, capability checks, and inter-process communication rules for SigmaOS.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and socket mechanisms must have defined ownership and lifecycle rules.
2. Every IPC channel creation and endpoint attachment must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.
4. IPC buffers must enforce strict bounds and backpressure controls to prevent unbounded memory allocation.
5. Signal delivery must handle signal masks and user framing deterministically without losing kernel invariants.

## Prohibited

- Unchecked message queue or pipe buffer allocation
- Bypassing capability checks on Unix domain socket or pipe connection
- Unhandled IPC deadlock or zombie channel retention
- Marking IPC features integrated without cross-process test logs

## Required evidence

- Unit test suite for IPC channel primitives
- Cross-process communication test report
- Fuzzing report for message/packet parsers
- QEMU runtime evidence for process signaling and pipe data transfers

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests for IPC data structures |
| Integrated | Verified cross-process message passing in QEMU environment |
| Hardened | IPC parser fuzzing and denial-of-service stress tests |
| Supported | Production IPC benchmarks and complete capability isolation |
