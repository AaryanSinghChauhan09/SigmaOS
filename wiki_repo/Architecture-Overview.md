# SigmaOS Architecture Overview

This page describes the high-level structure of the SigmaOS Zenith microkernel.

---

## Ring Architecture


```
┌─────────────────────────────────────────────────────┐
│  Ring-3 (Userland)                                  │
│  sigma-sh | sigma-forensics | Zenith Desktop UI     │
└─────────────────────┬───────────────────────────────┘
                      │ syscall / SYSRET
┌─────────────────────▼───────────────────────────────┐
│  SyscallDispatcher  (256-slot O(1) C table)         │
└─────────────────────┬───────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────┐
│  Ring-0 (Kernel Lattice)                            │
│  Scheduler | Allocator | VFS | IPC | PQC Engine     │
└─────────────────────┬───────────────────────────────┘
                      │
┌─────────────────────▼───────────────────────────────┐
│  S-HAL (Hardware Abstraction Layer)                 │
│  x86_64 APIC | ARM64 GIC | RISC-V PLIC/CLINT       │
└─────────────────────┬───────────────────────────────┘
                      │
              Physical Hardware


```

---

## Key Subsystems

| Subsystem | File | Purpose | 
| :--- | :--- | :--- | 
| **S-HAL** | `hal/SovereignHAL.cpp` | Platform-agnostic register access | 
| **Scheduler** | `kernel/scheduler/SovereignScheduler.cpp` | CFS + NUMA + SCHED_SOVEREIGN | 
| **Allocator** | `kernel/core/SovereignAllocator.cpp` | O(1) lockless slab | 
| **SPSC IPC** | `kernel/core/ipc/SovereignSPSCQueue.hpp` | Zero-copy ring buffers | 
| **Syscalls** | `kernel/core/SovereignSyscall.cpp` | Modular C dispatch table | 
| **VFS** | `kernel/core/SovereignVFS.cpp` | ZFS-inspired virtual FS | 
| **Vulkan** | `kernel/core/vulkan/sovereign_vulkan.c` | Direct SPIR-V GPU routing | 
| **UI** | `kernel/core/SovereignZenithUI.cpp` | Glassmorphic compositor | 
| **PQC** | `kernel/core/SovereignPQC.cpp` | Dilithium-5 attestation | 

---

## Boot Sequence

1. `SovereignHAL::initializeHAL()` — CPU arch detection, MMIO mapping
2. `SovereignAllocator::init()` — Slab bucket setup
3. `syscall_init()` — Dispatch table population
4. `sigma_scheduler_numa_balance()` — NUMA shard pinning
5. `SovereignVFS::mount()` — Root filesystem mount
6. `svk_init()` — GPU command queue reset
7. Drop to Ring-3, launch `sigma-sh`
