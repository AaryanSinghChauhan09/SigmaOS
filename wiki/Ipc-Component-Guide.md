# Inter-Process Communication & Message Bus Component Guide

## Component Overview
The IPC subsystem coordinates high-throughput Unix domain sockets, anonymous pipes, shared-memory rings, SigmaBus event broker, and POSIX message queues.

## Linux Inspiration
Linux Unix sockets (AF_UNIX with SCM_RIGHTS descriptor passing), futex fast userspace mutexes, eventfd, and memfd_create.

## BSD Inspiration
FreeBSD kqueue/kevent for unified event notifications, OpenBSD unveil and pledge capability passing, DragonFly BSD port messaging.

## Current SigmaOS Status
Pipes, shared memory segments, and basic SigmaBus message queues implemented. SCM_RIGHTS fd passing and zero-copy futex sync require completion.

## Critical Missing Features
Capability and file descriptor passing across Unix domain sockets, kernel-assisted futex waiting with priority inheritance, and high-performance D-Bus wire protocol compatibility.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/ipc/sigmabus.rs, src/ipc/pipe.rs, src/ipc/shared_mem.rs, src/ipc/futex.rs, src/ipc/unix_socket.rs

## Testing Strategy
IPC round-trip ping-pong benchmarks, multi-producer multi-consumer stress tests, and deadlock detection fuzzing.

## Dependencies
PROCESS_MANAGEMENT_AGENT, MEMORY_MANAGEMENT_AGENT, SECURITY_AGENT

## Success Criteria
Sub-microsecond message transit time, 10M+ messages/second throughput on shared memory buses, zero unauthorized cross-process snooping.

## Open Source Competitors Analysis
Replaces slow userspace dbus-daemon (which throttles GNOME/Linux Mint) with a kernel-accelerated lockless ring delivering 20x throughput.

## Future Enhancements
Zero-overhead shared memory enclaves for local AI model inference streaming.
