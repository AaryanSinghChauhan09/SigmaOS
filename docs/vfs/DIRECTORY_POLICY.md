# SigmaOS Directory Policy (DIRECTORY_POLICY)

## Core Objectives

- **Explicit Inode Lifecycle:** Inodes transition predictably through state changes and track active link counts and handle reference counts.
- **Bounded Directory Capacity:** Directories enforce hard limits on maximum contained entries (`MAX_DIRECTORY_ENTRIES`) to ensure predictable memory usage and execution bounds.
- **Race-Safe Operations:** Mutating directory operations (lookup, insertion, deletion) use atomic or lock-guarded directory structures.
- **Lost-Update Prevention:** Directory modifications update directory block metadata coherently before returning success.
- **Double-Unlink Prevention:** Unlinking a non-existent entry or unlinking an entry twice returns `FsError::NotFound`.
- **OOM Enforcement:** Allocating entries beyond directory capacity returns `FsError::StorageFull`.

---

## Directory Entry & Inode Invariants

### 1. Inode State Transitions

```text
Unallocated -> Allocated (create/mkdir) -> Active (open/link) -> Unlinked (unlink) -> Freed (refcount == 0)
```

- When `nlink` drops to 0 while open handles exist, the inode state becomes `UnlinkedPendingClose`.
- Physical block release occurs only when all active file handles and open descriptors are closed.

### 2. Directory Entry Layout
Each directory entry contains:
- `name`: Fixed-length or string slice entry name (1 to 255 characters).
- `inode_id`: Target inode identifier (`u64`).
- `file_type`: Type indicator (`RegularFile`, `Directory`, `Symlink`, `Fifo`, `Socket`, `CharDevice`, `BlockDevice`).

### 3. Duplicate & Unlink Rules
- **Creation:** `add_entry(name, inode_id)` fails with `FsError::FileAlreadyExists` if `name` is already present.
- **Deletion:** `remove_entry(name)` removes the matching entry, decrements the target inode's `nlink`, and returns the removed `inode_id`.
- **Non-Empty Directory Deletion:** `rmdir` on a directory inode fails with `FsError::DirectoryNotEmpty` if entries other than `.` and `..` exist.
