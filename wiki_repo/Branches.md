# SigmaOS Multi-Branch Architectural Grid & Parity Map (Branches.md)

SigmaOS implements a strict, 12-branch unified system architecture. The **Branch Uniformity & Synchronization Engine (S-BUSE)** programmatically guarantees that all branches remain identical in codebase structure while serving distinct system compilation targets.

---

## 📊 Branch Taxonomy & Compilation Targets

| Branch Name | Target System Archetype | Optimization Focus & Scheduler Pattern |
| :--- | :--- | :--- |
| **`main`** | Stable Production Launch | Standard balanced shard scheduler configuration. |
| **`release/standalone`** | Bare-Metal Desktop/Workstation | Focuses on direct CPU-bound execution and local peripheral integration. |
| **`release/rtos`** | Real-Time Embedded Systems | Enforces deterministic thread scheduling, high-precision timer limits, and zero-latency interrupts. |
| **`release/mobile`** | Energy-Aware Mobile Platforms | Implements dynamic voltage/frequency scaling and battery-friendly background task throttlers. |
| **`release/microkernel`** | Ultra-Minimal Computing | Bootstraps a skeletal 120-shard microkernel configuration for hyper-secure critical nodes. |
| **`release/dual-boot`** | Co-operative Monolithic Partitioning | Optimizes boot sector offsets to coexist alongside Windows or Linux bootloaders. |
| **`release/distributed`** | Cluster-Native Computing Nodes | Pre-configures direct remote procedure call (RPC) shard synchronization channels. |
| **`release/cloud`** | Headless Virtualization Servers | Optimizes memory pages for hypervisor hosting and multi-tenant memory sharing. |
| **`release/browser`** | In-Browser WebAssembly Runtime | Compiles core components to WebAssembly to execute in standard web browsers. |
| **`release/app`** | App-Store Sandbox Containers | Configures static container sandboxes with locked filesystem access profiles. |
| **`performance-optimized`** | Aggressively Vectorized Machines | Enables SIMD auto-vectorization (AVX-512/ARM Neon) at compile-time for max PQC throughput. |
| **`gh-pages`** | High-Performance Static Web | Serves the interactive desktop UI simulator, documentation, and live app installer guides. |

---

## 🔄 The S-BUSE Parity Pipeline

To prevent repository fragmentation and keep all 12 branches perfectly uniform:

1. All changes, bugfixes, and C++ modules are developed and committed onto `main`.
2. The Branch Uniformity & Synchronization Engine (`tools/sync_all_branches.py`) programmatically checks out each target branch, resets its workspace head to `main`, and force-pushes back to remote.
3. This guarantees bit-perfect uniformity and instant updates across all branches with zero merge conflicts!
 