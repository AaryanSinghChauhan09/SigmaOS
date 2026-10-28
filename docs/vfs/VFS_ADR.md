# SigmaOS VFS Architectural Decision Records (VFS_ADR)

## ADR-0001: Hybrid Request/Inode-Based VFS Architecture

### Context
SigmaOS requires a high-performance, safe, and recoverable Virtual Filesystem (VFS) abstraction capable of supporting diverse file systems (ext4, procfs, tmpfs, autofs, overlay) while enforcing strict memory bounds and POSIX compatibility.

### Decision
SigmaOS adopts a hybrid VFS architecture:
1. **Inode Layer**: Tracks file metadata, reference counts, lock queues, and lifecycle states in memory (`src/fs/inode.rs`).
2. **Handle/Descriptor Layer**: Manages process-local handle tables (`src/fs/fd_table.rs`) mapping integer file descriptors to open file handles with atomic seek offsets and cloexec flags.
3. **Request Router Layer**: Converts VFS operations (open, read, write, seek, lock) into validated requests executed exactly once.

### Consequences
- Guarantees zero descriptor leaks under stress.
- Enforces strict memory bounds without unbounded kernel allocations.
- Provides diagnosable, typed `FsError` status codes across all syscall paths.

---

## ADR-0002: Race-Free Advisory and Range Locking Discipline

### Context
Concurrent processes accessing shared files require synchronization mechanisms (`flock` advisory locking and `fcntl` byte-range locking) to prevent data corruption.

### Decision
File range locking is managed by `FileRangeLockManager` (`src/fs/flock.rs`). Shared locks can overlap, but exclusive locks require disjoint ranges. Deadlock prevention is guaranteed via atomic non-blocking trial evaluation and automatic lock teardown on file descriptor closure.
