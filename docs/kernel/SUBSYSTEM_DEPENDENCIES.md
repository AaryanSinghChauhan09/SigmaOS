# SigmaOS Kernel Subsystem Dependencies & Initialization Graph

## Hierarchy & Boot Sequence
Subsystem initialization follows a strict directed acyclic graph (DAG). Cyclic dependencies between kernel modules are prohibited by automated architecture checks.

```
+------------------------+
| 1. Firmware & ACPI     |
+-----------+------------+
            |
            v
+------------------------+
| 2. GDT / IDT & Traps   |
+-----------+------------+
            |
            v
+------------------------+
| 3. Physical Allocator  |
+-----------+------------+
            |
            v
+------------------------+
| 4. Virtual Memory/Page |
+-----------+------------+
            |
            v
+------------------------+
| 5. APIC & Timers       |
+-----------+------------+
            |
            v
+------------------------+
| 6. Scheduler Core      |
+-----------+------------+
            |
            v
+------------------------+
| 7. Virtual File System |
+-----------+------------+
            |
            v
+------------------------+
| 8. Device Drivers      |
+-----------+------------+
            |
            v
+------------------------+
| 9. Userspace Init (PID1|
+------------------------+
```

---

## Dependency Rules
1. **Memory Before Scheduler**: The scheduler must not allocate thread control blocks or run-queue buckets before the virtual memory manager has mapped heap and stack regions.
2. **Interrupt Safety**: Interrupts must remain disabled until the interrupt descriptor table (IDT) and APIC controllers are completely initialized.
3. **Driver Isolation**: Drivers interact with hardware solely via defined MMIO or I/O port abstractions granted by the architecture subsystem.
