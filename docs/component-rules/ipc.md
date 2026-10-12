# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux pipes, UNIX domain sockets, signals, and POSIX message queues
- FreeBSD IPC & Capsicum descriptor passing
- OpenBSD IPC pledging

## Purpose
Define ownership, lifetime, capability checks, and backpressure rules for inter-process communication mechanisms.

## Mandatory rules
1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC mechanisms must enforce capability and namespace checks before message routing.
3. Cross-process tests are mandatory for every IPC mechanism.
4. Non-blocking IPC paths must handle EOF, buffer saturation, and timeout bounds explicitly.
5. Signal delivery must prevent race conditions and uncatchable signal bypasses.

## Prohibited
- Unbounded kernel buffer allocation on IPC message queues
- Unsanitized descriptor passing across isolation boundaries
- Silent dropping of IPC error states
- Bypassing namespace sandbox policies

## Required evidence
- Unit tests for pipes, signals, and domain sockets
- Cross-process concurrency and stress test log
- IPC fuzz target execution results
- Capability denial test output

## Promotion criteria
| Level | Requirement |
|---|---|
| Proposed | IPC architecture ADR |
| Prototype | Unit tests for basic pipe/signal semantics |
| Integrated | Cross-process integration tests in runtime path |
| Hardened | IPC fuzzing, capability denial tests, security audit |
| Supported | Proven multi-process IPC stability in boot/shell tests |
