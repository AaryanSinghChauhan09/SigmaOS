# File Locking Policy

## Overview
This document specifies the invariants and lifecycle of advisory `flock` and POSIX byte-range locking in the SigmaOS Virtual File System.

## Locking Mechanisms

### 1. Advisory Whole-File Locking (`flock`)
- **Shared Lock (`LOCK_SH`)**: Multiple processes can hold shared locks simultaneously.
- **Exclusive Lock (`LOCK_EX`)**: Only one process can hold an exclusive lock; fails if shared or exclusive locks exist.
- **Non-Blocking Flag (`LOCK_NB`)**: Returns `FsError::ResourceBusy` immediately if the lock cannot be acquired.
- **Unlock (`LOCK_UN`)**: Releases held lock.

### 2. POSIX Byte-Range Locking (`FileRangeLock`)
- **Lock Types**: Read (`F_RDLCK`), Write (`F_WRLCK`), Unlock (`F_UNLCK`).
- **Range Invariants**: Specified by `start` offset and `len` bytes (`len == 0` signifies through end-of-file).

## Invariants and Guarantees
1. **Race-Free Enqueue and Dequeue**:
   - Lock requests acquire lock-table mutex protection during inspection, insertion, splitting, or removal.
2. **Double-Unlock Prevention**:
   - Releasing an unheld lock returns `FsError::NotFound` or succeeds silently without altering active locks.
3. **Lost-Lock Prevention on Process Termination / FD Close**:
   - Closing a file descriptor automatically removes all byte-range locks owned by the process on that inode.
4. **Bounded Metadata**:
   - Total active locks per process and per inode are bounded to prevent OOM exhaustion attacks.
