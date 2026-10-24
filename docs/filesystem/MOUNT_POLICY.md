# SigmaOS Filesystem Mount Policy

## Overview
The filesystem core (`src/filesystem/core.rs`) and mount manager (`src/filesystem/mount.rs`) govern volume capacity boundaries, filesystem transactions, mount table entry lifecycles, and unmount operations.

## Requirements

1. **Capacity Bounding**: Volumes track total/used/free byte and inode capacity bounds. Exceeding capacity returns `FsCoreError::CapacityExceeded`.
2. **Double-Unmount Prevention**: Calling unmount on an already unmounted or missing mount point returns `MountError::NotMounted` or `MountError::DoubleUnmount`.
3. **Transaction Journaling**: Sequence numbers prevent lost updates during file transaction commits (`FsCoreError::LostUpdateDetected`).
4. **Failure-Safe Rollback**: Mount failures restore previous mount table state cleanly without leaking resources.
