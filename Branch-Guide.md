# SigmaOS Branch Guide

SigmaOS implements a strict **12-branch unified architecture**. Every branch compiles the same sovereign codebase but targets a distinct hardware archetype, deployment model, or optimization profile.

> **S-BUSE Rule**: All feature development flows through `main`. The Branch Uniformity & Synchronization Engine (`tools/sync_all_branches.js`) propagates changes to all `release/*` branches automatically — ensuring zero merge conflicts and bit-perfect uniformity.

---

## 🗺️ Branch Overview Map

| Branch | Archetype | Scheduler Model | Optimization Target | Status |
| :--- | :--- | :--- | :--- | :--- |
| `main` | Stable Production | Balanced CFS | General-purpose, stable | ✅ Active |
| `release/standalone` | Bare-Metal Desktop | CPU-bound CFS | Local peripherals, high IOPS | 🔨 Dev |
| `release/rtos` | Real-Time Embedded | SCHED_SOVEREIGN RT | Zero-latency, deterministic | 🔨 Dev |
| `release/mobile` | Energy-Aware Mobile | DVFS-aware CFS | ARM64, battery life | 🔨 Dev |
| `release/microkernel` | Ultra-Minimal | Lock-free SPSC IPC | 120-shard hyper-secure | ✅ Test-verified |
| `release/dual-boot` | Co-operative Boot | Balanced CFS | GRUB chain-load, rollback | 🔨 Dev |
| `release/distributed` | Cluster-Native | RPC shard sync | CloudFS, container orchestration | 🔨 Dev |
| `release/cloud` | Headless Virtualization | Hypervisor-aware CFS | Multi-tenant memory pages | 🔨 Dev |
| `release/browser` | WebAssembly Runtime | WASM event loop | Sandboxed browser apps | 🔨 Dev |
| `release/app` | App-Store Sandbox | Static container CFS | Locked FS, pro tool suite | 🔨 Dev |
| `performance-optimized` | SIMD-Tuned | AVX-512 / ARM Neon | Max PQC throughput | 🔬 Experimental |
| `gh-pages` | Static Web Portal | N/A | Interactive demo, docs | ✅ Live |

---

## 🌿 Per-Branch Deep Dive

---

### `main` — Stable Production

**Purpose**: The source-of-truth branch. All feature development, bugfixes, and documentation flows through `main` first. Represents the most stable, well-tested configuration of SigmaOS.

**Kernel Config**:
- Standard balanced CFS scheduler with MLFQ fallback
- Full 600-shard lattice enabled
- All subsystems active: VFS, NetStack, PQC, Zenith UI

**Current Gaps & Plan**:
- Stabilize scattered experimental modules
- Unify HAL across all three ISAs
- Sync `/docs/` with Wiki via `wiki-sync.yml`
- Add comprehensive CI/CD coverage

**Who pushes here**: Maintainers only (via reviewed PRs from feature branches).

---

### `release/standalone` — Bare-Metal Desktop

**Purpose**: Targets direct bare-metal installation on desktop and workstation hardware. Configured for maximum CPU-bound execution throughput and full local peripheral integration.

**Kernel Config**:
- Direct CPU-bound execution tuning
- PS/2, USB HID, ATA disk, VGA drivers active
- Full Zenith Desktop UI with Vulkan compositor
- Local RegistryManager for user preference persistence

**Current Gaps & Plan**:
- Harden bootloader (GRUB/Limine chain-load)
- Complete RegistryManager persistence layer
- Full bare-metal init sequence (`/init/` Runlevel 1→5)
- ATA hot-swap driver shard

---

### `release/rtos` — Real-Time Embedded Systems

**Purpose**: Deploys SigmaOS on safety-critical real-time embedded hardware. Enforces deterministic thread scheduling with microsecond-precision timer guarantees.

**Kernel Config**:
- `SCHED_SOVEREIGN` real-time class (EDF + priority inheritance)
- High-resolution timer (HPET) with sub-100μs interrupt latency
- Zero-latency IPC via lock-free SPSC queues
- Stripped UI layer — headless by default

**Current Gaps & Plan**:
- Implement `SCHED_SOVEREIGN` RT scheduling class
- Add priority inheritance mutex protocol
- Formal verification of interrupt latency bounds
- Watchdog shard for crash recovery

---

### `release/mobile` — Energy-Aware Mobile

**Purpose**: Adapts SigmaOS for mobile and tablet platforms. Implements DVFS (Dynamic Voltage/Frequency Scaling) and battery-aware background task throttling.

**Kernel Config**:
- DVFS governor with ARM64 frequency domains
- Touch-optimized Zenith UI (gesture layer)
- ARM64 HAL tuned for Cortex-A series
- Background task throttler with battery-state awareness

**Current Gaps & Plan**:
- Power governor shard (`sigma_power_gov.c`)
- Touch input driver (multi-touch capacitive)
- ARM64 performance counter integration
- Low-power suspend/resume state machine

---

### `release/microkernel` — Ultra-Minimal (120-Shard)

**Purpose**: Bootstraps a skeletal 120-shard configuration for hyper-secure, resource-constrained critical nodes. Strips all non-essential shards to the absolute minimum viable kernel.

**Kernel Config**:
- 120 active shards (vs full 600)
- Lock-free SPSC message-passing IPC only
- Modular driver loading (no statically-linked drivers)
- Zero-copy IPC for all inter-process communication
- Formal micro-isolation guarantees

