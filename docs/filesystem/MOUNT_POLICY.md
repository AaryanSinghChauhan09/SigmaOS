# SigmaOS Mount and Filesystem Policy

## Overview
The mount and filesystem subsystem handles VFS mount points, filesystem lifecycle state machines,
volume capacity tracking, and transactional durability.

## Core Requirements

### 1. Atomic, Race-Free Mount Operations
- Mount points are registered in a synchronized table (`MountManager`).
- Re-mounting an active path returns `MountError::AlreadyMounted`.
- Mount initialization validates device presence, permissions, and superblock consistency.
- Any failure during mount setup initiates automatic rollback of acquired locks and data structures.

### 2. Double-Unmount Prevention
- Unmounting transitions mount state: `Mounted -> Unmounting -> Unmounted`.
- Attempting to unmount a path that is not mounted returns `MountError::NotMounted`.
- All file descriptors open on the unmounting filesystem must be drained or revoked prior to detachment.

### 3. Bounded Filesystem Capacity
- Volumes registered with `FilesystemCoreManager` specify fixed storage capacity limits (`total_capacity_bytes`).
- Attempting to allocate blocks beyond remaining volume capacity returns `FsCoreError::CapacityExceeded`.

### 4. Lost-Update Prevention
- Filesystem core tracks mutations through transactional records (`FileTransactionRecord`).
- Committing a transaction verifies current file version against the transaction start version.
- Mismatched versions indicate concurrent mutation and return `FsCoreError::LostUpdateDetected`.

### 5. Health Monitoring and Recovery
- Filesystem health status is tracked per volume (`FsHealthStatus::Healthy`, `Degraded`, `ReadOnly`, `Corrupted`).
- Storage errors cause automated transition to `ReadOnly` or trigger self-healing rebuilds.
