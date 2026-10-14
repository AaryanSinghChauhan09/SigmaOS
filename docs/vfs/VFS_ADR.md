# SigmaOS VFS Architecture Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a safe, observable, recoverable Virtual File System (VFS) subsystem. Code fragments in userland or isolated modules do not establish kernel integration or system resilience without clear subsystem rules, invariants, and strict lifecycle enforcement.

## Decision
1. **Authoritative Lifecycle State Machine**: All VFS objects must strictly follow `open -> read/write -> seek -> lock -> metadata -> close -> cleanup`.
2. **Process-Local File Descriptor Tables**: File descriptors are allocated starting from lowest free slot above reserved standard descriptors (0: Stdin, 1: Stdout, 2: Stderr). FD exhaustion returns `FsError::FdTableFull`.
3. **Double-Release & Lost-Completion Safety**: `close()` operations atomically clear table references and decrement Arc reference counts to ensure resources are freed exactly once.
4. **POSIX-Compatible Permissions**: Permission checks compare process effective credentials (`euid`/`egid`) against inode mode bits (`S_IRUSR`, `S_IWUSR`, etc.) with capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
5. **Conflict-Free File Locking**: Shared and exclusive advisory range locks track process ownership and automatically clean up on file close or process termination.

## Consequences
- Guarantees zero orphan descriptors and zero double-unlink incidents under load.
- Establishes deterministic failure boundaries with typed `FsError` returns.
- Requires explicit verification tests for all VFS operations.
