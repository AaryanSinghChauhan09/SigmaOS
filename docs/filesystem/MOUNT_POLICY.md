# Mount Subsystem Policy

## 1. Scope and Overview

The Mount Subsystem (`src/filesystem/core.rs`, `src/filesystem/mount.rs`) governs filesystem creation, volume capacity limits, file transaction tracking, and the mount/unmount lifecycle for all file trees in SigmaOS.

## 2. Mount Lifecycle State Machine

Each mount point entry (`MountEntry`) in `MountManager` follows a strict 4-state lifecycle:

```text
Unmounted -> Mounting -> Mounted -> Unmounting -> Unmounted
```

### State Rules
- **Unmounted**: Path target is free for new mount operations.
- **Mounting**: Device source and target path validated, flags checked, and superblock initialized.
- **Mounted**: Filesystem fully bound and active in mount tree.
- **Unmounting**: Syncing dirty buffers, checking active references, and unbinding.

## 3. Mount Invariants & Error Rules

1. **Duplicate Mount Prevention**: Mounting an already mounted path returns `Err(MountError::AlreadyMounted)`.
2. **Double Unmount Prevention**: Unmounting an unmounted target path returns `Err(MountError::DoubleUnmount)`.
3. **Resource Busy Protection**: Unmounting a filesystem with active open file descriptors or process current working directories fails with `Err(MountError::ResourceBusy)`.
4. **Custom Flags**: Support for `read_only`, `no_exec`, `no_suid`, `sync`, and `bind`.

## 4. Volume Capacity & Transaction Durability

1. **Capacity Enforcement**: Volume total bytes and total inodes are bounded (`VolumeCapacityBounds`). Exceeding capacity returns `Err(FsCoreError::CapacityExceeded)`.
2. **Lost Update Prevention**: Transactions track sequence numbers (`FileTransactionRecord`). Out-of-order or stale commits return `Err(FsCoreError::LostUpdateDetected)`.
