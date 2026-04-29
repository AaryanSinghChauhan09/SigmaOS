# SigmaOS — Architecture Overview

> A ground-truth description of the current SigmaOS architecture as implemented in the repository.

---

## 🏗️ Layer Diagram

```
┌─────────────────────────────────────────────────────────┐
│                  CLI ORCHESTRATOR                        │
│         orchestrator/main.cpp (OOP ICommand)            │
│  profile | build | test | benchmark | forge | link       │
└─────────────────┬───────────────────────────────────────┘
                  │ dispatches to
┌─────────────────▼───────────────────────────────────────┐
│              CORE OOP INTERFACES                         │
│         sigmaos/core/src/atomic_*.cpp/.hpp               │
│  ISigmaModule | ISigmaDriver | ICallback | IProcess      │
│  IRQDispatcher | WorkStealPool | SovereignVFS            │
└────┬──────────────┬────────────────┬────────────────────┘
     │              │                │
┌────▼────┐  ┌──────▼──────┐  ┌─────▼──────┐
│ KERNEL  │  │    HAL      │  │  SECURITY  │
│ S01     │  │   S04       │  │   S08      │
│ Genesis │  │ Hardware    │  │  Shards    │
└────┬────┘  └──────┬──────┘  └─────┬──────┘
     │              │                │
┌────▼────────────────────────────────▼─────┐
│         SUBSYSTEM MODULES                  │
│  IPC: S42  │  Perf: S28  │  Caps: S43     │
│  Pkg: S36  │  NUMA: S30  │  BPF:  S36     │
│  ImmFS:S31 │  CGroup:S27 │  ZKP:  S37     │
│  Journal:S46│ Rollback:S41│ Wire: S37     │
└───────────────────────────────────────────┘
```

---

## 📁 Repository Structure (Ground Truth)

```
SigmaOS/
│
├── orchestrator/
│   └── main.cpp              ← OOP CLI: ICommand → CommandDispatcher
│
├── sigmaos/core/src/
│   ├── atomic_sigma_oop_base.hpp   ← ISigmaModule, ISigmaDriver, ICallback
│   ├── atomic_sigma_process.hpp    ← IProcess lifecycle
│   ├── atomic_hal_irq.hpp          ← IRQ dispatcher
│   ├── atomic_sigma_alloc.cpp      ← Custom arena allocator
│   ├── atomic_sigma_sched.cpp      ← Inline-ASM context switch
│   ├── atomic_sigma_crypto.cpp     ← SIMD crypto parser
│   ├── atomic_sigma_net_oop.cpp    ← OOP NIC driver
│   ├── atomic_sigma_vfs_oop.cpp    ← OOP VFS + user I/O hooks
│   ├── atomic_sigma_auto_oop.cpp   ← Automation hook manager
│   ├── atomic_sec_*.cpp            ← Security shards (audit/encrypt/fw)
│   ├── atomic_mem_*.cpp            ← Memory shards (trace/prune)
│   ├── atomic_perf_*.cpp           ← Performance shards
│   ├── atomic_ui_*.cpp             ← UI shards (init/morph)
│   └── sigma_core.h                ← C-ABI registry
│
├── suites/
│   ├── S01_Genesis/          ← Kernel core: alloc, VMM, scheduler, proc, net
│   ├── S04_HAL/              ← Hardware drivers: NVMe, USB, IRQ, VGA, DMA
│   ├── S08_Security/         ← PQC, zero-trust, sandbox, audit, ZKP
│   ├── S27_ContainerLattice/ ← CGroup resource control
│   ├── S28_PerformanceLattice/ ← Work-stealing, LRU cache
│   ├── S30_NeuralPaging/     ← NUMA-aware allocator
│   ├── S31_ImmutableFS/      ← A/B slot immutable FS
│   ├── S36_SovereignBPF/     ← Programmable filter chain
│   ├── S36_SovereignPackageRegistry/ ← Package manager
│   ├── S37_ZeroKnowledgeProofLayer/  ← ZKP / Fiat-Shamir
│   ├── S37_SovereignWire/    ← Netfilter / packet firewall
│   ├── S41_SiliconBoot/      ← Auto-rollback, boot snapshots
│   ├── S42_RawIPC/           ← Ring buffer, async I/O (io_uring style)
│   ├── S43_SovereignCaps/    ← Capability token system
│   ├── S46_SovereignJournal/ ← Write-ahead log / journaling
│   └── S<50-83>_*/           ← Application-layer and extended shards
│
├── .github/workflows/
│   ├── 01_Sovereign_Build.yml      ← Build × 9 matrix jobs
│   ├── 02_Lattice_Verification.yml ← cppcheck + Kani + fuzzing
│   ├── 03_Web_Zenith.yml           ← Quality gate + sovereignty audit
│   └── 04_Sigma_Dev_Coverage.yml   ← Module audit + OOP/ASM count
│
├── README.md                ← USP showcase + CLI reference + comparison table
└── WIKI/                    ← Git submodule → SigmaOS.wiki.git
    ├── Architecture_Overview.md
    ├── Kernel_Module_Reference.md
    ├── Developer_Guide.md
    ├── Contribution_Guide.md
    ├── CI_Pipeline_Guide.md
    └── Competitor_Analysis.md
```

