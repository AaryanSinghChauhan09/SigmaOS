# SigmaOS Subsystem Dependencies

## Overview

SigmaOS defines strict initialization ordering and dependency topologies to prevent cyclic initialization and unhandled early-boot panics.

---

## Dependency Topology

```
+-------------------------------------------------------------+
|                      Firmware / Grub                        |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                      Physical Memory                        |
|                 (crate::memory::physical)                   |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                       Paging & VMM                          |
|                  (crate::mm::paging/virtual)                |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                  Interrupts & Timers                        |
|                  (APIC, IDT, PIC, Pit)                      |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                       Scheduler                             |
|                  (crate::scheduler::core)                   |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                     VFS & File Systems                      |
|                  (crate::fs / crate::storage)               |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                 Drivers & Network Stack                     |
|            (crate::drivers / crate::net)                    |
+-------------------------------------------------------------+
                               |
                               v
+-------------------------------------------------------------+
|                Userspace Init & Shell                       |
|               (crate::init / crate::shell)                  |
+-------------------------------------------------------------+
```

---

## Subsystem Dependency Matrix

| Subsystem | Depends On | Dependent Subsystems | Boot Level |
|---|---|---|---|
| Physical Allocator | Memory Map | Paging, VMM | Early (0) |
| Paging | Physical Allocator | VMM, Kernel Heap | Early (1) |
| Interrupts & IDT | Paging | Timers, Drivers | Early (2) |
| Scheduler | Interrupts, Paging | Processes, Syscalls | Kernel Init (3) |
| VFS | Physical Allocator, Locks | Block Storage, Mounts | System Init (4) |
| Syscall Dispatcher | Scheduler, Capability Policy | Userspace Binaries | System Init (5) |
| Userspace Shell | VFS, Syscall Dispatcher | None | Userspace (6) |
