# SigmaOS VFS & File Management Subsystem Policy

## Overview
SigmaOS manages **file management** (VFS, file descriptors, inodes, directories, permissions, locking, and file lifecycle) as a safe, observable, recoverable lifecycle subsystem.

## Objective
Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

## Authoritative File Lifecycle Contract
SigmaOS defines one authoritative file lifecycle:

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
| Close-on-exec | Descriptor semantics explicit |
| Error | Failure does not corrupt filesystem state |

## Subsystem Classes & Design Policies

### 1. VFS Class: Hybrid Request/Inode Architecture
- **Design**: Blends POSIX inode-based lookup with asynchronous, request-oriented I/O dispatch.
- **Path Validation**: Strictly canonicalized and validated before resolution; path traversing bounds-checked.
- **Error Handling**: All I/O errors are strongly typed (e.g., `NotFound`, `PermissionDenied`, `FdExhausted`, `StorageError`), diagnosable, and propagated cleanly without leaving corrupt filesystem or kernel state.

### 2. File Descriptor Class: Hybrid Descriptor/Handle Table
- **Design**: Process-local table mapping integer descriptors (FDs) to ref-counted underlying File Handles.
- **Allocation & Protection**: FDs 0 (stdin), 1 (stdout), and 2 (stderr) are protected reserved descriptors. FD allocation is race-free via atomic bitsets.
- **Resource Cleanup**: Closing an FD releases descriptor slots and decreases handle ref-counts cleanly. No FD is freed twice.

### 3. Inode & Directory Semantics
- **Lifecycle**: Inodes manage metadata, link counts, and storage blocks. Inode ref-counts prevent premature deletion during open handle operations.
- **Directory Operations**: Directories enforce bounded child entries, race-safe lookup/insertion/removal, double-unlink protection, and atomic directory rename operations.

### 4. Permission & Lock Semantics
- **Permissions**: Mode bits (`rwxrwxrwx`), UID/GID matching, capability tokens, and Landlock/Unveil path sandbox restrictions.
- **Locks**: POSIX advisory range locking (`fcntl`) and BSD file locking (`flock`). Lock queues prevent race conditions, deadlocks, and lost-lock scenarios.

### 5. Limits, NUMA, and Hotplug
- **Limits**: Fixed process maximum FDs, maximum system open inodes, and bounded directory capacity.
- **NUMA & Hotplug**: Node-local buffer caching where supported; graceful detachment and unmount error recovery under file hotplug events.
