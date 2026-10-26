# SigmaOS Kernel Subsystem Dependency Graph

```text
                        +----------------------+
                        |       Firmware       |
                        +----------+-----------+
                                   |
                                   v
                        +----------------------+
                        |      Bootloader      |
                        +----------+-----------+
                                   |
                                   v
                        +----------------------+
                        |  Physical Allocator  |
                        +----------+-----------+
                                   |
                                   v
                        +----------------------+
                        |  Paging / VM Manager |
                        +----------+-----------+
                                   |
                 +-----------------+-----------------+
                 |                                   |
                 v                                   v
    +-------------------------+         +-------------------------+
    |  Interrupt Controller   |         |   Block Device Manager  |
    +------------+------------+         +------------+------------+
                 |                                   |
                 v                                   v
    +-------------------------+         +-------------------------+
    | Scheduler & Run Queue   |         | VFS & Mount Hierarchy   |
    +------------+------------+         +------------+------------+
                 |                                   |
                 +-----------------+-----------------+
                                   |
                                   v
                        +----------------------+
                        |  Syscall Dispatcher  |
                        +----------+-----------+
                                   |
                                   v
                        +----------------------+
                        |   Userspace / Shell  |
                        +----------------------+
```

## Dependency Rules

1. Subsystems lower in the dependency graph must not invoke public APIs of subsystems above them (strict acyclic dependency hierarchy).
2. Memory managers must initialize before any heap allocation or device probing occurs.
3. Interrupt safety must be established before scheduler preemption is enabled.