---

## 🔑 Core Design Principles

| Principle | Implementation |
|-----------|----------------|
| One file = one function | Every `sigma_*.h` has one primary concern |
| Zero stdlib | No `<stdio.h>`, `<stdlib.h>`, `<string.h>` in kernel modules |
| Sovereign types | `sigma_u32`, `sigma_u8`, `sigma_size_t` — no stdint.h |
| OOP modular | Abstract base: `ISigmaModule`, `ISigmaDriver`, `ICallback`, `IProcess` |
| Inline ASM | Spinlock, scheduler, RDTSC, crypto, spawn — direct hardware |
| Capability-gated | All privileged ops require `SigmaCapToken` with `SIGMA_CAP_ADMIN` |

---

## ⚠️ Monoliths Being Actively Split

| Legacy File | Status | New Atomic Modules |
|-------------|--------|--------------------|
| `SovereignProcessManager.c` | 🔄 Partially split | `sigma_proc_pcb.h`, `sigma_proc_spawn.h`, `sigma_proc_kill.h` |
| `SovereignMemoryZenith.c` | 🔄 Partially split | `sigma_mem_pool.h`, `sigma_mem_audit.h` |
| `SovereignNetMesh.c` | 🔄 Partially split | `sigma_net_core.h` |
| `SovereignLibC.c` | ✅ Finalized | `sigma_io.c`, `sigma_string.c`, `sigma_mem.c` |
| `MODULAR_ARCHITECTURE_BLUEPRINT` | ✅ Finalized | `CORE`, `ESSENTIAL`, `OPTIONAL`, `THIRD_PARTY`, `INFINITE` |
| `SovereignShardKernel.c` | ⏳ Queued | `sigma_shard_init.h`, `sigma_shard_load.h` |
| `sigma_pqc.c` | ⏳ Queued | `sigma_pqc_keygen.h`, `sigma_pqc_sign.h`, `sigma_pqc_verify.h` |

---

## 📊 Current Stats

| Metric | Value |
|--------|-------|
| Total sovereign modules | 77+ |
| Core atomic `.cpp/.hpp` | 24 |
| Suite kernel headers | 53+ |
| CI workflows | 4 |
| Inline ASM modules | 6+ |
| OOP abstract interfaces | 5 |
| Zero stdlib violations | ✅ 0 in atomic core |

---

## 🧠 Core Algorithms

### 🌀 Quantum Scheduler: Predictive Priority Weighting (PPW)
The **Quantum Scheduler (S39)** utilizes the **PPW Algorithm** to manage task execution across the 600-shard lattice. 
- **Algorithm**: `Priority = Max(1, Floor(Complexity + LoadFactor))`
- **Goal**: Prevent priority inversion and ensure that high-complexity background tasks don't starve the real-time UI flow.
- **Implementation**: JS Shard `S39_QuantumScheduler.js`


