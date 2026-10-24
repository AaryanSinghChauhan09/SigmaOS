# Kernel Subsystem Dependency Graph & Verification

## Initialization Order Hierarchy

```
[Serial Early Console]
        │
        ▼
[Physical Allocator]
        │
        ▼
[Virtual Memory / Paging]
        │
        ▼
[Interrupt Controller / IDT]
        │
        ▼
[Scheduler Core]
        │
        ▼
[Process Manager / PCB Table]
        │
        ▼
[PCI / Driver Enumerator]
        │
        ▼
[Block Device Storage]
        │
        ▼
[Virtual File System (VFS)]
        │
        ▼
[Syscall Dispatcher & Capability Check]
        │
        ▼
[Userspace Init Launch]
```

---

## Strict Dependency Invariants

1. **Physical Allocator before Paging**: Paging structures must be allocated from physical frames.
2. **Paging before Process Allocation**: Process task structures and kernel stacks require mapped virtual memory.
3. **Interrupt Safety during Context Switch**: Critical scheduler state transitions must acquire spinlocks with interrupts disabled.
4. **VFS before Syscalls**: System calls handling file operations (`read`, `write`, `open`) require initialized VFS mounts.
