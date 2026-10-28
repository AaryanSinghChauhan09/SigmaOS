# Filesystem Core and Mount Manager Specification and Policy

## 1. Overview

The Filesystem Core Manager (`src/filesystem/core.rs`) and Mount Manager (`src/filesystem/mount.rs`) enforce volume capacity bounds, transaction sequencing, mount lifecycle, and failure recovery.

## 2. Capacity Bounding & Lost-Update Prevention

### 2.1 Volume Capacity Limits
- Every registered filesystem volume defines explicit limits: `total_bytes` and `total_inodes`.
- Space allocations (`allocate_space`) checking against `used_bytes` return `Err(FsCoreError::CapacityExceeded)` when requested allocation exceeds available volume capacity.

### 2.2 Transaction Logging & Lost-Update Prevention
- Filesystem updates begin transactions (`begin_transaction`), returning unique `tx_id` handles and current file sequence numbers.
- Concurrent updates that modify file sequence numbers before transaction commit cause `commit_transaction` to detect sequencing conflicts and return `Err(FsCoreError::LostUpdateDetected)`.

## 3. Mount Lifecycle and Safety

### 3.1 Mount State Machine
Mount points transition through explicit states:
```text
Unmounted -> Mounting -> Mounted -> Unmounting -> Unmounted
```

### 3.2 Double-Unmount and Error Recovery
- Mounting an already mounted path returns `Err(MountError::AlreadyMounted)`.
- Unmounting an unmounted path returns `Err(MountError::NotMounted)` or `Err(MountError::DoubleUnmount)`.
- Mount initialization failures automatically execute atomic state rollbacks, returning `Err(MountError::RollbackSuccessful)`.
