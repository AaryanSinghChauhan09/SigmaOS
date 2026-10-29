# SigmaOS Observability & Diagnostics Architecture

## Structured Logging
SigmaOS implements a multi-tier structured logger in `src/diagnostics/kernel_report.rs`.

### Log Severity Levels
- `TRACE`: Low-level register and memory page allocation traces.
- `DEBUG`: Subsystem initialization markers and driver probe states.
- `INFO`: Normal operational events (e.g., process fork, mount point ready).
- `WARN`: Non-fatal resource pressure or retried device timeouts.
- `ERROR`: Subsystem failures, permission denials, or invalid syscall parameters.
- `CRITICAL`: Panic events, kernel memory corruption, or hardware unrecoverable traps.

## Panic Diagnostics
In the event of a kernel panic, the panic handler records:
- Component name and panic message.
- Instruction pointer (RIP) and stack base pointer (RBP).
- Core CPU registers (RAX, RBX, RCX, RDX, RSI, RDI, RSP, CR2/CR3).
- Boot time uptime ticks.

## System Diagnostics Script
Subsystem logs and hardware diagnostics are collected via:
`scripts/diagnostics/collect_system_report.sh` producing `system_report.json`.
