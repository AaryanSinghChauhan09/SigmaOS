# SigmaOS Filesystem Core & Mount Policy

## Overview

The filesystem core (`src/filesystem/core.rs`) and mount manager (`src/filesystem/mount.rs`) govern volume capacity, directory entry paths, transaction records, mount namespace registration, and unmount cleanup.

## Mount Lifecycle & Safety

Mount entries transition through an atomic lifecycle:

```text
Unmounted ---> Validate/Initialize ---> Mounted ---> Unmount Sequence ---> Unmounting ---> Freed ---> Unmounted
```

### Mount Contract Requirements

1. **Atomic Registration**: Mounting a filesystem validates the device source, mount point target path, and custom mount flags (`CustomMountFlags`) before adding the entry to the global mount table.
2. **Failure Rollback**: If filesystem initialization or superblock verification fails at any point during mount, all allocated data structures are freed, the mount target reservation is released, and `MountError::InitializationFailed` (or specific cause) is returned without modifying the mount table.
3. **Double Unmount Prevention**: Unmounting checks the entry's atomic state. If the target is already unmounted or in `Unmounting` state, the request returns `MountError::NotMounted` or `MountError::AlreadyUnmounted`. A mount point cannot be scheduled or dereferenced after it is freed.
4. **Active Handle Lockout**: Unmounting fails with `MountError::Busy` if active file descriptors, working directory references, or child mounts exist on the mount target.

## Bounded Capacity & Lost-Update Prevention

### Capacity Bounding
- Every filesystem instance specifies `total_blocks` and `free_blocks`.
- Allocation requests exceeding available capacity return `FsCoreError::NoSpace`.
- Quotas and volume capacity limits are strictly enforced.

### Lost-Update Prevention
- File write operations log transactions using `FileTransactionRecord`.
- Dirty metadata buffers (inodes, block bitmaps) are locked during modification and added to the commit transaction list.
- Unmount and flush routines force a synchronous flush of all pending transaction records to physical storage before marking the filesystem clean.

## Observability & Diagnostic Health

Filesystem health is monitored via `FsHealthStatus`:
- `Healthy`: Standard operational mode.
- `Degraded`: Non-critical metadata repair performed or redundant mirror degraded.
- `ReadOnly`: I/O error threshold breached; write operations disabled to preserve data integrity.
- `Corrupted`: Severe structural corruption detected; mount rejected until recovery/FSCK is run.
