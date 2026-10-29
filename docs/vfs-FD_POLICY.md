# SigmaOS File Descriptor Subsystem Policy

## Objective

Govern process-local file descriptor allocation, table lookup, flags, reuse, duplicate descriptor duplication (`dup`, `dup2`), close-on-exec execution, and table teardown in `src/fs/fd_table.rs` and `src/fs/fd.rs`.

## Requirements & Rules

1. **Race-Free Descriptor Allocation**:
   - `ProcessFdTable::allocate_fd()` allocates the lowest available integer file descriptor index `fd >= 0`.
   - Thread-safe synchronization ensures parallel `open` or `dup` calls never allocate the same descriptor index twice.

2. **Descriptor Table Capacity & Exhaustion**:
   - Default process descriptor capacity is bounded (`DEFAULT_MAX_FDS = 1024`).
   - If allocation exceeds the table capacity limit, the manager returns `Err(FsError::NoFileDescriptorsAvailable)`.

3. **Reserved Descriptors**:
   - `STDIN_FILENO = 0`, `STDOUT_FILENO = 1`, and `STDERR_FILENO = 2` are reserved at process spawn.
   - Closing or remapping standard descriptors via `dup2` must maintain exact stream handle semantics.

4. **Close-on-Exec (`FD_CLOEXEC`)**:
   - Per-descriptor flags store `FD_CLOEXEC` (bit `0x01`).
   - `ProcessFdTable::close_cloexec_fds()` iterates through the table during process `execve` and closes all descriptors with `FD_CLOEXEC` enabled.

5. **Duplication (`dup` / `dup2`)**:
   - `dup(oldfd)` allocates a new descriptor index sharing the same underlying `FileHandle`.
   - `dup2(oldfd, newfd)` atomically closes `newfd` if open (unless `oldfd == newfd`) and duplicates `oldfd` to `newfd`.
   - `dup2` automatically clears `FD_CLOEXEC` on the new descriptor per POSIX rules.

6. **Single-Release Invariant**:
   - Closing a descriptor releases its table slot exactly once and decrements the underlying `FileHandle` reference count.
   - Double-close requests return `Err(FsError::InvalidFileDescriptor)`.
