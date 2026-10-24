# SigmaOS Subsystem Dependencies Map

## Dependency Hierarchy

```text
firmware / bootloader
   └─> arch_init (CPU & Traps)
        ├─> physical_memory_allocator
        │    └─> paging_virtual_memory
        │         ├─> kernel_heap_allocator
        │         └─> process_address_space (MMU / Page Tables)
        ├─> interrupt_controller (APIC / GIC)
        │    └─> timer_subsystem
        │         └─> scheduler_core
        │              └─> process_lifecycle & IPC
        └─> vfs_core
             ├─> ramfs_sysfs_procfs
             └─> block_device_layer
                  └─> storage_drivers & filesystems
```

## Critical Initialization Dependencies

1. **Memory Subsystem**: Depends on `arch_init` for memory map discovery and physical frame initialization.
2. **Interrupt & Timer Subsystems**: Depend on CPU descriptor tables and APIC/GIC configuration.
3. **Scheduler**: Depends on timer interrupts for time-slicing and page-table management for address space switching.
4. **VFS & Filesystems**: Depend on physical/virtual memory management and driver layer for block access.
5. **Syscall Boundary**: Depends on trap entry points, process table, capability verifier, and signal subsystem.
