# SigmaOS Multi-Branch Architectural Grid & Parity Map

SigmaOS implements a strict, **12-branch unified system architecture**. The **Branch Uniformity & Synchronization Engine (S-BUSE)** programmatically guarantees that all branches remain identical in codebase structure while serving distinct system compilation targets.

For the full per-branch guide with deep dives, current gaps, and improvement plans, see **[Branch-Guide](Branch-Guide)**.

---

## 📊 Branch Taxonomy & Compilation Targets

| Branch Name | Target System Archetype | Optimization Focus | Status |
| :--- | :--- | :--- | :--- |
| **`main`** | Stable Production Launch | Standard balanced shard scheduler configuration | ✅ Active |
| **`release/standalone`** | Bare-Metal Desktop/Workstation | Direct CPU-bound execution and local peripheral integration | 🔨 Dev |
| **`release/rtos`** | Real-Time Embedded Systems | Deterministic thread scheduling, high-precision timers, zero-latency interrupts | 🔨 Dev |
| **`release/mobile`** | Energy-Aware Mobile Platforms | Dynamic voltage/frequency scaling and battery-friendly background throttling | 🔨 Dev |
| **`release/microkernel`** | Ultra-Minimal Computing | 120-shard microkernel for hyper-secure critical nodes | ✅ Test-verified |
| **`release/dual-boot`** | Co-operative Monolithic Partitioning | Boot sector offsets to coexist alongside Windows or Linux bootloaders | 🔨 Dev |
| **`release/distributed`** | Cluster-Native Computing Nodes | Direct remote procedure call (RPC) shard synchronization channels | 🔨 Dev |
| **`release/cloud`** | Headless Virtualization Servers | Memory pages optimized for hypervisor hosting and multi-tenant sharing | 🔨 Dev |
| **`release/browser`** | In-Browser WebAssembly Runtime | Core components compiled to WASM for execution in web browsers | 🔨 Dev |
| **`release/app`** | App-Store Sandbox Containers | Static container sandboxes with locked filesystem access profiles | 🔨 Dev |
| **`performance-optimized`** | Aggressively Vectorized Machines | SIMD auto-vectorization (AVX-512/ARM Neon) for max PQC throughput | 🔬 Experimental |
| **`gh-pages`** | High-Performance Static Web | Interactive desktop UI simulator, documentation, live installer guides | ✅ Live |

---

## 🔄 The S-BUSE Parity Pipeline

To prevent repository fragmentation and keep all 12 branches perfectly uniform:

1. All changes, bugfixes, and C++ modules are developed and committed onto `main`.
2. The Branch Uniformity & Synchronization Engine (`tools/sync_all_branches.js`) programmatically checks out each target branch, merges from `main`, and pushes back to remote origin.
3. This guarantees bit-perfect uniformity and instant updates across all branches with zero merge conflicts.

---

## 🗺️ Per-Branch Gap & Improvement Summary

| Branch | Primary Gap | Improvement Plan |
| :--- | :--- | :--- |
| `main` | Scattered experimental modules | Stabilise kernel, unify HAL, sync /docs/ with Wiki, add CI/CD |
| `release/standalone` | Minimal OS incomplete | Harden bootloader, RegistryManager, bare-metal init sequence |
| `release/rtos` | No deterministic scheduling | `SCHED_SOVEREIGN` RT class, priority inheritance, lock-free IPC |
| `release/mobile` | No energy-aware features | Power governor, touch Zenith UI, ARM64 HAL tuning |
| `release/microkernel` | IPC not optimised | Lock-free SPSC message-passing, modular drivers, zero-copy IPC |
| `release/dual-boot` | Bootloader integration missing | GRUB/Limine chain-loading, rollback snapshot integration |
| `release/distributed` | No cluster FS | SovereignCloudFS, distributed scheduler, container orchestration |
| `release/cloud` | No container-native support | CoreOS/RancherOS ideas, SovereignCluster, immutable OS tree |
| `release/browser` | Browser-centric OS absent | Lightweight Chromium fork, sandboxed WASM apps, GPU acceleration |
| `release/app` | No professional tools | GST, court fees, BIS, forensic CLI tools fully integrated |
| `performance-optimized` | No tuned builds | Clear Linux–style flags, SIMD, adaptive O(1) slab allocator |
| `gh-pages` | Docs scattered | Contributor portal, interactive desktop demos, subsystem guides |

---

## 📖 See Also

- [Branch Guide](Branch-Guide) — Full per-branch deep dive with kernel configs and gap analysis
- [Architecture Overview](Architecture-Overview) — Shard map, HAL, and Ring-0/3 dispatch pipeline
- [Contributor Guidelines](Contributor-Guidelines) — Branch strategy and PR process for contributors

> *Last updated: 2026-05-23 · SigmaOS Zenith v15.2 [ZENITH-SINGULARITY]*
