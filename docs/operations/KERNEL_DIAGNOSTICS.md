# SigmaOS Kernel Panic and Diagnostic Rehearsal Runbook

## Overview

The Kernel Diagnostic Subsystem (`src/diagnostics/kernel_report.rs`) provides structured boot initialization reports, panic report formatting, CPU register context dumps, and greppable event logs.

## Diagnostic Standards

1. **Panic Reports**:
   - Component name, operation name, panic message.
   - Fault address (if memory fault).
   - Saved register context (RIP, RSP, RBP, RAX, RBX, RCX, RDX, RFLAGS).
   - Stack frame backtrace addresses.

2. **Boot Diagnostic Reports**:
   - OS version and boot timestamp.
   - Per-subsystem status (`Success`, `Warning`, `Failed`, `Skipped`).
   - Initialization duration in microseconds.
   - Greppable log tag format: `[BOOT_REPORT] os=... success=...` and `[BOOT_SUB] stage=... name=... status=... duration_us=...`.

3. **Monthly Rehearsal Procedure**:
   - Run `tests/diagnostics/kernel_diagnostics_test.rs` to verify panic and boot diagnostic serialization.
   - Validate that diagnostic outputs do not leak confidential keys or passwords.
