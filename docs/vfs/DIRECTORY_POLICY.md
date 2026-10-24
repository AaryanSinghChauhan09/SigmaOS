# Directory & Inode Management Policy

## 1. Overview

Directories and inodes form the structural core of the SigmaOS Virtual Filesystem. Inodes store metadata and content locations, while directories map textual filenames to inode identifiers.

---

## 2. Inode Lifecycle & Metadata Invariants

1. **Inode Structure (`Inode`):**
   - Identifiers: `inode_id` (unique 64-bit integer).
   - Metadata: `mode`, `uid`, `gid`, `size`, `atime`, `mtime`, `ctime`.
   - References: `ref_count` (in-memory handles) and `nlink` (directory hard links).
2. **State Lifecycle:**
   - Active -> Unlinked (`nlink == 0`) -> Closed (`ref_count == 0`) -> Reclaimed.
   - An inode with `nlink == 0` remains accessible as long as `ref_count > 0` (e.g. open handles). Once `ref_count` reaches zero, inode data blocks and metadata are freed.

---

## 3. Directory Invariants & Operations

1. **Capacity Boundaries:**
   - Maximum directory capacity: `MAX_DIR_ENTRIES` (4096 entries per directory).
   - Maximum filename length: `MAX_FILENAME_LEN` (255 characters).
2. **Reserved Entries:**
   - Every directory contains `.` (self) and `..` (parent inode).
   - Attempts to unlink `.` or `..` return `FsError::InvalidOperation`.
3. **Double-Unlink & Lost-Update Prevention:**
   - Directory lookups (`lookup`), entry additions (`add_entry`), and entry removals (`remove_entry`) are race-safe.
   - Adding an existing filename returns `FsError::FileAlreadyExists`.
   - Removing a non-existent entry returns `FsError::NotFound`.