**Current Status**: ✅ Test-verified — 82 tests passing on this branch.

**Current Gaps & Plan**:
- Optimize IPC throughput (target < 500ns round-trip)
- Modular driver hot-loading framework
- Formal proof-of-isolation for shard boundaries

---

### `release/dual-boot` — Co-operative Dual-Boot

**Purpose**: Configures SigmaOS to coexist alongside Windows or Linux bootloaders via GRUB/Limine chain-loading. Optimizes boot sector offsets and partition table handling.

**Kernel Config**:
- GRUB2 + Limine chain-loading support
- Shared partition table parser (FAT32/NTFS read access for handoff)
- Rollback snapshot integration on boot failure
- Windows PE detection and graceful co-existence

**Current Gaps & Plan**:
- GRUB/Limine chain-load module
- Snapshot-on-boot rollback mechanism
- Windows NTFS partition read driver
- Boot menu with graphical SigmaOS/Windows/Linux selector

---

### `release/distributed` — Cluster-Native Computing

**Purpose**: Configures SigmaOS nodes for cluster computing. Pre-configures RPC shard synchronization channels and the SovereignCloudFS distributed filesystem.

**Kernel Config**:
- RPC shard sync over UDP multicast
- SovereignCloudFS (distributed block layer)
- Container orchestration shard
- Consensus protocol (Raft-inspired) for cluster state

**Current Gaps & Plan**:
- SovereignCloudFS implementation
- Distributed CFS scheduler with cross-node load balancing
- Container orchestration shard (SovereignKube spec)

---

### `release/cloud` — Headless Virtualization

**Purpose**: Deploys SigmaOS as a headless hypervisor host. Optimizes memory pages for multi-tenant sharing and virtual machine isolation.

**Kernel Config**:
- Hypervisor-aware CFS (Xen/KVM paravirt hooks)
- Immutable OS tree (read-only root + overlay layers)
- Multi-tenant memory sharing with hardware isolation
- No graphical subsystem (headless by design)

**Current Gaps & Plan**:
- Xen/KVM paravirt driver shards
- Immutable OS tree implementation (OverlayFS-based)
- SovereignCluster resource manager

---

### `release/browser` — WebAssembly Runtime

**Purpose**: Compiles core SigmaOS components to WebAssembly for execution inside standard web browsers. Enables an in-browser OS simulation with sandboxed apps.

**Kernel Config**:
- WASM compilation targets for core shards
- Sandboxed WASM app execution environment
- GPU acceleration via WebGL/WebGPU
- Lightweight DOM-based Zenith UI renderer

**Current Gaps & Plan**:
- WASM build pipeline for kernel shards
- WebGPU-accelerated canvas compositor
- WASM app sandbox specification

---

### `release/app` — App-Store Sandbox Containers

**Purpose**: Configures static container sandboxes with locked filesystem access profiles for App-Store style distribution of SigmaOS applications.

**Kernel Config**:
- Static container sandboxes with locked FS profiles
- Professional tool suite fully integrated (GST, forensics, BIS calculators)
- App signing with PQC attestation
- Namespace isolation for each sandboxed app

**Current Gaps & Plan**:
- Professional tools: GST, court fees, BIS, forensic CLI tools
- App signing pipeline with Dilithium-5
- Container runtime specification

---

### `performance-optimized` — Aggressively Vectorized

**Purpose**: Enables SIMD auto-vectorization (AVX-512 on x86_64, ARM Neon on ARM64) at compile-time for maximum PQC throughput and memory bandwidth.

**Kernel Config**:
- AVX-512 / ARM Neon auto-vectorization flags
- Clear Linux–inspired aggressive compiler optimization
- Adaptive O(1) slab allocator with SIMD-accelerated memcpy
- Profile-guided optimization (PGO) builds

**Current Gaps & Plan**:
- PGO (Profile-Guided Optimization) build pipeline
- AVX-512 accelerated Dilithium-5 PQC path
- SIMD-optimized slab allocator `memcpy`

---

### `gh-pages` — Static Web Portal

**Purpose**: Hosts the interactive SigmaOS desktop UI simulator, documentation portal, and live app installer guides as a GitHub Pages static site.

**Content**:
- Interactive Zenith Desktop simulator (`zenith.html`)
- App Store showcase (`app_store.html`)
- Installer guide (`installer.html`)
- Roadmap visualization (`roadmap.html`)

**Current Status**: ✅ Live at `https://aaryansinghchauhan09.github.io/SigmaOS/`

**Current Gaps & Plan**:
- Contributor portal with onboarding flow
- Interactive subsystem diagram explorer
- Live branch status dashboard

---

## 🔄 S-BUSE Synchronization Pipeline

```
1.  Developer opens a feature branch from main
          │
          ▼
2.  Code review + CI passes on feature branch
          │
          ▼
3.  PR merged to main
          │
          ▼
4.  S-BUSE (tools/sync_all_branches.js) triggers
          │
    ┌─────┴──────────────────────────────────┐
    ▼       ▼       ▼       ▼       ▼       ▼
release/ release/ release/ release/ perf-  gh-pages
standalone rtos   mobile  cloud   optim.
    └─────┬──────────────────────────────────┘
          │
          ▼
5.  All 12 branches updated — zero merge conflicts
```

---

> *Last updated: 2026-05-23 · SigmaOS Zenith v15.2 [ZENITH-SINGULARITY]*
