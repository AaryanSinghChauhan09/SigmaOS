# SigmaOS Syscall Dispatcher

The Sovereign Syscall Dispatcher is the sole gateway between user space (Ring 3) and kernel space (Ring 0). It enforces `sigma_pledge` capability checks before every kernel transition.

---

## Entry Points

| Architecture | Instruction | Setup |
|-------------|-------------|-------|
| x86_64 | `syscall` | LSTAR MSR points to `sigma_syscall_entry` |
| ARM64 | `svc #0` | EL1 exception vector |
| RISC-V | `ecall` | mtvec trap handler |

---

## Dispatch Flow

```
userland: syscall instruction
  │
  ▼
sigma_syscall_entry (asm stub — saves all registers)
  │
  ▼
sigma_pledge_check(current->pledge_mask, syscall_nr)
  ├── DENIED → signal SIGABRT to process
  └── ALLOWED →
        ▼
      syscall_table[syscall_nr](args...)
        ▼
      return value in rax (x86_64)
        ▼
      sysret — restore registers, return to Ring 3
```

---

## Syscall Table (Phase G — 30 Essential Syscalls)

| ID | Name | Signature |
|----|------|-----------|
| 0 | `sys_read` | `(int fd, void* buf, size_t len) → ssize_t` |
| 1 | `sys_write` | `(int fd, const void* buf, size_t len) → ssize_t` |
| 2 | `sys_open` | `(const char* path, int flags, mode_t mode) → int` |
| 3 | `sys_close` | `(int fd) → int` |
| 4 | `sys_stat` | `(const char* path, struct sigma_stat*) → int` |
| 5 | `sys_fstat` | `(int fd, struct sigma_stat*) → int` |
| 6 | `sys_lseek` | `(int fd, off_t offset, int whence) → off_t` |
| 7 | `sys_mmap` | `(void* hint, size_t len, int prot, int flags, int fd, off_t) → void*` |
| 8 | `sys_munmap` | `(void* addr, size_t len) → int` |
| 9 | `sys_mprotect` | `(void* addr, size_t len, int prot) → int` |
| 10 | `sys_brk` | `(void* addr) → void*` |
| 11 | `sys_fork` | `(void) → pid_t` |
| 12 | `sys_execve` | `(const char* path, char* const argv[], char* const envp[]) → int` |
| 13 | `sys_exit` | `(int code) → noreturn` |
| 14 | `sys_waitpid` | `(pid_t pid, int* status, int options) → pid_t` |
| 15 | `sys_getpid` | `(void) → pid_t` |
| 16 | `sys_kill` | `(pid_t pid, int sig) → int` |
| 17 | `sys_sigaction` | `(int sig, const struct sigma_sigaction*, struct sigma_sigaction*) → int` |
| 18 | `sys_socket` | `(int domain, int type, int protocol) → int` |
| 19 | `sys_connect` | `(int fd, const struct sockaddr*, socklen_t) → int` |
| 20 | `sys_bind` | `(int fd, const struct sockaddr*, socklen_t) → int` |
| 21 | `sys_listen` | `(int fd, int backlog) → int` |
| 22 | `sys_accept` | `(int fd, struct sockaddr*, socklen_t*) → int` |
| 23 | `sys_send` | `(int fd, const void* buf, size_t len, int flags) → ssize_t` |
| 24 | `sys_recv` | `(int fd, void* buf, size_t len, int flags) → ssize_t` |
| 25 | `sys_ioctl` | `(int fd, unsigned long req, void* arg) → int` |
| 26 | `sys_pipe` | `(int fd[2]) → int` |
| 27 | `sys_dup2` | `(int oldfd, int newfd) → int` |
| 28 | `sys_pledge` | `(const char* promises, const char* execpromises) → int` |
| 29 | `sys_unveil` | `(const char* path, const char* permissions) → int` |

---

## Adding a Syscall

1. Add entry to `kernel/syscalls/sigma_syscall_table.cpp`
2. Add handler in `kernel/syscalls/sigma_sys_<subsystem>.cpp`
3. Declare in `include/sigma_syscall.h`
4. Add pledge capability check in `kernel/security/sigma_pledge.cpp`
5. Write a test in `tests/unit/syscalls/`

---

## Current Status

| Component | Status |
|-----------|--------|
| Syscall table (headers) | ✅ Complete |
| Syscall entry ASM stub | ⬜ Phase G |
| pledge enforcement | ✅ Implemented |
| unveil enforcement | ✅ Implemented |
| 30 syscall bodies | ⬜ Phase G |

---

*See also: [Kernel](Kernel) · [Security-Model](Security-Model) · [Architecture-Overview](Architecture-Overview)*
