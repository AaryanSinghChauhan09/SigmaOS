# Component Rules: IPC

## Component owner
ipc-agent

## Upstream references
- Linux IPC (Pipes, Unix Sockets, POSIX Message Queues)
- FreeBSD IPC & Capsicum Endpoints
- Mach Microkernel IPC

## Purpose
Define required behavior, safety boundaries, and validation
for SigmaOS Inter-Process Communication (Pipes, Signals, Shared Memory, Message Queues, Semaphores) components.

## Mandatory rules

1. Pipes, signals, shared memory, message queues, and event mechanisms must have defined ownership and lifetime rules.
2. Every IPC operation must enforce explicit capability checks on reader/writer handles.
3. Cross-process communication tests are mandatory for every IPC mechanism.
4. Signal delivery must preserve user-space stack framing (`ucontext_t`, `sigreturn` trampoline) and handle masked signals correctly.
5. IPC buffer operations must be bounded and race-free under concurrent read/write access.

## Prohibited

- Unbounded IPC queue allocations or buffer overflows
- Bypassing capability checks on IPC handle transfers
- Lost wakeup or deadlocks in semaphore/pipe synchronization
- Silent signal dropping for unmasked critical signals
- Parity claims without cross-process integration tests

## Required evidence

- Unit tests for pipes, unix sockets, signal masking, and shared memory
- Cross-process thread/process synchronization integration test report
- QEMU IPC benchmark and correctness log
- Security regression test for IPC access denial without capability tokens

## Promotion criteria

| Level | Requirement |
|---|---|
| Proposed | ADR and IPC model specification |
| Prototype | Unit tests for IPC data structures |
| Integrated | Cross-process pipe and signal delivery integration tests |
| Hardened | IPC stress testing, race condition fuzzing, security review |
| Supported | Multi-process userspace workload execution evidence |
