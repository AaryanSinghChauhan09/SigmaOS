# SigmaOS Kernel Diagnostics Protocol

## Overview
Kernel diagnostics provide structured, greppable, and stable diagnostic reporting during boot initialization and kernel panics.

## Boot Diagnostic Reports
Every critical kernel subsystem reports initialization status using structured events:

```text
[BOOT_DIAGNOSTIC] stage=memory_init status=OK details="Allocated 16384 MB physical memory frames"
[BOOT_DIAGNOSTIC] stage=vfs_init status=OK details="Mounted root ramfs, procfs, sysfs"
[BOOT_DIAGNOSTIC] stage=shell_ready status=OK details="Interactive shell ready for input"
```

## Panic Diagnostic Format
In the event of an unrecoverable kernel panic, the diagnostic engine formats panic reports containing component, cause, register context, and stack backtrace:

```text
================================================================================
KERNEL PANIC DIAGNOSTIC REPORT
================================================================================
Timestamp   : 1729810000
Component   : kernel::mm::paging
Operation   : map_page_range
Error Code  : PAGE_FAULT_WRITE_PROTECTED (0x0002)
CPU Register State:
  RAX: 0x0000000000000000  RBX: 0xffff800000102000  RCX: 0x0000000000000001
  RDX: 0x0000000000000000  RSI: 0xffff800000001000  RDI: 0x0000000000002000
  RBP: 0xffff80000000fff0  RSP: 0xffff80000000ffe0  RIP: 0xffffffff80100420
Diagnostic  : Attempted write to read-only kernel page frame.
================================================================================
