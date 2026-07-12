# SigmaOS Syscall Dispatcher Architecture

The SigmaOS Syscall Dispatcher is a modular, zero-dependency C/C++ implementation designed to replace high-level abstractions with silicon-direct dispatch tables.

## Mechanism

- `syscalls.h`: Defines sequential syscall identifiers (`SYSCALL_GETPID`, `SYSCALL_WRITE`, etc.) and function prototypes.

- `dispatcher.c` / `dispatcher.cpp`: Implements direct table lookup O(1) dispatching, validating syscall numbers and forwarding register arguments directly to kernel handlers.

## Architecture

```
User-space Application
   └─ SYSCALL instruction (int 0x80 / syscall)
         └─ Syscall Dispatcher (O(1) table lookup)
               ├─ Validation (syscall number, arguments)
               ├─ Capability Check (CAP_SYS_* tokens)
               └─ Kernel Handler
                     ├─ VFS syscalls (read, write, open, close)
                     ├─ Network syscalls (socket, connect, send, recv)
                     ├─ Process syscalls (spawn, exit, wait)
                     ├─ Memory syscalls (mmap, munmap, brk)
                     └─ IPC syscalls (shard_send, shard_recv)
```

## Syscall Table

| Syscall Number | Name | Handler | Description |
|---|---|---|---|
| 0 | SYSCALL_NOOP | `handle_noop` | No-op for testing |
| 1 | SYSCALL_READ | `handle_read` | Read from file descriptor |
| 2 | SYSCALL_WRITE | `handle_write` | Write to file descriptor |
| 3 | SYSCALL_OPEN | `handle_open` | Open/create file |
| 4 | SYSCALL_CLOSE | `handle_close` | Close file descriptor |
| 5 | SYSCALL_SPAWN | `handle_spawn` | Spawn new shard |
| 6 | SYSCALL_EXIT | `handle_exit` | Exit current shard |
| 7 | SYSCALL_MMAP | `handle_mmap` | Map memory region |
| 8 | SYSCALL_MUNMAP | `handle_munmap` | Unmap memory region |
| 9 | SYSCALL_SOCKET | `handle_socket` | Create socket |
| 10 | SYSCALL_CONNECT | `handle_connect` | Connect to remote host |
| 11 | SYSCALL_SEND | `handle_send` | Send data over socket |
| 12 | SYSCALL_RECV | `handle_recv` | Receive data from socket |
| 13 | SYSCALL_SHARD_SEND | `handle_shard_send` | Send IPC message |
| 14 | SYSCALL_SHARD_RECV | `handle_shard_recv` | Receive IPC message |

## API Interface

```c
// Syscall handler function signature
typedef int64_t (*syscall_handler_t)(uint64_t arg0, uint64_t arg1,
                                      uint64_t arg2, uint64_t arg3,
                                      uint64_t arg4, uint64_t arg5);

// Register a syscall handler
void syscall_register(uint64_t syscall_num, syscall_handler_t handler);

// Dispatch a syscall (called from assembly entry point)
int64_t syscall_dispatch(uint64_t syscall_num, uint64_t *args);

// Validate syscall arguments
int syscall_validate_args(uint64_t syscall_num, uint64_t *args);

// Check capability token for syscall
int syscall_check_capability(uint64_t syscall_num, cap_token_t token);

// Initialize syscall dispatcher
void init_syscall_dispatcher(void);
```

## Argument Passing

Arguments are passed in registers following the System V AMD64 ABI:

| Register | Argument |
|---|---|
| RAX | Syscall number |
| RDI | arg0 |
| RSI | arg1 |
| RDX | arg2 |
| RCX | arg3 |
| R8 | arg4 |
| R9 | arg5 |

Return value is placed in RAX.

## Capability Enforcement

Every syscall checks the caller's capability token before execution:

```c
if (!syscall_check_capability(syscall_num, caller_cap)) {
    return -EPERM;  // Permission denied
}
```

## Performance Characteristics

- **O(1) dispatch**: Direct table lookup, no linear search
- **Zero-copy**: Arguments passed in registers, no kernel stack copy
- **Minimal overhead**: ~50ns per syscall on modern x86_64

## Roadmap

- [x] Basic syscall table and dispatcher
- [x] Argument validation
- [x] Capability token checking
- [ ] Fast path for common syscalls (read/write)
- [ ] Syscall batching for bulk operations
- [ ] Formal verification of syscall safety properties
- [ ] seccomp-style syscall filtering
- [ ] Syscall statistics and profiling

## Related Modules

- [`modules/core/kernel`](../../modules/core/kernel/README.md) — Kernel core
- [`modules/security/access_control`](../../modules/security/access_control/README.md) — Capability enforcement
