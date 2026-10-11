# SigmaOS Kernel Signal Processing and Interruption Architecture Plan

## 1. Executive Summary and Architecture Overview
This document specifies the SigmaOS Kernel Signal Processing and Interruption Architecture, drawing inspiration from Linux kernel signal dispatch (6.12+ LTS) and FreeBSD kernel process descriptor (`pdfork`) signal handling. Signals are asynchronous control flow primitives processed strictly at controlled kernel-to-user return boundaries (e.g. syscall return or hardware interrupt return).

## 2. Kernel Signal Principles
1. **Asynchronous Execution Boundaries**: Signal handling routines are triggered at controlled kernel-to-user transitions. Arbitrary interrupt context never executes user-space handlers directly.
2. **Strict Mask Isolation**: Thread signal masks (`sigset_t`) govern pending signal visibility. Uncatchable signals (`SIGKILL` and `SIGSTOP`) are hardwired to bypass blocking masks.
3. **Safe Context Framing**: At kernel-to-user return, user signal frames containing saved register context and signal masks are set up on the user stack with trampoline redirect pointers (`sa_restorer`).
4. **Context Restoration Verification**: `sigreturn` validates frame magic numbers, privilege bits, and memory bounds to prevent privilege escalation or corrupted state restoration.
5. **Syscall Interruption Matrix**: Blocking calls evaluate signal dispositions to determine whether to abort with `EINTR` or automatically restart under `SA_RESTART`.

## 3. Kernel Subsystem Features and PR Sequence
- **Kernel Generation & Pending Queues**: Separate process-directed and thread-directed pending queues.
- **Signal Set Operations**: 64-bit mask manipulation with `SIGKILL`/`SIGSTOP` unblockable protection.
- **Safe Copy-In/Copy-Out**: Userspace `sigaction` structures validated with atomic fallback on copy failure.
- **User Frame Trampoline Dispatch**: x86-64 / AArch64 Ring 3 frame setup and register save/restore.
- **Blocking Syscall Interruption**: Interruption matrix for `read`, `write`, `poll`, `wait`, `sigsuspend`.
- **Queued Real-Time Signals**: FIFO ordering for signals 32..64 with process queue limit enforcement.
- **`signalfd` Polling Integration**: Pollable FD subscription to blocked signals.

## 4. Maintenance and Security Cadence
- **CI Safety Checks**: Automated verification of UAPI signal numbers, structure alignment, and safe copy-in/out.
- **Audit & DoS Protection**: Resource bounds on queued real-time signals, permission checks on `kill` generation.
