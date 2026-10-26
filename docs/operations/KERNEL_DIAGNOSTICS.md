# SigmaOS Kernel Diagnostic Protocols & Panic Specification

This document details kernel panic reporting, boot diagnostic logging, and register context serialization protocols.

## Panic Diagnostic Format

When an unrecoverable kernel invariant violation occurs, the kernel produces a structured panic report:

```text
================================================================================
KERNEL PANIC DIAGNOSTIC REPORT
================================================================================
Timestamp   : <timestamp_us> µs
Subsystem   : <failing_subsystem>
Message     : <panic_message>
Location    : <file_path>:<line_number>
--------------------------------------------------------------------------------
REGISTER DUMP & CPU CONTEXT:
  RAX: 0x0000000000000000  RBX: 0x0000000000000000  RCX: 0x0000000000000000
  RDX: 0x0000000000000000  RSI: 0x0000000000000000  RDI: 0x0000000000000000
  RBP: 0x0000000000000000  RSP: 0x0000000000000000  RIP: 0x0000000000000000
  RFLAGS: 0x00000002
================================================================================
```

## Boot Diagnostic Logging Protocol

Boot initialization must report each critical subsystem result with greppable markers:

- `[BOOT_DIAG] [OK] <subsystem_name> initialized in <duration_us> µs`
- `[BOOT_DIAG] [FAIL] <subsystem_name> initialization failed: <error_code>`

## Implementation References

- `src/diagnostics/kernel_report.rs`: Implements `KernelBootDiagnosticReport` and `KernelPanicDiagnosticReport`.
- `tests/diagnostics/`: Verified via standalone `rustc --test` suite.
