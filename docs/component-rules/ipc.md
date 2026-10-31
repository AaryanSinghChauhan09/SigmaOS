# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (pipes, UNIX domain sockets, futex)
- FreeBSD IPC (capsicum, kqueue)
- Redox Schemes IPC

## Purpose
Define required behavior, ownership, lifetime rules, capability checks, and validation
for SigmaOS Inter-Process Communication (pipes, signals, shared memory, message queues).

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.

## Prohibited

- Orphaned shared memory mappings without reference tracking
- Bypassing capability checks on IPC endpoints
- Unbounded kernel message queues that risk denial of service
- Undefined signal disposition behavior across process lifecycles

## Required evidence

- Unit test report for IPC buffer management
- Cross-process end-to-end data transfer integration tests
- Denial test results confirming unauthorized processes cannot open IPC channels
- Non-blocking and EOF lifecycle behavioral test logs

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | IPC ADR and protocol specification |
| Prototype | Single-process pipe and queue unit tests |
| Integrated | Cross-process execution path tests in QEMU |
| Hardened | IPC fuzzing and starvation denial tests |
| Supported | Multi-client high-throughput benchmark validation |
