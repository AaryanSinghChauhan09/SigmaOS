# SigmaOS Kernel Diagnostics & Panic Protocol

This document specifies the structured diagnostic and panic reporting requirements for the SigmaOS kernel.

## Principles

1. **Failure Transparency:** Boot failures and unrecoverable kernel panics must produce structured, greppable serial diagnostic output.
2. **Context Preservation:** CPU register context, process ID, subsystem name, and call stack hints must be captured prior to system halt.
3. **No Silent Failures:** Critical subsystem initialization failures must trigger panic reports rather than silently continuing.

---

## Diagnostic Report Formats

### Boot Initialization Diagnostic Report

Boot log events use greppable JSON format over serial port 0x3F8:

```json
{"event":"boot_subsystem_init","subsystem":"paging","status":"ok","free_frames":262144,"timestamp_ticks":1024}
```

### Kernel Panic Diagnostic Report

When `panic!` is invoked, the panic handler serializes context into stdout/serial:

```text
================================================================================
                    SIGMAOS KERNEL PANIC DIAGNOSTIC REPORT
================================================================================
Component:      kernel::mm::paging
Operation:      page_fault_handler
Error Message:  PAGE_FAULT: linear address 0x0000000000000008 (Present: No, Write: Yes, User: No)
Process ID:     42 (Process Name: "user_app")
CPU ID:         0

--- REGISTER CONTEXT ---
RAX: 0x0000000000000000  RBX: 0xffff800000102000  RCX: 0x0000000000000008
RDX: 0x0000000000000001  RSI: 0x0000000000000010  RDI: 0xffff800000102000
RBP: 0xffff800000200f00  RSP: 0xffff800000200eb0  RIP: 0xffffffff801054a0
R8:  0x0000000000000000  R9:  0x0000000000000000  R10: 0x0000000000000000
CR2: 0x0000000000000008  CR3: 0x0000000001000000  FLAGS: 0x0000000000010202

--- ACTION ---
System Halted.
================================================================================
```

---

## Rehearsal Procedure

Periodic kernel panic rehearsal is performed via:

```bash
rustc --test tests/diagnostics/kernel_diagnostics_test.rs --edition=2021 -o build/test_diag
./build/test_diag
```
