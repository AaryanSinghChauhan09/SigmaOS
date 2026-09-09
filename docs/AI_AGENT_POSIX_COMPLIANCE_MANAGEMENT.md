# AI Agent POSIX Compliance & C Library Management Guidelines

## 1. Overview & Architecture
This document defines AI agent protocols for implementing, expanding, and auditing POSIX.1-2017 syscall coverage, POSIX error handling (`errno`), and Glibc/musl C library compatibility layers in SigmaOS (`src/klib/error.rs`, `src/process/syscall.rs`, `src/compatibility/`).

---

## 2. Operational Directives for AI Agents

### 2.1 Syscall Implementation Rules
- **No Unhandled Syscalls**: When adding or updating syscalls in `src/process/syscall.rs`, AI agents must ensure appropriate POSIX error codes (e.g., `ENOENT`, `EINVAL`, `EACCES`, `ENOMEM`) are returned via `src/klib/error.rs`.
- **64-Bit & 32-Bit Thunk Verification**: Syscall dispatch tables must handle both native 64-bit calls and 32-bit compatibility thunks without integer truncation or buffer overflows.

### 2.2 Glibc & Musl Compatibility
- **C Library ABI Translation**: AI agents extending C library wrappers must map POSIX `libc` data structures (`struct stat`, `struct timeval`, `sigaction`) to native Rust kernel primitives.
- **Zero-Copy Memory Passing**: File descriptor I/O (`read`, `write`, `preadv`, `pwritev`) must use zero-copy memory transfers between userland and kernel space.

---

## 3. Related Files
- `src/klib/error.rs`
- `src/process/syscall.rs`
- `docs/LINUX_DISTRO_PARITY_CHECKLIST.md`
