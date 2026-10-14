# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux Pipes, Signals, and System V / POSIX IPC
- FreeBSD IPC Primitives
- Mach Microkernel Port Messaging

## Purpose
Define required ownership, lifecycle, backpressure, and capability rules for SigmaOS IPC mechanisms.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. Every IPC operation must enforce capability checks prior to message delivery or channel establishment.
3. Cross-process tests are mandatory for every IPC mechanism.
4. Non-blocking IPC operations must yield standardized error codes (e.g. `EAGAIN` / `EWOULDBLOCK`) on exhaustion.
5. IPC buffer allocations must be bounded and explicitly enforce backpressure limits.

## Prohibited

- Unbounded message queue allocation or kernel stack overflow on signal dispatch
- Bypassing capability verification on cross-process endpoints
- Silent packet loss or unhandled channel closure conditions
- Lock-ordering violations across process table and IPC locks

## Required evidence

- Unit test report for pipe, signal, and socket operations
- Cross-process interoperability and stress test logs
- IPC fuzzing report (e.g. `fuzz/fuzz_targets/ipc_parser.rs`)

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | IPC semantics ADR and thread-safety review |
| Prototype | Standalone pipe/signal unit tests |
| Integrated | Cross-process communication and scheduler integration tests |
| Hardened | IPC fuzzing, capability denial tests, security review |
| Supported | Multi-process stress and boot-to-shell evidence |
