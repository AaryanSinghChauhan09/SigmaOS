# Core Shards Specification (Layer 0 & 1)

This specification defines the low-level system modules residing in Layer 0 (HAL & Boot) and Layer 1 (Kernel Core) of the Sovereign Lattice architecture, as declared in the `SHARDS.manifest`.

---

## 🧱 Layer 0: HAL & Boot Shards

Layer 0 modules execute in Ring 0 with raw hardware privileges. They abstract physical CPU registers and board layouts into clean interfaces for the rest of the kernel.

### 1. Shard Manifest Directory

| Shard Path | Component Class | Responsibilities |
| :--- | :--- | :--- |
| `kernel/core/boot/bootloader_shard.cpp` | Stage 2 Bootloader | Sets up 64-bit long mode, paging tables, parses memory map |
| `kernel/core/boot/SovereignInit.cpp` | BSP Init | Initializes Symmetric Multiprocessing (SMP) and local APICs |
| `kernel/core/hal/SovereignHAL.cpp` | Hardware Abstraction | High-level interface to interrupt lines, timers, I/O ports |
| `kernel/core/hal/SovereignPMM.cpp` | Buddy Allocator | Page-granularity physical memory allocator |
| `kernel/core/hal/SovereignVMM.cpp` | Page Directory Builder | Configures PML4 tables and virtual memory protection rings |

---

## ⚙️ Layer 1: Kernel Core Shards

Layer 1 shards handle task scheduler structures, virtual filesystem nodes, IPC message buses, and security capability namespaces.

### 1. Shard Manifest Directory

| Shard Path | Subsystem | Responsibilities |
| :--- | :--- | :--- |
| `kernel/core/system/SovereignScheduler.cpp` | Scheduler Core | Manages thread queues and executes context swaps |
| `kernel/core/system/SovereignFairSched.cpp` | EEVDF Engine | Tracks virtual runtimes and calculates deadlines |
| `kernel/core/fs/SovereignLatticeFS.cpp` | Virtual File System | Extends base VFS with mount nodes and file descriptors |
| `kernel/core/network/SovereignNetStack.cpp`| Net Core | Buffers network packet segments and routes to interfaces |
| `kernel/core/security/SovereignMAC.cpp` | Access Control | Evaluates security capabilities and isolates system handles |

---

## 🔄 Core Initialization Sequence

```
[bootloader_shard]  ──► Load Kernel ELF into high memory
                            │
                            ▼
[SovereignInit]      ──► Parse ACPI Tables, start AP cores (SMP)
                            │
                            ▼
[SovereignPMM]       ──► Enumerate pages and initialize Buddy Allocator
                            │
                            ▼
[SovereignVMM]       ──► Map kernel memory space and userland boundary
                            │
                            ▼
[SovereignScheduler] ──► Start CPU runqueues, spin EEVDF timer thread
                            │
                            ▼
[Launch PID 1]       ──► Exec userland initialization daemon (sigmad)
```
