# SigmaOS Subsystem Dependencies Map

## Dependency Hierarchy

```text
1. Firmware (UEFI / BIOS)
   └─► 2. Bootloader (boot/arch init)
         └─► 3. Kernel Entry & Early CPU Init
               ├─► 4. Memory Allocators & Paging (src/memory, src/mm)
               ├─► 5. Interrupt & Exception Handlers (src/interrupt)
               ├─► 6. Scheduler & Process Control (src/scheduler, src/process)
               ├─► 7. VFS & Filesystem Mounts (src/fs, src/filesystem)
               ├─► 8. Driver Probe & Device Initialization (src/drivers, src/storage)
               ├─► 9. IPC, Signals, Timers (src/ipc, src/signal, src/timer)
               └─► 10. Userspace Init Launch -> 11. Interactive Shell Ready
```

## Subsystem Isolation Rules

1. Paging and physical memory managers must not depend on high-level VFS or process structures.
2. Interrupt handlers must be interrupt-safe and must not invoke blocking locks or dynamic allocation unless explicitly permitted.
3. Drivers interface with generic kernel subsystems (VFS, block layer, network stack) through typed abstractions.
