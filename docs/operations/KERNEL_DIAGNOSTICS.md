# SigmaOS Kernel Diagnostics Specification

## Overview

SigmaOS mandates structured, greppable diagnostic reporting across early boot initialization, subsystem probes, and kernel panics. Failure reports must capture register states, faulting addresses, and subsystem call stacks.

---

## Structured Boot Diagnostics

During early boot, each subsystem emits standardized log markers:

```text
[BOOT] Stage: memory_init | Status: OK | Component: PhysicalAllocator | Frames Free: 65536
[BOOT] Stage: paging_init | Status: OK | Component: 4LevelPageTable | CR3: 0x0000000000100000
[BOOT] Stage: interrupt_init | Status: OK | Component: IOAPIC | Vectors: 32..255
[BOOT] Stage: scheduler_init | Status: OK | Component: BoundedRunQueue | Priorities: 64
[BOOT] Stage: vfs_init | Status: OK | Component: RamFS | Mount: /
```

---

## Panic Diagnostic Format

When an unrecoverable kernel panic occurs, `KernelPanicDiagnosticReport` formats register contexts and stack traces into JSON and text outputs:

```text
================================================================================
KERNEL PANIC DIAGNOSTIC REPORT
================================================================================
Timestamp: 1728770400
Component: Kernel::MM::Paging
Reason:    Page Fault Invariant Violation (CR2: 0x0000000000000008)

REGISTER CONTEXT:
RAX: 0x0000000000000000  RBX: 0x0000000000000008  RCX: 0x0000000000000001
RDX: 0x0000000000000000  RSI: 0x0000000000200000  RDI: 0x0000000000000008
RBP: 0x00000000001ff800  RSP: 0x00000000001ff7d0  RIP: 0x00000000001042a0
CR2: 0x0000000000000008  CR3: 0x0000000000100000  RFLAGS: 0x0000000000010202

STACK BACKTRACE:
  0: 0x00000000001042a0 - page_fault_handler
  1: 0x0000000000108f10 - handle_demand_paging
  2: 0x000000000010c300 - syscall_dispatch
================================================================================
```

---

## Verification

Diagnostic serialization and formatting logic are tested via `tests/diagnostics/kernel_diagnostics_test.rs` and `src/diagnostics/kernel_report.rs`.
