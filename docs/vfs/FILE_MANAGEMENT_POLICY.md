# SigmaOS VFS File Management Subsystem Policy

## Overview
SigmaOS manages **file management**—VFS, file descriptors, inodes, directories, permissions, locking, and file lifecycle—as a lifecycle subsystem: recurring development, compatibility, recovery, and maintenance work, each landed as a small, evidence-backed pull request.

## Authoritative File Lifecycle Contract
Every file object in SigmaOS follows an explicit 7-stage state machine:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

| State | Required Behavior |
|---|---|
| **Open** | File descriptor allocated, inode resolved, permissions checked |
| **Read/Write** | Requests validated, queued, completed exactly once |
| **Seek** | Offset updated race-safe |
| **Lock** | Lock lifecycle failure-safe |
| **Metadata** | Metadata updates coherent |
| **Close** | Descriptor and references released exactly once |
| **Close-on-exec** | Descriptor semantics explicit (`FD_CLOEXEC`) |
| **Error** | Failure does not corrupt filesystem state |

## System Specifications
- **VFS Class**: Request-based hybrid Virtual File System with object caching and mounted backends.
- **File Descriptor Class**: Process-local fd table referencing kernel-managed file handles (`FileHandle`).
- **Inode Class**: 64-bit atomic inode identifier with state lifecycle tracking (`Allocated`, `Active`, `Unlinked`, `Reclaimed`).
- **Directory Semantics**: Bounded entries (`MAX_DIR_ENTRIES = 4096`), entry validation, race-safe append/remove.
- **Permission Semantics**: POSIX mode bit mask evaluation, user/group IDs, effective credentials, and Linux capability checks (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
- **Lock Semantics**: BSD `flock` and POSIX `fcntl` file range locking with deadlock prevention and failure-safe cleanup on close.
- **Metadata Semantics**: Coherent update propagation across inodes and directory entries.
- **Error Policy**: Typed `FsError` return codes (`NotFound`, `PermissionDenied`, `FileExists`, `FdTableFull`, `InvalidFd`, `IoError`, `IsDirectory`, `NotADirectory`, `DirectoryNotEmpty`, `ResourceBusy`, `WouldBlock`, `InvalidArgument`, `TooManyOpenFiles`).
- **Limits**: Process table max FDs bounded (default 1024), max filename length 255 chars, max dir entries 4096.
- **NUMA Behavior**: Local memory pool allocations for inode caches and page descriptors where available.
- **File Hotplug Behavior**: Dynamic mount/unmount device discovery, exact-once resource detachment, and mount integrity verification.
