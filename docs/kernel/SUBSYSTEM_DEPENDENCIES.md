# SigmaOS Subsystem Dependencies Graph

```
                   +-------------------+
                   | PhysicalAllocator |
                   +---------+---------+
                             |
                             v
                       +-----+-----+
                       |  Paging   |
                       +--+-----+--+
                          |     |
            +-------------+     +-------------+
            |                                 |
            v                                 v
  +---------+-------+               +---------+-------+
  | ProcessManager  |               |  StorageBlock   |
  +---------+-------+               +---------+-------+
            |                                 |
            v                                 |
  +---------+-------+                         |
  | SyscallDispatcher |                        |
  +---------+-------+                         |
            |                                 |
            +-----------------+---------------+
                              |
                              v
                        +-----+-----+
                        |    Vfs    |
                        +-----------+
```

## Topological Boot Sequence

1. `PhysicalAllocator`
2. `Paging`
3. `ProcessManager` & `StorageBlock` (Parallel / Independent)
4. `SyscallDispatcher` (Requires `Paging` + `ProcessManager`)
5. `Vfs` (Requires `StorageBlock` + `SyscallDispatcher`)
