# SigmaOS AI Agent File Operation Management Guidelines

## 1. Executive Summary & Overview

SigmaOS implements a sovereign, high-performance Virtual File System (VFS) and File Operation framework designed to seamlessly unify POSIX, Linux, BSD, and macOS file operation semantics. For AI agents interacting with or modifying the SigmaOS file system, file operations must adhere to strict principles of atomicity, non-blocking I/O safety, permission sandboxing, and data integrity guarantees.

This document establishes the official guidelines and architectural standards for AI agents performing file read, write, lock, stream redirection, and transactional file operation management in SigmaOS.

---

## 2. Core Architectural Components for File Operations

AI agents performing file operations interface with several core subsystem engines in SigmaOS:

| Engine / Component | Module Path | Operational Responsibilities |
| :--- | :--- | :--- |
| `SovereignMountManager` | `src/kernel/vfs/vfs.rs` | VFS mount point management, extended mount flags, OpenBSD `securelevel` enforcement |
| `StandardStreamController` | `src/ipc/std_streams.rs` | TTY detection (`isatty`), stream buffering override (`stdbuf`), `SIGPIPE` handling |
| `RedirectionEngine` | `src/userland/shell.rs` | Stream redirection (`>`, `>>`, `<`), pipe redirection, and buffer capture |
| `IoUringEngine` | `src/distro/missing_distro_innovations.rs` | Asynchronous zero-copy SQ/CQ ring-buffer I/O processing |
| `OpenBsdUnveilManager` | `src/security/selinux.rs` | Path-level sandbox restriction (`unveil` read, write, execute, create rules) |
| `ConffileMergeEngine` | `src/sigpkg/universal_oop_system.rs` | 3-way transactional configuration file merge and reconciliation |

---

## 3. File Operation Management Standards for AI Agents

### 3.1 POSIX & BSD File I/O Semantics

AI agents executing file reads, writes, and seek operations must adhere to standard POSIX and BSD guarantees:

1. **Atomic File Replacement**:
   - Never overwrite active configuration files, system binaries, or database files directly in-place.
   - Write new content to a temporary sibling file (e.g., `file.tmp.<pid>`), flush to disk (`fsync` / `fdatasync`), and execute an atomic rename (`renameat2` with `RENAME_NOREPLACE` or `RENAME_EXCHANGE`).

2. **Positional Offset Operations (`pread` / `pwrite`)**:
   - Concurrent multi-threaded file operations must use `pread` and `pwrite` to prevent race conditions on shared file descriptor seek pointers (`f_pos`).

3. **Data Integrity & Sync Guarantees**:
   - File updates that affect system state must invoke `fdatasync` (for file content) or `fsync` (for content and metadata) prior to closing the file descriptor.

---

### 3.2 Standard Streams & Buffering Operations

SigmaOS features standard stream management parity via `StandardStreamController`:

- **Interactive TTY Detection**: Stream buffers automatically adjust depending on `isatty` checks (`LineBuffered` for interactive TTY, `BlockBuffered(4096)` for piped streams).
- **Buffering Overrides**: AI agents executing subprocesses or userland pipelines can override stream buffering using `apply_stdbuf_override` (matching Linux `stdbuf` behavior).
- **Multi-Stream Synchronization**: Invoking `flush_all()` synchronizes and flushes all active output streams (`fflush(NULL)` standard).
- **SIGPIPE Broken Stream Protection**: Piped stream writes detect broken readers (`EPIPE`) and trigger `SIGPIPE` signal handling without crashing kernel processes.

---

### 3.3 File Locking & Concurrency Control

AI agents accessing shared files must prevent data corruption using appropriate locking mechanisms:

1. **POSIX `fcntl` Record Locking**: Use process-independent `fcntl` byte-range locks (`F_RDLCK`, `F_WRLCK`, `F_UNLCK`) for multi-process database and package manifest access.
2. **BSD `flock` Locks**: Use `flock` (`LOCK_SH`, `LOCK_EX`, `LOCK_UN`) for whole-file advisory locks on log files and locks.
3. **Lock Deadlock Avoidance**: Locks must be acquired in a deterministic lexicographical path order to prevent lock deadlocks. Non-blocking lock requests (`LOCK_NB` / `F_SETLK`) should be attempted with backoff retry loops.

---

### 3.4 Asynchronous I/O & Event Demultiplexing

For high-throughput file operations, AI agents utilize async I/O demultiplexers:

- **Linux `io_uring` Engine**: High-performance I/O operations submit submission queue entries (`SQE`) and consume completion queue entries (`CQE`) via `IoUringEngine` without syscall overhead.
- **BSD `kqueue` & Linux `epoll`**: Non-blocking file descriptors use `epoll` or `kqueue` event notifications (`EVFILT_READ`, `EVFILT_WRITE`, `EPOLLIN`, `EPOLLOUT`) for non-blocking stream processing.

---

### 3.5 Security Sandboxing & Unveil Rules

AI agents operate under capability tokens and OpenBSD `unveil` path restrictions:

- **Path Restrictions**: An agent restricted by `unveil("/var/log", "rw")` can only execute read and write operations under `/var/log`. Attempts to access paths outside unveiled bounds return `EACCES` / `EPERM`.
- **Landlock LSM Compliance**: Userland agents enforce Landlock LSM path hierarchy rules for filesystem sandboxing.

---

### 3.6 Conffile 3-Way Merge Reconciliation

When performing automated package upgrades or system configuration edits, AI agents must use `ConffileMergeEngine` to reconcile local changes with upstream vendor updates:

```
           [ Original Base File ]
                   /    \
                  /      \
[ Local User Changes ]  [ Upstream Vendor Update ]
                  \      /
                   \    /
          [ 3-Way Merged Result ]
```

- If conflict arises, create a `.signew` or `.sigold` backup file and preserve local modifications until explicitly resolved.

---

## 4. Verification & Testing Protocol for AI Agents

All file operation changes introduced by AI agents must pass verification against the SigmaOS test suite:

1. **Unit & Subsystem Tests**:
   - Run `./run_sigma_tests.sh` or `cargo test --lib` to verify file system and stream operations.
2. **Stress & Fuzzing Matrix**:
   - Run `tests/stress_and_fuzz_tests.rs` to validate file locking, concurrent I/O, and non-blocking stream redirection under high workload conditions.

---

*Document approved by the SigmaOS AI Agent & File System Architecture Steering Committee.*
