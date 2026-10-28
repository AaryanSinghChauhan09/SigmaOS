# SigmaOS Kernel Diagnostics Specification

## Overview
SigmaOS enforces structured boot diagnostic logging and stable panic reports to make failures immediately actionable and greppable.

---

## Log & Panic Formats

### 1. Boot Initialization Log Format
Boot logs utilize key-value pairs formatted as:
`[BOOT] <step_num>: <subsystem_name> -> status=<SUCCESS|FAILURE|WARN>, latency_us=<us>`

### 2. Panic Diagnostic Report Format
When a kernel panic occurs, `KernelPanicDiagnosticReport` captures:
- Panic message & location (file, line, column).
- Faulting process ID, thread ID, and process name.
- Hardware register state (RIP, RSP, RBP, RFLAGS, CR2/CR3).
- Call stack backtrace frames.
- Subsystem health status snapshot.
