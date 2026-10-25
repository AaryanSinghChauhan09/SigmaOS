# SigmaOS Signal Development and Maintenance Plan

## Executive Summary
This document defines the engineering standards, architecture rules, 18-PR development sequence, and lifecycle maintenance cadence for signal handling in SigmaOS.

## Core Rules & Invariants
1. **POSIX.1-2024 Baseline**: All core signal APIs (`sigaction`, `sigprocmask`, `sigpending`, `sigsuspend`, `kill`, `raise`) strictly follow POSIX.1-2024 semantics.
2. **Uncatchable Signals**: `SIGKILL` and `SIGSTOP` cannot be caught, ignored, or blocked under any circumstances.
3. **Thread Mask Isolation**: Process-wide signal dispositions are separated from per-thread blocked signal masks.
4. **Frame Safety & Privilege Isolation**: User-space frame allocation and register restoration (`sigreturn`) validate canonical user addresses and ensure ring transitions cannot escalate privileges.
5. **Non-blocking Kernel Delivery**: Kernel delivery paths operate without heap allocation or sleeping in atomic context.

## Proposed 18-PR Roadmap Sequence
- PR 1: Audit signal path and define ABI profile
- PR 2: Add ABI and signal-frame layout checks
- PR 3: Connect signal syscalls to kernel dispatch
- PR 4: Add per-thread masks and pending signal queues
- PR 5: Deliver default actions at safe return points
- PR 6: Construct validated x86_64 user signal frame
- PR 7: Implement x86_64 sigreturn validation
- PR 8: Implement action masks and restart semantics
- PR 9: Add siginfo and SA_SIGINFO subset
- PR 10: Add alternate stack support
- PR 11: Integrate SIGCHLD and wait/reap lifecycle
- PR 12: Add synchronous signal-wait APIs
- PR 13: Add queued real-time signal support with limits
- PR 14: Integrate signalfd as Linux-specific extension
- PR 15: Expose signal APIs and safe handler examples in libc
- PR 16: Add fault-injection and concurrency suite
- PR 17: Validate AArch64 and RISC-V handler frames
- PR 18: Publish API, safety, and support matrix

## Maintenance Cadence
- **Every PR**: CI layout, unit test, and frame validation pass.
- **Weekly**: Triage handler crashes and update feature maturity status.
- **Monthly**: Review frame construction assembly and syscall interruption/restart matrix.
- **Quarterly**: Audit against POSIX.1-2024 and Linux/FreeBSD reference implementations.
- **RC & Incidents**: Release certification and post-mortem analysis.
