# Subsystem Dependency Hierarchy & Policies

## Subsystem Dependency Graph

The boot-time dependency graph for SigmaOS kernel initialization is strictly acyclic:

```text
[firmware]
    │
    ▼
[bootloader / uefi_entry]
    │
    ▼
[arch_init / gdt_idt_pic]
    │
    ▼
[physical_memory_allocator]
    │
    ▼
[paging_page_tables]
    │
    ▼
[virtual_memory_manager]
    │
    ▼
[interrupt_controller_apic]
    │
    ▼
[timer_subsystem]
    │
    ▼
[scheduler_run_queue]
    │
    ▼
[vfs_inode_rootfs]
    │
    ▼
[driver_subsystem]
    │
    ▼
[userspace_init / shell]
```

---

## Initialization Order & Rules

1. **Boot Dependencies (Critical):**
   - Boot-critical subsystems (`physical_memory`, `paging`, `interrupts`, `scheduler`, `vfs`) MUST pass initialization before driver loading or userspace startup.
   - If any boot-critical subsystem fails during entry, the kernel invokes `panic_with_diagnostic()` with full register state and halts execution safely.

2. **Non-Critical Dependencies (Modular):**
   - Secondary drivers, network interfaces, and userland services are non-critical.
   - Failure during non-critical initialization results in subsystem isolation, error reporting, and degraded operation rather than kernel panic.

3. **Cyclic Dependency Guard:**
   - Circular references between subsystems (e.g., VFS calling Scheduler while Scheduler locks VFS) are explicitly prohibited. Locking orders across subsystems must strictly adhere to `VirtualMemory -> Paging -> PhysicalMemory -> Swap`.
