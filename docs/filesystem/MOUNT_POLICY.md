# SigmaOS Filesystem & Mount Policy

## Overview
This document specifies the filesystem core manager and mount subsystem contracts implemented in `src/filesystem/core.rs` and `src/filesystem/mount.rs`.

## Requirements & Invariants

### 1. Race-Free Mount Operations
- All mount operations register in the global mount table (`MountManager`) protected by atomic lifecycle checks.
- A mount entry transitions monotonically: `Unmounted -> Mounting -> Mounted -> Unmounting -> Unmounted`.
- Concurrent mount attempts targeting the same mount point or block device are serialized or rejected.

### 2. Bounded Filesystem Capacity
- Volumes register with strict capacity bounds (`VolumeCapacityBounds`):
  - Total and used storage bytes (`total_bytes`, `used_bytes`).
  - Total and used inode counts (`total_inodes`, `used_inodes`).
- Allocation requests exceeding free space return `Err(FsCoreError::CapacityExceeded)` immediately.

### 3. Failure-Safe Mount / Unmount and Rollback
- If volume verification, journal replay, or permission checks fail during `mount()`, `MountManager` triggers an automatic rollback (`MountError::RollbackSuccessful`), restoring the mount state cleanly to `Unmounted`.
- No orphan mounts or half-mounted filesystems can linger.

### 4. Lost-Update Prevention
- File transactions and metadata modifications record atomic sequence numbers (`FileTransactionRecord`).
- When committing changes via `commit_transaction(tx_id, sequence_number)`, the core verifies that the transaction sequence matches expected monotonic progression.
- Mismatched or stale sequence commits return `Err(FsCoreError::LostUpdateDetected)` and are rejected.

### 5. Double-Unmount Prevention
- Attempting to unmount a target that is already unmounted or not registered returns `Err(MountError::NotMounted)` or `Err(MountError::DoubleUnmount)`.
- References and allocated resources are freed exactly once.

### 6. Observable Health and OOM Enforcement
- Volume health status is continuously tracked in `health_map` (`Healthy`, `Degraded`, `ReadOnly`, `Corrupted`).
- Capacity thresholds and out-of-space events emit structured diagnostics, enabling administrative intervention before hard filesystem corruption occurs.
