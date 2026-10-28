# SigmaOS Filesystem Core & Mount Policy

## 1. Overview

The SigmaOS filesystem core and mount manager (`src/filesystem/core.rs` and `src/filesystem/mount.rs`) manage volume capacity, transaction sequencing, and mount/unmount lifecycles.

## 2. Invariants and Safety Guarantees

### 2.1 Race-Free Mount Operations
- Mount points transition through `Unmounted` -> `Mounting` -> `Mounted` -> `Unmounting` -> `Unmounted`.
- Concurrent mount requests on the same path return `MountError::AlreadyMounted`.

### 2.2 Double-Unmount Prevention
- Attempting to unmount an unmounted or invalid target returns `MountError::DoubleUnmount`.

### 2.3 Bounded Filesystem Capacity
- Total volume capacity (`VolumeCapacityBounds`) tracks total bytes, used bytes, free bytes, and inode counts.
- Operations exceeding volume capacity return `FsCoreError::CapacityExceeded`.

### 2.4 Lost-Update Prevention
- Filesystem transactions use strictly increasing sequence numbers (`FileTransactionRecord`).
- Out-of-order or duplicate updates trigger `FsCoreError::LostUpdateDetected`.

### 2.5 Failure-Safe Mount Rollback
- If an error occurs during mount setup, all partial allocations are undone and the mount entry rolls back to `Unmounted` (`MountError::RollbackSuccessful`).
