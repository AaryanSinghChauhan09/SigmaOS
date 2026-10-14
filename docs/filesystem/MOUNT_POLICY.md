# SigmaOS Mount & Filesystem Core Policy Specification

## 1. Overview

This document specifies the safety, lifecycle, and concurrency rules for filesystem mounting and core operations in `src/filesystem/core.rs` (`FilesystemCoreManager`) and `src/filesystem/mount.rs` (`MountManager`).

---

## 2. Mount Lifecycle & Rollback Mechanics

### 2.1 Race-Free Mounting
- File system mounting registers device sources to mount targets in a thread-safe registry (`MountManager`).
- Re-mounting an already active mount target returns `MountError::AlreadyMounted`.

### 2.2 Failure Safety & Rollback
- If superblock verification, journal replay, or root inode initialization fails during mount execution:
  1. Any partially allocated mount table entries are rolled back.
  2. Acquired block device handles are released.
  3. The error is returned as `MountError::InitializationFailed`.

---

## 3. Unmount Safety & Concurrency

### 3.1 Lost-Update & Dirty Data Flush
- Performing an `unmount` triggers a mandatory flush of all cached inodes, directory entries, and dirty pages associated with the filesystem.
- If flush encounters an unrecoverable write error, the filesystem transitions to `FsHealthStatus::ReadOnly` or `FsHealthStatus::Corrupted` and returns an error.

### 3.2 Double-Unmount Prevention
- Target mount status transitions to `Unmounting` before resource release.
- Subsequent unmount requests on the same path return `MountError::NotMounted`.

---

## 4. Filesystem Capacity & Resource Limits

### 4.1 Bounded Capacity & Usage
- `FilesystemCoreManager` tracks total capacity, used blocks, and free blocks (`total_blocks`, `used_blocks`).
- Block allocation requests exceeding available capacity return `FsCoreError::NoSpace`.

### 4.2 Health Status Monitoring
- Filesystems maintain health status markers: `Healthy`, `Degraded`, `ReadOnly`, `Corrupted`.
- I/O failures automatically transition the health status to `ReadOnly` to prevent further disk corruption.
