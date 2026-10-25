# Kernel Subsystem Dependency & Initialization Order

## Initialization Sequence Graph

```
Level 0 (Platform Bring-up)
  ├── Early UART Serial Driver
  ├── GDT / IDT Setup
  └── Physical Frame Allocator (`src/memory/physical.rs`)
        │
        ▼
Level 1 (Memory & Paging)
  ├── 4-Level Page Table Paging Init (`src/mm/paging.rs`)
  └── Kernel Heap Allocator
        │
        ▼
Level 2 (Core Kernel Services)
  ├── Timer Subsystem (RTC / APIC Timer)
  ├── Process & PID Allocator (`src/process/pid.rs`)
  └── Red-Black CFS Scheduler (`src/scheduler/core.rs`)
        │
        ▼
Level 3 (I/O & Storage Subsystems)
  ├── VirtIO Block Device Manager (`src/storage/block.rs`)
  ├── Block Cache Manager (`src/fs/cache.rs`)
  └── VFS Core & Mount Manager (`src/filesystem/core.rs`)
        │
        ▼
Level 4 (IPC & Network)
  ├── Capability Policy Engine (`src/userland/security_sandbox.rs`)
  ├── Unix Sockets & Pipes (`src/kernel/tier2_usability_stack.rs`)
  └── Loopback & Network Stack
        │
        ▼
Level 5 (Userland Handoff)
  └── PID 1 Init Spawn (`/sbin/init` or `/bin/sh`)
```

---

## Boot Order Rules
1. No subsystem at Level N may depend on an uninitialized subsystem at Level N+1.
2. Failure at Level 0–2 triggers kernel panic with diagnostic dump.
3. Failure at Level 3–4 degrades functionality gracefully or drops to emergency recovery shell.
