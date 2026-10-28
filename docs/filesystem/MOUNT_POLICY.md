# Filesystem Mount Policy Specification

## Architectural Overview

`FilesystemCoreManager` and `MountManager` govern VFS mount table lifecycle, directory node isolation, capacity bounds, and failure-safe unmounting.

## Core Requirements & Rules

### 1. Safe Mount Lifecycle
- Mount requests verify target path validity and mount point availability.
- Mount points are registered atomically in the system mount table.

### 2. Capacity Bounds & Lost-Update Prevention
- Filesystems track used capacity (`used_capacity_bytes`) and maximum bounds (`total_capacity_bytes`).
- Capacity checks occur prior to block writes; exceeding limits returns `FsCoreError::CapacityExceeded`.
- File write operations update file sizes atomically; lost updates are strictly prevented.

### 3. Failure-Safe Unmount & Double-Unmount Safety
- Unmounting flushes remaining dirty blocks and unregisters the mount point.
- Unmounting an unmounted target or nonexistent mount point returns `MountError::NotMounted` or `MountError::DoubleUnmount`.
- Mount failures during setup perform immediate rollback of allocated mount resources.
