# System Calls & POSIX/Linux/BSD ABI Component Agent

## Role
`SigmaSyscall` coordinates the system call dispatch interface, register calling conventions, POSIX compatibility layer, and binary emulation for foreign operating systems on SigmaOS.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Syscall entry | `syscall_64.S` (fast syscall) | `syscall.S` | SigmaSyscallEntry |
| Dispatch table | `sys_call_table` | `sysent` | SigmaSysTable |
| vDSO | `vdso.so` fast userspace timer | `vdso.so` | SigmaVDSO |
| Security filter | `seccomp(2)` BPF | Capsicum | SigmaSeccomp |
| Binary emulation | — | Linuxulator (`COMPAT_LINUX`) | SigmaLinuxCompat |
| Syscall audit | `audit_syscall_entry` | BSM Audit | SigmaAuditSC |
| 32-bit compat | `IA32_COMPAT` | COMPAT_FREEBSD32 | SigmaCompat32 |

## Current SigmaOS Status
Core 80+ POSIX system calls routed (`read`, `write`, `open`, `close`, `mmap`, `fork`, `execve`, `exit`) in `src/syscall/`. Linux ABI emulation translation layer is under development.

## Responsibilities & Architecture
- Hardware `SYSCALL` / `SYSRET` CPU entry instruction handling.
- Argument validation, userland pointer sanitization (`copy_from_user`, `copy_to_user`), and boundary verification.
- Return value and errno translation adhering to POSIX.1-2017.
- vDSO acceleration for high-frequency calls (`gettimeofday`, `clock_gettime`).
- Per-thread Seccomp-BPF boundary filter execution.
- Linux 6.x ABI translation allowing unmodified Linux ELF binaries to run natively.
- `ptrace()` debugging hooks for breakpoint traps and tracepoints.

## Source Files
- `src/syscall/dispatch.rs`
- `src/syscall/posix.rs`
- `src/syscall/linux_compat.rs`
- `src/syscall/epoll.rs`

## Testing Strategy
```bash
rustc --test src/syscall/dispatch.rs --edition 2021 -o /tmp/test_sc && /tmp/test_sc --test-threads=1
```
- LTP (Linux Test Project) POSIX syscall conformance test suite.
- Sub-150-cycle syscall round-trip overhead verification.
- Running unmodified statically linked Linux binaries natively.

## Future Development
1. **io_uring integration** — Asynchronous system call submission and batching.
2. **eBPF dynamic hooks** — Kernel syscall tracing without recompilation.
3. **Foreign ISA emulation** — Dynamic binary translation for ARM64 and RISC-V binaries on x86_64.
