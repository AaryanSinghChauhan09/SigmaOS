# Architectural Decision Record: VFS Architecture and File Subsystem Boundaries

- **Status:** Approved
- **Deciders:** SigmaOS Architecture Committee & File Subsystem Stewards
- **Date:** 2026-03-30

---

## 1. Context and Problem Statement

SigmaOS requires a unified, safe, and observable Virtual Filesystem (VFS) architecture capable of orchestrating multiple storage engines (`SigmaFs`, `Btrfs`, `XFS`, `bcachefs`, `OverlayFs`, `tmpfs`, `procfs`, `sysfs`) while providing standard POSIX file descriptor and lifecycle semantics to userspace processes.

Model-only file experiments or unintegrated abstractions present risks of descriptor leaks, lost lock updates, double-unlinking, or unhandled I/O failures. A canonical decision is required to solidify VFS contracts, process fd tables, and thread-safe locking models.

---

## 2. Decision Outcome

SigmaOS adopts a **hybrid request-based / inode-based VFS model** with process-local handle tables (`ProcessFdTable`) and explicit permissions/locking managers.

### Architectural Invariants

1. **Explicit Lifecycle State Machine:** All file accesses strictly adhere to `open -> read/write -> seek -> lock -> metadata -> close -> cleanup`.
2. **Process-Local Handle Architecture:** `FileDescriptor` instances map process fd integers (`i32`) to reference-counted `FileHandle` instances containing thread-safe atomic offsets (`AtomicU64`).
3. **Decoupled Security Gate:** All permission checks pass through `PermissionEngine` before performing inode state modifications or block I/O operations.
4. **Advisory & Byte-Range Locking Discipline:** Lock enforcement in `FileLockManager` maintains conflict detection and automatic release on fd close or process termination.
5. **Exact-Once Resource Release:** File descriptors, inode references, and advisory locks are guaranteed to be released exactly once without double-free defects or dangling handle leaks.

---

## 3. Subsystem Boundaries and Responsibilities

```text
+-------------------------------------------------------------------+
|                        Userspace / System Calls                   |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
|               Process File Descriptor Table (ProcessFdTable)      |
|               - Allocation, dup/dup2, CLOEXEC handling             |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
|               Permission & Access Engine (PermissionEngine)       |
|               - UID/GID, DAC mode bits, Capabilities              |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
|               File Lock Manager (FileLockManager)                 |
|               - Advisory flock & POSIX range locks                |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
|               Inode & Directory Manager (InodeManager, Directory) |
|               - Path resolution, inode ref counts, unlinking      |
+-------------------------------------------------------------------+
                                  |
                                  v
+-------------------------------------------------------------------+
|               Virtual Filesystem Operations Layer (VFS Core)      |
|               - Mount table, buffer cache, storage device I/O     |
+-------------------------------------------------------------------+
```

---

## 4. Consequences and Compliance

- **Positive Impact:** Eliminates descriptor leaks, race conditions during offset mutation, and deadlocks across concurrent filesystem operations.
- **Enforcement:** Enforced via `VFS_RULES.toml` and standalone test suites (`tests/fs/` and `tests/filesystem/`).
