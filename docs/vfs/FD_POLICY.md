# SigmaOS File Descriptor Manager Policy

## Overview

The file descriptor manager governs process-local file descriptor tables (`ProcessFdTable`), mapping integer file descriptors (`i32`) to shared file handle objects (`FileHandle`).

## Key Policy Directives

1. **Race-Free Allocation**: `ProcessFdTable::allocate_fd` locates the lowest unused descriptor index starting from `0` (or `3` for user applications) under a process-level table lock.
2. **Descriptor Reuse**: Once a descriptor is closed (`close_fd`), its numeric slot is freed and made available for subsequent allocation.
3. **Exhaustion Management**: When the allocated descriptor count reaches `max_fds` (default 1024), allocation fails deterministically with `FsError::NoFileDescriptorsAvailable`.
4. **Reserved Descriptor Protection**: `STDIN_FILENO` (0), `STDOUT_FILENO` (1), and `STDERR_FILENO` (2) are pre-allocated upon table creation and protected against unintended unallocated reuse.
5. **Exact-Once Closure**: Closing an unallocated or already-closed descriptor returns `FsError::InvalidFileDescriptor`, preventing double-free bugs.
6. **Close-On-Exec (`FD_CLOEXEC`)**: `close_on_exec()` iterates through the table and closes all file descriptors with the `FD_CLOEXEC` flag set, ensuring clean process boundary transitions during `execve`.
