# SigmaOS Kernel Subsystem Dependency Map

```
Firmware / Bootloader
        │
        ▼
Physical Allocator
        │
        ▼
Page Table & Virtual Memory
        │
        ├──────────────────────┐
        ▼                      ▼
Process Manager           Storage Block Layer
        │                      │
        ▼                      │
Syscall Dispatcher             │
        │                      │
        └──────────┬───────────┘
                   ▼
         VFS & Filesystem Core
                   │
                   ▼
         Drivers & Devices
                   │
                   ▼
         Userspace Start & Shell
```

## Initialization Order Verification

Subsystem contracts and initialization ordering are verified programmatically via `tests/kernel/integration/subsystem_contracts_test.rs`.
