# File Descriptor Management Policy

## Overview
This document specifies the rules and invariants governing file descriptor allocation, process-local table management, handle sharing, and cloexec semantics in SigmaOS.

## Requirements and Invariants

1. **Race-Free Descriptor Allocation**:
   - Process file descriptor tables (`ProcessFdTable`) allocate lowest-available non-negative integer descriptors atomically under table lock protection.

2. **Defined Reuse Policy**:
   - Closed file descriptor slots become immediately available for reuse in subsequent `open` or `dup` calls.
   - Closed slots are scrubbed to `None` before being returned to the free pool.

3. **Typed Exhaustion Errors**:
   - Attempting to allocate beyond the configured `max_fds` limit (default 1024) immediately returns `FsError::NoFileDescriptorsAvailable`.

4. **Protected Reserved Descriptors**:
   - Standard descriptors (`0 = STDIN_FILENO`, `1 = STDOUT_FILENO`, `2 = STDERR_FILENO`) are protected against accidental uninitialized reuse and are allocated during process initialization.

5. **Exact-Once Release**:
   - Calling `close(fd)` releases the table entry and decrements the underlying `Arc<FileHandle>` reference count exactly once. Double-close returns `FsError::InvalidFileDescriptor`.

6. **Lost-Completion Prevention on Close**:
   - Pending I/O operations associated with a file descriptor must complete or be explicitly canceled before the file descriptor entry is invalidated.

7. **Close-on-Exec (`FD_CLOEXEC`)**:
   - `execve` iteration over `ProcessFdTable` automatically closes any descriptor where `flags & FD_CLOEXEC != 0`.
