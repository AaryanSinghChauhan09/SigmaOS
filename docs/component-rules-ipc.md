# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (pipes, signals, unix sockets)
- FreeBSD IPC & Capsicum Casper
- Redox OS Scheme IPC

## Purpose
Define required behavior, safety boundaries, and validation for SigmaOS IPC components.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. IPC must enforce capability checks.
3. Cross-process tests are mandatory for every IPC mechanism.
4. Zero-copy IPC channels must validate buffer boundaries and process access privileges.

## Prohibited

- Unchecked shared memory mappings across untrusted processes
- Bypassing capability validation in message passing
- Dangling IPC file descriptor handles

## Required evidence

- Cross-process interoperability test report
- Capability denial test report
- QEMU IPC benchmark/test log

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and gap analysis |
| Prototype | Unit tests |
| Integrated | Cross-process integration tests |
| Hardened | Fuzzing & security review |
| Supported | End-to-end multi-process evidence |
