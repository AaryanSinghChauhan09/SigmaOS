# AI Agent File Operations Management Specification for SigmaOS

This document defines the operational specification for AI agents managing file operations, stream redirection, POSIX/BSD I/O syscalls, and transactional file merges in **SigmaOS**.

---

## 1. Overview & Architecture

SigmaOS provides a zero-dependency, safe, and robust Virtual File System (VFS) and File Operation framework. AI agents interacting with standard streams, file descriptors, filesystem paths, and configuration files must adhere to zero-copy principles, atomic writes, non-blocking stream synchronization, and OpenBSD `unveil` sandboxing.

---

## 2. Standard Streams & Redirection Rules

1. **Standard Stream Handling (`StandardStreamController`)**:
   - `isatty()` query checks determine default buffering modes (`LineBuffered` for TTY, `BlockBuffered(4096)` for non-TTY streams).
   - Use `apply_stdbuf_override()` to adjust standard input, output, and error stream buffering behavior dynamically.
   - Use `flush_all()` to execute buffer synchronization across all active output descriptors.
   - Handle broken stream pipes (`SIGPIPE` / `EPIPE`) gracefully without unhandled kernel panics.

2. **Stream Redirection (`RedirectionEngine`)**:
   - Capture stdin, stdout, and stderr streams into memory buffers or target file descriptors using `RedirectionEngine` (`src/userland/shell.rs`).

---

## 3. Atomic File Writes & Lock Management

1. **Atomic Replacement Protocol**:
   - Always write file updates to a temporary sibling file (`.path.tmp.<pid>`).
   - Call `fdatasync()` or `fsync()` to flush file blocks to physical media.
   - Atomically rename the temporary file over the target path using `renameat2` with `RENAME_EXCHANGE` or atomic `rename()`.

2. **File Locking**:
   - Acquire POSIX `fcntl` or BSD `flock` locks prior to reading or modifying shared system databases or lockfiles.
   - Always acquire locks in deterministic lexicographical order to prevent deadlocks.

---

## 4. Unveil & Landlock Sandboxing

1. **OpenBSD `unveil` Path Restrictions**:
   - Restrict process file access using `unveil(path, permissions)` (`r` read, `w` write, `x` execute, `c` create).
   - Once `unveil(NULL, NULL)` is called, no new path unveil rules can be registered.

2. **3-Way Conffile Reconciliation (`ConffileMergeEngine`)**:
   - Package upgrades modifying configuration files must perform a 3-way merge comparing original base, local modifications, and upstream updates.

---

*Maintained by the SigmaOS Storage & Core Subsystems Committee.*
