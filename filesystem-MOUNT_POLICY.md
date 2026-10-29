# Filesystem Mount Policy

## Overview
The filesystem mount subsystem manages volume capacity bounds, atomic mount points, reference-counted file usage, and unmount safety.

## Policy Rules
1. **Volume Bounds**: Registered volumes specify total byte and inode limits. Allocations exceeding capacity return `FsCoreError::CapacityExceeded`.
2. **Lost-Update Prevention**: Write transactions track sequence numbers (`commit_transaction`). Mismatched sequence numbers return `FsCoreError::LostUpdateDetected`.
3. **Mount Reference Safety**: Active files increment open reference count (`acquire_ref`). Unmounting a busy mount point returns `MountError::ResourceBusy`.
4. **Double Unmount Prevention**: Unmounting an unmounted target returns `MountError::NotMounted` or `MountError::DoubleUnmount`.
5. **Rollback Safety**: Failed initializations during `mount_with_rollback` restore `MountState::Unmounted` and purge temporary state.
