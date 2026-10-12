# SigmaOS Filesystem Core & Mount Policy

## Overview

The Filesystem Manager Subsystem (`src/filesystem/core.rs` and `src/filesystem/mount.rs`) handles volume management, capacity bounds, transaction logging, and VFS mount point lifecycle safety.

---

## Core Requirements & Specifications

### 1. Volume Capacity Clamping & Lost-Update Prevention
- **Bounded Capacity**: Each volume enforces `max_capacity_bytes`.
- **Space Allocation**: Requests exceeding available capacity return `Err(FsCoreError::CapacityExceeded)`.
- **Lost-Update Prevention**: Data modifications are recorded in transaction logs (`FileTransactionRecord`). Uncommitted modifications are rolled back during recovery to prevent partial/lost updates.

### 2. Mount / Unmount Lifecycle Safety
Mount point states transition through:
```text
Unmounted -> Mounting -> Mounted -> Unmounting -> Unmounted
```

- **Atomic Mount**: `mount_device(mount_point, source, fs_type, flags)` validates path/device and registers mount entry atomically.
- **Double-Mount Prevention**: Attempting to mount on an already mounted directory returns `Err(MountError::AlreadyMounted)`.
- **Double-Unmount Prevention**: Attempting to unmount a path that is not mounted returns `Err(MountError::NotMounted)`.
- **Failure Recovery**: Initialization failure during mounting triggers automatic rollback, leaving mount point in `Unmounted` state.

---

## Verification & Testing

Compliance is verified via `tests/filesystem/core.rs` and `tests/filesystem_core_tests.rs`:
- Volume capacity clamping and `CapacityExceeded` error handling.
- Transaction record creation and lost-update prevention.
- Safe mount/unmount lifecycle and `DoubleUnmount` prevention.
- Rollback on mount initialization failure.
