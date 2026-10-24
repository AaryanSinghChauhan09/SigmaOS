# File Descriptor Table & Management Policy

## 1. Scope & Objectives

The process-local File Descriptor (FD) table manages user-space process interactions with handles, open files, sockets, pipes, and special character devices in SigmaOS.

---

## 2. Descriptor Table Allocation & Boundaries

1. **Table Structure:** `ProcessFdTable` maintains a fixed-capacity vector (`Vec<Option<FileDescriptor>>`) initialized up to `max_fds` (default `1024`, configurable per process).
2. **Standard Descriptors:**
   - `0`: Standard Input (`STDIN_FILENO`)
   - `1`: Standard Output (`STDOUT_FILENO`)
   - `2`: Standard Error (`STDERR_FILENO`)
3. **Allocation Rules:**
   - `allocate_fd` allocates the lowest available integer index starting from `0`.
   - `allocate_fd_above(min_fd)` allocates the lowest available integer index equal to or greater than `min_fd`.
   - When no slots remain, the table returns `Err(FsError::NoFileDescriptorsAvailable)`.

---

## 3. Operations & Safety Contracts

### Descriptor Operations

- **Duplication (`dup`, `dup2`):**
  - `dup_fd(old_fd, min_fd, flags)` clones the underlying `FileHandle` (incrementing atomic reference count) and assigns a new descriptor index.
  - `dup2_fd(old_fd, new_fd)` atomically closes `new_fd` if open (unless `old_fd == new_fd`) and duplicates `old_fd` into `new_fd`.
- **Close-on-Exec (`O_CLOEXEC`, `FD_CLOEXEC`):**
  - `handle_exec()` scans the active entries and drops any `FileDescriptor` with `FD_CLOEXEC` enabled.
- **Closure (`close_fd`):**
  - `close_fd(fd)` removes the entry from the table, releasing the descriptor index and decrementing the handle's reference count. Returns `FsError::InvalidFileDescriptor` if the index is out of bounds or unused.

### Concurrency and Reference Rules

- Offsets in shared `FileHandle` instances are stored as `Arc<AtomicU64>`, guaranteeing thread-safe, atomic seek and read/write offset progress across duplicated descriptors.
- No descriptor is freed twice; table mutations are guarded by process structure locks.
