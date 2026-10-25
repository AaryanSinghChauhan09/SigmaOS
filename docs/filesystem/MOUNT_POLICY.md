# SigmaOS Filesystem Core & Mount Manager Policy

## Overview

The filesystem management layer (`src/filesystem/core.rs` and `src/filesystem/mount.rs`) governs filesystem volume allocation, mount table lifecycle transitions, reference counting, lost-update prevention, and failure-safe rollback mechanics.

---

## Core Invariants

### 1. Failure-Safe Mount Lifecycle State Machine
Mount entries transition through a safe lifecycle state machine:
`Unmounted` -> `Mounting` -> `Mounted` -> `Unmounting` -> `Unmounted`

- **Already Mounted**: Mounting a target path that is already in `Mounted` state returns `MountError::AlreadyMounted`.
- **Failure Rollback**: `mount_with_rollback` executes driver initialization callbacks while the target is in `Mounting` state. If driver initialization fails, the mount target is cleanly rolled back to `Unmounted` and removed from the mount table without leaving orphaned entries.

### 2. Active Reference Counting & Double-Unmount Prevention
- **Reference Tracking**: Open file handles and working directories increment `open_ref_count` via `acquire_ref` and decrement via `release_ref`.
- **Active Lock**: Unmounting a filesystem with `open_ref_count > 0` fails immediately with `MountError::ResourceBusy`.
- **Double-Unmount Protection**: Attempting to unmount a target that has already been unmounted returns `MountError::NotMounted`.

### 3. Filesystem Capacity Bounds
- Volume capacity is hard-bounded by `VolumeCapacityBounds` specifying `total_bytes` and `total_inodes`.
- Space allocation (`allocate_space`) checks remaining free bytes and free inodes. Allocations exceeding `total_bytes` or `total_inodes` fail cleanly with `FsCoreError::CapacityExceeded`.
- Deallocations (`free_space`) saturate at zero, preventing integer underflow corruption.

### 4. Lost-Update Prevention
- Concurrent or sequential updates to file metadata/data use transaction logging via `begin_transaction` and `commit_transaction`.
- Transactions record sequence numbers (`sequence_number`). Committing a transaction with a stale sequence number returns `FsCoreError::LostUpdateDetected`, safeguarding filesystem structure against race conditions.

---

## Error Specification

### Mount Errors (`MountError`)
| Variant | Trigger Condition |
|---|---|
| `AlreadyMounted` | Target mount point is already mounted |
| `NotMounted` | Unmount requested on non-existent or unmounted path |
| `DoubleUnmount` | Target mount point was already unmounted |
| `ResourceBusy` | Active open file references prevent unmounting |
| `InvalidSource` | Specified device or block source path is invalid |
| `InvalidTarget` | Mount point directory does not exist or is invalid |
| `InitializationFailed` | Driver/filesystem startup failed |
| `RollbackSuccessful` | Mount failed and system cleanly rolled back state |

### Filesystem Core Errors (`FsCoreError`)
| Variant | Trigger Condition |
|---|---|
| `AlreadyExists` | Volume name already registered |
| `NotFound` | Volume name or transaction ID not found |
| `CapacityExceeded` | Requested allocation exceeds volume byte/inode capacity |
| `TransactionFailed` | Transaction handle is invalid or cancelled |
| `LostUpdateDetected` | Expected sequence number mismatch during transaction commit |
