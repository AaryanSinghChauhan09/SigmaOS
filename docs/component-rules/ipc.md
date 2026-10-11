# Component Rules: IPC (Inter-Process Communication)

## Component owner
ipc-agent

## Upstream references
- Linux IPC (pipes, signals, shared memory, Unix sockets)
- FreeBSD IPC & Capsicum rights
- Redox OS IPC Schemes

## Purpose
Regulate inter-process communication mechanisms, lifecycle management, and security checks.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC endpoints and channels must enforce explicit capability checks on initialization and data transfer.
3. Cross-process integration tests are mandatory for every IPC mechanism change.
4. Signal handling must frame user-space stacks safely without privilege escalation.
5. Shared memory regions must enforce clear read/write permission bounds and cleanup on process termination.

## Prohibited

- Unchecked IPC message buffer reads or writes
- Resource leaks in open pipe file descriptors or message queue nodes
- Process isolation bypasses via shared memory channels
- Promoting IPC status without cross-process integration test evidence

## Required evidence

- Unit test suite for message packing/unpacking (`cargo test --lib`)
- Cross-process communication test results (`./run_sigma_tests.sh`)
- Signal delivery and trampoline verification log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and protocol definition |
| Prototype | Unit tests for queue/pipe buffer operations |
| Integrated | Cross-process execution evidence in QEMU runtime |
| Hardened | IPC fuzzing and buffer overflow denial tests |
| Supported | Multi-process stress-tested runtime evidence |
