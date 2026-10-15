# Kernel Crash Dump & Core Analysis Component Agent

## Component Overview
Captures kernel panic state, registers, stack traces, and physical memory pages into crash dump images for post-mortem automated triage.

## Linux Inspiration
Linux kdump/kexec secondary kernel crash collection, systemd-coredump, and crash utility.

## BSD Inspiration
FreeBSD savecore and minidump crash architecture, OpenBSD dmesg crash buffers.

## Current SigmaOS Status
Early kernel panic handler, register frame dumper, and serial stack trace printer implemented. Persistent disk crash dump capture in progress.

## Critical Missing Features
Compressed minidump physical memory writing to dedicated swap/raw partitions, symbol demangling, and automated backtrace JSON generator.

## Implementation Priority
HIGH (Phase 1)

## Key Files to Create/Improve
src/kernel/panic.rs, src/kernel/crash_dump.rs, src/storage/block_queue.rs

## Testing Strategy
Kernel crash fault injection tests (simulated null pointer dereference), dump integrity verification after reboot.

## Dependencies
KERNEL_CORE_AGENT, STORAGE_AGENT, LOGGING_AGENT

## Success Criteria
Sub-500ms crash dump disk persistence, 100% recovery of register state and stack traces on panic.

## Open Source Competitors Analysis
Bypasses the heavy second-kernel kdump setup of Linux by utilizing lightweight, zero-allocation minidump streaming directly to raw disk sectors.

## Future Enhancements
Autonomous AI triage agent analyzing crash dumps on next boot to pinpoint faulty driver source code lines.
