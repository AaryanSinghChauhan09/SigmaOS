# Filesystem Mount & Core Policy

## Overview
The filesystem core manager (`src/filesystem/core.rs`) and mount manager (`src/filesystem/mount.rs`) govern volume initialization, capacity accounting, and active mount lifecycle management.

## Core Requirements

### 1. Safe Mount & Unmount Lifecycle
- Mount points are path-based and registered in a thread-safe `MountManager`.
- Mounting over an existing mount point returns `MountError::AlreadyMounted`.
- Unmounting non-existent mount points returns `MountError::NotMounted`.

### 2. Double-Unmount Prevention
- Once a mount entry transitions to `Unmounting` or is removed, subsequent unmount calls fail safely returning `MountError::AlreadyUnmounted`.

### 3. Bounded Capacity & Bounded Filesystem State
- `FilesystemCoreManager` tracks total bytes and used bytes.
- Writing data exceeding available capacity returns `FsCoreError::NoSpace`.

### 4. Lost-Update Prevention
- Metadata and transaction logs (`FileTransactionRecord`) enforce atomic updates.
- Transaction records are committed prior to returning success on filesystem modification operations.

### 5. Failure-Safe Rollback
- If a mount operation fails during driver or superblock initialization, all allocated mount resources are rolled back immediately without leaking memory or table slots.
