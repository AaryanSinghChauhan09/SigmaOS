# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux Unix Sockets & FIFO Pipes
- FreeBSD Capsicum & POSIX IPC
- OpenBSD pledge/unveil file descriptor lifecycle

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS IPC mechanisms.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks prior to message transmission or channel opening.
3. Cross-process tests are mandatory for every IPC mechanism.
4. File descriptor passing must validate permissions and ownership at boundary.
5. Signal delivery must prevent race conditions and lost wakeups.

## Prohibited

- Unchecked cross-process handle leakage
- Capability bypass during IPC message delivery
- Deadlocks in synchronous IPC rendez-vous channels
- Memory leaks on terminated process endpoints

## Required evidence

- Cross-process pipe and signal unit test report
- Unix socket message queue integration test report
- Fuzzing report for IPC serialization payloads
- Capability denial test logs

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and IPC interface specification |
| Prototype | Unit tests for pipes and signals |
| Integrated | Cross-process IPC integration suite in QEMU |
| Hardened | Concurrency stress testing and boundary security review |
| Supported | Multi-process desktop and service IPC verification |
