# System Logic & Architecture Relationships

This document defines the complete file and directory structural mapping for the SigmaOS Zenith microkernel lattice.

---

## Ring Architecture Overview

```
Ring-3 (Userland)  ──▶  sigma-sh / sigma-forensics / Zenith Desktop
        │                        │
        ▼                        ▼
    SyscallDispatcher (kernel/core/SovereignSyscall.cpp)
        │
        ▼
Ring-0 (Kernel)  ──▶  SovereignScheduler / SovereignAllocator / SovereignVFS
        │
        ▼
    S-HAL (hal/SovereignHAL.cpp)
        │
        ▼
Hardware (x86_64 / ARM64 / RISC-V)
```

---

## Core File Relationships

| File/Directory | Role | Depends On | Used By | 
| :--- | :--- | :--- | :--- | 
| `include/sigma_kernel_types.h` | Primitive types, intrinsics | None | **Everything** | 
| `include/sigma_log.h` | Kernel logging macros | `sigma_kernel_types.h` | All shards | 
| `hal/SovereignHAL.hpp/.cpp` | Hardware abstraction | `sigma_kernel_types.h` | Drivers, Scheduler, Boot | 
| `kernel/core/SovereignSyscall.cpp` | Modular syscall dispatch table | `sigma_kernel_types.h`, `sigma_log.h` | Userland, Shell | 
| `kernel/scheduler/SovereignScheduler.cpp` | CFS + NUMA + RT scheduling | `sigma_kernel_types.h`, HAL | Kernel init, IPC | 
| `kernel/core/SovereignAllocator.cpp` | O(1) Slab allocator | `sigma_kernel_types.h` | All shards needing heap | 
| `kernel/core/ipc/SovereignSPSCQueue.hpp` | Lock-free SPSC IPC | `sigma_kernel_types.h` | Scheduler, Drivers | 
| `kernel/core/SovereignVFS.cpp` | Virtual File System | HAL, Allocator | Storage, Shell | 
| `kernel/core/vulkan/sovereign_vulkan.c` | Direct SPIR-V GPU routing | HAL MMIO macros | Zenith Desktop UI | 
| `kernel/core/SovereignZenithUI.cpp` | Glassmorphic compositor | Vulkan layer | Desktop environment | 
| `kernel/core/SovereignPQC.cpp` | Post-quantum cryptography | `sigma_kernel_types.h` | Boot attestation, IPC | 

---

## Dependency Rules (Preventing Recursive Loops)

1. `sigma_kernel_types.h` **must never** include any other project header.
2. `sigma_log.h` may only include `sigma_kernel_types.h`.
3. HAL headers may include `sigma_kernel_types.h` but **not** scheduler or VFS headers.
4. Scheduler may include HAL but **not** VFS or Vulkan headers.
5. VFS may include HAL and Allocator but **not** the Scheduler directly.
6. UI layer may include Vulkan and VFS but **not** low-level scheduler types.

---

## Subsystem Boot Initialization Order

1. `SovereignHAL::initializeHAL()` — detects CPU arch, sets MMIO maps
2. `SovereignAllocator::init()` — establishes slab buckets
3. `syscall_init()` — populates syscall dispatch table
4. `sigma_scheduler_numa_balance()` — pins shards to NUMA nodes
5. `SovereignVFS::mount()` — mounts root filesystem
6. `svk_init()` — resets GPU command queue
7. `sigma_sh` userland launch — drops to Ring-3
