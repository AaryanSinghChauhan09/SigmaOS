# SigmaOS Kernel Subsystem Dependency Map

## Overview
This document models the explicit dependency graph across SigmaOS kernel subsystems, preventing cyclic initialization dependencies and defining boot-time load order.

---

## Subsystem Dependency Map

```text
+-----------------------+
|  Firmware / Bootloader |
+-----------+-----------+
            |
            v
+-----------+-----------+
| Physical Memory Mgr   |
+-----------+-----------+
            |
            v
+-----------+-----------+
| Paging & Page Tables  |
+-----------+-----------+
            |
            v
+-----------+-----------+
| Virtual Memory Manager|
+-----------+-----------+
            |
            +-----------------------+
            |                       |
            v                       v
+-----------+-----------+   +-------+---------------+
| Interrupts & Timers   |   | Block Storage Devices |
+-----------+-----------+   +-------+---------------+
            |                       |
            v                       v
+-----------+-----------+   +-------+---------------+
| Process & Scheduler   |   | Swap Subsystem        |
+-----------+-----------+   +-----------------------+
            |
            v
+-----------+-----------+
| VFS & Mount Manager   |
+-----------+-----------+
            |
            v
+-----------+-----------+
| Syscall Dispatcher    |
+-----------+-----------+
            |
            v
+-----------+-----------+
| Userspace Init / Shell|
+-----------------------+
```

---

## Initialization Order Verification Rules

1. Memory Subsystems must complete before interrupt handlers register dynamic buffers.
2. Timers must initialize before `scheduler_init`.
3. Block Storage devices must probe before VFS mounts root filesystem or swap initializes.
4. VFS must initialize before Syscall Dispatch accepts userland file i/o requests.
5. Userspace Init / Shell is launched as PID 1 after all boot-critical subsystems report ready.
