# POSIX Compatibility Syscall Layer

To allow binary compatibility with mainstream Linux distributions, SigmaOS includes a lightweight **POSIX Compatibility Syscall Layer** (`posix/sigma_posix.rs`).

## Architecture

```
+-------------------------------------------------------+
|                Linux ELF Binary / Userland            |
+-------------------------------------------------------+
                           |
                           v  [syscall instruction / rax]
+-------------------------------------------------------+
|             sigma_posix_dispatch (Assembly Entry)      |
+-------------------------------------------------------+
                           |
                           v  [Binary Search (O(log n))]
+-------------------------------------------------------+
|             POSIX Compatibility Layer (no_std)        |
+-------------------------------------------------------+
                           |
                           v
+-------------------------------------------------------+
|             SigmaOS Sovereign Kernel Subsystems       |
+-------------------------------------------------------+
```

## Key Components

1. **`SyscallEntry`**: Holds the syscall number, name, and function handler pointer.
2. **`PosixCompat`**: An encapsulated dispatch table containing 50 of the most common Linux x86-64 syscalls.
   - At boot time, it registers all handlers and sorts them by syscall number.
   - Dispatch uses a binary search algorithm to look up the correct handler in **O(log n)** time.
3. **`SyscallArgs`**: Represents the CPU registers used to pass arguments under the x86-64 Linux ABI (`rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`).

## Implemented Syscalls
- **File System**: `SYS_READ`, `SYS_WRITE`, `SYS_OPEN`, `SYS_CLOSE`, `SYS_STAT`, `SYS_LSEEK`, `SYS_CHMOD`, `SYS_UNLINK`, etc.
- **Process Management**: `SYS_FORK`, `SYS_EXECVE`, `SYS_EXIT`, `SYS_KILL`, `SYS_GETPID`, `SYS_WAIT4`, etc.
- **Memory Management**: `SYS_MMAP`, `SYS_MUNMAP`, `SYS_MPROTECT`, `SYS_BRK`, etc.
- **Networking**: `SYS_SOCKET`, `SYS_CONNECT`, `SYS_ACCEPT`, `SYS_BIND`, `SYS_LISTEN`, etc.

All handlers return standard negative POSIX errnos (e.g. `-EINVAL`, `-EBADF`, `-ENOSYS`) on failure.
