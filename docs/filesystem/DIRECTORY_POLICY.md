# Inode and Directory Management Policy

## Overview
This document defines the inode lifecycle state machine, directory capacity limits, and file unlinking invariants for the SigmaOS Virtual File System.

## Inode Lifecycle State Machine
Each inode progresses through an explicit, trackable state machine:
`Allocated (0) -> Active (1) -> Unlinked (2) -> Reclaimed (3)`

- **Allocated**: Inode block assigned by allocator.
- **Active**: Inode referenced by one or more directory entries or open file descriptors (`nlink > 0` or `ref_count > 0`).
- **Unlinked**: Inode removed from directory hierarchy (`nlink == 0`), but remains readable/writable while open handles exist (`ref_count > 0`).
- **Reclaimed**: All open handles closed and `nlink == 0`; data blocks freed.

## Directory Operation Invariants
1. **Bounded Directory Capacity**:
   - Every directory is bounded by `MAX_DIR_ENTRIES = 4096`. Operations exceeding this bound return `FsError::StorageFull`.
   - Filenames are limited to `MAX_FILENAME_LEN = 255` bytes. Operations exceeding this bound return `FsError::InvalidOperation`.

2. **Race-Safe Directory Operations**:
   - Directory lookups, additions (`add_entry`), and removals (`remove_entry`) execute under mutex / read-write locks to prevent duplicate entries or concurrent corruption.

3. **Double-Unlink Prevention**:
   - Attempting to remove an entry that does not exist in a directory returns `FsError::NotFound`.
   - Unlinking an active inode decrements `nlink`. When `nlink` reaches 0, the inode transitions to `Unlinked`.

4. **Lost-Update Prevention**:
   - Concurrent directory updates serialize changes so directory state remains fully coherent across processes.
