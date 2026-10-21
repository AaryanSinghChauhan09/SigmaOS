# SigmaOS File Management Policy Specification

## Subsystem Overview

SigmaOS treats **file management** (VFS, file descriptors, inodes, directories, permissions, locking, and file lifecycle) as an authoritative, observable, recoverable lifecycle subsystem. The primary objective is:

> **Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.**

---

## Architectural Classification

- **VFS Class:** Request-based, inode-based hybrid VFS layer.
- **File Descriptor Class:** Bounded fd-table per process mapping to shared `FileHandle` ref-counted kernel objects.
- **Inode Class:** Inode-based virtual filesystem nodes with explicit state machines (`Unallocated`, `Allocated`, `UnlinkedPendingClose`, `Freed`).
- **Directory Semantics:** POSIX-compatible hierarchical tree with bounded directory capacity, linear scanning, and duplicate entry prevention.
- **Permission Semantics:** POSIX owner/group/other mode bits (`0rwx-rwx-rwx`) with Linux-style `CAP_DAC_OVERRIDE` and `CAP_FOWNER` capability overrides.
- **Lock Semantics:** Advisory & mandatory range locking with deadlock prevention and automatic cleanup on process termination / descriptor release.
- **Metadata Semantics:** Coherent inode metadata updates (UID, GID, mode, size, timestamps) with lock-ordering invariants.
- **Error Policy:** Typed, diagnosable, non-panicking `FsError` status codes returned on all failure paths.
- **Limits:** Hard caps on per-process file descriptors (default: 1024), global inodes (default: 65,536), directory capacity (default: 512 entries per directory inode), and range lock table size (default: 256 per file).
- **NUMA Behavior:** Non-Uniform Memory Access domain-aware inode allocation and buffer cache page mapping where hardware topologies report multiple NUMA nodes.
- **File Hotplug Behavior:** Graceful detachment and invalidation of mount points and descriptors referencing removed backing storage devices without memory or resource leaks.

---

## Authoritative File Lifecycle Contract

The file management subsystem enforces an exact 7-stage lifecycle state machine:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

### State Contract Requirements

| State | Required Behavior |
|---|---|
| **Open** | File descriptor allocated race-free, inode resolved, permissions verified, file handle bound. |
| **Read/Write** | I/O requests validated against descriptor open mode flags, queued, and completed exactly once. |
| **Seek** | File position offset updated in a thread-safe atomic manner. |
| **Lock** | Byte-range locks (`flock`/`fcntl`) requested, evaluated for conflicts, and enqueued failure-safely. |
| **Metadata** | Inode attributes updated coherently without losing concurrent modifications. |
| **Close** | Descriptor slot cleared, process reference count decremented, and unlinked inodes reaped when refcount reaches 0. |
| **Close-on-Exec** | Descriptors flagged with `FD_CLOEXEC` explicitly released during `execve` process image replacement. |
| **Error** | File I/O failures return typed `FsError` without corrupting filesystem structures or triggering kernel panic. |

---

## Concurrency & Lock Ordering Invariants

To eliminate deadlocks and race conditions, kernel code must adhere to the global lock hierarchy:

1. **Process FD Table Lock** (`fd_table.lock`)
2. **Global Mount Table Lock** (`mount_manager.lock`)
3. **Inode Manager Lock** (`inode_manager.lock`)
4. **Individual Inode Lock** (`inode.lock`)
5. **File Lock Manager Lock** (`lock_manager.lock`)
6. **Block Cache / Buffer Pool Lock** (`cache.lock`)

No lock higher in the hierarchy may be acquired while holding a lower-level lock.

---

## Periodic Subsystem Cadence

- **Daily:** Nightly CI triage and error log analysis.
- **Weekly:** VFS lifecycle review and Linux/BSD comparative study.
- **Biweekly:** File descriptor table memory leak and descriptor reuse race audit.
- **Monthly:** Boot-to-shell file validation smoke test, lock enforcement rehearsal, and fuzzing triage.
- **Quarterly:** VFS Architecture Review (ADR), threat model updates, and release readiness verification.
