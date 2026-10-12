# SigmaOS VFS File Management Policy

## Overview & Objective

Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

## Authoritative File Lifecycle State Machine

SigmaOS defines one authoritative file lifecycle contract:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

| State | Required Behavior |
|---|---|
| Open | File descriptor allocated, inode resolved, permissions checked |
| Read/Write | Requests validated, queued, completed exactly once |
| Seek | Offset updated race-safe |
| Lock | Lock lifecycle failure-safe |
| Metadata | Metadata updates coherent |
| Close | Descriptor and references released exactly once |
| Close-on-exec | Descriptor semantics explicit (`FD_CLOEXEC` handling) |
| Error | Failure does not corrupt filesystem state |

## Subsystem Architecture & Classification

- **VFS Class**: Hybrid (Request-based and Inode-based) VFS enabling high-throughput block caching and precise POSIX VFS tree operations.
- **File Descriptor Class**: Hybrid process-local bounded descriptor table (`ProcessFdTable`) mapping integer handles (`fd`) to global thread-safe `FileHandle` instances.
- **Inode Class**: Inode-based lifecycle governance (`InodeManager`, `Inode`) with reference counting, link tracking, and dynamic state transitions (`Clean`, `Dirty`, `Unlinked`, `Free`).
- **Directory Semantics**: Bounded entries (`Directory`), race-safe lookup, insertion, and deletion, with explicit double-unlink and lost-update protection.
- **Permission Semantics**: POSIX octal mode bitwise checks combined with capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`) evaluated by `PermissionEngine`.
- **Lock Semantics**: Fail-safe BSD `flock` file locking and POSIX byte-range locking (`FileLockManager`, `FileRangeLock`) with overlap detection and automatic cleanup on process descriptor release.
- **Metadata Semantics**: Atomically updated inode metadata (`size`, `atime`, `mtime`, `ctime`, `nlink`, `uid`, `gid`, `mode`).

## Error Policy & Limits

- **Typed Errors**: All file-management paths return strongly typed `FsError` codes (e.g., `NotFound`, `PermissionDenied`, `InvalidFileDescriptor`, `NoFileDescriptorsAvailable`, `StorageFull`, `OutOfMemory`, `BadOffset`).
- **Bounded Storage & Memory**: File descriptor tables are hard-capped (`DEFAULT_MAX_FDS = 1024` per process). Directory entries are hard-capped (`MAX_DIR_ENTRIES = 1024`).
- **NUMA & Storage Hotplug**: File operations support NUMA-local slab allocations for buffers and handles. Hotplug detach triggers explicit invalidation of pending operations without dangling memory or lost completions.
