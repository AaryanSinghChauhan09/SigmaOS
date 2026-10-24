# SigmaOS Observability & Diagnostics Architecture

This document defines structured logging, panic reporting, system diagnostic reports, and runtime health check protocols for SigmaOS.

## Diagnostic Architecture

SigmaOS provides observability across kernel, userspace, installer, and storage subsystems via structured diagnostic interfaces defined in `src/diagnostics/`.

### Subsystem Components

- **Kernel Diagnostics (`src/diagnostics/kernel_report.rs`)**:
  Structures boot sequence initialization markers (`KernelBootDiagnosticReport`) and panic contexts (`KernelPanicDiagnosticReport`).
- **Crash & Panic Handler (`src/diagnostics/crash.rs`)**:
  Serializes CPU registers, fault addresses, call stacks, and component identifier tags during kernel panic or userland crash.
- **System Report Collector (`scripts/diagnostics/collect_system_report.sh`)**:
  Generates a consolidated JSON diagnostic artifact containing kernel logs, memory allocator stats, driver state, and storage volume metrics.

---

## Panic Diagnostic Format

Kernel panics emit structured text and JSON output formatted as:

```text
================[ SIGMAOS KERNEL PANIC ]================
Component:  kernel::mm::paging
Reason:     Page fault at unmapped virtual address
CR2 Fault:  0x00007fff82a0b120
Registers:  RAX=0x0000000000000000 RBX=0x0000000000000001
            RIP=0xffffffff80201040 RSP=0xffffffff80407e00
Status:     Diagnostic artifact dumped to serial & crash buffer
=========================================================
```

---

## Health Checks

Runtime health checks validate system state during boot and routine operation:

- **Installer & Recovery Health**: Validates rollback journal integrity.
- **Storage & VFS Health**: Validates cache dirty page thresholds and mount limits.
- **Desktop Session Health**: Monitors compositor latency and interactive input event loops.
