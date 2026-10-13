# SigmaOS Mount & Filesystem Core Policy Specification

## 1. Overview

The Filesystem Core Manager (`FilesystemCoreManager` in `src/filesystem/core.rs`) and Mount Table Manager (`MountManager` in `src/filesystem/mount.rs`) govern volume capacity bounds, mount table entries, transaction journaling, and filesystem lifecycle safety.

## 2. Invariants & Policy Directives

### 2.1 Bounded Filesystem Capacity & Quotas
- Volume size is capped by `max_capacity_bytes`.
- Allocating beyond capacity returns `Err(FsCoreError::VolumeFull)`.

### 2.2 Lost-Update & Crash Prevention
- Transactions are recorded with transactional journaling (`FileTransactionRecord`).
- Concurrent mutations maintain consistent inode and metadata state.

### 2.3 Safe Mount & Unmount Lifecycle
- Active mounts transition strictly through `MountState::Mounted` -> `MountState::Unmounting` -> `MountState::Unmounted`.
- Mounting over an active mount point or unmounting an unmounted volume returns `Err(MountError::AlreadyMounted)` or `Err(MountError::NotMounted)`.

### 2.4 Double-Unmount Prevention
- Calling `unmount()` on a target that is already in state `Unmounted` or `Unmounting` fails safely without corrupting the mount table or scheduler references.

### 2.5 Rollback on Mount Failure
- If initialization fails during `mount()`, the mount manager automatically executes rollback procedures to clean up partial mount entries and free locked resources.

### 2.6 Health & OOM Observability
- Filesystem health status (`FsHealthStatus::Healthy`, `Degraded`, `ReadOnly`, `Corrupted`) and capacity usage are continuously exposed for diagnostic monitoring.
