# SigmaOS Syscall Reference

SigmaOS implements a Linux-compatible syscall ABI on x86_64, plus custom `SYS_SIGMA_*` extensions.
All syscalls go through `sigma_syscall_dispatch()` in `kernel/core/syscall_dispatch.rs`.

---

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Fully implemented |
| 🔄 | Partial (stub with basic logic) |
| ⬜ | Planned (returns ENOSYS) |

---

## Process Control

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 39 | `getpid` | ✅ | Returns current process ID |
| 110 | `getppid` | ✅ | Returns 1 (init) as default |
| 102 | `getuid` | ✅ | Returns 0 (root) |
| 107 | `geteuid` | ✅ | Returns 0 (root) |
| 57 | `fork` | ⬜ | Needs process manager |
| 56 | `clone` | ⬜ | Needs thread support |
| 59 | `execve` | ⬜ | Needs ELF loader wiring |
| 60 | `exit` | 🔄 | Signals process manager |
| 231 | `exit_group` | 🔄 | Exits all threads |
| 61 | `wait4` | ⬜ | Needs process manager |
| 62 | `kill` | ⬜ | Needs signal system |

---

## Memory Management

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 9 | `mmap` | 🔄 | Maps via slab allocator |
| 11 | `munmap` | 🔄 | Frees slab allocation |
| 12 | `brk` | 🔄 | Returns next heap address |
| 10 | `mprotect` | ⬜ | W^X enforced at kernel level |
| 28 | `madvise` | ⬜ | Hint to kernel (no-op safe) |

---

## File I/O

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 0 | `read` | ⬜ | Needs VFS |
| 1 | `write` | ⬜ | Needs VFS |
| 2 | `open` | ⬜ | Needs VFS |
| 3 | `close` | ⬜ | Needs VFS |
| 8 | `lseek` | ⬜ | Needs VFS |
| 17 | `pread64` | ⬜ | Needs VFS |
| 18 | `pwrite64` | ⬜ | Needs VFS |
| 19 | `readv` | ⬜ | Scatter-gather read |
| 20 | `writev` | ⬜ | Scatter-gather write |
| 72 | `fcntl` | ⬜ | File descriptor flags |
| 32 | `dup` | ⬜ | Duplicate fd |
| 33 | `dup2` | ⬜ | Duplicate to specific fd |

---

## Filesystem

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 4 | `stat` | ⬜ | File metadata |
| 5 | `fstat` | ⬜ | fd metadata |
| 6 | `lstat` | ⬜ | Symlink metadata |
| 79 | `getcwd` | ⬜ | Current directory |
| 80 | `chdir` | ⬜ | Change directory |
| 83 | `mkdir` | ⬜ | Create directory |
| 84 | `rmdir` | ⬜ | Remove directory |
| 87 | `unlink` | ⬜ | Remove file |
| 82 | `rename` | ⬜ | Rename file |
| 90 | `chmod` | ⬜ | Change permissions |
| 92 | `chown` | ⬜ | Change ownership |

---

## IPC & Signals

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 202 | `futex` | 🔄 | FUTEX_WAIT + FUTEX_WAKE |
| 22 | `pipe` | ⬜ | Create pipe |
| 293 | `pipe2` | ⬜ | Pipe with flags |
| 13 | `rt_sigaction` | ⬜ | Signal handler |
| 14 | `rt_sigprocmask` | ⬜ | Signal mask |

---

## Networking

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 41 | `socket` | ⬜ | Create socket |
| 42 | `connect` | ⬜ | Connect socket |
| 43 | `accept` | ⬜ | Accept connection |
| 44 | `sendto` | ⬜ | Send data |
| 45 | `recvfrom` | ⬜ | Receive data |
| 49 | `bind` | ⬜ | Bind address |
| 50 | `listen` | ⬜ | Listen for connections |
| 7 | `poll` | ⬜ | Wait for events |
| 232 | `epoll_wait` | ⬜ | Async event wait |
| 233 | `epoll_ctl` | ⬜ | Add/remove from epoll |
| 291 | `epoll_create1` | ⬜ | Create epoll fd |

---

## Time

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 35 | `nanosleep` | ✅ | Converts to sigma_sleep_ms |
| 228 | `clock_gettime` | ✅ | Returns sigma_clock_ns() |
| 63 | `uname` | ✅ | Returns SigmaOS utsname |

---

## Randomness & Misc

| Nr | Name | Status | Notes |
|----|------|--------|-------|
| 318 | `getrandom` | ✅ | PRNG from clock |
| 218 | `set_tid_address` | ✅ | Returns sigma_gettid() |
| 319 | `memfd_create` | ⬜ | In-memory file |
| 16 | `ioctl` | ⬜ | Device control |

---

## SigmaOS Custom Syscalls (>= 400)

| Nr | Name | Status | Description |
|----|------|--------|-------------|
| 400 | `sigma_pledge` | ✅ | Restrict process capabilities |
| 401 | `sigma_unveil` | ✅ | Restrict filesystem access |
| 402 | `sigma_attest` | 🔄 | PQC attestation request |
| 403 | `sigma_bus_send` | ✅ | Send to sigma-bus IPC channel |
| 404 | `sigma_bus_recv` | ✅ | Receive from sigma-bus channel |
| 405 | `sigma_capability` | 🔄 | Capability token operations |

---

## Example: Adding a Syscall

```rust
// In kernel/core/syscall_dispatch.rs

// 1. Define the syscall number
pub const SYS_MY_SYSCALL: u64 = 499;

// 2. Implement the handler
unsafe fn sys_my_syscall(a1: u64, a2: u64, _: u64, _: u64, _: u64, _: u64) -> i64 {
    // Implementation
    0
}

// 3. Add to dispatch match
SYS_MY_SYSCALL => sys_my_syscall(a1, a2, a3, a4, a5, a6),
```

---

*Source: `kernel/core/syscall_dispatch.rs` · See also: [Kernel Developer Handbook](../docs/KERNEL_DEVELOPER_HANDBOOK.md)*
