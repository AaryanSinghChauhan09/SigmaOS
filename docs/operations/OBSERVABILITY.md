# SigmaOS Observability & Diagnostics Architecture

## Structured Logging
SigmaOS uses `src/diagnostics/kernel_report.rs` and `src/logging/sigmalog.rs` to capture structured kernel event streams:
- `KernelBootDiagnosticReport`: Microsecond-precision subsystem init timings.
- `KernelPanicDiagnosticReport`: Formatted register dumps (RAX, RBX, RCX, RDX, RSP, RBP, RIP, CR0, CR2, CR3, CR4) and fault location context.

## Diagnostics Collection
Execute `scripts/diagnostics/collect_system_report.sh` to extract system state into a unified report bundle containing:
- Kernel printk log buffer
- Mount point status and filesystem usage
- Active processes and memory allocation counters
- Device driver probing log
