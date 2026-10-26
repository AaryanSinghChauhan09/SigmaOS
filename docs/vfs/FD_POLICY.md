# File Descriptor Management Policy

## Overview

The file descriptor manager (`src/fs/fd_table.rs` and `src/fs/fd.rs`) manages process-local file descriptor allocation, table indexing, handle reference binding, and lifecycle release.

## Key Rules & Invariants

1. **Race-Free Allocation**: `ProcessFdTable` allocates descriptors sequentially starting from the lowest available non-negative integer slot.
2. **Reuse Policy**: Descriptor slots are returned to the free pool immediately upon `close(fd)`. The next allocation reuses the lowest free descriptor.
3. **Exhaustion Handling**: When the table reaches `max_fds` (default: 1024), descriptor allocation fails with `FsError::NoFileDescriptorsAvailable`.
4. **Reserved Streams**: Descriptors 0, 1, and 2 are pre-allocated or reserved for standard input, standard output, and standard error.
5. **Double-Free Protection**: Closing an unallocated descriptor slot or an already closed slot returns `FsError::InvalidFileDescriptor` without modifying table state.
6. **Close-on-Exec (`FD_CLOEXEC`)**: During `execve`, the process descriptor table iterates through all entries and automatically closes any descriptor marked with `FD_CLOEXEC` / `O_CLOEXEC`.
7. **`dup` and `dup2` Semantics**:
   - `dup(fd)` allocates the lowest available descriptor pointing to the same underlying `FileHandle`.
   - `dup2(old_fd, new_fd)` closes `new_fd` if open and atomically assigns `new_fd` to point to `old_fd`'s file handle.

## Testing & Verification

File descriptor table management is tested via `tests/fs/fd_table.rs` including exhaustion, cloexec flags, dup2 replacement, and race safety.
