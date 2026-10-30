# SigmaOS File Management Subsystem Policy

## Overview and Objective
Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

## Authoritative Lifecycle State Machine
All file operations in SigmaOS follow an authoritative, single file lifecycle contract:
`open -> read/write -> seek -> lock -> metadata -> close -> cleanup`

| State | Required Behavior |
|---|---|
| Open | File descriptor allocated, inode resolved, permissions checked |
| Read/Write | Requests validated, queued, completed exactly once |
| Seek | Offset updated race-safe |
| Lock | Lock lifecycle failure-safe |
| Metadata | Metadata updates coherent |
| Close | Descriptor and references released exactly once |
| Close-on-exec | Descriptor semantics explicit (`FD_CLOEXEC`) |
| Error | Failure does not corrupt filesystem state |

## Subsystem Architecture Classes

### 1. VFS Class
- **Type**: Hybrid (Request-Based + Inode-Based).
- **Behavior**: Request-based async I/O dispatching layered on top of strictly reference-counted, unique inode structures in memory.

### 2. File Descriptor Class
- **Type**: Hybrid (Descriptor Index + Arc-backed FileHandle).
- **Behavior**: Process-local non-negative integer table indices mapping to shared, thread-safe file handle atomic structures with atomic offset control.

### 3. Inode Class
- **Type**: Inode-Based.
- **Behavior**: Unique uint64 inode identifier tracking nlink counter, type, atomic state transitions (`Allocated -> Active -> Unlinked -> Reclaimed`), owner UID/GID, mode permissions, and file byte size.

## Semantics and Guarantees

### Directory Semantics
- Directory capacities are strictly bounded (`MAX_DIR_ENTRIES = 4096`).
- Directory filename lengths bounded to 255 bytes (`MAX_FILENAME_LEN = 255`).
- Directory lookups, additions, and removals are protected by interior read/write locks.
- Double-unlink prevention ensured via atomic state verification.

### Permission Semantics
- Bitfield permission representation (POSIX 0o777 octal mode mask: owner, group, other read/write/execute flags).
- Privilege override capabilities checked via Process Credentials (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
- Denied access returns typed error `FsError::PermissionDenied`.

### Lock Semantics
- Advisory range locking (`F_RDLCK`, `F_WRLCK`, `F_UNLCK`) and `flock` BSD-style locking (`LOCK_SH`, `LOCK_EX`, `LOCK_NB`, `LOCK_UN`).
- Overlapping range merges and splits are managed safely without lost locks or dangling allocations.
- Double-unlock prevention is guaranteed.

### Metadata Semantics
- Metadata updates are atomic and atomic-state coherent across processes.
- Modification and access timestamps maintained with explicit offset sync.

### Error Policy and Bounds
- All error states produce typed `FsError` variants (`NotFound`, `PermissionDenied`, `InvalidFileDescriptor`, `NoFileDescriptorsAvailable`, `FileAlreadyExists`, `IsADirectory`, `NotADirectory`, `DirectoryNotEmpty`, `ResourceBusy`, `StorageFull`, `OutOfMemory`, `IoError`, `BadOffset`, `InvalidOperation`).
- No unbounded allocations permitted in kernel queues, caches, or directory entry tables.

### NUMA and Hotplug Behavior
- NUMA memory allocations for page cache and descriptor tables favor local node domain memory.
- File hotplug/disconnection triggers graceful invalidation (`FsError::IoError`), completing in-flight requests before unmounting.
