# SigmaOS Subsystem Dependencies Matrix

## Dependency Topology

```text
                  +-----------------------+
                  | Physical Memory (PMM) |
                  +-----------+-----------+
                              |
                              v
                  +-----------+-----------+
                  |  Virtual Memory (VMM) |
                  +-----------+-----------+
                              |
                              v
                  +-----------+-----------+
                  |   Kernel Scheduler    |
                  +-----------+-----------+
                              |
            +-----------------+-----------------+
            |                                   |
            v                                   v
+-----------+-----------+           +-----------+-----------+
| Direct VFS / Storage  |           | Process / Syscall IPC |
+-----------------------+           +-----------------------+
```

## Ordering Rules
1. **Stage 0**: Physical Allocator initialization.
2. **Stage 1**: Page Table setup and VMM address space isolation.
3. **Stage 2**: Interrupt Vector Table (IDT/APIC) & Timer enabling.
4. **Stage 3**: Scheduler core & Task PCB creation.
5. **Stage 4**: VFS root mount & device driver probing.
6. **Stage 5**: Userland init execution.
