
# Σ SigmaOS — Architecture Deep Dive



## Sovereign Lattice Overview


```
┌─────────────────────────────────────────────────────────┐
│                   Zenith UI Dashboard                   │
│  (sigma-auth-guard → telemetry panels via SigmaAPI)     │
└────────────────────────┬────────────────────────────────┘
                         │ HTTP / WebSocket events
┌────────────────────────▼────────────────────────────────┐
│                   S03_Orchestrator                      │
│  PersistenceOps trait  │  sigma_ipc.c  │  CRDT LWW     │
│  Monotonic seq IDs     │  Rollback FFI │  Replication   │
└────────┬───────────────┴───────────────┴────────────────┘
         │ capability-checked IPC channels
┌────────▼───────────────┬──────────────────┬────────────┐
│   S07_Scheduling       │  S04_HAL         │ S05_Memory │
│   CPU ↔ NPU dispatch   │  Exception vecs  │ DMA alloc  │
│   Tensor batching      │  MMIO drivers    │ Page tables│
└────────────────────────┴──────────────────┴────────────┘
         │ silicon boundary
┌────────▼───────────────────────────────────────────────┐
│   Bare-Metal Hardware: AArch64 / RISC-V                 │
│   Raspberry Pi 4 (BCM2711)  │  SiFive HiFive            │
└────────────────────────────────────────────────────────┘
```

---


## IPC + Persistence Flow


Every IPC message is **durably persisted before delivery**, preventing message loss on crash:

```
ipc_send(channel_id, msg)
    │
    ├── 1. Acquire monotonic seq_id (Lamport clock increment)
    ├── 2. Build SigmaIPCMsg slot (sender, type, payload)
    ├── 3. TRANSACTION START:
    │       persistence_write_ffi(seq_id, shard_id, msg)
    │           ├── OK  → commit msg to ring queue → K_OK
    │           └── ERR → persistence_rollback_ffi(seq_id)
    │                     return K_ERR_NODEV (delivery denied)
    └── 4. sched_wake_task(receiver_id)
```

On restart, the Orchestrator replays messages in ascending `seq_id` order — guaranteeing exactly-once, in-order delivery after a crash.

---


## DMA + Cache Coherency


```
sigma_dma_alloc(size, &paddr)
    ├── 1. First-fit search in 16MB DMA pool (page-aligned)
    ├── 2. AArch64: dc civac (clean+invalidate) entire range
    ├── 3. dsb sy  (data synchronization barrier)
    └── 4. Return (vaddr → CPU, paddr → NPU registers)
```

**Physical Memory Map (Raspberry Pi 4 / BCM2711)**

| Region | Physical Base | Size | Purpose |
|--------|--------------|------|---------|
| IPC Channels | `0x1000_0000` | 4 MB | Shard message mailboxes |
| DMA Coherent Pool | `0x2000_0000` | 16 MB | NPU tensor buffers |
| Kernel Text/Data | `0x8000_0000` | 32 MB | Kernel binary |
| V3D GPU MMIO | `0xC000_0000` | 4 KB | Broadcom VideoCore regs |

---


## Exception Handling Flow


```
Hardware Fault (e.g. Data Abort / DMA Violation)
    │
    ▼
AArch64 Vector Table (VBAR_EL1) ─── save x0–x30 onto stack
    │
    ▼
aarch64_exception_router(type)
    │ mrs esr_el1 → decode Exception Class [31:26]
    ├── 0x24 / 0x25  Data Abort  → log DMA/memory violation
    ├── 0x20 / 0x21  Instr Abort → log page fault
    └── 0x00         Unknown     → log illegal instruction
    │
    ▼
sched_kill_current_task(ec)
    ├── mark offending shard → TASK_DEAD
    └── sched_yield() → next healthy shard resumes
```

---


## Formal Verification Status


| Property | Tool | Status |
|----------|------|--------|
| DMA ∩ IPC = ∅ (non-interference) | Kani | ✅ Proved |
| Dispatch requires capability ownership | Kani | ✅ Proved |
| Sequence numbers are monotonic | Kani | ✅ Proved |
| Rollback removes exactly one entry | Kani | ✅ Proved |
| NPU matmul never panics | Kani | ✅ Proved |
| IPC channel disjointness | Coq | 🔶 Sketch |
| CRDT merge convergence | Isabelle | 🔶 Sketch |


## Sovereign Native Toolchain


To minimize dependencies on high-level runtimes (Python, Node), SigmaOS utilizes a **Native C++ Build Orchestrator** (`scripts/orchestrator`).


### Design Principles

1. **Zero-Dependency**: Written in pure C++20 using only standard headers. No external libraries (e.g., `nlohmann/json`) are permitted.
2. **Silicon-Native**: Compiled to a native binary for the host architecture, ensuring maximum build performance and "sovereignty".
3. **Lattice-Aware**: Understands `module.json` metadata and performs topological sorting for correct dependency resolution.


### Build Lifecycle

1. **Discovery**: Recursively scans `modules/` and `suites/` for `module.json`.
2. **Resolution**: Generates a dependency graph and calculates the linear build order.
3. **Incremental Synthesis**: Compiles only modified shards by comparing file timestamps.
4. **Final Linkage**: Orchestrates `ld` with target-specific linker scripts to produce the final `sigmaos.bin`.

See [`scripts/orchestrator.cpp`](../scripts/orchestrator.cpp) for implementation details.
