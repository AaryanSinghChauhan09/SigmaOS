# SigmaOS Kernel Subsystem Dependencies

## Boot Topological Order

To achieve deterministic initialization without circular dependencies or race conditions, kernel subsystems must initialize in the following strict sequential order:

```
1. Physical Memory Allocator (frame allocation)
   └── 2. Paging & Virtual Memory Manager (4-level page tables, kernel mappings)
       └── 3. Interrupt & APIC Controller (IDT, GDT, LAPIC/IOAPIC)
           └── 4. System Timer & Clock (PIT / HPET / LAPIC Timer)
               └── 5. Process & Scheduler Engine (PCB, Run Queues, Context Switch)
                   └── 6. VFS & Storage Block Layer (FD table, Inode Manager, RAMFS)
                       └── 7. IPC & Signal Delivery Engine
                           └── 8. Capabilities & Security Engine
                               └── 9. Network Protocol Stack
                                   └── 10. Userland Init Process Launch
```

## Failure Propagation Matrix

| Subsystem | Failure Behavior | System Response |
| :--- | :--- | :--- |
| **Physical Allocator / Paging** | Allocation panic / Page Fault in kernel space | Trigger Kernel Panic Report with CR2 & register dump |
| **Scheduler** | Run queue corruption / Lost wakeups | Starvation watchdog triggers CPU reset or panic diagnostic |
| **VFS / Block Layer** | Disk I/O error or corruption | Isolate mount point / return POSIX EIO / Read-only remount |
| **User Process / Syscall** | Invalid memory access / Capability violation | Deliver SIGSEGV / SIGSYS to target process |
| **Swap Space** | Swap disk full / I/O timeout | Invoke OOM Killer to reclaim physical memory |
