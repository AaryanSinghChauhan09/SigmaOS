# Syscall Interface Agent

## Role
`SigmaSyscall` implements the kernel–userspace boundary for SigmaOS — system call dispatch, ABI compatibility, seccomp filtering, and vsyscall/vDSO — inspired by Linux's `arch/x86/entry/` and FreeBSD's `sys/kern/kern_syscall.c`.

## Status
🔴 Not Started — Critical gap. Required for any real userland execution.

## Source File
`src/syscall/` (planned: `dispatch.rs`, `table.rs`, `vdso.rs`, `compat.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Syscall entry | `syscall_64.S` | `syscall.S` | SigmaSyscallEntry |
| Dispatch table | `sys_call_table` | `sysent` | SigmaSysTable |
| vDSO | `vdso.so` | `vdso.so` | SigmaVDSO |
| Seccomp | `seccomp(2)` | Capsicum | SigmaSeccomp |
| Audit | `audit_syscall_entry` | BSM Audit | SigmaAuditSC |
| 32-bit compat | `IA32_COMPAT` | COMPAT_FREEBSD32 | SigmaCompat32 |

## Responsibilities
- SYSCALL/SYSENTER/INT 0x80 entry point dispatch
- System call number → handler function table
- Argument validation and pointer sanitization (copy_from_user)
- Return value and errno propagation to userland
- vDSO: fast userspace implementations of `gettimeofday`, `clock_gettime`
- Seccomp-BPF filter evaluation per-thread
- Syscall auditing hooks for SigmaAudit
- 32-bit compatibility layer for legacy binaries
- ptrace() support for debuggers

## Implementation Notes
- `#![no_std]` — syscall table as `static SYSENT: [SyscallEntry; NR_SYSCALLS]`
- `SyscallEntry { name: &str, handler: fn(Args) -> isize, nargs: u8 }`
- Pointer validation: check that user pointers are below kernel base
- Use `SYSCALL_DEFINE` macro pattern for type-safe dispatch

## Testing
Syscall interface requires integration testing with userland test programs. Unit-test argument validation logic:
```bash
rustc --test src/syscall/dispatch.rs --edition 2021 -o /tmp/test_sc && /tmp/test_sc --test-threads=1
```

## Future Development
1. **io_uring** — Async syscall batching (Linux 5.1+)
2. **EBPF syscall hooks** — Dynamic syscall tracing without kernel rebuild
3. **Landlock** — Userspace sandbox via restrictive syscall policies
4. **POSIX.1-2024 compliance** — Track new syscalls (close_range, openat2, etc.)
5. **Syscall overhead reduction** — Lazy TSC switching, shared page optimizations
6. **Arm64 syscall ABI** — SVC-based dispatch for aarch64 support
