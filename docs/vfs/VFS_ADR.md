# VFS Architectural Decision Record (ADR)

## Title
SigmaOS Hybrid Request/Inode VFS Architecture & Lifecycle Contract

## Context
SigmaOS requires a safe, high-performance Virtual File System (VFS) to coordinate file descriptors, inodes, directories, locking, and permissions across monolithic and microkernel IPC interfaces. Existing file abstractions must comply with deterministic resource cleanup, race freedom, and POSIX compliance.

## Decision
1. **Hybrid VFS Architecture**: Adopt an inode-centric hierarchy coupled with asynchronous request queues for non-blocking I/O.
2. **Authoritative Lifecycle State Machine**: Enforce the exact state progression: `open -> read/write -> seek -> lock -> metadata -> close -> cleanup`.
3. **Descriptor Isolation**: Isolate process file descriptor tables with atomic bitsets for FD allocation, protecting reserved descriptors (0, 1, 2).
4. **Typed Error Hierarchy**: All VFS routines return explicit, non-panicking typed errors. Unbounded operations and panic paths are strictly forbidden.

## Consequences
- **Pros**: Clean isolation between process file handles and kernel VFS layers; race-free descriptor allocation and cleanup; full auditability under stress tests.
- **Cons**: Requires explicit ref-counting and double-unlock / double-unlink validation logic across all VFS entry points.
