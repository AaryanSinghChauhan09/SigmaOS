# Filesystem Core & Mount Manager Policy Specification

## 1. Scope & Purpose

This document specifies the operational policies, capacity limits, mount state transitions, and recovery semantics enforced by `FilesystemCoreManager` (`src/filesystem/core.rs`) and `MountManager` (`src/filesystem/mount.rs`).

---

## 2. Mount Lifecycle State Machine

Mount operations follow a deterministic 4-state lifecycle:

```text
Unmounted -> Mounting -> Mounted -> Unmounting -> Unmounted
```

### State Transitions & Rules
- **Mount Request**: Initiating a mount transitions target state from `Unmounted` to `Mounting`.
- **Initialization & Options**: Applies `CustomMountFlags` (read-only, no-exec, no-suid, sync, bind).
- **Mount Completion**: On successful driver initialization, state transitions to `Mounted`.
- **Rollback Guarantee**: If mount initialization fails, dirty mount entries are undone and state rolls back to `Unmounted`, returning `Err(MountError::RollbackSuccessful)`.
- **Double-Unmount Prevention**: Calling `unmount` on an already unmounted or non-existent path returns `Err(MountError::DoubleUnmount)`.

---

## 3. Capacity Bounds & Transaction Isolation

### 3.1 Bounded Filesystem Capacity
- Every volume registered with `FilesystemCoreManager` maintains strict `VolumeCapacityBounds`: `total_bytes`, `used_bytes`, `free_bytes`, `total_inodes`, `used_inodes`.
- Space allocation (`allocate_space`) checks bounds. Requests exceeding capacity fail with `Err(FsCoreError::CapacityExceeded)`.

### 3.2 Sequence Tracking & Lost-Update Prevention
- File transactions (`begin_transaction`, `commit_transaction`) assign sequence numbers.
- Committing a transaction with an out-of-order sequence number returns `Err(FsCoreError::LostUpdateDetected)`.

### 3.3 Health Status Monitoring
- Volume health is tracked via `FsHealthStatus`: `Healthy`, `Degraded`, `ReadOnly`, `Corrupted`.

---

## 4. Verification

Enforced via unit test suite `tests/filesystem/core.rs`:
- `test_filesystem_core_capacity_and_lost_update_prevention`
- `test_mount_lifecycle_and_double_unmount_prevention`
- `test_mount_rollback_on_failure`
