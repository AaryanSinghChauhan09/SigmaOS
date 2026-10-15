# System Calls & POSIX/Linux/BSD ABI Component Guide

## Component Overview
Coordinates the system call dispatch interface, register calling conventions, POSIX compatibility layer, and binary emulation for foreign operating systems.

## Linux Inspiration
Linux x86_64 syscall interface (fast syscall instruction), vDSO acceleration, POSIX.1-2017 standard compliance, and Linux ABI profile.

## BSD Inspiration
FreeBSD Linuxulator (binary compatibility layer running unmodified Linux binaries), NetBSD COMPAT_* emulation subsystem.

## Current SigmaOS Status
Core 80+ standard POSIX syscalls routed (read, write, open, close, mmap, fork, execve, exit). Linux ABI emulation translation layer is under development.

## Critical Missing Features
Complete Linux 6.x syscall ABI emulation table (allowing unmodified Linux ELF binaries to run natively on SigmaOS), POSIX signal masking, and epoll_create/epoll_wait.

## Implementation Priority
CRITICAL (Phase 1)

## Key Files to Create/Improve
src/syscall/dispatch.rs, src/syscall/posix.rs, src/syscall/linux_compat.rs, src/syscall/epoll.rs

## Testing Strategy
LTP (Linux Test Project) POSIX syscall test suite, running unmodified statically linked Linux binaries natively.

## Dependencies
KERNEL_CORE_AGENT, MEMORY_MANAGEMENT_AGENT, PROCESS_MANAGEMENT_AGENT

## Success Criteria
Sub-150-cycle syscall round-trip overhead, 99.5% POSIX.1-2017 compliance, seamless execution of Linux userland utilities.

## Open Source Competitors Analysis
Surpasses FreeBSD Linuxulator by offering zero-copy register mapping and direct native syscall translation without translation daemon latency.

## Future Enhancements
Automatic dynamic binary recompilation for running ARM64 and RISC-V foreign binaries on x86_64 host hardware.
