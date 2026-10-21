# SigmaOS File Descriptor Policy (FD_POLICY)

## Core Objectives

- **Race-Free Allocation:** File descriptor indices are allocated using a deterministic strategy (lowest available non-reserved slot).
- **Descriptor Reuse Policy:** Released file descriptor integers are made available for reuse only after complete handle disassociation and resource release.
- **Typed Exhaustion Errors:** Exhaustion of per-process FD capacity returns `FsError::NoFileDescriptorsAvailable`.
- **Reserved Descriptors:** Standard streams (`0 = stdin`, `1 = stdout`, `2 = stderr`) are protected against accidental implicit release during standard operations.
- **Lost-Completion Prevention:** `close()` operations flush pending metadata/buffer operations before clearing the FD slot.
- **Double-Free Prevention:** Clearing an unallocated or previously freed FD returns `FsError::InvalidFileDescriptor`.

---

## File Descriptor State Machine

```text
Unallocated -> Allocated (open/dup) -> Active (read/write/seek) -> Closed (close) -> Unallocated
                                     \
                                      -> Exec-Released (execve + CLOEXEC) -> Unallocated
```

---

## Semantics & Operational Invariants

### 1. Descriptor Allocation (`allocate_fd`)
- Scans `descriptors` array starting at index 0 (or index 3 if skipping standard streams during user allocation).
- Finds the first `None` slot, places `FileDescriptor`, and returns `Ok(fd)`.
- If no slot is free up to `MAX_DESCRIPTORS_PER_PROCESS`, returns `Err(FsError::NoFileDescriptorsAvailable)`.

### 2. Descriptor Duplication (`dup` / `dup2`)
- `dup(old_fd)` allocates the lowest available integer and points it to the same `FileHandle`.
- `dup2(old_fd, new_fd)` closes `new_fd` if currently open (handling `FD_CLOEXEC` clearing) and binds `new_fd` to `old_fd`'s handle.
- Attempting to `dup2` an invalid `old_fd` fails with `FsError::InvalidFileDescriptor` without modifying `new_fd`.

### 3. Execution Flags (`FD_CLOEXEC`)
- Each `FileDescriptor` maintains a boolean flag for `FD_CLOEXEC`.
- `close_cloexec_descriptors()` is invoked during `execve()` image replacement, closing all descriptors where `is_cloexec() == true`.

### 4. Close and Reference Counting
- Closing an FD disassociates the `FileDescriptor` from the process table.
- Decrements the reference count on the underlying `FileHandle`.
- When `FileHandle` refcount reaches 0, the associated inode's active handle count is decremented.
