# Filesystem Mount Manager Policy

## Overview
The Filesystem Mount Manager (`MountManager`) in `src/filesystem/mount.rs` controls filesystem mounting, active reference management, unmounting, and transactional rollback.

## Requirements & Rules

1. **Safe Mount & Unmount Lifecycle**:
   - Valid mount states: `Unmounted`, `Mounting`, `Mounted`, `Unmounting`.
   - Attempting to mount an already mounted target returns `MountError::AlreadyMounted`.

2. **Active Reference Tracking**:
   - `acquire_ref` and `release_ref` maintain open file descriptor and process handle counts (`open_ref_count`).
   - Unmounting a target with `open_ref_count > 0` returns `MountError::ResourceBusy`.

3. **Double-Unmount Protection**:
   - Unmounting an unmounted target returns `MountError::NotMounted` or `MountError::DoubleUnmount`.

4. **Transactional Rollback**:
   - `mount_with_rollback` executes custom initialization hooks.
   - If initialization fails, mount state is atomically restored to `Unmounted` and returns `MountError::RollbackSuccessful`.
